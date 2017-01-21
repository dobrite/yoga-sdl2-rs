use sdl2;
use yoga_wrapper;

pub struct Measurer<'font> {
    font: &'font sdl2::ttf::Font<'font>,
}

impl<'font> Measurer<'font> {
    pub fn new(font: &'font sdl2::ttf::Font<'font>) -> Self {
        Measurer { font: font }
    }
}

impl<'font> yoga_wrapper::Measures for Measurer<'font> {
    fn measure(&self, text: &str) -> yoga_wrapper::Size {
        let (width, height) = self.font.size_of_latin1(text.as_bytes()).unwrap();
        yoga_wrapper::Size {
            width: width as f32,
            height: height as f32,
        }
    }
}
