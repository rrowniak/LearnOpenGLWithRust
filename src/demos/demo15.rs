use super::common::*;
use crate::demos::Demo;
use crate::gfx::camera::{CamMovement, Camera};
use crate::gfx::lights::VSMatrices;
use crate::gfx::models::*;
use crate::gfx::shaders::*;
use crate::gfx::{glutils::*, system, system::IoEvents, utils::*};
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

pub struct DemoImpl {
    mvp: VSMatrices,
    obj_cube: ModelWrapT,
    tex_cube: u32,
    obj_plane: ModelWrapT,
    tex_plane: u32,
    obj_grass: ModelWrapT,
    tex_grass: u32,
    obj_transparent: ModelWrapT,
    tex_transparent: u32,
    shader: Shaders,
    stencil_shader: Shaders,
    discard_shader: Shaders,
    timer: Instant,
    first_logic_pass: bool,
    camera: Camera,
    io_flags: BitFields<u16>,
}

const GRASS_POSITIONS: [Vec3; 5] = [
    Vec3::new(-1.5, 0.0, -0.48),
    Vec3::new(1.5, 0.0, 0.51),
    Vec3::new(0.0, 0.0, 0.7),
    Vec3::new(-0.3, 0.0, -2.3),
    Vec3::new(1.5, 0.0, 2.6),
];

#[rustfmt::skip]
const GRASS_QUAD: [f32; 48] = [
    // positions      // fake normals   // texture Coords (swapped y coordinates because texture is flipped upside down)
    0.0,  0.5,  0.0,  1.0, 0.0, 0.0,     0.0, 1.0, // 0.0,  0.0,
    0.0, -0.5,  0.0,  1.0, 0.0, 0.0,     0.0, 0.0, // 0.0,  1.0,
    1.0, -0.5,  0.0,  1.0, 0.0, 0.0,     1.0, 0.0, // 1.0,  1.0,

    0.0,  0.5,  0.0,  1.0, 0.0, 0.0,     0.0, 1.0, // 0.0,  0.0,
    1.0, -0.5,  0.0,  1.0, 0.0, 0.0,     1.0, 0.0,// 1.0,  1.0,
    1.0,  0.5,  0.0,  1.0, 0.0, 0.0,     1.0, 1.0,// 1.0,  0.0,
];

const WINDOW_POSITIONS: [Vec3; 5] = [
    Vec3::new(-0.3, 0.0, -2.3),
    Vec3::new(-1.7, 0.0, -1.48),
    Vec3::new(1.5, 0.0, 0.6),
    Vec3::new(1.5, 0.0, 1.51),
    Vec3::new(0.0, 0.0, 1.7),
];

#[rustfmt::skip]
const TRANSPARENT_QUAD: [f32; 48] = [
    // positions        // fake normals   // texture Coords (swapped y coordinates because texture is flipped upside down)
    0.0,  0.5,  0.0,    1.0, 0.0, 0.0,    0.0,  1.0,
    0.0, -0.5,  0.0,    1.0, 0.0, 0.0,    0.0,  0.0,
    1.0, -0.5,  0.0,    1.0, 0.0, 0.0,    1.0,  0.0,

    0.0,  0.5,  0.0,    1.0, 0.0, 0.0,    0.0,  1.0,
    1.0, -0.5,  0.0,    1.0, 0.0, 0.0,    1.0,  0.0,
    1.0,  0.5,  0.0,    1.0, 0.0, 0.0,    1.0,  1.0
];

impl DemoImpl {
    fn new() -> Self {
        DemoImpl {
            mvp: VSMatrices::default(),
            obj_cube: ModelWrapT::None,
            tex_cube: 0,
            obj_plane: ModelWrapT::None,
            tex_plane: 0,
            obj_grass: ModelWrapT::None,
            tex_grass: 0,
            obj_transparent: ModelWrapT::None,
            tex_transparent: 0,
            shader: Shaders::default(),
            stencil_shader: Shaders::default(),
            discard_shader: Shaders::default(),
            timer: Instant::now(),
            first_logic_pass: true,
            camera: Camera::new(),
            io_flags: BitFields::<u16>::default(),
        }
    }

