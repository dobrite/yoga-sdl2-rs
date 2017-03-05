use rand::Rng;

use sdl2;
use rand; // TODO not in final version
use yoga;

use yoga::Renders;

use Builder;

pub struct Renderer<'a> {
    pub renderer: sdl2::render::Renderer<'a>,
    pub font: &'a sdl2::ttf::Font<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(renderer: sdl2::render::Renderer<'a>,
               font: &'a sdl2::ttf::Font<'a>)
               -> Renderer<'a> {
        Renderer {
            renderer: renderer,
            font: font,
        }
    }
}

impl<'a> Renderer<'a> {
    fn walk(&mut self, node: &yoga::Renderable<sdl2::pixels::Color>) {
        let width = node.get_layout_width() as u32;
        let height = node.get_layout_height() as u32;
        let top = node.get_layout_top() as i32;
        let left = node.get_layout_left() as i32;

        let mut rng = rand::thread_rng();

        // TODO fix psychedelic colors!
        let color = sdl2::pixels::Color::RGB(rng.gen(), rng.gen(), rng.gen());
        self.renderer.set_draw_color(color);

        let rect = sdl2::rect::Rect::new(left, top, width, height);
        let _ = self.renderer.fill_rect(rect);

        let surface = self.font.render("yo!!!").blended(sdl2::pixels::Color::RGB(0, 0, 0)).unwrap();
        let mut texture = self.renderer.create_texture_from_surface(&surface).unwrap();
        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
        let rect = sdl2::rect::Rect::new(left, top, width, height);
        let _ = self.renderer.copy(&mut texture, None, Some(rect));

        for i in 0..node.get_child_count() {
            let child = node.get_child(i).unwrap();
            self.render(child);
        }
    }
}

impl<'a> yoga::Renders<'a> for Renderer<'a> {
    type Color = sdl2::pixels::Color;
    type Builder = Builder<'a>;

    fn render(&mut self, node: &yoga::Renderable<Self::Color>) {
        self.renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.renderer.clear();
        self.walk(node);
        self.renderer.present();
    }
}
