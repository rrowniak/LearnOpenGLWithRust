use super::common::*;
use crate::demos::Demo;
use crate::gfx::camera::{CamMovement, Camera};
use crate::gfx::lights::{DirLight, MaterialTexMap, PointLight, SpotLight, VSMatrices};
use crate::gfx::{glutils::*, shaders::Shaders, system, system::IoEvents, utils::*};
use gl33::GlFns;
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

const POINT_LIGHT_POSITIONS: [Vec3; 4] = [
    Vec3::new(0.7, 0.2, 2.0),
    Vec3::new(2.3, -3.3, -4.0),
    Vec3::new(-4.0, 2.0, -12.0),
    Vec3::new(0.0, 0.0, -3.0),
];

struct MultLights {
    material: MaterialTexMap,
    dir_light: DirLight,
    point_lights: [PointLight; 4],
    spot_light: SpotLight,
}

impl MultLights {
    fn new() -> Self {
        let ambient = Vec3::new(0.05, 0.05, 0.05);
        let diffuse = Vec3::new(0.8, 0.8, 0.8);
        let specular = Vec3::new(1.0, 1.0, 1.0);
        MultLights {
            material: MaterialTexMap::default(),
            dir_light: DirLight::new(
                Vec3::new(-0.2, -1.0, -0.3),
                ambient,
                Vec3::new(0.4, 0.4, 0.4),
                Vec3::new(0.5, 0.5, 0.5),
                "dirLight",
            ),
            point_lights: [
                PointLight::new(
                    POINT_LIGHT_POSITIONS[0],
                    ambient,
                    diffuse,
                    specular,
                    1.0,
                    0.09,
                    0.032,
                    "pointLights[0]",
                ),
                PointLight::new(
                    POINT_LIGHT_POSITIONS[1],
                    ambient,
                    diffuse,
                    specular,
                    1.0,
                    0.09,
                    0.032,
                    "pointLights[1]",
                ),
                PointLight::new(
                    POINT_LIGHT_POSITIONS[2],
                    ambient,
                    diffuse,
                    specular,
                    1.0,
                    0.09,
                    0.032,
                    "pointLights[2]",
                ),
                PointLight::new(
                    POINT_LIGHT_POSITIONS[3],
                    ambient,
                    diffuse,
                    specular,
                    1.0,
                    0.09,
                    0.032,
                    "pointLights[3]",
                ),
            ],
            spot_light: SpotLight::new(
                Vec3::default(),
                Vec3::default(),
                12.5f32.to_radians().cos(),
                15.0f32.to_radians().cos(),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                1.0,
                0.09,
                0.032,
                "spotLight",
            ),
        }
    }

    fn pass_uniforms(&mut self, gl: &GlFns, shader: &Shaders, camera: &Camera) {
        self.material.pass_uniforms(gl, shader);
        self.dir_light.pass_uniforms(gl, shader);
        self.point_lights[0].pass_uniforms(gl, shader);
        self.point_lights[1].pass_uniforms(gl, shader);
        self.point_lights[2].pass_uniforms(gl, shader);
        self.point_lights[3].pass_uniforms(gl, shader);

        self.spot_light.position = camera.position;
        self.spot_light.direction = camera.front;
        self.spot_light.pass_uniforms(gl, shader);
    }
}

pub struct DemoImpl {
    lamp_shader: Shaders,
    cube_shader: Shaders,
    lights: MultLights,
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
            lights: MultLights::new(),
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
            "./demo/demo13_multiple_lights.fs",
        )?;

        self.texture = load_texture(&system.gl, "./demo/container2.png", true)?;
        self.texture_specular_map =
            load_texture(&system.gl, "./demo/container2_specular.png", true)?;

        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        self.cubes = NormTexCubeObj::from(&system.gl, DEFAULT_POS_NORM_TEX_CUBE_VERT)?;

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
        self.mvp.view = self.camera.get_view_matrix();
        // draw the cube object with specular map
        self.cube_shader.use_program(&system.gl);
        self.lights
            .pass_uniforms(&system.gl, &self.cube_shader, &self.camera);

        self.cube_shader.set_vec3(
            &system.gl,
            "viewPos",
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z,
        );

        for (i, p) in CUBE_POSITIONS.iter().enumerate() {
            self.mvp.model = Mat4::default();
            let angle = 10.0 * (i as f32);
            self.mvp.model =
                Mat4::from_rotation_around(Vec4::new(1.0, 0.3, 0.5, 0.0), angle.to_radians());
            self.mvp.model.translate(p);
            self.mvp.pass_uniforms(&system.gl, &self.cube_shader);
            unsafe {
                system.gl.ActiveTexture(gl33::GL_TEXTURE0);
                system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.texture);

                system.gl.ActiveTexture(gl33::GL_TEXTURE1);
                system
                    .gl
                    .BindTexture(gl33::GL_TEXTURE_2D, self.texture_specular_map);
            }
            self.cubes.draw(&system.gl, 0);
        }

        // draw the lamp objects
        self.lamp_shader.use_program(&system.gl);
        for p in POINT_LIGHT_POSITIONS {
            self.mvp.model = Mat4::default();
            self.mvp.model.translate(&p);
            self.mvp.model = self.mvp.model * Mat4::from_scale(0.2);
            self.mvp.pass_uniforms(&system.gl, &self.lamp_shader);
            self.cubes.draw(&system.gl, 0);
        }

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
