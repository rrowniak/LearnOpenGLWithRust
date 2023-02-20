use crate::gfx::glutils::*;
use crate::gfx::models::*;
use gl33::*;
use ultraviolet::*;

macro_rules! impl_demo_trait {
    ($($t:ty),+ $(,)?) => ($(
        impl Demo for $t {
            fn run(&self) -> Result<(), String> {
                self.main()
            }
            fn name(&self) -> String {
                self.name.to_string()
            }
            fn description(&self) -> String {
                self.description.to_string()
            }
        }
    )+)
}

pub(crate) use impl_demo_trait;

pub const DEFAULT_SIMPL_CUBE_VERT: [f32; 108] = [
    -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
    -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
    0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5,
    -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5,
    -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
];

#[derive(Default)]
pub struct SimplestCubeObj {
    vbo: u32,
    pub vaos: Vec<u32>,
}

impl SimplestCubeObj {
    // 6 sides x 2 triangles x 3 vertices
    pub fn from(gl: &GlFns, data: [f32; 108]) -> Result<Self, String> {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            if vao == 0 {
                return Err("failed: gl.GenVertexArrays(1, &mut cube.vao)".to_string());
            }

            gl.GenBuffers(1, &mut vbo);
            if vbo == 0 {
                return Err("failed: gl.GenBuffers(1, &mut vbo)".to_string());
            }

            gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

            gl_buffer_data_arr_stat(gl, &data);

            gl.BindVertexArray(vao);

            // position attribute
            gl_vertex_attrib_ptr_enab(gl, 0, 3, 3, 0);
        }

        Ok(SimplestCubeObj {
            vbo,
            vaos: vec![vao],
        })
    }

    pub fn add_another_cube(&mut self, gl: &GlFns) -> usize {
        let mut vao: u32 = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            gl.BindBuffer(GL_ARRAY_BUFFER, self.vbo);

            gl_vertex_attrib_ptr_enab(gl, 0, 3, 3, 0);
        }

        self.vaos.push(vao);

        self.vaos.len() - 1
    }

    pub fn draw(&self, gl: &GlFns, indx: usize) {
        gl.BindVertexArray(self.vaos[indx]);
        unsafe {
            gl.DrawArrays(gl33::GL_TRIANGLES, 0, 36);
        }
    }
}

#[rustfmt::skip]
pub const DEFAULT_POS_NORM_TEX_CUBE_VERT: [f32; 288] = [
    // positions      // normals        // texture coords
   -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
    0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
   -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
   -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
   -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
    0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
   -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
   -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
   -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
   -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
   -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
   -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
   -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
   -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
    0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
   -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
    0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
   -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
   -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
   -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
    0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
   -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
   -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
];

#[derive(Default)]
pub struct NormTexCubeObj {
    vbo: u32,
    pub vaos: Vec<u32>,
}

impl NormTexCubeObj {
    // 6 sides x 2 triangles x 3 vertices x 3 normal x 2 tex coord
    pub fn from(gl: &GlFns, data: [f32; 288]) -> Result<Self, String> {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            if vao == 0 {
                return Err("failed: gl.GenVertexArrays(1, &mut cube.vao)".to_string());
            }

            gl.GenBuffers(1, &mut vbo);
            if vbo == 0 {
                return Err("failed: gl.GenBuffers(1, &mut vbo)".to_string());
            }

            gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

            gl_buffer_data_arr_stat(gl, &data);

            gl.BindVertexArray(vao);

            // position attribute
            gl_vertex_attrib_ptr_enab(gl, 0, 3, 8, 0);

            // normals attribute
            gl_vertex_attrib_ptr_enab(gl, 1, 3, 8, 3);

            // tex coords attribute
            gl_vertex_attrib_ptr_enab(gl, 2, 2, 8, 6);
        }

        Ok(NormTexCubeObj {
            vbo,
            vaos: vec![vao],
        })
    }

    pub fn add_another_cube(&mut self, gl: &GlFns) -> usize {
        let mut vao: u32 = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            gl.BindBuffer(GL_ARRAY_BUFFER, self.vbo);

            // position attribute
            gl_vertex_attrib_ptr_enab(gl, 0, 3, 8, 0);

            // normals attribute
            gl_vertex_attrib_ptr_enab(gl, 1, 3, 8, 3);

            // tex coords attribute
            gl_vertex_attrib_ptr_enab(gl, 2, 2, 8, 6);
        }

        self.vaos.push(vao);

        self.vaos.len() - 1
    }

    pub fn draw(&self, gl: &GlFns, indx: usize) {
        gl.BindVertexArray(self.vaos[indx]);
        unsafe {
            gl.DrawArrays(gl33::GL_TRIANGLES, 0, 36);
        }
    }
}

