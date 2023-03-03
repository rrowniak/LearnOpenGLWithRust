use gl33::*;
use ultraviolet::*;

#[derive(Default)]
pub struct FrameBuffer {
    bound: bool,
    pub fbo: u32,
    pub rbo: u32,
    pub tex: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(gl: &GlFns) -> Self {
        let mut fb = Self {
            bound: false,
            fbo: 0,
            rbo: 0,
            tex: Default::default(),
        };
        unsafe {
            gl.GenFramebuffers(1, &mut fb.fbo);
        }
        fb
    }

    pub fn bind(&mut self, gl: &GlFns) {
        unsafe {
            gl.BindFramebuffer(gl33::GL_FRAMEBUFFER, self.fbo);
        }
        self.bound = true;
    }

    pub fn unbind(&mut self, gl: &GlFns) {
        self.bound = false;
        unsafe {
            gl.BindFramebuffer(gl33::GL_FRAMEBUFFER, 0);
        }
    }

    pub fn clear(&self, gl: &GlFns, col: Vec4) {
        unsafe {
            gl.ClearColor(col.x, col.y, col.z, col.w);
            gl.Clear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT); // we're not using the stencil buffer now
            gl.Enable(gl33::GL_DEPTH_TEST);
        }
    }

    pub fn attach_texture(&mut self, gl: &GlFns, w: usize, h: usize) -> u32 {
        if !self.bound {
            panic!("Call Self.bind() first!")
        }
        self.tex.push(0);
        let tex = self.tex.last_mut().unwrap();
        unsafe {
            gl.GenTextures(1, &mut *tex);
            gl.BindTexture(gl33::GL_TEXTURE_2D, *tex);
            gl.TexImage2D(
                gl33::GL_TEXTURE_2D,
                0,
                gl33::GL_RGB.0 as i32,
                w as i32,
                h as i32,
                0,
                gl33::GL_RGB,
                gl33::GL_UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MIN_FILTER,
                gl33::GL_LINEAR.0 as i32,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MAG_FILTER,
                gl33::GL_LINEAR.0 as i32,
            );
            // unbind texture
            gl.BindTexture(gl33::GL_TEXTURE_2D, 0);

            gl.FramebufferTexture2D(
                gl33::GL_FRAMEBUFFER,
                gl33::GL_COLOR_ATTACHMENT0,
                gl33::GL_TEXTURE_2D,
                *tex,
                0,
            );
        }
        *self.tex.last().unwrap()
    }

    pub fn attach_depth_stencil(&mut self, gl: &GlFns, w: usize, h: usize) -> u32 {
        if !self.bound {
            panic!("Call Self.bind() first!")
        }
        self.tex.push(0);
        let tex = self.tex.last_mut().unwrap();
        unsafe {
            gl.GenTextures(1, &mut *tex);
            gl.BindTexture(gl33::GL_TEXTURE_2D, *tex);
            gl.TexImage2D(
                gl33::GL_TEXTURE_2D,
                0,
                gl33::GL_DEPTH24_STENCIL8.0 as i32,
                w as i32,
                h as i32,
                0,
                gl33::GL_DEPTH_STENCIL,
                gl33::GL_UNSIGNED_INT_24_8,
                std::ptr::null(),
            );
            gl.BindTexture(gl33::GL_TEXTURE_2D, 0);

            gl.FramebufferTexture2D(
                gl33::GL_FRAMEBUFFER,
                gl33::GL_DEPTH_STENCIL_ATTACHMENT,
                gl33::GL_TEXTURE_2D,
                *tex,
                0,
            );
        }
        *self.tex.last().unwrap()
    }

    pub fn attach_render_buffer(&mut self, gl: &GlFns, w: usize, h: usize) {
        if !self.bound {
            panic!("Call Self.bind() first!")
        }
        unsafe {
            gl.GenRenderbuffers(1, &mut self.rbo);
            gl.BindRenderbuffer(gl33::GL_RENDERBUFFER, self.rbo);
            gl.RenderbufferStorage(
                gl33::GL_RENDERBUFFER,
                gl33::GL_DEPTH24_STENCIL8,
                w as i32,
                h as i32,
            );
            gl.BindRenderbuffer(gl33::GL_RENDERBUFFER, 0);

            gl.FramebufferRenderbuffer(
                gl33::GL_FRAMEBUFFER,
                gl33::GL_DEPTH_STENCIL_ATTACHMENT,
                gl33::GL_RENDERBUFFER,
                self.rbo,
            );
        }
    }

    pub fn check_success_or_panic(&self, gl: &GlFns) {
        unsafe {
            if gl.CheckFramebufferStatus(gl33::GL_FRAMEBUFFER) != gl33::GL_FRAMEBUFFER_COMPLETE {
                panic!("framebuffer is not completed");
            }
        }
    }

    pub fn delete(&mut self, gl: &GlFns) {
        unsafe {
            gl.DeleteFramebuffers(1, &self.fbo);
            self.fbo = 0;
        }
    }
}
