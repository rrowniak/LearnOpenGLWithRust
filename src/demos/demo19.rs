use super::common::*;
use crate::demos::Demo;
use crate::gfx::camera::Camera;
use crate::gfx::lights::VSMatrices;
use crate::gfx::models::Model;
use crate::gfx::shaders::Shaders;
use crate::gfx::system;
use gl33::*;
use rand::Rng;
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
    // scene objects
    rock: ModelWrapT,
    rock_shader: Shaders,
    planet: ModelWrapT,
    planet_shader: Shaders,
    // asteroids
    asteroids: Vec<Mat4>,
}

impl DemoImpl {
    fn new() -> Self {
        DemoImpl {
            mvp: VSMatrices::default(),
            inputs: Default::default(),
            timer: Instant::now(),
            first_logic_pass: true,
            camera: Camera::new(),
            // scene objects
            rock: ModelWrapT::None,
            rock_shader: Default::default(),
            planet: ModelWrapT::None,
            planet_shader: Default::default(),
            // asteroids
            asteroids: Default::default(),
        }
    }

    fn init(&mut self, system: &system::System) -> Result<(), String> {
        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        // load objects
        self.rock = ModelWrapT::Some(Box::new(Model::from(&system.gl, "./demo/rock/rock.obj")?));
        self.rock.as_mut().unwrap().setup(&system.gl)?;
        self.planet = ModelWrapT::Some(Box::new(Model::from(
            &system.gl,
            "./demo/planet/planet.obj",
        )?));
        self.planet.as_mut().unwrap().setup(&system.gl)?;
        // load shaders
        self.rock_shader = Shaders::from_str(&system.gl, ASTEROID_VS, ASTEROID_FS)?;
        self.planet_shader = Shaders::from_str(&system.gl, PLANET_VS, PLANET_FS)?;

        self.gen_asteroids(100_000);
        self.init_rocks(&system.gl);

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

        self.mvp.model = Mat4::default();
        self.draw_planet(&system.gl);
        self.draw_asteroids(&system.gl);

        Ok(())
    }

    fn draw_planet(&mut self, gl: &gl33::GlFns) {
        self.planet_shader.use_program(gl);
        self.mvp.pass_uniforms(gl, &self.planet_shader);

        self.planet.as_mut().unwrap().draw(gl, &self.planet_shader);
    }

    fn draw_asteroids(&mut self, gl: &gl33::GlFns) {
        self.rock_shader.use_program(gl);
        self.rock_shader
            .set_mat4fv_uv(gl, "projection", &self.mvp.projection);
        self.rock_shader.set_mat4fv_uv(gl, "view", &self.mvp.view);
        for mesh in self.rock.as_ref().unwrap().meshes.iter() {
            mesh.prepare_tex(gl, &self.rock_shader);
            unsafe {
                gl.BindVertexArray(mesh.gl_vao);
                gl.DrawElementsInstanced(
                    GL_TRIANGLES,
                    mesh.indices.len() as i32,
                    GL_UNSIGNED_INT,
                    std::ptr::null(),
                    100_000,
                );
            }
        }
    }

    fn gen_asteroids(&mut self, amount: usize) {
        let mut rng = rand::thread_rng();
        let radius = 100.0;
        let offset = 25.0;
        for i in 0..amount {
            let angle = (i as f32) / (amount as f32) * 360.0;
            let displacement =
                rng.gen_range(0..(2 * (offset as u32) * 100)) as f32 / 100.0 - offset;
            let x = angle.to_radians().sin() * radius + displacement;

            let displacement =
                rng.gen_range(0..(2 * (offset as u32) * 100)) as f32 / 100.0 - offset;
            let y = displacement * 0.4;

            let displacement =
                rng.gen_range(0..(2 * (offset as u32) * 100)) as f32 / 100.0 - offset;
            let z = angle.to_radians().cos() * radius + displacement;

            let scale = rng.gen_range(0.0..20.0) / 100.0 + 0.05;
            let rot_angle = rng.gen_range(0.0..360.0);

            let mut model = Mat4::default();
            model.translate(&Vec3::new(x, y, z));
            model = model * Mat4::from_scale(scale);
            model = model * Mat4::from_rotation_around(Vec4::new(0.4, 0.6, 0.8, 1.0), rot_angle);

            self.asteroids.push(model);
        }
    }

    fn init_rocks(&mut self, gl: &GlFns) {
        let mut buffer = 0;
        unsafe {
            gl.GenBuffers(1, &mut buffer);
            gl.BindBuffer(GL_ARRAY_BUFFER, buffer);
            gl.BufferData(
                GL_ARRAY_BUFFER,
                (std::mem::size_of::<Mat4>() * self.asteroids.len()) as isize,
                self.asteroids.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }

        for vao in self.rock.as_ref().unwrap().meshes.iter().map(|x| x.gl_vao) {
            unsafe {
                gl.BindVertexArray(vao);

                gl.EnableVertexAttribArray(3);
                gl.VertexAttribPointer(
                    3,
                    4,
                    GL_FLOAT,
                    0,
                    std::mem::size_of::<Mat4>() as i32,
                    std::ptr::null(),
                );

                gl.EnableVertexAttribArray(4);
                gl.VertexAttribPointer(
                    4,
                    4,
                    GL_FLOAT,
                    0,
                    std::mem::size_of::<Mat4>() as i32,
                    std::mem::size_of::<Vec4>() as *const _,
                );

                gl.EnableVertexAttribArray(5);
                gl.VertexAttribPointer(
                    5,
                    4,
                    GL_FLOAT,
                    0,
                    std::mem::size_of::<Mat4>() as i32,
                    (2 * std::mem::size_of::<Vec4>()) as *const _,
                );

                gl.EnableVertexAttribArray(6);
                gl.VertexAttribPointer(
                    6,
                    4,
                    GL_FLOAT,
                    0,
                    std::mem::size_of::<Mat4>() as i32,
                    (3 * std::mem::size_of::<Vec4>()) as *const _,
                );

                gl.VertexAttribDivisor(3, 1);
                gl.VertexAttribDivisor(4, 1);
                gl.VertexAttribDivisor(5, 1);
                gl.VertexAttribDivisor(6, 1);

                gl.BindVertexArray(0);
            }
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
}

const PLANET_VS: &str = "
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoords;

out vec2 TexCoords;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main()
{
    TexCoords = aTexCoords;
    gl_Position = projection * view * model * vec4(aPos, 1.0f); 
}
";

const PLANET_FS: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture_diffuse1;

void main()
{
    FragColor = texture(texture_diffuse1, TexCoords);
}
";

const ASTEROID_VS: &str = "
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in mat4 aInstanceMatrix;

out vec2 TexCoords;

uniform mat4 projection;
uniform mat4 view;

void main()
{
    TexCoords = aTexCoords;
    gl_Position = projection * view * aInstanceMatrix * vec4(aPos, 1.0f); 
}
";

const ASTEROID_FS: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture_diffuse1;

void main()
{
    FragColor = texture(texture_diffuse1, TexCoords);
}
";
