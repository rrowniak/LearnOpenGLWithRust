use super::shaders::*;
use gl33::*;
use std::ffi::CString;
use ultraviolet::*;

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
    pub fn new(position: Vec3, ambient: Vec3, diffuse: Vec3, specular: Vec3, prefix: &str) -> Self {
        LightSolid {
            position,
            ambient,
            diffuse,
            specular,
            _position: CString::new(format!("{}.position", prefix)).expect("CString::new failed"),
            _ambient: CString::new(format!("{}.ambient", prefix)).expect("CString::new failed"),
            _diffuse: CString::new(format!("{}.diffuse", prefix)).expect("CString::new failed"),
            _specular: CString::new(format!("{}.specular", prefix)).expect("CString::new failed"),
        }
    }
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

pub struct DirLight {
    pub direction: Vec3,
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    _direction: CString,
    _ambient: CString,
    _diffuse: CString,
    _specular: CString,
}

impl Default for DirLight {
    fn default() -> Self {
        DirLight {
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

impl DirLight {
    pub fn new(
        direction: Vec3,
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
        prefix: &str,
    ) -> Self {
        DirLight {
            direction,
            ambient,
            diffuse,
            specular,
            _direction: CString::new(format!("{}.direction", prefix)).expect("CString::new failed"),
            _ambient: CString::new(format!("{}.ambient", prefix)).expect("CString::new failed"),
            _diffuse: CString::new(format!("{}.diffuse", prefix)).expect("CString::new failed"),
            _specular: CString::new(format!("{}.specular", prefix)).expect("CString::new failed"),
        }
    }
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
pub struct PointLight {
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

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
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

impl PointLight {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn new(
        position: Vec3,
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
        constant: f32,
        linear: f32,
        quadratic: f32,
        prefix: &str,
    ) -> Self {
        PointLight {
            position,
            ambient,
            diffuse,
            specular,
            constant,
            linear,
            quadratic,
            _position: CString::new(format!("{}.position", prefix)).expect("CString::new failed"),
            _ambient: CString::new(format!("{}.ambient", prefix)).expect("CString::new failed"),
            _diffuse: CString::new(format!("{}.diffuse", prefix)).expect("CString::new failed"),
            _specular: CString::new(format!("{}.specular", prefix)).expect("CString::new failed"),
            _constant: CString::new(format!("{}.constant", prefix)).expect("CString::new failed"),
            _linear: CString::new(format!("{}.linear", prefix)).expect("CString::new failed"),
            _quadratic: CString::new(format!("{}.quadratic", prefix)).expect("CString::new failed"),
        }
    }

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

pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub cut_off: f32,
    pub outer_cut_off: f32,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    _position: CString,
    _direction: CString,
    _cut_off: CString,
    _outer_cut_off: CString,
    _ambient: CString,
    _diffuse: CString,
    _specular: CString,
    _constant: CString,
    _linear: CString,
    _quadratic: CString,
}

impl Default for SpotLight {
    fn default() -> Self {
        SpotLight {
            position: Vec3::default(),
            direction: Vec3::default(),
            cut_off: 0.0,
            outer_cut_off: 0.0,
            ambient: Vec3::default(),
            diffuse: Vec3::default(),
            specular: Vec3::default(),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
            _position: CString::new("light.position").expect("CString::new failed"),
            _direction: CString::new("light.direction").expect("CString::new failed"),
            _cut_off: CString::new("light.cutOff").expect("CString::new failed"),
            _outer_cut_off: CString::new("light.outerCutOff").expect("CString::new failed"),
            _ambient: CString::new("light.ambient").expect("CString::new failed"),
            _diffuse: CString::new("light.diffuse").expect("CString::new failed"),
            _specular: CString::new("light.specular").expect("CString::new failed"),
            _constant: CString::new("light.constant").expect("CString::new failed"),
            _linear: CString::new("light.linear").expect("CString::new failed"),
            _quadratic: CString::new("light.quadratic").expect("CString::new failed"),
        }
    }
}

impl SpotLight {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn new(
        position: Vec3,
        direction: Vec3,
        cut_off: f32,
        outer_cut_off: f32,
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
        constant: f32,
        linear: f32,
        quadratic: f32,
        prefix: &str,
    ) -> Self {
        SpotLight {
            position,
            direction,
            cut_off,
            outer_cut_off,
            ambient,
            diffuse,
            specular,
            constant,
            linear,
            quadratic,
            _position: CString::new(format!("{}.position", prefix)).expect("CString::new failed"),
            _direction: CString::new(format!("{}.direction", prefix)).expect("CString::new failed"),
            _cut_off: CString::new(format!("{}.cutOff", prefix)).expect("CString::new failed"),
            _outer_cut_off: CString::new(format!("{}.outerCutOff", prefix))
                .expect("CString::new failed"),
            _ambient: CString::new(format!("{}.ambient", prefix)).expect("CString::new failed"),
            _diffuse: CString::new(format!("{}.diffuse", prefix)).expect("CString::new failed"),
            _specular: CString::new(format!("{}.specular", prefix)).expect("CString::new failed"),
            _constant: CString::new(format!("{}.constant", prefix)).expect("CString::new failed"),
            _linear: CString::new(format!("{}.linear", prefix)).expect("CString::new failed"),
            _quadratic: CString::new(format!("{}.quadratic", prefix)).expect("CString::new failed"),
        }
    }
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
            &self._direction,
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );

        shader.set_f32_cstr(gl, &self._cut_off, self.cut_off);
        shader.set_f32_cstr(gl, &self._outer_cut_off, self.outer_cut_off);

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
    pub fn try_pass_uniforms(&self, gl: &GlFns, shader: &Shaders) {
        shader.try_set_mat4fv_uv(gl, "projection", &self.projection);
        shader.try_set_mat4fv_uv(gl, "model", &self.model);
        shader.try_set_mat4fv_uv(gl, "view", &self.view);
    }
}
