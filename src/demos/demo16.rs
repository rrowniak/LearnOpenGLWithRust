use super::common::*;
use crate::demos::Demo;
use crate::gfx::camera::{CamMovement, Camera};
use crate::gfx::lights::VSMatrices;
use crate::gfx::models::*;
use crate::gfx::shaders::*;
use crate::gfx::{framebuffer::*, glutils::*, system, system::IoEvents, utils::*};
use std::time::Instant;
use ultraviolet::*;

pub struct DemoN {
    pub name: &'static str,
    pub description: &'static str,
}

impl_demo_trait!(DemoN);

impl DemoN {
    fn main(&self) -> Result<(), String> {
        let mut system = system::System::new(800, 600);

        let mut demo = DemoImpl::new();
        demo.init(&system)?;

        loop {
            if !system.process_io_events() {
                break;
            } else {
                // logic code here
                demo.update_logic(&system)?;
                // end logic code

                system.clear_screen(0.1, 0.1, 0.1);
                unsafe {
                    system.gl.Clear(gl33::GL_COLOR_BUFFER_BIT);
                }
                // graphics render here
                demo.render(&system)?;
                // end graphics render

                system.draw_to_screen();
            }
        }
        Ok(())
    }
}

const KEY_UP: u16 = 0b_0000_0000_0000_0001;
const KEY_DOWN: u16 = 0b_0000_0000_0000_0010;
const KEY_LEFT: u16 = 0b_0000_0000_0000_0100;
const KEY_RIGHT: u16 = 0b_0000_0000_0000_1000;
const KEY_PGUP: u16 = 0b_0000_0000_0001_0000;
const KEY_PGDOWN: u16 = 0b_0000_0000_0010_0000;
const KEY_HOME: u16 = 0b_0000_0000_0100_0000;
const KEY_END: u16 = 0b_0000_0000_1000_0000;
const KEY_INS: u16 = 0b_0000_0001_0000_0000;
const KEY_DEL: u16 = 0b_0000_0010_0000_0000;

type ModelWrapT = Option<Box<Model>>;

const VERTEX_CODE: &str = "
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoords;

out vec2 TexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    TexCoords = aTexCoords;    
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
} 
";

const FRAGMENT_CODE: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture1;

void main()
{    
    FragColor = texture(texture1, TexCoords);
}
";

const FRAGMENT_CODE_INV: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture1;

void main()
{    
    FragColor = vec4(vec3(1.0 - texture(texture1, TexCoords)), 1.0);
}
";

const FRAGMENT_CODE_GREYSCALE: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture1;

void main()
{    
    FragColor = texture(texture1, TexCoords);
    float average = 0.2126 * FragColor.r + 0.7152 * FragColor.g + 0.0722 * FragColor.b;
    FragColor = vec4(average, average, average, 1.0);
}
";

const FRAGMENT_CODE_KERN1: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture1;

const float offset = 1.0 / 300.0;

void main()
{    
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right    
    );

    float kernel[9] = float[](
        -1, -1, -1,
        -1,  9, -1,
        -1, -1, -1
    );
    
    vec3 sampleTex[9];
    for(int i = 0; i < 9; i++)
    {
        sampleTex[i] = vec3(texture(texture1, TexCoords.st + offsets[i]));
    }
    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++)
        col += sampleTex[i] * kernel[i];
    
    FragColor = vec4(col, 1.0);
}
";

const FRAGMENT_CODE_KERN2: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture1;

const float offset = 1.0 / 300.0;

void main()
{    
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right    
    );

    float kernel[9] = float[](
        1.0 / 16, 2.0 / 16, 1.0 / 16,
        2.0 / 16, 4.0 / 16, 2.0 / 16,
        1.0 / 16, 2.0 / 16, 1.0 / 16  
    );

    vec3 sampleTex[9];
    for(int i = 0; i < 9; i++)
    {
        sampleTex[i] = vec3(texture(texture1, TexCoords.st + offsets[i]));
    }
    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++)
        col += sampleTex[i] * kernel[i];
    
    FragColor = vec4(col, 1.0);
}
";

