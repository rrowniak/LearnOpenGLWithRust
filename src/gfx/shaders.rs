use super::glutils;
use gl33::*;
use std::ffi::CString;
use std::fs;
use ultraviolet::*;

#[derive(Default)]
pub struct Shaders {
    program_id: u32,
}

impl Shaders {
    pub fn from_files(
        gl: &GlFns,
        vertex_file: &str,
        fragment_file: &str,
    ) -> Result<Shaders, String> {
        let vertex_code = match fs::read_to_string(vertex_file) {
            Ok(v) => v,
            Err(e) => return Err(format!("error reading {}: {}", vertex_file, e)),
        };

        let fragment_code = match fs::read_to_string(fragment_file) {
            Ok(v) => v,
            Err(e) => return Err(format!("error reading {}: {}", vertex_file, e)),
        };

        Shaders::from_str(gl, vertex_code.as_str(), fragment_code.as_str())
    }

    pub fn from_str(gl: &GlFns, vertex_code: &str, fragment_code: &str) -> Result<Shaders, String> {
        // create vertex shader
        let vertex_shader = gl.CreateShader(gl33::GL_VERTEX_SHADER);
        if vertex_shader == 0 {
            return Err("glCreateShader(GL_VERTEX_SHADER) failed".to_string());
        }

        if let Err(e) = Self::compile(gl, vertex_shader, vertex_code) {
            return Err(format!("vertex shader compilation error: {}", e));
        }

        // create fragment shader
        let fragment_shader = gl.CreateShader(gl33::GL_FRAGMENT_SHADER);
        if fragment_shader == 0 {
            return Err("glCreateShader(GL_FRAGMENT_SHADER) failed".to_string());
        }

        if let Err(e) = Self::compile(gl, fragment_shader, fragment_code) {
            return Err(format!("fragment shader compilation error: {}", e));
        }

        // create program and link shaders
        let shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program, vertex_shader);
        gl.AttachShader(shader_program, fragment_shader);
        gl.LinkProgram(shader_program);

