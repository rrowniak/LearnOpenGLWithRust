use crate::demos::Demo;
use crate::gfx::{glutils::*, shaders::Shaders, system};
use super::common::*;
use std::time::Instant;

pub struct Demo3 {
    pub name: &'static str,
    pub description: &'static str,
}

impl_demo_trait!(Demo3);

impl Demo3 {
    fn main(&self) -> Result<(), String> {
        let mut system = system::System::new(800, 600);

        let vao = gen_textured_box_2d(&system.gl);

        let texture = load_texture(&system.gl, "./demo/container.jpg", false)?;
        let awesome_texture = load_texture(&system.gl, "./demo/awesomeface.png", true)?;

        let shaders_mix = Shaders::from_files(
            &system.gl,
            "./demo/demo3_tex_mix.vs",
            "./demo/demo3_tex_mix.fs",
        )?;
        shaders_mix.use_program(&system.gl);

        shaders_mix.set_i32(&system.gl, "the_texture1", 0);
        shaders_mix.set_i32(&system.gl, "texture2", 1);

        // geometry
        #[rustfmt::skip]
        let trans_orig = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        );

        let trans =
            glm::ext::rotate::<f32>(&trans_orig, glm::radians(90.0), glm::vec3(0.0, 0.0, 1.0));
        let trans = glm::ext::scale::<f32>(&trans, glm::vec3(0.5, 0.5, 0.5));
        let mut trans = glm::ext::translate(&trans, glm::vec3(0.4, -0.4, 0.4));

        shaders_mix.set_mat4fv(&system.gl, "transform", &trans);

        let mut start = Instant::now();
        let mut rot_angle = 0.0;

        loop {
            if !system.process_io_events() {
                break;
            } else {
                // logic code here
                if start.elapsed().as_millis() > 10 {
                    start = Instant::now();

                    rot_angle += 1.0;

                    trans = glm::ext::rotate::<f32>(
                        &trans_orig,
                        glm::radians(90.0 + rot_angle),
                        glm::vec3(0.0, 0.0, 1.0),
                    );
                    trans = glm::ext::scale::<f32>(&trans, glm::vec3(0.5, 0.5, 0.5));
                    trans = glm::ext::translate(&trans, glm::vec3(0.4, -0.4, 0.4));

                    shaders_mix.set_mat4fv(&system.gl, "transform", &trans);
                }

                system.clear_screen(0.2, 0.3, 0.4);
                unsafe {
                    system.gl.Clear(gl33::GL_COLOR_BUFFER_BIT);
                }

                unsafe {
                    // system.gl.BindTexture(gl33::GL_TEXTURE_2D, 0);

                    system.gl.ActiveTexture(gl33::GL_TEXTURE0);
                    system.gl.BindTexture(gl33::GL_TEXTURE_2D, texture);

                    system.gl.ActiveTexture(gl33::GL_TEXTURE1);
                    system.gl.BindTexture(gl33::GL_TEXTURE_2D, awesome_texture);

                    shaders_mix.use_program(&system.gl);
                    system.gl.BindVertexArray(vao);
                    system.gl.DrawElements(
                        gl33::GL_TRIANGLES,
                        6,
                        gl33::GL_UNSIGNED_INT,
                        std::ptr::null(),
                    );
                }
                // end logic code here
                system.draw_to_screen();
            }
        }
        Ok(())
    }
}