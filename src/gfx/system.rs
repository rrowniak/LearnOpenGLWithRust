use fermium::{error::*, events::*, keycode::*, video::*, *};
use gl33::*;

// example: https://github.com/Lokathor/fermium/blob/main/examples/controller_events.rs

const IO_DEBUG_PRINT: bool = false;
const IO_DEBUG_PRINT_VERY_NOISY: bool = false; // eg mouse motion

pub const KEY_RIGHT: i32 = SDLK_RIGHT.0;
pub const KEY_LEFT: i32 = SDLK_LEFT.0;
pub const KEY_DOWN: i32 = SDLK_DOWN.0;
pub const KEY_UP: i32 = SDLK_UP.0;

pub enum MouseButtonId {
    // x, y
    Left(i32, i32),   // button: 3
    Right(i32, i32),  // button: 1
    Middle(i32, i32), // button: 2
    Other(i32, i32),
}
pub enum IoEvents {
    Quit,
    // key code
    KeyDown(i32),
    // key code
    KeyUp(i32),
    ControllerAxisMotion(i32),
    ControllerButtonDown(i32),
    ControllerButtonUp(i32),
    // x, y, xrel, yrel
    MouseMotion(i32, i32, i32, i32),
    MouseButtonUp(MouseButtonId),
    MouseButtonDown(MouseButtonId),
    // dx, dy (usually -1 or 1 based on direction)
    MouseWheel(i32, i32),
}

pub struct System {
    pub w: usize,
    pub h: usize,
    win: *mut SDL_Window,
    pub gl: GlFns,
    pub events: Vec<IoEvents>,
}

impl System {
    pub fn new(w: usize, h: usize) -> System {
        unsafe {
            // initialize SLD with OpenGL context
            SDL_Init(SDL_INIT_VIDEO);
            if SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3) != 0 {
                panic!("SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3) failed")
            }

            if SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3) != 0 {
                panic!("SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3) failed")
            }

