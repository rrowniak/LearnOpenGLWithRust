use crate::gfx::utils;
use gl33::*;

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

// gl33 utitls
pub fn gl_buffer_data_arr_stat<T: Sized>(gl: &gl33::GlFns, buffer: &[T]) {
    unsafe {
        gl.BufferData(
            gl33::GL_ARRAY_BUFFER,
            std::mem::size_of_val(buffer) as isize,
            buffer.as_ptr().cast(),
            gl33::GL_STATIC_DRAW,
        );
    }
}

pub fn gl_buffer_data_element_stat<T: Sized>(gl: &gl33::GlFns, buffer: &[T]) {
    unsafe {
        gl.BufferData(
            gl33::GL_ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(buffer) as isize,
            buffer.as_ptr().cast(),
            gl33::GL_STATIC_DRAW,
        );
    }
}

pub fn gl_vertex_attrib_ptr_enab(
    gl: &gl33::GlFns,
    index: u32,
    size: u32,
    stride: u32,
    pointer: usize,
) {
    unsafe {
        gl.VertexAttribPointer(
            index,
            size as i32,
            gl33::GL_FLOAT,
            1, //gl33::GL_FALSE,
            (stride as usize * std::mem::size_of::<f32>()) as i32,
            (pointer * std::mem::size_of::<f32>()) as *const _,
        );
        gl.EnableVertexAttribArray(index);
    }
}

#[derive(Default)]
pub struct BitFields<T> {
    bits: T,
}

impl<
        T: std::ops::BitAndAssign
            + std::ops::BitAnd
            + std::ops::BitOrAssign
            + std::ops::BitXorAssign
            + std::ops::Not<Output = T>
            + Default,
    > BitFields<T>
where
    <T as std::ops::BitAnd>::Output: PartialEq<T>,
    T: Copy,
{
    pub fn is_set(&self, flag: T) -> bool {
        self.bits & flag != T::default()
    }

    pub fn set(&mut self, flag: T) {
        self.bits |= flag
    }

    pub fn unset(&mut self, flag: T) {
        self.bits &= !flag
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self, flag: T) {
        self.bits ^= flag
    }
}

pub fn load_texture(gl: &GlFns, filename: &str, rgba: bool) -> Result<u32, String> {
    let mut texture = 0;
    unsafe {
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl33::GL_TEXTURE_2D, texture);

        gl.TexParameteri(
            gl33::GL_TEXTURE_2D,
            gl33::GL_TEXTURE_WRAP_S,
            gl33::GL_REPEAT.0 as i32,
            // 0x2901,
        );
        gl.TexParameteri(
            gl33::GL_TEXTURE_2D,
            gl33::GL_TEXTURE_WRAP_T,
            gl33::GL_REPEAT.0 as i32,
            // 0x2901,
        );
        gl.TexParameteri(
            gl33::GL_TEXTURE_2D,
            gl33::GL_TEXTURE_MIN_FILTER,
            gl33::GL_LINEAR.0 as i32,
            // 0x2601,
        );
        gl.TexParameteri(
            gl33::GL_TEXTURE_2D,
            gl33::GL_TEXTURE_MAG_FILTER,
            gl33::GL_LINEAR.0 as i32,
            // 0x2601,
        );
    }
    unsafe {
        stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    }
    let img = match stb_image::image::load(filename) {
        stb_image::image::LoadResult::ImageF32(_) => {
            return Err("32-bit images not supported here".to_string());
            // img
        }
        stb_image::image::LoadResult::ImageU8(img) => {
            // return Err("8-bit images not supported here".to_string())
            img
        }
        stb_image::image::LoadResult::Error(e) => {
            return Err(format!("loading image {} error: {}", filename, e))
        }
    };

    let mut format = gl33::GL_RGB;
    if rgba {
        format = gl33::GL_RGBA;
    }

    unsafe {
        gl.TexImage2D(
            gl33::GL_TEXTURE_2D,
            0,
            gl33::GL_RGBA.0 as i32,
            img.width as i32,
            img.height as i32,
            0,
            format,
            gl33::GL_UNSIGNED_BYTE,
            img.data.as_ptr().cast(),
        );
        utils::check_gl_err(gl);
        //gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
    }

    Ok(texture)
}

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

            gl.BufferData(
                GL_ARRAY_BUFFER,
                std::mem::size_of_val(&data) as isize,
                data.as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            gl.BindVertexArray(vao);

            // position attribute
            gl.VertexAttribPointer(
                0,
                3,
                gl33::GL_FLOAT,
                0,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );

            gl.EnableVertexAttribArray(0);
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

            gl.VertexAttribPointer(
                0,
                3,
                gl33::GL_FLOAT,
                0,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl.EnableVertexAttribArray(0);
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

            gl.BufferData(
                GL_ARRAY_BUFFER,
                std::mem::size_of_val(&data) as isize,
                data.as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            gl.BindVertexArray(vao);

            // position attribute
            gl.VertexAttribPointer(
                0,
                3,
                gl33::GL_FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );

            gl.EnableVertexAttribArray(0);

            // normals attribute
            gl.VertexAttribPointer(
                1,
                3,
                gl33::GL_FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );

            gl.EnableVertexAttribArray(1);

            // tex coords attribute
            gl.VertexAttribPointer(
                2,
                2,
                gl33::GL_FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as i32,
                (6 * std::mem::size_of::<f32>()) as *const _,
            );

            gl.EnableVertexAttribArray(2);
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

            gl.VertexAttribPointer(
                0,
                3,
                gl33::GL_FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl.EnableVertexAttribArray(0);
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
