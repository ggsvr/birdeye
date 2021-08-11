#![allow(dead_code)]
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
    Rgb,
    Pixel,
    io::Reader as ImageReader,
};

use std::convert::TryInto;

use crate::math::*;
use crate::data::*;

// constants for convenience
pub const BLACK: Color = Rgb::<u8>([0, 0, 0]);
pub const WHITE: Color = Rgb::<u8>([255, 255, 255]);
pub const RED: Color = Rgb::<u8>([255, 0, 0]);
pub const GREEN: Color = Rgb::<u8>([0, 255, 0]);
pub const BLUE: Color = Rgb::<u8>([0, 0, 255]);


fn isolate(image: &DynamicImage, color_data: ColorData) {

    let cols = [color_data.back, color_data.front, color_data.dest];

    let mut result = DynamicImage::new_rgb8(image.dimensions().0, image.dimensions().1);

    for pixel in image.pixels() {
        for color in cols {
            if eq_tolerance(pixel.2.to_rgb(), color, color_data.tolerance) {
                result.put_pixel(pixel.0, pixel.1, color.to_rgba());
            }
        }
    }

    result.save("/home/ggsvr/dev/rust/birdeye/isolated.png").unwrap();
}

// get points from image
pub fn get_points(image: &DynamicImage, color_data: ColorData) -> PointData {


    // back, front, and destination points array
    let mut points: [Option<Point>; 3] = [None, None, None];
    let point_cols: [Color; 3] = [color_data.back, color_data.front, color_data.dest];

    for pixel in image.pixels() {

        let pixel_col = pixel.2.to_rgb();
        // which point to choose based on color
        let mut point_ref: Option<&mut Option<Point>> = None;

        // iterate through colors, and check if corresponds to any point's color
        for i in 0..3 {
            if eq_tolerance(pixel_col, point_cols[i], color_data.tolerance) {
                point_ref = Some(&mut points[i]);
            }
        }

        match point_ref {
            Some(pt_opt) => {
                match pt_opt {
                    // if it already exists, get average between current and new pixels,
                    Some(pt) => {
                        *pt = pt.midpoint(Point::new(pixel.0 as f64, pixel.1 as f64));
                    },
                    // if not, create new from pixel position
                    None => *pt_opt = Some(Point::new(pixel.0 as f64, pixel.1 as f64)),
                }
            },
            // if is None, The pixel color did not match any point, do nothing.
            None => (),
        }


    }

    PointData {
        back: points[0],
        front: points[1],
        dest: points[2],
    }
}

// compare colors with tolerance
fn eq_tolerance(col1: Color, col2: Color, tolerance: f64) -> bool {

    // tolerance shouls always be between 0 and 1; Like 0% and 100% tolerance
    let tolerance = tolerance.clamp(0., 1.);

    // iterate through components, checking if first is in the second's range of tolerance
    for index in 0..3 {
        let c1 = col1[index] as f64;
        let c2 = col2[index] as f64;

        if !((c1 >= c2 - c2*tolerance) && (c1 <= c2 + c2*tolerance)) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tolerance_test() {

        assert!(eq_tolerance(BLACK, WHITE, 1.));
        assert!(!eq_tolerance(BLACK, WHITE, 0.5));

        assert!(eq_tolerance(GREEN, GREEN, 0.));
        assert!(eq_tolerance(GREEN, GREEN, 0.2));


    }


    #[test]
    fn isolate_test() {
        let img = ImageReader::open("/home/ggsvr/dev/rust/birdeye/test_image.jpg").unwrap().decode().unwrap();

        let color_data = ColorData {
            back: Color::from([29,110,108]),
            front: Color::from([105, 58, 46]),
            dest: Color::from([2, 42, 108]),
            tolerance: 0.3,
        };

        isolate(&img, color_data);
    }
}