use image::Rgb;
use super::Data;

pub type Color = Rgb<u8>;

pub struct ColorData {

    pub colors: [Color; 3],

    pub tolerance: f64,
}

impl ColorData {
    pub fn new(back: Color, front: Color, dest: Color, tolerance: f64) -> Self {
        Self {
            colors: [
                back,
                front,
                dest
            ],
            tolerance
        }
    }
}

impl Data for ColorData {
    type Inner = Color;

    fn list(&self) -> &[Self::Inner; 3] {
        &self.colors
    }

    fn list_mut(&mut self) -> &mut [Self::Inner; 3] {
        &mut self.colors
    }

}
