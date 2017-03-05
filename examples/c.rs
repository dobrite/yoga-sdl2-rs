extern crate sdl2;
extern crate yoga;
extern crate yoga_sdl2;
extern crate yoga_wrapper;

use std::path::Path;
use yoga::{Backend, Builds, Renderable};
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

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let builder = yoga_sdl2::Builder::new(&font);

    let mut text = builder.view();
    text.set_height(25.0);
    text.set_align_self(yoga_wrapper::Align::Center);
    text.set_flex_grow(1.0);

    let mut image = builder.view();
    image.set_width(80.0);
    image.set_margin(yoga_wrapper::Edge::End, 20.0);

    let mut root = builder.view();
    root.set_width(500.0);
    root.set_height(120.0);
    root.set_flex_direction(yoga_wrapper::FlexDirection::Row);
    root.set_padding(yoga_wrapper::Edge::All, 20.0);

    root.insert_child(&image, 0);
    root.insert_child(&text, 1);

    root.calculate_layout();

    let mut be = yoga_sdl2::Backend::new(renderer, &font);

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        be.render(&root);
    }
}
