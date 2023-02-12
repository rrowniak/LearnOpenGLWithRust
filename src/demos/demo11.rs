use super::common::*;
use crate::demos::Demo;
use crate::gfx::camera::{CamMovement, Camera};
use crate::gfx::lights::{MaterialTex, MaterialTexMap, PointLight, VSMatrices};
use crate::gfx::{glutils::*, shaders::Shaders, system, system::IoEvents, utils::*};
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

const KEY_UP: u8 = 0x01;
const KEY_DOWN: u8 = 0x02;
const KEY_LEFT: u8 = 0x04;
const KEY_RIGHT: u8 = 0x08;

pub struct DemoImpl {
    lamp_shader: Shaders,
    cube_shader: Shaders,
    cube_mat: MaterialTex,
    light: PointLight,
    cube_sm_shader: Shaders,
    cube_sm_mat: MaterialTexMap,
    cubes: NormTexCubeObj,
    mvp: VSMatrices,
    timer: Instant,
    first_logic_pass: bool,
    camera: Camera,
    io_flags: BitFields<u8>,
    texture: u32,
    texture_specular_map: u32,
}

impl DemoImpl {
    fn new() -> Self {
        DemoImpl {
            lamp_shader: Shaders::default(),
            cube_shader: Shaders::default(),
            cube_mat: MaterialTex::default(),
            light: PointLight::default(),
            cube_sm_shader: Shaders::default(),
            cube_sm_mat: MaterialTexMap::default(),
            cubes: NormTexCubeObj::default(),
            mvp: VSMatrices::default(),
            timer: Instant::now(),
            first_logic_pass: true,
            camera: Camera::new(),
            io_flags: BitFields::<u8>::default(),
            texture: 0,
            texture_specular_map: 0,
        }
    }
    fn init(&mut self, system: &system::System) -> Result<(), String> {
        self.lamp_shader =
            Shaders::from_files(&system.gl, "./demo/demo7_lig.vs", "./demo/demo7_lig.fs")?;

        self.cube_shader = Shaders::from_files(
            &system.gl,
            "./demo/demo9_box.vs",
            "./demo/demo11_box_point.fs",
        )?;

        self.cube_sm_shader = Shaders::from_files(
            &system.gl,
            "./demo/demo9_box.vs",
            "./demo/demo11_box_specular_map_point.fs",
        )?;

        self.texture = load_texture(&system.gl, "./demo/container2.png")?;
        self.texture_specular_map = load_texture(&system.gl, "./demo/container2_specular.png")?;

        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        self.cubes = NormTexCubeObj::from(&system.gl, DEFAULT_POS_NORM_TEX_CUBE_VERT)?;
        self.cubes.add_another_cube(&system.gl);
        self.cubes.add_another_cube(&system.gl);
        // add 10 more
        for _ in 0..10 {
            self.cubes.add_another_cube(&system.gl);
        }

        self.light.position = Vec3::new(1.2, 1.0, 2.0);
        self.light.ambient = Vec3::new(0.2, 0.2, 0.2);
        self.light.diffuse = Vec3::new(0.5, 0.5, 0.5);
        self.light.specular = Vec3::new(1.0, 1.0, 1.0);

        self.cube_mat.diffuse = 0;
        self.cube_mat.specular = Vec3::new(0.5, 0.5, 0.5);
        self.cube_mat.shininess = 32.0;

        self.cube_sm_mat.diffuse = 0;
        self.cube_sm_mat.specular = 1;
        self.cube_sm_mat.shininess = 32.0;

        Ok(())
    }