    fn init(&mut self, system: &system::System) -> Result<(), String> {
        // for stencil
        unsafe {
            system.gl.Enable(GL_STENCIL_TEST);

            system.gl.Enable(GL_BLEND);
            system.gl.BlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            // better off enabling/discabling culling per obeject
            // system.gl.Enable(GL_CULL_FACE);
        }

        self.build_projection_matrix(system, 45.0f32.to_radians());
        self.camera.position.z += 7.0;
        self.camera.mouse_sensitivity = 0.1;

        self.obj_cube = ModelWrapT::Some(Box::new(setup_model_box(DEFAULT_POS_NORM_TEX_CUBE_VERT)));
        self.obj_plane = ModelWrapT::Some(Box::new(setup_model_plane(DEFAULT_PLANE)));
        self.obj_grass = ModelWrapT::Some(Box::new(setup_model_plane(GRASS_QUAD)));
        self.obj_transparent = ModelWrapT::Some(Box::new(setup_model_plane(TRANSPARENT_QUAD)));

        self.obj_cube.as_mut().unwrap().setup(&system.gl)?;
        self.obj_plane.as_mut().unwrap().setup(&system.gl)?;
        self.obj_grass.as_mut().unwrap().setup(&system.gl)?;
        self.obj_transparent.as_mut().unwrap().setup(&system.gl)?;

        self.tex_cube = load_texture(&system.gl, "./demo/marble.jpg")?;
        self.tex_plane = load_texture(&system.gl, "./demo/metal.png")?;
        use gl33::*;
        let params = [
            (GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE),
            (GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE),
            (GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR),
            (GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR),
        ];
        self.tex_grass = load_texture_params(&system.gl, "./demo/grass.png", &params)?;
        self.tex_transparent = load_texture_params(&system.gl, "./demo/window.png", &params)?;

        self.shader = Shaders::from_files(&system.gl, "./demo/demo15.vs", "./demo/demo15.fs")?;
        self.stencil_shader =
            Shaders::from_files(&system.gl, "./demo/demo15.vs", "./demo/demo15_monocol.fs")?;
        self.discard_shader =
            Shaders::from_files(&system.gl, "./demo/demo15.vs", "./demo/demo15_discard.fs")?;

        Ok(())
    }

    fn update_logic(&mut self, system: &system::System) -> Result<(), String> {
        if self.timer.elapsed().as_millis() > 10 || self.first_logic_pass {
            self.first_logic_pass = false;
            self.timer = Instant::now();

            self.process_io(system);
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
        // disable stencil effect
        stencil::select_eff_off(&system.gl);

        self.mvp.view = self.camera.get_view_matrix();

        // Draw Plane
        self.mvp.model = Mat4::default();
        self.draw_plane(&system.gl);

        // Draw Cube
        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(0.0, 0.0, -4.5));
        self.draw_cube(&system.gl);

        ///////////////////////////////////////////////////////////////////////////
        // Begin stencil effect
        ///////////////////////////////////////////////////////////////////////////
        stencil::select_eff_prepare(&system.gl);

        // Draw Cube stencil 1
        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(1.0, 0.0, 0.0));
        self.draw_cube(&system.gl);

        // Draw Cube stencil 2
        self.mvp.model = Mat4::default();
        self.mvp.model.translate(&Vec3::new(-1.0, 0.0, 0.0));
        self.draw_cube(&system.gl);

        ///////////////////////////////////////////////////////////////////////////
        // Draw scaled objects
        stencil::select_eff_begin(&system.gl);
        // Draw Cube stencil 1
        self.mvp.model = Mat4::from_scale(1.1);
        self.mvp.model.translate(&Vec3::new(1.0, 0.0, 0.0));
        self.draw_cube_st_eff(&system.gl);

        // Draw Cube stencil 2
        self.mvp.model = Mat4::from_scale(1.1);
        self.mvp.model.translate(&Vec3::new(-1.0, 0.0, 0.0));
        self.draw_cube_st_eff(&system.gl);

        stencil::select_eff_end(&system.gl);
        ///////////////////////////////////////////////////////////////////////////

        // draw grass
        for v in GRASS_POSITIONS {
            self.mvp.model = Mat4::default();
            self.mvp.model.translate(&v);
            self.draw_grass(&system.gl);
        }

        // draw transparent windows
        // Note: rendering order shall be from the most distant window
        for v in WINDOW_POSITIONS {
            self.mvp.model = Mat4::default();
            self.mvp.model.translate(&v);
            self.draw_window(&system.gl);
        }

        Ok(())
    }

    fn build_projection_matrix(&mut self, system: &system::System, fov_rad: f32) {
        self.mvp.projection = projection::rh_yup::perspective_gl(
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

    fn draw_cube_st_eff(&mut self, gl: &gl33::GlFns) {
        self.stencil_shader.use_program(gl);
        unsafe {
            gl.ActiveTexture(gl33::GL_TEXTURE0);
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_cube);
        }

        self.mvp.pass_uniforms(gl, &self.stencil_shader);
        self.obj_cube
            .as_mut()
            .unwrap()
            .draw(gl, &self.stencil_shader);
    }

    fn draw_grass(&mut self, gl: &gl33::GlFns) {
        self.discard_shader.use_program(gl);
        unsafe {
            gl.ActiveTexture(gl33::GL_TEXTURE0);
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_grass);
        }

        self.mvp.pass_uniforms(gl, &self.discard_shader);
        self.obj_grass
            .as_mut()
            .unwrap()
            .draw(gl, &self.discard_shader);
    }

    fn draw_window(&mut self, gl: &gl33::GlFns) {
        self.shader.use_program(gl);
        unsafe {
            gl.ActiveTexture(gl33::GL_TEXTURE0);
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.tex_transparent);
        }

        self.mvp.pass_uniforms(gl, &self.shader);
        self.obj_grass.as_mut().unwrap().draw(gl, &self.shader);
    }
}
