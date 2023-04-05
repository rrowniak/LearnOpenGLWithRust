/// https://rust-tutorials.github.io/learn-opengl/basics/index.html
extern crate open_gl;
use open_gl::demos::get_all_demos;
use std::io::prelude::*;
use std::process;

fn main() {
    let mut stderr = std::io::stderr();

    let mut arg = std::env::args();
    arg.next();
    let selected = match arg.next() {
        Some(a) => a,
        None => "demo20".to_string(),
    };

    let programs = get_all_demos();
    let prog = match programs.iter().find(|&&p| p.name() == selected) {
        Some(p) => p,
        None => {
            writeln!(&mut stderr, "Program {} not found", selected).expect("stderr failure");
            writeln!(&mut stderr, "Usage: demo <program_name>").expect("stderr failure");
            writeln!(&mut stderr, "Possibilities:").expect("stderr failure");
            for p in programs {
                writeln!(&mut stderr, "\t{}: {}", p.name(), p.description())
                    .expect("stderr failure");
            }
            process::exit(1);
        }
    };

    println!("Running {}...", prog.name());
    prog.run().unwrap_or_else(|e| {
        writeln!(&mut stderr, "Demo error: {}", e).expect("stderr failure");
        process::exit(1);
    });
}
