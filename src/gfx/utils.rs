use gl33::*;

pub fn check_gl_err(gl: &GlFns) {
    unsafe {
        let err = gl.GetError();
        if err == gl33::GL_NO_ERROR {
            return;
        }
        panic!("error: {:?}", err);
    }
}

pub fn print_opengl_info(gl: &GlFns) {
    let mut mtu: i32 = 0;
    unsafe {
        gl.GetIntegerv(gl33::GL_MAX_TEXTURE_IMAGE_UNITS, &mut mtu);
        println!("GL_MAX_TEXTURE_IMAGE_UNITS = {}", mtu);

        gl.GetIntegerv(gl33::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS, &mut mtu);
        println!("GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS = {}", mtu);
    }
}