        let mut success = 0;
        unsafe {
            gl.GetProgramiv(shader_program, gl33::GL_LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            unsafe {
                gl.GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            return Err(format!(
                "program link error: {}",
                String::from_utf8_lossy(&v)
            ));
        }

        // not needed anymore
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        Ok(Shaders {
            program_id: shader_program,
        })
    }
    fn compile(gl: &GlFns, shader_id: u32, shader_code: &str) -> Result<(), String> {
        unsafe {
            gl.ShaderSource(
                shader_id,
                1,
                &(shader_code.as_bytes().as_ptr().cast()),
                &(shader_code.len().try_into().unwrap()),
            );
        }

        gl.CompileShader(shader_id);

        // check if there are compilation errors
        let mut success = 0;
        unsafe {
            gl.GetShaderiv(shader_id, gl33::GL_COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            unsafe {
                gl.GetShaderInfoLog(shader_id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }

            return Err(String::from_utf8_lossy(&v).to_string());
        }
        Ok(())
    }

    fn get_uniform_location(&self, gl: &GlFns, name: &str) -> i32 {
        let c_name = std::ffi::CString::new(name).unwrap_or_else(|_| {
            panic!("get_uniform_location: CString::new failed for '{}'", name);
        });

        self.get_uniform_location_cstr(gl, &c_name)
    }

    fn get_uniform_location_cstr(&self, gl: &GlFns, c_name: &CString) -> i32 {
        let location;
        unsafe {
            location = gl.GetUniformLocation(self.program_id, c_name.as_ptr().cast());
        }
        glutils::check_gl_err(gl);
        if location == -1 {
            let name = c_name.to_str().unwrap_or("<cstring decoding error>");
            panic!(
                "program({}): location '{}' does not correspond to an active uniform variable in program",
                self.program_id,
                name
            );
        }
        location
    }

    pub fn use_program(&self, gl: &GlFns) {
        gl.UseProgram(self.program_id);
        glutils::check_gl_err(gl);
    }

    pub fn set_bool(&self, gl: &GlFns, name: &str, value: bool) {
        unsafe {
            gl.Uniform1i(self.get_uniform_location(gl, name), i32::from(value));
        }
    }

    pub fn set_bool_cstr(&self, gl: &GlFns, name: &CString, value: bool) {
        unsafe {
            gl.Uniform1i(self.get_uniform_location_cstr(gl, name), i32::from(value));
        }
    }

    pub fn set_i32(&self, gl: &GlFns, name: &str, value: i32) {
        unsafe {
            gl.Uniform1i(self.get_uniform_location(gl, name), value);
        }
    }

    pub fn set_i32_cstr(&self, gl: &GlFns, name: &CString, value: i32) {
        unsafe {
            gl.Uniform1i(self.get_uniform_location_cstr(gl, name), value);
        }
    }

    pub fn set_f32(&self, gl: &GlFns, name: &str, value: f32) {
        unsafe {
            gl.Uniform1f(self.get_uniform_location(gl, name), value);
        }
    }

    pub fn set_f32_cstr(&self, gl: &GlFns, name: &CString, value: f32) {
        unsafe {
            gl.Uniform1f(self.get_uniform_location_cstr(gl, name), value);
        }
    }

    pub fn set_vec3(&self, gl: &GlFns, name: &str, v0: f32, v1: f32, v2: f32) {
        unsafe {
            gl.Uniform3f(self.get_uniform_location(gl, name), v0, v1, v2);
        }
    }

    pub fn set_vec3_cstr(&self, gl: &GlFns, name: &CString, v0: f32, v1: f32, v2: f32) {
        unsafe {
            gl.Uniform3f(self.get_uniform_location_cstr(gl, name), v0, v1, v2);
        }
    }

    pub fn set_vec4(&self, gl: &GlFns, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            gl.Uniform4f(self.get_uniform_location(gl, name), v0, v1, v2, v3);
        }
    }

    pub fn set_vec4_cstr(&self, gl: &GlFns, name: &CString, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            gl.Uniform4f(self.get_uniform_location_cstr(gl, name), v0, v1, v2, v3);
        }
    }

    pub fn set_mat4fv(&self, gl: &GlFns, name: &str, mat: &glm::Matrix4<f32>) {
        let location = self.get_uniform_location(gl, name);
        unsafe {
            let arr: [f32; 16] = [
                mat.c0[0], mat.c0[1], mat.c0[2], mat.c0[3], mat.c1[0], mat.c1[1], mat.c1[2],
                mat.c1[3], mat.c2[0], mat.c2[1], mat.c2[2], mat.c2[3], mat.c3[0], mat.c3[1],
                mat.c3[2], mat.c3[3],
            ];
            gl.UniformMatrix4fv(location, 1, gl33::GL_FALSE.0 as u8, arr.as_ptr().cast());
        }
    }

    pub fn set_mat4fv_uv(&self, gl: &GlFns, name: &str, mat: &Mat4) {
        let location = self.get_uniform_location(gl, name);
        unsafe {
            gl.UniformMatrix4fv(location, 1, gl33::GL_FALSE.0 as u8, mat.as_slice().as_ptr());
        }
    }

    pub fn set_mat4fv_uv_cstr(&self, gl: &GlFns, name: &CString, mat: &Mat4) {
        let location = self.get_uniform_location_cstr(gl, name);
        unsafe {
            gl.UniformMatrix4fv(location, 1, gl33::GL_FALSE.0 as u8, mat.as_slice().as_ptr());
        }
    }
}

pub struct LightSolid {
    pub position: Vec3,
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    _position: CString,
    _ambient: CString,
    _diffuse: CString,
    _specular: CString,
}

impl Default for LightSolid {
    fn default() -> Self {
        LightSolid {
            position: Vec3::default(),
            ambient: Vec3::default(),
            diffuse: Vec3::default(),
            specular: Vec3::default(),
            _position: CString::new("light.position").expect("CString::new failed"),
            _ambient: CString::new("light.ambient").expect("CString::new failed"),
            _diffuse: CString::new("light.diffuse").expect("CString::new failed"),
            _specular: CString::new("light.specular").expect("CString::new failed"),
        }
    }
}

impl LightSolid {
    pub fn pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.set_vec3_cstr(
            gl,
            &self._position,
            self.position.x,
            self.position.y,
            self.position.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._ambient,
            self.ambient.x,
            self.ambient.y,
            self.ambient.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._diffuse,
            self.diffuse.x,
            self.diffuse.y,
            self.diffuse.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._specular,
            self.specular.x,
            self.specular.y,
            self.specular.z,
        );
    }
}

