use super::common::*;
use crate::demos::Demo;
use crate::gfx::{glutils::*, shaders::Shaders, system};
use std::time::Instant;

pub struct Demo2 {
    pub name: &'static str,
    pub description: &'static str,
}

impl_demo_trait!(Demo2);

impl Demo2 {
    fn main(&self) -> Result<(), String> {
        let mut system = system::System::new(800, 600);
        system.clear_screen(0.2, 0.3, 0.4);

        let texture = load_texture(&system.gl, "./demo/container.jpg")?;
        let awesome_texture = load_texture(&system.gl, "./demo/awesomeface.png")?;

        let vao = gen_textured_box_2d(&system.gl);
        let shaders = Shaders::from_files(
            &system.gl,
            "./demo/demo2_texture_simple.vs",
            "./demo/demo2_texture_simple.fs",
        )?;
        let shaders_disco = Shaders::from_files(
            &system.gl,
            "./demo/demo2_texture_simple.vs",
            "./demo/demo2_texture_disco.fs",
        )?;
        let shaders_mix = Shaders::from_files(
            &system.gl,
            "./demo/demo2_texture_simple.vs",
            "./demo/demo2_texture_mix.fs",
        )?;
        shaders_mix.use_program(&system.gl);
        check_gl_err(&system.gl);

        shaders_mix.set_i32(&system.gl, "texture1", 0);
        shaders_mix.set_i32(&system.gl, "texture2", 1);

        let mut state = 0;
        let mut start = Instant::now();

        // getting max texture units
        print_opengl_info(&system.gl);

        loop {
            if !system.process_io_events() {
                break;
            } else {
                // logic code here
                if start.elapsed().as_secs() > 1 {
                    state += 1;
                    if state > 2 {
                        state = 0;
                    }
                    start = Instant::now();
                }
                if state == 0 {
                    shaders.use_program(&system.gl);
                    unsafe {
                        system.gl.ActiveTexture(gl33::GL_TEXTURE0);
                        system.gl.BindTexture(gl33::GL_TEXTURE_2D, texture);
                    }
                } else if state == 1 {
                    shaders_disco.use_program(&system.gl);
                    unsafe {
                        system.gl.ActiveTexture(gl33::GL_TEXTURE0);
                        system.gl.BindTexture(gl33::GL_TEXTURE_2D, awesome_texture);
                    }
                } else {
                    shaders_mix.use_program(&system.gl);
                    unsafe {
                        system.gl.BindTexture(gl33::GL_TEXTURE_2D, 0);

                        system.gl.ActiveTexture(gl33::GL_TEXTURE0);
                        system.gl.BindTexture(gl33::GL_TEXTURE_2D, texture);

                        system.gl.ActiveTexture(gl33::GL_TEXTURE1);
                        system.gl.BindTexture(gl33::GL_TEXTURE_2D, awesome_texture);
                    }
                }
                unsafe {
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
