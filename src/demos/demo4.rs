use super::common::*;
use crate::demos::Demo;
use crate::gfx::{glutils::*, shaders::Shaders, system};
use std::time::Instant;
use ultraviolet::*;

pub struct Demo4 {
    pub name: &'static str,
    pub description: &'static str,
}

impl_demo_trait!(Demo4);

impl Demo4 {
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

                system.clear_screen(0.2, 0.3, 0.4);
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

pub struct DemoImpl {
    shaders_mix: Shaders,
    vao: u32,
    texture: u32,
    awesome_texture: u32,
    rot_angle: f32,
    view: Mat4,
    projection: Mat4,
    timer: Instant,
    cube_positions: [Vec3; 10],
    first_logic_pass: bool,
    cam_pos: Vec3,
}

impl DemoImpl {
    fn new() -> Self {
        DemoImpl {
            shaders_mix: Shaders::default(),
            vao: 0,
            texture: 0,
            awesome_texture: 0,
            rot_angle: 90.0,
            // final matrices
            view: Mat4::default(),
            projection: Mat4::default(),
            timer: Instant::now(),
            cube_positions: [
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
            ],
            first_logic_pass: true,
            cam_pos: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    fn init(&mut self, system: &system::System) -> Result<(), String> {
        print_opengl_info(&system.gl);

        self.vao = gen_textured_box_3d(&system.gl);

        self.texture = load_texture(&system.gl, "./demo/container.jpg")?;
        self.awesome_texture = load_texture(&system.gl, "./demo/awesomeface.png")?;

        self.shaders_mix = Shaders::from_files(
            &system.gl,
            "./demo/demo4_tex_mix.vs",
            "./demo/demo4_tex_mix.fs",
        )?;
        self.shaders_mix.use_program(&system.gl);

        self.shaders_mix.set_i32(&system.gl, "the_texture1", 0);
        self.shaders_mix.set_i32(&system.gl, "texture2", 1);

        self.build_projection_matrix(system);

        Ok(())
    }

    fn update_logic(&mut self, system: &system::System) -> Result<(), String> {
        if self.timer.elapsed().as_millis() > 10 || self.first_logic_pass {
            self.first_logic_pass = false;
            self.timer = Instant::now();

            self.rot_angle += 3.0;

            self.cam_pos.x = ((self.rot_angle / 3.0).to_radians() + 30.0).sin() * 10.0;
            self.cam_pos.z = ((self.rot_angle / 3.0).to_radians() + 70.0).sin() * 10.0;

            self.build_view_matrix(system);
        }
        Ok(())
    }

    fn render(&self, system: &system::System) -> Result<(), String> {
        unsafe {
            system.gl.ActiveTexture(gl33::GL_TEXTURE0);
            system.gl.BindTexture(gl33::GL_TEXTURE_2D, self.texture);

            system.gl.ActiveTexture(gl33::GL_TEXTURE1);
            system
                .gl
                .BindTexture(gl33::GL_TEXTURE_2D, self.awesome_texture);

            self.shaders_mix.use_program(&system.gl);
            system.gl.BindVertexArray(self.vao);

            for (i, v) in self.cube_positions.iter().enumerate() {
                let mut model = Mat4::from_rotation_around(
                    Vec4::new(1.0, 0.0, 1.0, 1.0),
                    (self.rot_angle + (i * 13) as f32).to_radians(),
                );
                model.translate(v);

                self.shaders_mix.set_mat4fv_uv(&system.gl, "model", &model);
                system.gl.DrawArrays(gl33::GL_TRIANGLES, 0, 36);
            }
        }
        Ok(())
    }
    fn build_view_matrix(&mut self, system: &system::System) {
        self.view = Mat4::look_at(
            self.cam_pos,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        self.shaders_mix
            .set_mat4fv_uv(&system.gl, "view", &self.view);
    }
    fn build_projection_matrix(&mut self, system: &system::System) {
        // let proj = projection::rh_yup::orthographic_gl(0.0, 800.0, 0.0, 600.0, 0.1, 100.0);
        self.projection = projection::rh_yup::perspective_gl(
            45.0f32.to_radians(),
            (system.w as f32) / (system.h as f32),
            0.1,
            100.0,
        );

        self.shaders_mix
            .set_mat4fv_uv(&system.gl, "projection", &self.projection);
    }
}