    fn update_logic(&mut self, system: &system::System) -> Result<(), String> {
        if self.timer.elapsed().as_millis() > 10 || self.first_logic_pass {
            self.first_logic_pass = false;
            self.timer = Instant::now();

            // process io
            for io in system.events.iter() {
                match io {
                    IoEvents::MouseMotion(_, _, dx, dy) => {
                        self.camera
                            .process_mouse_movement(*dx as f32, *dy as f32, false);
                    }
                    IoEvents::KeyDown(key_code) => match *key_code {
                        system::KEY_DOWN => self.io_flags.set(KEY_DOWN),
                        system::KEY_UP => self.io_flags.set(KEY_UP),
                        system::KEY_LEFT => self.io_flags.set(KEY_LEFT),
                        system::KEY_RIGHT => self.io_flags.set(KEY_RIGHT),
                        _ => {}
                    },
                    IoEvents::KeyUp(key_code) => match *key_code {
                        system::KEY_DOWN => self.io_flags.unset(KEY_DOWN),
                        system::KEY_UP => self.io_flags.unset(KEY_UP),
                        system::KEY_LEFT => self.io_flags.unset(KEY_LEFT),
                        system::KEY_RIGHT => self.io_flags.unset(KEY_RIGHT),
                        _ => {}
                    },
                    IoEvents::MouseWheel(_, dy) => {
                        self.camera.process_mouse_scroll(*dy as f32);
                        self.build_projection_matrix(system, self.camera.zoom.to_radians());
                    }
                    _ => {}
                }
            }

            let delta_t = 0.05;

            if self.io_flags.is_set(KEY_UP) {
                self.camera.process_keyboard(CamMovement::Forward, delta_t)
            }
            if self.io_flags.is_set(KEY_DOWN) {
                self.camera.process_keyboard(CamMovement::Backward, delta_t)
            }
            if self.io_flags.is_set(KEY_LEFT) {
                self.camera.process_keyboard(CamMovement::Left, delta_t)
            }
            if self.io_flags.is_set(KEY_RIGHT) {
                self.camera.process_keyboard(CamMovement::Right, delta_t)
            }
        }
        Ok(())
    }

    const CUBE_POSITIONS: [Vec3; 10] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new(2.4, -0.4, -3.5),
        Vec3::new(-1.7, 3.0, -7.5),
        Vec3::new(1.3, -2.0, -2.5),
        Vec3::new(1.5, 2.0, -2.5),
        Vec3::new(1.5, 0.2, -1.5),
        Vec3::new(-1.3, 1.0, -1.5),
    ];

    fn render(&mut self, system: &system::System) -> Result<(), String> {
        // draw the cube object
        self.cube_shader.use_program(&system.gl);
        self.cube_mat.pass_uniforms(&system.gl, &self.cube_shader);
        self.light.pass_uniforms(&system.gl, &self.cube_shader);

        self.cube_shader.set_vec3(
            &system.gl,
            "viewPos",
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z,
        );

        self.mvp.view = self.camera.get_view_matrix();
        self.mvp.model = Mat4::default();
        self.mvp.pass_uniforms(&system.gl, &self.cube_shader);

        unsafe {
            system.gl.ActiveTexture(gl33::GL_TEXTURE0);
            system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.texture);
        }
        self.cubes.draw(&system.gl, 0);

        // draw the cube object with specular map
        self.cube_sm_shader.use_program(&system.gl);
        self.cube_sm_mat
            .pass_uniforms(&system.gl, &self.cube_sm_shader);
        self.light.pass_uniforms(&system.gl, &self.cube_sm_shader);

        self.cube_sm_shader.set_vec3(
            &system.gl,
            "viewPos",
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z,
        );

        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(1.1, 0.0, 0.0));
        self.mvp.pass_uniforms(&system.gl, &self.cube_sm_shader);
        unsafe {
            system.gl.ActiveTexture(gl33::GL_TEXTURE0);
            system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.texture);

            system.gl.ActiveTexture(gl33::GL_TEXTURE1);
            system
                .gl
                .BindTexture(gl33::GL_TEXTURE_2D, self.texture_specular_map);
        }
        self.cubes.draw(&system.gl, 2);

        for i in 0..10 {
            self.mvp.model = Mat4::default();
            let angle = 10.0 * (i as f32);
            self.mvp.model =
                Mat4::from_rotation_around(Vec4::new(1.0, 0.3, 0.5, 0.0), angle.to_radians());
            self.mvp.model.translate(&Self::CUBE_POSITIONS[i]);
            self.mvp.pass_uniforms(&system.gl, &self.cube_sm_shader);
            unsafe {
                system.gl.ActiveTexture(gl33::GL_TEXTURE0);
                system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.texture);

                system.gl.ActiveTexture(gl33::GL_TEXTURE1);
                system
                    .gl
                    .BindTexture(gl33::GL_TEXTURE_2D, self.texture_specular_map);
            }
            self.cubes.draw(&system.gl, i + 3);
        }

        // draw the lamp object
        self.lamp_shader.use_program(&system.gl);
        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(1.2, 1.0, 2.0));
        self.mvp.model = self.mvp.model * Mat4::from_scale(0.2);
        self.mvp.pass_uniforms(&system.gl, &self.lamp_shader);
        self.cubes.draw(&system.gl, 1);

        Ok(())
    }

    fn build_projection_matrix(&mut self, system: &system::System, fov_rad: f32) {
        self.mvp.projection = projection::rh_yup::perspective_gl(
            fov_rad,
            (system.w as f32) / (system.h as f32),
            0.1,
            100.0,
        );
    }
}