const FRAGMENT_CODE_KERN3: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture1;

const float offset = 1.0 / 300.0;

void main()
{    
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right    
    );

    float kernel[9] = float[](
        1, 1, 1,
        1,-8, 1,
        1, 1, 1
    );
    
    vec3 sampleTex[9];
    for(int i = 0; i < 9; i++)
    {
        sampleTex[i] = vec3(texture(texture1, TexCoords.st + offsets[i]));
    }
    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++)
        col += sampleTex[i] * kernel[i];
    
    FragColor = vec4(col, 1.0);
}
";

pub struct DemoImpl {
    mvp: VSMatrices,
    mvp2: VSMatrices,
    obj_cube: ModelWrapT,
    tex_cube: u32,
    obj_plane: ModelWrapT,
    obj_plane2: ModelWrapT,
    tex_plane: u32,
    tex_plane_angle: f32,
    angle_dx: f32,
    shader: Shaders,
    quad_shaders: Vec<Shaders>,
    shader_cnt: u32,
    frame_buffer: FrameBuffer,
    timer: Instant,
    first_logic_pass: bool,
    camera: Camera,
    io_flags: BitFields<u16>,
}

impl DemoImpl {
    fn new() -> Self {
        DemoImpl {
            mvp: VSMatrices::default(),
            mvp2: VSMatrices::default(),
            obj_cube: ModelWrapT::None,
            tex_cube: 0,
            obj_plane: ModelWrapT::None,
            obj_plane2: ModelWrapT::None,
            tex_plane: 0,
            tex_plane_angle: 45.0,
            angle_dx: 1.0,
            shader: Shaders::default(),
            quad_shaders: Default::default(),
            shader_cnt: 0,
            frame_buffer: Default::default(),
            timer: Instant::now(),
            first_logic_pass: true,
            camera: Camera::new(),
            io_flags: BitFields::<u16>::default(),
        }
    }

    fn init(&mut self, system: &system::System) -> Result<(), String> {
        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        self.obj_cube = ModelWrapT::Some(Box::new(setup_model_box(DEFAULT_POS_NORM_TEX_CUBE_VERT)));
        self.obj_plane = ModelWrapT::Some(Box::new(setup_model_plane(DEFAULT_PLANE)));
        self.obj_plane2 = ModelWrapT::Some(Box::new(setup_model_plane(DEFAULT_PLANE2)));

        self.obj_cube.as_mut().unwrap().setup(&system.gl)?;
        self.obj_plane.as_mut().unwrap().setup(&system.gl)?;
        self.obj_plane2.as_mut().unwrap().setup(&system.gl)?;

        self.tex_cube = load_texture(&system.gl, "./demo/marble.jpg")?;
        self.tex_plane = load_texture(&system.gl, "./demo/metal.png")?;

        self.shader = Shaders::from_files(&system.gl, "./demo/demo15.vs", "./demo/demo15.fs")?;

        self.quad_shaders
            .push(Shaders::from_str(&system.gl, VERTEX_CODE, FRAGMENT_CODE)?);

        self.quad_shaders.push(Shaders::from_str(
            &system.gl,
            VERTEX_CODE,
            FRAGMENT_CODE_INV,
        )?);

        self.quad_shaders.push(Shaders::from_str(
            &system.gl,
            VERTEX_CODE,
            FRAGMENT_CODE_GREYSCALE,
        )?);

        self.quad_shaders.push(Shaders::from_str(
            &system.gl,
            VERTEX_CODE,
            FRAGMENT_CODE_KERN1,
        )?);

        self.quad_shaders.push(Shaders::from_str(
            &system.gl,
            VERTEX_CODE,
            FRAGMENT_CODE_KERN2,
        )?);

        self.quad_shaders.push(Shaders::from_str(
            &system.gl,
            VERTEX_CODE,
            FRAGMENT_CODE_KERN3,
        )?);

        self.frame_buffer = FrameBuffer::new(&system.gl);
        self.frame_buffer.bind(&system.gl);
        self.frame_buffer
            .attach_texture(&system.gl, system.w, system.h);
        self.frame_buffer
            .attach_depth_stencil(&system.gl, system.w, system.h);
        self.frame_buffer
            .attach_render_buffer(&system.gl, system.w, system.h);
        self.frame_buffer.check_success_or_panic(&system.gl);
        self.frame_buffer.unbind(&system.gl);

        Ok(())
    }

