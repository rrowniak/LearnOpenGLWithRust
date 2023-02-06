use gl33::*;

pub fn check_gl_err(gl: &GlFns) {
    unsafe {
        let err = gl.GetError();
        if err == GL_NO_ERROR {
            return;
        }
        panic!("error: {:?}", err);
    }
}

pub fn print_opengl_info(gl: &GlFns) {
    let mut mtu: i32 = 0;
    unsafe {
        gl.GetIntegerv(GL_MAX_TEXTURE_IMAGE_UNITS, &mut mtu);
        println!("GL_MAX_TEXTURE_IMAGE_UNITS = {}", mtu);

        gl.GetIntegerv(GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS, &mut mtu);
        println!("GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS = {}", mtu);
    }
}

pub fn gl_buffer_data_arr_stat<T: Sized>(gl: &GlFns, buffer: &[T]) {
    unsafe {
        gl.BufferData(
            GL_ARRAY_BUFFER,
            std::mem::size_of_val(buffer) as isize,
            buffer.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }
}

pub fn gl_buffer_data_element_stat<T: Sized>(gl: &GlFns, buffer: &[T]) {
    unsafe {
        gl.BufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(buffer) as isize,
            buffer.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }
}

pub fn gl_vertex_attrib_ptr_enab(gl: &GlFns, index: u32, size: u32, stride: u32, pointer: usize) {
    unsafe {
        gl.VertexAttribPointer(
            index,
            size as i32,
            GL_FLOAT,
            1, //gl33::GL_FALSE,
            (stride as usize * std::mem::size_of::<f32>()) as i32,
            (pointer * std::mem::size_of::<f32>()) as *const _,
        );
        gl.EnableVertexAttribArray(index);
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
        check_gl_err(gl);
        //gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
    }

    Ok(texture)
}
