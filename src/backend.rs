use sdl2;
use yoga;
use yoga_wrapper;

use sdl2::pixels::Color;

use yoga::Renders;

use measurer;
use renderer;

// TODO could create font from passed in string path
// to remove this lifetime
// maybe also Renderer?
pub struct Backend<'a> {
    renderer: renderer::Renderer<'a>,
    measurer: measurer::Measurer<'a>,
}

impl<'a> Backend<'a> {
    pub fn new(renderer: sdl2::render::Renderer<'a>, font: &'a sdl2::ttf::Font<'a>) -> Backend<'a> {
        Backend {
            renderer: renderer::Renderer::new(renderer, font),
            measurer: measurer::Measurer::new(font),
        }
    }
}

impl<'a, 'meas> yoga::Backend<'meas> for Backend<'a> {
    type Color = i32;
    type Renderer = renderer::Renderer<'a>;
    type Measurer = measurer::Measurer<'a>;

    fn render(&mut self, node: &yoga_wrapper::Node) {
        self.renderer.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.renderer.clear();
        self.renderer.render(node);
        self.renderer.renderer.present();
    }

    // TODO maybe remove this from trait
    fn get_renderer(&mut self) -> &mut renderer::Renderer<'a> {
        &mut self.renderer
    }

    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas> {
        yoga_wrapper::Context::new(text, &self.measurer)
    }
}
