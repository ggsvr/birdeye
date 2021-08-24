use image::Rgb;
use std::vec;

pub type Inner = Rgb<u8>;

pub struct ColorData {

    pub colors: [Inner; 3],

    pub tolerance: f64,
}