            if SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_PROFILE_MASK,
                SDL_GL_CONTEXT_PROFILE_CORE.0 as _,
            ) != 0
            {
                panic!("SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK,SDL_GL_CONTEXT_PROFILE_CORE.0 as _) failed")
            }

            // create windows
            let win = SDL_CreateWindow(
                b"gl33 fermium demo\0".as_ptr().cast(),
                50,
                50,
                800,
                600,
                (SDL_WINDOW_SHOWN | SDL_WINDOW_OPENGL).0 as _,
            );
            if win.is_null() {
                let mut v = Vec::with_capacity(4096);
                let mut p = SDL_GetErrorMsg(v.as_mut_ptr(), v.capacity() as _);
                while *p != 0 {
                    print!("{}", *p as u8 as char);
                    p = p.add(1);
                }
                println!();
                panic!();
            }
            // make context the window will use
            let ctx = SDL_GL_CreateContext(win);
            if ctx.0.is_null() {
                let mut v = Vec::with_capacity(4096);
                let mut p = SDL_GetErrorMsg(v.as_mut_ptr(), v.capacity() as _);
                while *p != 0 {
                    print!("{}", *p as u8 as char);
                    p = p.add(1);
                }
                println!();
                panic!();
            }

            SDL_GL_SetSwapInterval(1);

            let gl =
                GlFns::load_from(&|c_char_ptr| SDL_GL_GetProcAddress(c_char_ptr.cast())).unwrap();

            // gl_loader::init_gl();
            // let gl = GlFns::load_from(&|symbol| {
            //     let i8_symbol = std::mem::transmute(symbol);
            //     let c_char_ptr = std::ffi::CStr::from_ptr(i8_symbol).to_str().unwrap();
            //     gl_loader::get_proc_address(c_char_ptr) as *const _
            // })
            // .unwrap();

            gl.Viewport(0, 0, w as i32, h as i32);
            gl.Enable(gl33::GL_DEPTH_TEST);

            System {
                w,
                h,
                win,
                gl,
                events: Vec::<IoEvents>::new(),
            }
        }
    }

    pub fn process_io_events(&mut self) -> bool {
        unsafe {
            // if SDL_PollEvent(&mut self.event) != 0 && self.event.common.type_ == SDL_QUIT as _ {
            //     return false;
            // }
            self.events.clear();

            let mut event = SDL_Event::default();

            while SDL_PollEvent(&mut event) != 0 {
                match event.type_ {
                    SDL_QUIT => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_QUIT");
                        }
                        self.events.push(IoEvents::Quit);
                        return false;
                    }
                    SDL_KEYDOWN => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_KEYDOWN");
                            println!("{:?}", event.key);
                        }
                        self.events.push(IoEvents::KeyDown(event.key.keysym.sym.0));
                    }
                    SDL_KEYUP => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_KEYUP");
                            println!("{:?}", event.key);
                        }
                        self.events.push(IoEvents::KeyUp(event.key.keysym.sym.0));
                    }
                    SDL_CONTROLLERAXISMOTION => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERAXISMOTION");
                            println!("{:?}", event.caxis);
                        }
                    }
                    SDL_CONTROLLERBUTTONDOWN => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERBUTTONDOWN");
                            println!("{:?}", event.cbutton);
                        }
                    }
                    SDL_CONTROLLERBUTTONUP => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERBUTTONUP");
                            println!("{:?}", event.cbutton);
                        }
                    }
                    SDL_CONTROLLERDEVICEADDED => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERDEVICEADDED");
                            println!("{:?}", event.cdevice);
                        }
                        // let id = event.cdevice.which;
                        // println!("Opening joystick {} as a controller...", id);
                        // let controller = SDL_GameControllerOpen(id);
                        // if controller.is_null() {
                        //     print!("Error while opening: ");
                        //     print_error();
                        //     println!();
                        // } else {
                        //     print!("> Name: ");
                        //     let name_p = SDL_GameControllerName(controller).cast();
                        //     print_ptr(name_p);
                        //     println!();
                        //     //
                        //     println!("> Type: {:?}", SDL_GameControllerGetType(controller));
                        //     println!(
                        //         "> PlayerIndex: {:?}",
                        //         SDL_GameControllerGetPlayerIndex(controller)
                        //     );
                        //     println!("> Vendor: {:#X?}", SDL_GameControllerGetVendor(controller));
                        //     println!(
                        //         "> Product: {:#X?}",
                        //         SDL_GameControllerGetProduct(controller)
                        //     );
                        //     println!(
                        //         "> ProductVersion: {:?}",
                        //         SDL_GameControllerGetProductVersion(controller)
                        //     );
                        //     //
                        //     print!("> Serial: ");
                        //     let serial_p: *const u8 =
                        //         SDL_GameControllerGetSerial(controller).cast();
                        //     if serial_p.is_null() {
                        //         println!("not available");
                        //     } else {
                        //         print_ptr(serial_p);
                        //         println!();
                        //     }
                        // }
                    }
                    SDL_CONTROLLERDEVICEREMOVED => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERDEVICEREMOVED");
                            println!("{:?}", event.cdevice);
                        }
                        // let id = event.cdevice.which;
                        // println!("Closing ID {}...", id);
                        // let controller = SDL_GameControllerFromInstanceID(SDL_JoystickID(id));
                        // if controller.is_null() {
                        //     println!("but it was already closed!");
                        // } else {
                        //     SDL_GameControllerClose(controller);
                        // }
                    }
                    SDL_CONTROLLERDEVICEREMAPPED => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERDEVICEREMAPPED");
                            println!("{:?}", event.cdevice);
                        }
                    }
                    SDL_CONTROLLERTOUCHPADDOWN => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERTOUCHPADDOWN");
                            println!("{:?}", event.ctouchpad);
                        }
                    }
                    SDL_CONTROLLERTOUCHPADMOTION => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERTOUCHPADMOTION");
                            println!("{:?}", event.ctouchpad);
                        }
                    }
                    SDL_CONTROLLERTOUCHPADUP => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERTOUCHPADUP");
                            println!("{:?}", event.ctouchpad);
                        }
                    }
                    SDL_CONTROLLERSENSORUPDATE => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_CONTROLLERSENSORUPDATE");
                            println!("{:?}", event.ctouchpad);
                        }
                    }
                    SDL_MOUSEBUTTONDOWN => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_MOUSEBUTTONDOWN");
                            println!("{:?}", event.button);
                        }
                        match event.button.button {
                            1 => self
                                .events
                                .push(IoEvents::MouseButtonDown(MouseButtonId::Right(
                                    event.button.x,
                                    event.button.y,
                                ))),
                            2 => {
                                self.events
                                    .push(IoEvents::MouseButtonDown(MouseButtonId::Middle(
                                        event.button.x,
                                        event.button.y,
                                    )))
                            }
                            3 => self
                                .events
                                .push(IoEvents::MouseButtonDown(MouseButtonId::Left(
                                    event.button.x,
                                    event.button.y,
                                ))),
                            _ => self
                                .events
                                .push(IoEvents::MouseButtonDown(MouseButtonId::Other(
                                    event.button.x,
                                    event.button.y,
                                ))),
                        }
                    }
                    SDL_MOUSEBUTTONUP => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_MOUSEBUTTONUP");
                            println!("{:?}", event.button);
                        }
                        match event.button.button {
                            1 => self
                                .events
                                .push(IoEvents::MouseButtonUp(MouseButtonId::Right(
                                    event.button.x,
                                    event.button.y,
                                ))),
                            2 => self
                                .events
                                .push(IoEvents::MouseButtonUp(MouseButtonId::Middle(
                                    event.button.x,
                                    event.button.y,
                                ))),
                            3 => self
                                .events
                                .push(IoEvents::MouseButtonUp(MouseButtonId::Left(
                                    event.button.x,
                                    event.button.y,
                                ))),
                            _ => self
                                .events
                                .push(IoEvents::MouseButtonUp(MouseButtonId::Other(
                                    event.button.x,
                                    event.button.y,
                                ))),
                        }
                    }
                    SDL_MOUSEMOTION => {
                        if IO_DEBUG_PRINT && IO_DEBUG_PRINT_VERY_NOISY {
                            println!("SDL_MOUSEMOTION");
                            println!("{:?}", event.motion);
                        }
                        self.events.push(IoEvents::MouseMotion(
                            event.motion.x,
                            event.motion.y,
                            event.motion.xrel,
                            event.motion.yrel,
                        ));
                    }
                    SDL_MOUSEWHEEL => {
                        if IO_DEBUG_PRINT {
                            println!("SDL_MOUSEWHEEL");
                            println!("{:?}", event.wheel);
                        }
                        self.events
                            .push(IoEvents::MouseWheel(event.wheel.x, event.wheel.y));
                    }
                    _ => (),
                }
            }
            self.gl
                .Clear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
        }
        true
    }

    pub fn draw_to_screen(&mut self) {
        unsafe {
            SDL_GL_SwapWindow(self.win);
        }
    }

    pub fn clear_screen(&mut self, r: f32, g: f32, b: f32) {
        unsafe {
            self.gl.ClearColor(r, g, b, 1.0);
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.win);
            SDL_Quit();
        }
    }
}