pub struct LightDir {
    pub direction: Vec3,
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    _direction: CString,
    _ambient: CString,
    _diffuse: CString,
    _specular: CString,
}

impl Default for LightDir {
    fn default() -> Self {
        LightDir {
            direction: Vec3::default(),
            ambient: Vec3::default(),
            diffuse: Vec3::default(),
            specular: Vec3::default(),
            _direction: CString::new("light.direction").expect("CString::new failed"),
            _ambient: CString::new("light.ambient").expect("CString::new failed"),
            _diffuse: CString::new("light.diffuse").expect("CString::new failed"),
            _specular: CString::new("light.specular").expect("CString::new failed"),
        }
    }
}

impl LightDir {
    pub fn pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.set_vec3_cstr(
            gl,
            &self._direction,
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._ambient,
            self.ambient.x,
            self.ambient.y,
            self.ambient.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._diffuse,
            self.diffuse.x,
            self.diffuse.y,
            self.diffuse.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._specular,
            self.specular.x,
            self.specular.y,
            self.specular.z,
        );
    }
}

// Distance Constant    Linear      Quadratic
// 3250,    1.0,        0.0014,     0.000007
// 600,     1.0,        0.007,      0.0002
// 325,     1.0,        0.014,      0.0007
// 200,     1.0,        0.022,      0.0019
// 160,     1.0,        0.027,      0.0028
// 100,     1.0,        0.045,      0.0075
// 65,      1.0,        0.07,       0.017
// 50,      1.0,        0.09,       0.032
// 32,      1.0,        0.14,       0.07
// 20,      1.0,        0.22,       0.20
// 13,      1.0,        0.35,       0.44
// 7,       1.0,        0.7,        1.8
pub struct LightPoint {
    pub position: Vec3,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    _position: CString,
    _ambient: CString,
    _diffuse: CString,
    _specular: CString,
    _constant: CString,
    _linear: CString,
    _quadratic: CString,
}

impl Default for LightPoint {
    fn default() -> Self {
        LightPoint {
            position: Vec3::default(),
            ambient: Vec3::default(),
            diffuse: Vec3::default(),
            specular: Vec3::default(),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
            _position: CString::new("light.position").expect("CString::new failed"),
            _ambient: CString::new("light.ambient").expect("CString::new failed"),
            _diffuse: CString::new("light.diffuse").expect("CString::new failed"),
            _specular: CString::new("light.specular").expect("CString::new failed"),
            _constant: CString::new("light.constant").expect("CString::new failed"),
            _linear: CString::new("light.linear").expect("CString::new failed"),
            _quadratic: CString::new("light.quadratic").expect("CString::new failed"),
        }
    }
}

impl LightPoint {
    pub fn pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.set_vec3_cstr(
            gl,
            &self._position,
            self.position.x,
            self.position.y,
            self.position.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._ambient,
            self.ambient.x,
            self.ambient.y,
            self.ambient.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._diffuse,
            self.diffuse.x,
            self.diffuse.y,
            self.diffuse.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._specular,
            self.specular.x,
            self.specular.y,
            self.specular.z,
        );

