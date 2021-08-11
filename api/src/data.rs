use image::Rgb;
use crate::math::Point;

pub type Color = Rgb<u8>;

pub struct ColorData {

    pub back: Color,
    pub front: Color,
    pub dest: Color,
    pub tolerance: f64,
}

pub struct PointData {
    pub back: Option<Point>,
    pub front: Option<Point>,
    pub dest: Option<Point>,
}