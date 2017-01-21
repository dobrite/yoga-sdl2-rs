extern crate sdl2;
extern crate yoga;
extern crate yoga_sdl2;
extern crate yoga_wrapper;

use std::path::Path;

use yoga::Backend;

use sdl2_utils::Events;

mod sdl2_utils;

fn main() {
    let path = Path::new("./examples/assets/fonts/Monospace.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(path, 16).unwrap();

    let sdl_context = sdl2::init().unwrap();

    let renderer = {
        let video = sdl_context.video().unwrap();

        let window = video.window("yoga-sdl2-rs c example", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        window.renderer().accelerated().build().unwrap()
    };

    let mut be = yoga_sdl2::Backend::new(renderer, &font);

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let mut root = yoga_wrapper::Node::new();
    root.set_width(500.0);
    root.set_height(120.0);
    root.set_flex_direction(yoga_wrapper::FlexDirection::Row);
    root.set_padding(yoga_wrapper::Edge::All, 20.0);

    let mut text = yoga_wrapper::Node::new();
    text.set_measure_func(yoga_wrapper::measure);
    text.set_context(&mut be.create_context("yo!!!"));

    root.insert_child(&text, 0);

    root.calculate_layout();

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        be.render(&root);
    }
}
