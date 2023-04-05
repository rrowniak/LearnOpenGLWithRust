use super::common::*;
use crate::demos::Demo;
use crate::gfx::camera::Camera;
use crate::gfx::glutils::{check_gl_err, load_texture};
use crate::gfx::lights::VSMatrices;
use crate::gfx::models::Model;
use crate::gfx::shaders::Shaders;
use crate::gfx::system;
use gl33::*;
// use rand::Rng;
use std::time::Instant;
use ultraviolet::*;

pub struct DemoN {
    pub name: &'static str,
    pub description: &'static str,
}

impl_demo_trait!(DemoN);

impl DemoN {
    fn main(&self) -> Result<(), String> {
        let mut system = system::System::new(800, 600);

        let mut demo = DemoImpl::new();
        demo.init(&system)?;

        loop {
            if !system.process_io_events() {
                break;
            } else {
                // logic code here
                demo.update_logic(&system)?;
                // end logic code

                system.clear_screen(0.1, 0.1, 0.1);
                unsafe {
                    system.gl.Clear(gl33::GL_COLOR_BUFFER_BIT);
                }
                // graphics render here
                demo.render(&system)?;
                // end graphics render

                system.draw_to_screen();
            }
        }
        Ok(())
    }
}

type ModelWrapT = Option<Box<Model>>;

pub struct DemoImpl {
    mvp: VSMatrices,
    inputs: usr_inputs::Io,
    timer: Instant,
    first_logic_pass: bool,
    camera: Camera,
    // shaders
    shader: Shaders,
    simple_depth_shader: Shaders,
    debug_depth_shader: Shaders,
    // scene objects
    plane: ModelWrapT,
    cube: ModelWrapT,
    tex_wood: u32,
    // depth map frame buffer
    depth_map_fbo: u32,
    depth_map: u32,
    light_pos: Vec3,
    shadow_width: i32,
    shadow_height: i32,
}

impl DemoImpl {
    fn new() -> Self {
        DemoImpl {
            mvp: VSMatrices::default(),
            inputs: Default::default(),
            timer: Instant::now(),
            first_logic_pass: true,
            camera: Camera::new(),
            // shaders
            shader: Shaders::default(),
            simple_depth_shader: Shaders::default(),
            debug_depth_shader: Shaders::default(),
            // scene objects
            plane: ModelWrapT::default(),
            cube: ModelWrapT::default(),
            tex_wood: 0,
            // depth map fram buffer
            depth_map_fbo: 0,
            depth_map: 0,
            light_pos: Vec3::new(-2.0, 4.0, -1.0),
            shadow_width: 1024,
            shadow_height: 1024,
        }
    }

    fn init(&mut self, system: &system::System) -> Result<(), String> {
        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        // init shaders
        let path = "./demo/shadow_mapping";
        self.shader = Shaders::from_files(
            &system.gl,
            &format!("{}/shadow_mapping.vs", path),
            &format!("{}/shadow_mapping.fs", path),
        )?;
        self.simple_depth_shader = Shaders::from_files(
            &system.gl,
            &format!("{}/shadow_mapping_depth.vs", path),
            &format!("{}/shadow_mapping_depth.fs", path),
        )?;
        self.debug_depth_shader = Shaders::from_files(
            &system.gl,
            &format!("{}/debug_quad.vs", path),
            &format!("{}/debug_quad.fs", path),
        )?;

        self.shader.use_program(&system.gl);
        self.shader.set_i32(&system.gl, "diffuseTexture", 0);
        self.shader.set_i32(&system.gl, "shadowMap", 1);

        // init scene
        self.plane = ModelWrapT::Some(Box::new(setup_model_plane(PLANE_VERTICES)));
        self.plane.as_mut().unwrap().setup(&system.gl)?;

        self.cube = ModelWrapT::Some(Box::new(setup_model_box(DEFAULT_POS_NORM_TEX_CUBE_VERT)));
        self.cube.as_mut().unwrap().setup(&system.gl)?;

        self.tex_wood = load_texture(&system.gl, "./demo/wood.png")?;

        // init depth map fbo
        self.init_depth_map_fbo(&system.gl);
        Ok(())
    }

    fn update_logic(&mut self, system: &system::System) -> Result<(), String> {
        if self.timer.elapsed().as_millis() > 10 || self.first_logic_pass {
            self.first_logic_pass = false;
            self.timer = Instant::now();

            if self.inputs.process_io(&mut self.camera, system) {
                self.build_projection_matrix(system, self.camera.zoom);
            }
        }

        Ok(())
    }