pub fn gen_textured_box_2d(gl: &GlFns) -> u32 {
    #[rustfmt::skip]
    const VERTICES: [f32; 32] = [
        // positions          // colors           // texture coords
         0.5,  0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0, // top right
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,    1.0, 1.0, 0.0,      0.0, 1.0, // top left
    ];
    const INDICES: [u32; 6] = [
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let mut vao = 0;
    unsafe {
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
    }
    gl_buffer_data_element_stat(gl, &INDICES);
    // position attribute
    gl_vertex_attrib_ptr_enab(gl, 0, 3, 8, 0);
    // color attribute
    gl_vertex_attrib_ptr_enab(gl, 1, 3, 8, 3);
    // texture coord attribute
    gl_vertex_attrib_ptr_enab(gl, 2, 2, 8, 6);

    vao
}

pub fn gen_textured_box_3d(gl: &GlFns) -> u32 {
    const VERTICES: [f32; 180] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
        -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5,
        0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5,
        0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5,
        1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0,
        0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
        -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5,
        1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5,
        -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0,
        -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
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
        gl_vertex_attrib_ptr_enab(gl, 0, 3, 5, 0);
        gl_vertex_attrib_ptr_enab(gl, 1, 2, 5, 3);

        vao
    }
}

// array format:
// position.xyz normal.xyz tex_coords.xy
pub fn setup_model_from_slice(d: &[f32]) -> Model {
    let mut i = 0;
    let mut model = Model::default();
    model.meshes.push(Mesh::default());

    loop {
        let v = Vertex {
            position: Vec3::new(d[i], d[i + 1], d[i + 2]),
            normal: Vec3::new(d[i + 3], d[i + 4], d[i + 5]),
            tex_coords: Vec2::new(d[i + 6], d[i + 7]),
        };

        model.meshes[0].vertices.push(v);
        let vlen = model.meshes[0].vertices.len();
        model.meshes[0].indices.push((vlen - 1) as u32);

        i += 8;
        if i >= d.len() {
            break;
        }
    }
    model
}

pub fn setup_model_box(d: [f32; 288]) -> Model {
    setup_model_from_slice(&d)
}

#[rustfmt::skip]
pub const DEFAULT_PLANE: [f32; 48] = [
    // positions        // fake normals   // tex coords
    5.0, -0.5,  5.0,    1.0, 0.0, 0.0,    2.0, 0.0,
   -5.0, -0.5,  5.0,    1.0, 0.0, 0.0,    0.0, 0.0,
   -5.0, -0.5, -5.0,    1.0, 0.0, 0.0,    0.0, 2.0,
    5.0, -0.5,  5.0,    1.0, 0.0, 0.0,    2.0, 0.0,
   -5.0, -0.5, -5.0,    1.0, 0.0, 0.0,    0.0, 2.0,
    5.0, -0.5, -5.0,    1.0, 0.0, 0.0,    2.0, 2.0,
];

pub fn setup_model_plane(d: [f32; 48]) -> Model {
    setup_model_from_slice(&d)
}

pub mod stencil {
    use gl33::*;

    pub fn select_eff_off(gl: &GlFns) {
        unsafe {
            gl.Enable(gl33::GL_DEPTH_TEST);

            gl.StencilOp(gl33::GL_KEEP, gl33::GL_KEEP, gl33::GL_REPLACE);
            gl.Clear(gl33::GL_STENCIL_BUFFER_BIT);
            gl.StencilMask(0x00);
        }
    }

    pub fn select_eff_prepare(gl: &GlFns) {
        unsafe {
            gl.StencilFunc(gl33::GL_ALWAYS, 1, 0xff);
            gl.StencilMask(0xff);
        }
    }

    pub fn select_eff_begin(gl: &GlFns) {
        unsafe {
            gl.StencilFunc(gl33::GL_NOTEQUAL, 1, 0xff);
            gl.StencilMask(0x00);
            gl.Disable(gl33::GL_DEPTH_TEST);
        }
    }

    pub fn select_eff_end(gl: &GlFns) {
        unsafe {
            gl.StencilFunc(gl33::GL_ALWAYS, 1, 0xff);
            gl.StencilMask(0xff);
            gl.Enable(gl33::GL_DEPTH_TEST);
        }
    }
}
