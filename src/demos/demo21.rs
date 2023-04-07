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

                check_gl_err(&system.gl);
                demo.render(&system)?;
                check_gl_err(&system.gl);
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
    // scene objects
    cube: ModelWrapT,
    tex_wood: u32,
    // depth map frame buffer
    depth_map_fbo: u32,
    depth_cube_map: u32,
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
            // scene objects
            tex_wood: 0,
            cube: ModelWrapT::default(),
            // depth map fram buffer
            depth_map_fbo: 0,
            depth_cube_map: 0,
            light_pos: Vec3::new(0.0, 0.0, 0.0),
            shadow_width: 1024,
            shadow_height: 1024,
        }
    }

    fn init(&mut self, system: &system::System) -> Result<(), String> {
        unsafe {
            system.gl.Enable(gl33::GL_DEPTH_TEST);
            system.gl.Enable(gl33::GL_CULL_FACE);
        }
        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        // init shaders
        let path = "./demo/point_shadows";
        self.shader = Shaders::from_files(
            &system.gl,
            &format!("{}/point_shadows.vs", path),
            &format!("{}/point_shadows.fs", path),
        )?;
        self.simple_depth_shader = Shaders::from_files_full(
            &system.gl,
            &format!("{}/point_shadows_depth.vs", path),
            &format!("{}/point_shadows_depth.fs", path),
            &format!("{}/point_shadows_depth.gs", path),
        )?;

        self.shader.use_program(&system.gl);
        self.shader.set_i32(&system.gl, "diffuseTexture", 0);
        self.shader.set_i32(&system.gl, "depthMap", 1);

        // init scene

        self.cube = ModelWrapT::Some(Box::new(setup_model_box(CUBE_VERTICES)));
        self.cube.as_mut().unwrap().setup(&system.gl)?;

        self.tex_wood = load_texture(&system.gl, "./demo/wood.png")?;

        // init depth map fbo
        self.init_depth_map_fbo(&system.gl);
        check_gl_err(&system.gl);
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
        let far_plane = 25.0;
        let shadow_proj = projection::rh_yup::perspective_gl(
            90.0_f32.to_radians(),
            self.shadow_width as f32 / self.shadow_height as f32,
            near_plane,
            far_plane,
        );
        let shadow_transforms = vec![
            shadow_proj
                * Mat4::look_at(
                    self.light_pos,
                    self.light_pos + Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(0.0, -1.0, 0.0),
                ),
            shadow_proj
                * Mat4::look_at(
                    self.light_pos,
                    self.light_pos + Vec3::new(-1.0, 0.0, 0.0),
                    Vec3::new(0.0, -1.0, 0.0),
                ),
            shadow_proj
                * Mat4::look_at(
                    self.light_pos,
                    self.light_pos + Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ),
            shadow_proj
                * Mat4::look_at(
                    self.light_pos,
                    self.light_pos + Vec3::new(0.0, -1.0, 0.0),
                    Vec3::new(0.0, 0.0, -1.0),
                ),
            shadow_proj
                * Mat4::look_at(
                    self.light_pos,
                    self.light_pos + Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(0.0, -1.0, 0.0),
                ),
            shadow_proj
                * Mat4::look_at(
                    self.light_pos,
                    self.light_pos + Vec3::new(0.0, 0.0, -1.0),
                    Vec3::new(0.0, -1.0, 0.0),
                ),
        ];
        // render scene to depth cubemap
        self.simple_depth_shader.use_program(&system.gl);
        self.simple_depth_shader
            .set_f32(&system.gl, "far_plane", far_plane);
        self.simple_depth_shader.set_vec3(
            &system.gl,
            "lightPos",
            self.light_pos.x,
            self.light_pos.y,
            self.light_pos.z,
        );
        for (i, st) in shadow_transforms.iter().enumerate().take(6) {
            self.simple_depth_shader.set_mat4fv_uv(
                &system.gl,
                &format!("shadowMatrices[{}]", i),
                st,
            );
        }
        unsafe {
            system
                .gl
                .Viewport(0, 0, self.shadow_width, self.shadow_height);
            system
                .gl
                .BindFramebuffer(gl33::GL_FRAMEBUFFER, self.depth_map_fbo);
            system.gl.Clear(gl33::GL_DEPTH_BUFFER_BIT);
            // system.gl.ActiveTexture(gl33::GL_TEXTURE0);
            // system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_wood);
        }

        self.render_scene(system, &self.simple_depth_shader.clone());

        unsafe {
            system.gl.BindFramebuffer(gl33::GL_FRAMEBUFFER, 0);
            system.gl.Viewport(0, 0, system.w as i32, system.h as i32);
            system
                .gl
                .Clear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
        }

        check_gl_err(&system.gl);
        // render scene as normal
        self.shader.use_program(&system.gl);
        self.shader.set_vec3(
            &system.gl,
            "lightPos",
            self.light_pos.x,
            self.light_pos.y,
            self.light_pos.x,
        );
        self.shader.set_f32(&system.gl, "far_plane", far_plane);
        self.shader.set_i32(&system.gl, "shadows", 1);
        self.mvp.model = Mat4::default();
        self.mvp.pass_uniforms(&system.gl, &self.shader);

        check_gl_err(&system.gl);
        unsafe {
            system.gl.ActiveTexture(gl33::GL_TEXTURE0);
            system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_wood);
            system.gl.ActiveTexture(GL_TEXTURE1);
            system
                .gl
                .BindTexture(gl33::GL_TEXTURE_CUBE_MAP, self.depth_cube_map);
        }

        self.render_scene(system, &self.shader.clone());
        check_gl_err(&system.gl);

        Ok(())
    }

    fn render_scene(&mut self, system: &system::System, shader: &Shaders) {
        unsafe {
            system.gl.Disable(gl33::GL_CULL_FACE);
        }
        self.mvp.model = Mat4::from_scale(5.0);
        self.mvp.try_pass_uniforms(&system.gl, shader);
        shader.try_set_i32(&system.gl, "reverse_normals", 1);
        check_gl_err(&system.gl);
        self.render_cube(system, shader);
        check_gl_err(&system.gl);
        shader.try_set_i32(&system.gl, "reverse_normals", 0);
        unsafe {
            system.gl.Enable(gl33::GL_CULL_FACE);
        }
        // translate_vec, rot_angle, rot_vec, scale
        let trans: [(Vec3, f32, Vec4, f32); 5] = [
            (Vec3::new(4.0, -3.5, 0.0), 0.0, Vec4::default(), 0.5),
            (Vec3::new(2.0, 3.0, 1.0), 0.0, Vec4::default(), 0.75),
            (Vec3::new(-3.0, -1.0, 0.0), 0.0, Vec4::default(), 0.75),
            (Vec3::new(-1.5, 1.0, 1.5), 0.0, Vec4::default(), 0.5),
            (
                Vec3::new(-1.5, 2.0, -3.0),
                60.0,
                Vec4::new(1.0, 0.0, 1.0, 1.0),
                0.75,
            ),
        ];

        for t in trans {
            self.mvp.model = Mat4::default();
            self.mvp.model.translate(&t.0);
            self.mvp.model = self.mvp.model * Mat4::from_rotation_around(t.2, t.1.to_radians());
            if t.3 != 0.0 {
                self.mvp.model = self.mvp.model * Mat4::from_scale(t.3);
            }
            self.mvp.try_pass_uniforms(&system.gl, shader);
            self.render_cube(system, shader);
        }
    }

    fn render_cube(&mut self, system: &system::System, shader: &Shaders) {
        self.cube.as_mut().unwrap().draw(&system.gl, shader);
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
            gl.GenTextures(1, &mut self.depth_cube_map);
            gl.BindTexture(gl33::GL_TEXTURE_CUBE_MAP, self.depth_cube_map);
            let targets = [
                gl33::GL_TEXTURE_CUBE_MAP_POSITIVE_X, // right
                gl33::GL_TEXTURE_CUBE_MAP_NEGATIVE_X, // left
                gl33::GL_TEXTURE_CUBE_MAP_POSITIVE_Y, // top
                gl33::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, // bottom
                gl33::GL_TEXTURE_CUBE_MAP_POSITIVE_Z, // back
                gl33::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, // front
            ];
            for t in targets {
                // for i in 0..6 {
                gl.TexImage2D(
                    t,
                    0,
                    gl33::GL_DEPTH_COMPONENT.0 as i32,
                    self.shadow_width,
                    self.shadow_height,
                    0,
                    gl33::GL_DEPTH_COMPONENT,
                    gl33::GL_FLOAT,
                    std::ptr::null(),
                );
            }
            check_gl_err(gl);
            gl.TexParameteri(
                gl33::GL_TEXTURE_CUBE_MAP,
                gl33::GL_TEXTURE_MIN_FILTER,
                gl33::GL_NEAREST.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_CUBE_MAP,
                gl33::GL_TEXTURE_MAG_FILTER,
                gl33::GL_NEAREST.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_CUBE_MAP,
                gl33::GL_TEXTURE_WRAP_S,
                gl33::GL_CLAMP_TO_EDGE.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_CUBE_MAP,
                gl33::GL_TEXTURE_WRAP_T,
                gl33::GL_CLAMP_TO_EDGE.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_CUBE_MAP,
                gl33::GL_TEXTURE_WRAP_R,
                gl33::GL_CLAMP_TO_EDGE.0 as i32,
            );

            check_gl_err(gl);
            gl.BindFramebuffer(gl33::GL_FRAMEBUFFER, self.depth_map_fbo);
            check_gl_err(gl);
            gl.FramebufferTexture(
                gl33::GL_FRAMEBUFFER,
                gl33::GL_DEPTH_ATTACHMENT,
                self.depth_cube_map,
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
const CUBE_VERTICES: [f32; 288] = [
            // back ace
            -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-let
             1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
             1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 0.0, // bottom-right         
             1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
            -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-let
            -1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 1.0, // top-let
            // ront ace
            -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-let
             1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 0.0, // bottom-right
             1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
             1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
            -1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0, // top-let
            -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-let
            // let ace
            -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
            -1.0,  1.0, -1.0, -1.0,  0.0,  0.0, 1.0, 1.0, // top-let
            -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-let
            -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-let
            -1.0, -1.0,  1.0, -1.0,  0.0,  0.0, 0.0, 0.0, // bottom-right
            -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
            // right ace
             1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-let
             1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
             1.0,  1.0, -1.0,  1.0,  0.0,  0.0, 1.0, 1.0, // top-right         
             1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
             1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-let
             1.0, -1.0,  1.0,  1.0,  0.0,  0.0, 0.0, 0.0, // bottom-let     
            // bottom ace
            -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
             1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 1.0, 1.0, // top-let
             1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-let
             1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-let
            -1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 0.0, 0.0, // bottom-right
            -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
            // top ace
            -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-let
             1.0,  1.0 , 1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
             1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 1.0, 1.0, // top-right     
             1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
            -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-let
            -1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 0.0, 0.0  // bottom-let        
];