    fn update_logic(&mut self, system: &system::System) -> Result<(), String> {
        if self.timer.elapsed().as_millis() > 10 || self.first_logic_pass {
            self.first_logic_pass = false;
            self.timer = Instant::now();

            self.process_io(system);

            self.tex_plane_angle += self.angle_dx;

            if self.tex_plane_angle > 90.0 || self.tex_plane_angle < 20.0 {
                self.angle_dx *= -1.0;
            }

            self.shader_cnt += 1;
            if self.shader_cnt >= self.quad_shaders.len() as u32 * 300 {
                self.shader_cnt = 0;
            }
        }

        Ok(())
    }

    fn process_io(&mut self, system: &system::System) {
        // process io
        for io in system.events.iter() {
            match io {
                IoEvents::MouseMotion(_, _, dx, dy) => {
                    self.camera
                        .process_mouse_movement(*dx as f32, *dy as f32, false);
                }
                IoEvents::KeyDown(key_code) => match *key_code {
                    system::KEY_DOWN => self.io_flags.set(KEY_DOWN),
                    system::KEY_UP => self.io_flags.set(KEY_UP),
                    system::KEY_LEFT => self.io_flags.set(KEY_LEFT),
                    system::KEY_RIGHT => self.io_flags.set(KEY_RIGHT),
                    system::KEY_PAGEDOWN => self.io_flags.set(KEY_PGDOWN),
                    system::KEY_PAGEUP => self.io_flags.set(KEY_PGUP),
                    system::KEY_HOME => self.io_flags.set(KEY_HOME),
                    system::KEY_END => self.io_flags.set(KEY_END),
                    system::KEY_INSERT => self.io_flags.set(KEY_INS),
                    system::KEY_DELETE => self.io_flags.set(KEY_DEL),
                    _ => {}
                },
                IoEvents::KeyUp(key_code) => match *key_code {
                    system::KEY_DOWN => self.io_flags.unset(KEY_DOWN),
                    system::KEY_UP => self.io_flags.unset(KEY_UP),
                    system::KEY_LEFT => self.io_flags.unset(KEY_LEFT),
                    system::KEY_RIGHT => self.io_flags.unset(KEY_RIGHT),
                    system::KEY_PAGEDOWN => self.io_flags.unset(KEY_PGDOWN),
                    system::KEY_PAGEUP => self.io_flags.unset(KEY_PGUP),
                    system::KEY_HOME => self.io_flags.unset(KEY_HOME),
                    system::KEY_END => self.io_flags.unset(KEY_END),
                    system::KEY_INSERT => self.io_flags.unset(KEY_INS),
                    system::KEY_DELETE => self.io_flags.unset(KEY_DEL),
                    _ => {}
                },
                IoEvents::MouseWheel(_, dy) => {
                    self.camera.process_mouse_scroll(*dy as f32);
                    self.build_projection_matrix(system, self.camera.zoom.to_radians());
                }
                _ => {}
            }
        }

        let delta_t = 0.05;
        let look_speed = 5.0;

        if self.io_flags.is_set(KEY_UP) {
            self.camera.process_keyboard(CamMovement::Forward, delta_t)
        }
        if self.io_flags.is_set(KEY_DOWN) {
            self.camera.process_keyboard(CamMovement::Backward, delta_t)
        }
        if self.io_flags.is_set(KEY_LEFT) {
            self.camera.process_keyboard(CamMovement::Left, delta_t)
        }
        if self.io_flags.is_set(KEY_RIGHT) {
            self.camera.process_keyboard(CamMovement::Right, delta_t)
        }

        if self.io_flags.is_set(KEY_PGUP) {
            self.camera.process_keyboard(CamMovement::Up, delta_t)
        }
        if self.io_flags.is_set(KEY_PGDOWN) {
            self.camera.process_keyboard(CamMovement::Down, delta_t)
        }
        if self.io_flags.is_set(KEY_HOME) {
            self.camera.process_mouse_movement(look_speed, 0.0, false);
        }
        if self.io_flags.is_set(KEY_END) {
            self.camera.process_mouse_movement(-look_speed, 0.0, false);
        }
        if self.io_flags.is_set(KEY_INS) {
            self.camera.process_mouse_movement(0.0, look_speed, false);
        }
        if self.io_flags.is_set(KEY_DEL) {
            self.camera.process_mouse_movement(0.0, -look_speed, false);
        }
    }

