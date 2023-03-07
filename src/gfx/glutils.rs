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

pub fn load_texture(gl: &GlFns, filename: &str) -> Result<u32, String> {
    use gl33::*;
    let params = [
        (GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT),
        (GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT),
        (GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR),
        (GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR),
    ];
    load_texture_params(gl, filename, &params)
}

pub fn load_texture_params(
    gl: &GlFns,
    filename: &str,
    params: &[(GLenum, GLenum, GLenum)],
) -> Result<u32, String> {
    let mut texture = 0;
    unsafe {
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl33::GL_TEXTURE_2D, texture);

        for (t, n, p) in params {
            gl.TexParameteri(
                *t, *n, p.0 as i32,
                // 0x2901,
            );
        }
    }
    unsafe {
        stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    }
    let img = match stb_image::image::load(filename) {
        stb_image::image::LoadResult::ImageF32(_) => {
            return Err("32-bit images not supported here".to_string());
            // img
        }
        stb_image::image::LoadResult::ImageU8(img) => img,
        stb_image::image::LoadResult::Error(e) => {
            return Err(format!("loading image {} error: {}", filename, e))
        }
    };

    let mut format = gl33::GL_RGB;
    if img.depth == 1 {
        format = gl33::GL_RED;
    } else if img.depth == 3 {
        format = gl33::GL_RGB;
    } else if img.depth == 4 {
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
        gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
    }

    Ok(texture)
}

/// Creates a cube map texture
///
/// # Arguments
///
/// * `gl` - OpenGl context
/// * `filenames` - An array of image filenames according to the following
///   orientation: [right, left, top, bottom, back, front]
///
pub fn load_cube_map_texture(gl: &GlFns, filenames: &[&str]) -> Result<u32, String> {
    use gl33::*;
    let params = [
        (GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE),
        (GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE),
        (GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_R, GL_CLAMP_TO_EDGE),
        (GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MIN_FILTER, GL_LINEAR),
        (GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MAG_FILTER, GL_LINEAR),
    ];
    load_cube_map_texture_params(gl, filenames, &params)
}

pub fn load_cube_map_texture_params(
    gl: &GlFns,
    filenames: &[&str],
    params: &[(GLenum, GLenum, GLenum)],
) -> Result<u32, String> {
    let mut texture = 0;
    unsafe {
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl33::GL_TEXTURE_CUBE_MAP, texture);
        check_gl_err(gl);

        for (t, n, p) in params {
            gl.TexParameteri(
                *t, *n, p.0 as i32,
                // 0x2901,
            );
            check_gl_err(gl);
        }
    }
    unsafe {
        // stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
        stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(0);
    }
    let targets = [
        gl33::GL_TEXTURE_CUBE_MAP_POSITIVE_X, // right
        gl33::GL_TEXTURE_CUBE_MAP_NEGATIVE_X, // left
        gl33::GL_TEXTURE_CUBE_MAP_POSITIVE_Y, // top
        gl33::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, // bottom
        gl33::GL_TEXTURE_CUBE_MAP_POSITIVE_Z, // back
        gl33::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, // front
    ];
    for (target, filename) in targets.iter().zip(filenames) {
        let img = match stb_image::image::load(filename) {
            stb_image::image::LoadResult::ImageF32(_) => {
                return Err("32-bit images not supported here".to_string());
                // img
            }
            stb_image::image::LoadResult::ImageU8(img) => img,
            stb_image::image::LoadResult::Error(e) => {
                return Err(format!("loading image {} error: {}", filename, e))
            }
        };

        let mut format = gl33::GL_RGB;
        if img.depth == 1 {
            format = gl33::GL_RED;
        } else if img.depth == 3 {
            format = gl33::GL_RGB;
        } else if img.depth == 4 {
            format = gl33::GL_RGBA;
        }

        unsafe {
            gl.TexImage2D(
                *target,
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
        }
    }

    Ok(texture)
}
