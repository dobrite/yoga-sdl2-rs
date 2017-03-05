use sdl2;
use yoga;
use yoga_wrapper;

use measurer;

pub struct Builder<'meas> {
    measurer: measurer::Measurer<'meas>,
}

impl<'meas> Builder<'meas> {
    pub fn new(font: &'meas sdl2::ttf::Font<'meas>) -> Self {
        Builder { measurer: measurer::Measurer::new(font) }
    }
}

impl<'meas> yoga::Builds<'meas, sdl2::pixels::Color> for Builder<'meas> {
    fn create_context<'text>(&'meas self,
                             text: &'text str)
                             -> Box<yoga_wrapper::Context<'text, 'meas>> {
        Box::new(yoga_wrapper::Context::new(text, &self.measurer))
    }

    fn view<'r>(&self) -> yoga::View<'r, sdl2::pixels::Color> {
        yoga::View::new()
    }

    fn text<'text>(&'meas self, text: &'text str) -> yoga::Text<'text, 'meas, sdl2::pixels::Color> {
        yoga::Text::new(text, self.create_context(text))
    }
}
