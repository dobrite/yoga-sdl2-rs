use sdl2;
use yoga;

use renderer;

// TODO could create font from passed in string path
// to remove this lifetime
// maybe also Renderer?
pub struct Backend<'a> {
    renderer: renderer::Renderer<'a>,
}

impl<'a> Backend<'a> {
    pub fn new(renderer: sdl2::render::Renderer<'a>, font: &'a sdl2::ttf::Font<'a>) -> Backend<'a> {
        Backend { renderer: renderer::Renderer::new(renderer, font) }
    }
}

impl<'a> yoga::Backend<'a> for Backend<'a> {
    type Renderer = renderer::Renderer<'a>;

    fn get_renderer(&mut self) -> &mut renderer::Renderer<'a> {
        &mut self.renderer
    }
}