    fn render(&mut self, system: &system::System) -> Result<(), String> {
        self.mvp.view = self.camera.get_view_matrix();

        check_gl_err(&system.gl);

        // render depth of scene to texture from light's perspective
        let near_plane = 1.0;
        let far_plane = 7.5;
        let light_projection =
            projection::rh_yup::orthographic_gl(-10.0, 10.0, -10.0, 10.0, near_plane, far_plane);
        let light_view = Mat4::look_at(
            self.light_pos,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let light_space_matrix = light_projection * light_view;
        // render
        self.simple_depth_shader.use_program(&system.gl);
        self.simple_depth_shader
            .set_mat4fv_uv(&system.gl, "lightSpaceMatrix", &light_space_matrix);
        self.simple_depth_shader
            .set_mat4fv_uv(&system.gl, "model", &Mat4::default());

        unsafe {
            system
                .gl
                .Viewport(0, 0, self.shadow_width, self.shadow_height);
            system
                .gl
                .BindFramebuffer(gl33::GL_FRAMEBUFFER, self.depth_map_fbo);
            system.gl.Clear(gl33::GL_DEPTH_BUFFER_BIT);
            system.gl.ActiveTexture(gl33::GL_TEXTURE0);
            system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_wood);
        }
        self.plane
            .as_mut()
            .unwrap()
            .draw(&system.gl, &self.simple_depth_shader);

        self.render_cubes(system, &self.simple_depth_shader.clone(), true);

        unsafe {
            system.gl.BindFramebuffer(gl33::GL_FRAMEBUFFER, 0);
            system.gl.Viewport(0, 0, system.w as i32, system.h as i32);
            system
                .gl
                .Clear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
        }

        // render scene as normal
        self.shader.use_program(&system.gl);
        self.shader.set_vec3(
            &system.gl,
            "lightPos",
            self.light_pos.x,
            self.light_pos.y,
            self.light_pos.x,
        );
        self.shader
            .set_mat4fv_uv(&system.gl, "lightSpaceMatrix", &light_space_matrix);
        self.mvp.model = Mat4::default();
        self.mvp.pass_uniforms(&system.gl, &self.shader);
        self.plane.as_mut().unwrap().draw(&system.gl, &self.shader);

        unsafe {
            system.gl.ActiveTexture(gl33::GL_TEXTURE0);
            system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_wood);
            system.gl.ActiveTexture(GL_TEXTURE1);
            system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.depth_map);
        }

        self.render_cubes(system, &self.shader.clone(), false);

        Ok(())
    }

    fn render_cubes(&mut self, system: &system::System, shader: &Shaders, primitive: bool) {
        // translate_vec, rot_angle, rot_vec, scale
        let trans: [(Vec3, f32, Vec4, f32); 3] = [
            (Vec3::new(0.0, 1.5, 0.0), 0.0, Vec4::default(), 0.5),
            (Vec3::new(2.0, 0.0, 1.0), 0.0, Vec4::default(), 0.5),
            (
                Vec3::new(-1.0, 0.0, 2.0),
                60.0,
                Vec4::new(1.0, 0.0, 1.0, 1.0),
                0.25,
            ),
        ];

        for t in trans {
            self.mvp.model = Mat4::default();
            self.mvp.model.translate(&t.0);
            self.mvp.model = self.mvp.model * Mat4::from_rotation_around(t.2, t.1.to_radians());
            if t.3 != 0.0 {
                self.mvp.model = self.mvp.model * Mat4::from_scale(t.3);
            }

            if primitive {
                shader.set_mat4fv_uv(&system.gl, "model", &self.mvp.model);
            } else {
                self.mvp.pass_uniforms(&system.gl, shader);
            }
            self.cube.as_mut().unwrap().draw(&system.gl, shader);
        }
    }

    fn build_projection_matrix(&mut self, system: &system::System, fov_rad: f32) {
        self.mvp.projection = projection::rh_yup::perspective_gl(
            fov_rad,
            (system.w as f32) / (system.h as f32),
            0.1,
            1000.0,
        );
    }

    fn init_depth_map_fbo(&mut self, gl: &GlFns) {
        unsafe {
            gl.GenFramebuffers(1, &mut self.depth_map_fbo);
            gl.GenTextures(1, &mut self.depth_map);
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.depth_map);
            check_gl_err(gl);
            gl.TexImage2D(
                gl33::GL_TEXTURE_2D,
                0,
                gl33::GL_DEPTH_COMPONENT.0 as i32,
                self.shadow_width,
                self.shadow_height,
                0,
                gl33::GL_DEPTH_COMPONENT,
                gl33::GL_FLOAT,
                std::ptr::null(),
            );
            check_gl_err(gl);
            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MIN_FILTER,
                gl33::GL_NEAREST.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MAG_FILTER,
                gl33::GL_NEAREST.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_WRAP_S,
                gl33::GL_CLAMP_TO_BORDER.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_WRAP_T,
                gl33::GL_CLAMP_TO_BORDER.0 as i32,
            );
            let colors = [1.0, 1.0, 1.0, 1.0];
            gl.TexParameterfv(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_BORDER_COLOR,
                colors.as_ptr(),
            );
            check_gl_err(gl);
            gl.BindFramebuffer(gl33::GL_FRAMEBUFFER, self.depth_map_fbo);
            check_gl_err(gl);
            gl.FramebufferTexture2D(
                gl33::GL_FRAMEBUFFER,
                gl33::GL_DEPTH_ATTACHMENT,
                gl33::GL_TEXTURE_2D,
                self.depth_map,
                0,
            );
            check_gl_err(gl);
            gl.DrawBuffer(gl33::GL_NONE);
            gl.ReadBuffer(gl33::GL_NONE);
            gl.BindFramebuffer(gl33::GL_FRAMEBUFFER, 0);
            check_gl_err(gl);
        }
    }
}

#[rustfmt::skip]
const PLANE_VERTICES: [f32; 48] = [
        // positions            // normals         // texcoords
         25.0, -0.5,  25.0,  0.0, 1.0, 0.0,  25.0,  0.0,
        -25.0, -0.5,  25.0,  0.0, 1.0, 0.0,   0.0,  0.0,
        -25.0, -0.5, -25.0,  0.0, 1.0, 0.0,   0.0, 25.0,

         25.0, -0.5,  25.0,  0.0, 1.0, 0.0,  25.0,  0.0,
        -25.0, -0.5, -25.0,  0.0, 1.0, 0.0,   0.0, 25.0,
         25.0, -0.5, -25.0,  0.0, 1.0, 0.0,  25.0, 25.0
];
