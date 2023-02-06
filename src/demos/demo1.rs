use super::common::*;
use crate::demos::Demo;
use crate::gfx::{glutils::*, shaders::Shaders, system};
use gl33::*;
use std::time::Instant;

pub struct Demo1 {
    pub name: &'static str,
    pub description: &'static str,
}

impl_demo_trait!(Demo1);

impl Demo1 {
    fn main(&self) -> Result<(), String> {
        let mut system = system::System::new(800, 600);
        system.clear_screen(0.2, 0.3, 0.4);

        let simplest_shaders = Shaders::from_files(
            &system.gl,
            "demo/demo1_simplest.vs",
            "demo/demo1_simplest.fs",
        )?;

        let simplest_col_shaders = Shaders::from_files(
            &system.gl,
            "demo/demo1_simplest.vs",
            "demo/demo1_simplest_col.fs",
        )?;

        let simple_2_layouts_shaders = Shaders::from_files(
            &system.gl,
            "demo/demo1_simple_two_layouts.vs",
            "demo/demo1_simple_two_layouts.fs",
        )?;

        let triangle_vao = prepare_triangle(&system.gl);
        let rectangle_vao = prepare_rectangle(&system.gl);
        let triangle_col = prepare_triangle_colored(&system.gl);

        let mut start = Instant::now();
        let mut col: f32 = 0.0;
        let mut cnt = 0;

        loop {
            if start.elapsed().as_millis() > 100 {
                // do update every 100ms
                cnt += 1;
                let alpha = 10.0 * 2.0 * std::f32::consts::PI * (cnt as f32) / 360.0;

                col = alpha.sin() / 2.0 + 0.5;

                start = Instant::now();
            }

            if !system.process_io_events() {
                break;
            } else {
                // draw triangle
                simplest_col_shaders.use_program(&system.gl);
                simplest_col_shaders.set_vec4(&system.gl, "color", 0.5, col, 0.5, 1.0);
                system.gl.BindVertexArray(triangle_vao);
                unsafe {
                    system
                        .gl
                        .PolygonMode(gl33::GL_FRONT_AND_BACK, gl33::GL_FILL);
                    system.gl.DrawArrays(gl33::GL_TRIANGLES, 0, 3);
                }
                simplest_shaders.use_program(&system.gl);
                // draw rectangle
                unsafe {
                    system.gl.BindVertexArray(rectangle_vao);
                    system
                        .gl
                        .PolygonMode(gl33::GL_FRONT_AND_BACK, gl33::GL_LINE);
                    system.gl.DrawElements(
                        gl33::GL_TRIANGLES,
                        6,
                        gl33::GL_UNSIGNED_INT,
                        std::ptr::null(),
                    );
                }
                // draw triangle coloured
                simple_2_layouts_shaders.use_program(&system.gl);
                unsafe {
                    system.gl.BindVertexArray(triangle_col);
                    system
                        .gl
                        .PolygonMode(gl33::GL_FRONT_AND_BACK, gl33::GL_FILL);
                    system.gl.DrawArrays(gl33::GL_TRIANGLES, 0, 3);
                }

                system.draw_to_screen();
            }
        }
        Ok(())
    }
}

fn prepare_triangle(gl: &GlFns) -> u32 {
    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 3] = [[-0.9, -0.9, 0.0], [0.1, -0.9, 0.0], [-0.4, 0.1, 0.0]];

    let mut vao = 0;

    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl.BindVertexArray(vao);

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        gl.BindBuffer(gl33::GL_ARRAY_BUFFER, vbo);
    }

    gl_buffer_data_arr_stat(gl, &VERTICES);
    gl_vertex_attrib_ptr_enab(gl, 0, 3, 3, 0);

    vao
}

fn prepare_rectangle(gl: &GlFns) -> u32 {
    #[rustfmt::skip]
    const VERTICES: [f32; 12] = [
        0.9,    0.9,    0.0, 
        0.9,   -0.1,    0.0,
       -0.1,   -0.1,    0.0, 
       -0.1,    0.9,    0.0,
    ];

    const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl.BindVertexArray(vao);

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        gl.BindBuffer(gl33::GL_ARRAY_BUFFER, vbo);

        gl_buffer_data_arr_stat(gl, &VERTICES);

        let mut ebo = 0;
        gl.GenBuffers(1, &mut ebo);
        gl.BindBuffer(gl33::GL_ELEMENT_ARRAY_BUFFER, ebo);

        gl_buffer_data_element_stat(gl, &INDICES);
        gl_vertex_attrib_ptr_enab(gl, 0, 3, 3, 0);
        vao
    }
}

fn prepare_triangle_colored(gl: &GlFns) -> u32 {
    #[rustfmt::skip]
    const VERTICES: [f32; 18] = [
        0.5,   -0.5,    0.0, 
        1.0,    0.0,    0.0, 
       -0.5,   -0.5,    0.0, 
        0.0,    1.0,    0.0, 
        0.0,    0.5,    0.0, 
        0.0,    0.0,    0.1,
    ];

    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl.BindVertexArray(vao);

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        gl.BindBuffer(gl33::GL_ARRAY_BUFFER, vbo);

        gl_buffer_data_arr_stat(gl, &VERTICES);

        gl_vertex_attrib_ptr_enab(gl, 0, 3, 6, 0);

        gl_vertex_attrib_ptr_enab(gl, 1, 3, 6, 3);

        vao
    }
}