        shader.set_f32_cstr(gl, &self._constant, self.constant);
        shader.set_f32_cstr(gl, &self._linear, self.linear);
        shader.set_f32_cstr(gl, &self._quadratic, self.quadratic);
    }
}
pub struct MaterialSolid {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f32,
    _ambient: CString,
    _diffuse: CString,
    _specular: CString,
    _shininess: CString,
}

impl Default for MaterialSolid {
    fn default() -> Self {
        MaterialSolid {
            ambient: Vec3::default(),
            diffuse: Vec3::default(),
            specular: Vec3::default(),
            shininess: 32.0,
            _ambient: CString::new("material.ambient").expect("CString::new failed"),
            _diffuse: CString::new("material.diffuse").expect("CString::new failed"),
            _specular: CString::new("material.specular").expect("CString::new failed"),
            _shininess: CString::new("material.shininess").expect("CString::new failed"),
        }
    }
}

impl MaterialSolid {
    pub fn pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.set_vec3_cstr(
            gl,
            &self._ambient,
            self.ambient.x,
            self.ambient.y,
            self.ambient.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._diffuse,
            self.diffuse.x,
            self.diffuse.y,
            self.diffuse.z,
        );

        shader.set_vec3_cstr(
            gl,
            &self._specular,
            self.specular.x,
            self.specular.y,
            self.specular.z,
        );

        shader.set_f32_cstr(gl, &self._shininess, self.shininess);
    }
}

pub struct MaterialTex {
    pub diffuse: i32,
    pub specular: Vec3,
    pub shininess: f32,
    _diffuse: CString,
    _specular: CString,
    _shininess: CString,
}

impl Default for MaterialTex {
    fn default() -> Self {
        MaterialTex {
            diffuse: 0,
            specular: Vec3::default(),
            shininess: 32.0,
            _diffuse: CString::new("material.diffuse").expect("CString::new failed"),
            _specular: CString::new("material.specular").expect("CString::new failed"),
            _shininess: CString::new("material.shininess").expect("CString::new failed"),
        }
    }
}

impl MaterialTex {
    pub fn pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.set_i32_cstr(gl, &self._diffuse, self.diffuse);

        shader.set_vec3_cstr(
            gl,
            &self._specular,
            self.specular.x,
            self.specular.y,
            self.specular.z,
        );

        shader.set_f32_cstr(gl, &self._shininess, self.shininess);
    }
}

pub struct MaterialTexMap {
    pub diffuse: i32,
    pub specular: i32,
    pub shininess: f32,
    _diffuse: CString,
    _specular: CString,
    _shininess: CString,
}

impl Default for MaterialTexMap {
    fn default() -> Self {
        MaterialTexMap {
            diffuse: 0,
            specular: 1,
            shininess: 32.0,
            _diffuse: CString::new("material.diffuse").expect("CString::new failed"),
            _specular: CString::new("material.specular").expect("CString::new failed"),
            _shininess: CString::new("material.shininess").expect("CString::new failed"),
        }
    }
}

impl MaterialTexMap {
    pub fn pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.set_i32_cstr(gl, &self._diffuse, self.diffuse);
        shader.set_i32_cstr(gl, &self._specular, self.specular);
        shader.set_f32_cstr(gl, &self._shininess, self.shininess);
    }
}

pub struct VSMatrices {
    pub projection: Mat4,
    pub model: Mat4,
    pub view: Mat4,
    _projection: CString,
    _model: CString,
    _view: CString,
}

impl Default for VSMatrices {
    fn default() -> Self {
        VSMatrices {
            projection: Mat4::default(),
            model: Mat4::default(),
            view: Mat4::default(),
            _projection: CString::new("projection").expect("CString::new failed"),
            _model: CString::new("model").expect("CString::new failed"),
            _view: CString::new("view").expect("CString::new failed"),
        }
    }
}

impl VSMatrices {
    pub fn pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.set_mat4fv_uv(gl, "projection", &self.projection);
        shader.set_mat4fv_uv(gl, "model", &self.model);
        shader.set_mat4fv_uv(gl, "view", &self.view);
    }
}