    fn render(&mut self, system: &system::System) -> Result<(), String> {
        self.mvp.view = self.camera.get_view_matrix();

        ///////////////////////////////////////////////////////////
        // Draw to framebuffer
        ///////////////////////////////////////////////////////////
        self.frame_buffer.bind(&system.gl);
        self.frame_buffer
            .clear(&system.gl, Vec4::new(0.2, 0.2, 0.2, 1.0));

        // Draw Plane
        self.mvp.model = Mat4::default();
        self.draw_plane(&system.gl);

        // Draw Cube 1
        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(0.0, 0.0, -4.5));
        self.draw_cube(&system.gl);

        // Draw Cube 2
        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(1.0, 0.0, 0.0));
        self.draw_cube(&system.gl);

        // Draw Cube 3
        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(-1.0, 0.0, 0.0));
        self.draw_cube(&system.gl);

        self.frame_buffer.unbind(&system.gl);

        ///////////////////////////////////////////////////////////
        // Draw to screen
        ///////////////////////////////////////////////////////////
        self.mvp2.model = Mat4::from_rotation_x(self.tex_plane_angle.to_radians());
        let dz = self.tex_plane_angle.to_radians().sin() * 10.0 - 20.0;
        self.mvp2.model.translate(&Vec3::new(0.0, 0.0, dz));
        self.draw_plane_from_fb_tex(&system.gl);

        Ok(())
    }

    fn build_projection_matrix(&mut self, system: &system::System, fov_rad: f32) {
        self.mvp.projection = projection::rh_yup::perspective_gl(
            fov_rad,
            (system.w as f32) / (system.h as f32),
            0.1,
            100.0,
        );

        self.mvp2.projection = projection::rh_yup::perspective_gl(
            fov_rad,
            (system.w as f32) / (system.h as f32),
            0.1,
            100.0,
        );
    }

    fn draw_plane(&mut self, gl: &gl33::GlFns) {
        self.shader.use_program(gl);
        unsafe {
            gl.ActiveTexture(gl33::GL_TEXTURE0);
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_plane);
        }
        self.mvp.pass_uniforms(gl, &self.shader);

        self.obj_plane.as_mut().unwrap().draw(gl, &self.shader);
    }

    fn draw_cube(&mut self, gl: &gl33::GlFns) {
        self.shader.use_program(gl);
        unsafe {
            gl.ActiveTexture(gl33::GL_TEXTURE0);
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_cube);
        }

        self.mvp.pass_uniforms(gl, &self.shader);
        self.obj_cube.as_mut().unwrap().draw(gl, &self.shader);
    }

    fn draw_plane_from_fb_tex(&mut self, gl: &gl33::GlFns) {
        let shader_i = self.shader_cnt / 300;
        self.quad_shaders[shader_i as usize].use_program(gl);
        unsafe {
            gl.ActiveTexture(gl33::GL_TEXTURE0);
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.frame_buffer.tex[0]);
        }
        self.mvp2
            .pass_uniforms(gl, &self.quad_shaders[shader_i as usize]);

        self.obj_plane2
            .as_mut()
            .unwrap()
            .draw(gl, &self.quad_shaders[shader_i as usize]);
    }
}
