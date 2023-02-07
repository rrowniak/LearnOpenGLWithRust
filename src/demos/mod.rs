mod common;
mod demo1;
mod demo10;
mod demo11;
mod demo2;
mod demo3;
mod demo4;
mod demo5;
mod demo6;
mod demo7;
mod demo8;
mod demo9;

static DEMO1: demo1::Demo1 = demo1::Demo1 {
    name: "demo1",
    description: "Learn OpenGL - Graphics Programming ('Getting started'): triangles, shaders",
};

static DEMO2: demo2::Demo2 = demo2::Demo2 {
    name: "demo2",
    description: "Learn OpenGL - Graphics Programming ('Getting started'): textures",
};

static DEMO3: demo3::Demo3 = demo3::Demo3 {
    name: "demo3",
    description: "Learn OpenGL - Graphics Programming ('Getting started'): textures - advanced",
};

static DEMO4: demo4::Demo4 = demo4::Demo4 {
    name: "demo4",
    description: "Learn OpenGL - Graphics Programming ('Getting started'): transformations",
};

static DEMO5: demo5::Demo5 = demo5::Demo5 {
    name: "demo5",
    description: "Learn OpenGL - Graphics Programming ('Getting started'): moving camera",
};

static DEMO6: demo6::DemoN = demo6::DemoN {
    name: "demo6",
    description: "Learn OpenGL - Graphics Programming ('Lighting'): lighting",
};

static DEMO7: demo7::DemoN = demo7::DemoN {
    name: "demo7",
    description: "Learn OpenGL - Graphics Programming ('Lighting'): lighting Phong",
};

static DEMO8: demo8::DemoN = demo8::DemoN {
    name: "demo8",
    description: "Learn OpenGL - Graphics Programming ('Lighting'): lighting - materials",
};

static DEMO9: demo9::DemoN = demo9::DemoN {
    name: "demo9",
    description: "Learn OpenGL - Graphics Programming ('Lighting'): lighting maps",
};

static DEMO10: demo10::DemoN = demo10::DemoN {
    name: "demo10",
    description: "Learn OpenGL - Graphics Programming ('Lighting'): directional light",
};

static DEMO11: demo11::DemoN = demo11::DemoN {
    name: "demo11",
    description: "Learn OpenGL - Graphics Programming ('Lighting'): point lights",
};

pub trait Demo {
    fn run(&self) -> Result<(), String>;
    fn name(&self) -> String;
    fn description(&self) -> String;
}

pub fn get_all_demos() -> Vec<&'static dyn Demo> {
    vec![
        &DEMO1, &DEMO2, &DEMO3, &DEMO4, &DEMO5, &DEMO6, &DEMO7, &DEMO8, &DEMO9, &DEMO10, &DEMO11,
    ]
}
