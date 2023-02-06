use super::common::*;
use crate::demos::Demo;
use crate::gfx::camera::{CamMovement, Camera};
use crate::gfx::shaders::{LightSolid, MaterialSolid};
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
    lighting_shader: Shaders,
    cube_shader: Shaders,
    cubes: NormTexCubeObj,
    light: LightSolid,
    material: MaterialSolid,
    projection: Mat4,
    timer: Instant,
    first_logic_pass: bool,
    camera: Camera,
    io_flags: BitFields<u8>,
}

impl DemoImpl {
    fn new() -> Self {
        DemoImpl {
            lighting_shader: Shaders::default(),
            cube_shader: Shaders::default(),
            cubes: NormTexCubeObj::default(),
            light: LightSolid::default(),
            material: MaterialSolid::default(),
            projection: Mat4::default(),
            timer: Instant::now(),
            first_logic_pass: true,
            camera: Camera::new(),
            io_flags: BitFields::<u8>::default(),
        }
    }
    fn init(&mut self, system: &system::System) -> Result<(), String> {
        print_opengl_info(&system.gl);

        self.lighting_shader =
            Shaders::from_files(&system.gl, "./demo/demo7_lig.vs", "./demo/demo7_lig.fs")?;

        self.cube_shader =
            Shaders::from_files(&system.gl, "./demo/demo7_box.vs", "./demo/demo8_box.fs")?;

        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        self.cubes = NormTexCubeObj::from(&system.gl, DEFAULT_POS_NORM_TEX_CUBE_VERT)?;
        self.cubes.add_another_cube(&system.gl);

        self.light.position = Vec3::new(1.2, 1.0, 2.0);
        self.light.ambient = Vec3::new(0.2, 0.2, 0.2);
        self.light.diffuse = Vec3::new(0.5, 0.5, 0.5);
        self.light.specular = Vec3::new(1.0, 1.0, 1.0);

        self.material.ambient = Vec3::new(1.0, 0.5, 0.31);
        self.material.diffuse = Vec3::new(1.0, 0.5, 0.31);
        self.material.specular = Vec3::new(0.5, 0.5, 0.5);
        self.material.shininess = 32.0;

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

    fn render(&mut self, system: &system::System) -> Result<(), String> {
        // draw the cube object
        self.cube_shader.use_program(&system.gl);

        self.material.pass_uniforms(&system.gl, &self.cube_shader);
        self.light.pass_uniforms(&system.gl, &self.cube_shader);

        self.cube_shader.set_vec3(
            &system.gl,
            "viewPos",
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z,
        );

        self.cube_shader
            .set_mat4fv_uv(&system.gl, "projection", &self.projection);

        let view = self.camera.get_view_matrix();
        self.cube_shader.set_mat4fv_uv(&system.gl, "view", &view);

        self.cube_shader
            .set_mat4fv_uv(&system.gl, "model", &Mat4::default());
        self.cubes.draw(&system.gl, 0);

        // draw the lamp object
        self.lighting_shader.use_program(&system.gl);
        self.lighting_shader
            .set_mat4fv_uv(&system.gl, "projection", &self.projection);
        self.lighting_shader
            .set_mat4fv_uv(&system.gl, "view", &view);
        let mut model = Mat4::default();
        model.translate(&Vec3::new(1.2, 1.0, 2.0));
        model = model * Mat4::from_scale(0.2);
        self.lighting_shader
            .set_mat4fv_uv(&system.gl, "model", &model);
        self.cubes.draw(&system.gl, 1);

        Ok(())
    }

    fn build_projection_matrix(&mut self, system: &system::System, fov_rad: f32) {
        self.projection = projection::rh_yup::perspective_gl(
            fov_rad,
            (system.w as f32) / (system.h as f32),
            0.1,
            100.0,
        );
    }
}
