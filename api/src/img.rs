#![allow(dead_code)]
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
    Rgb,
    Pixel,
};


use crate::math::*;
use crate::data::*;

// constants for convenience
pub const BLACK: Color = Rgb::<u8>([0, 0, 0]);
pub const WHITE: Color = Rgb::<u8>([255, 255, 255]);
pub const RED: Color = Rgb::<u8>([255, 0, 0]);
pub const GREEN: Color = Rgb::<u8>([0, 255, 0]);
pub const BLUE: Color = Rgb::<u8>([0, 0, 255]);


fn isolate(image: &DynamicImage, color_data: ColorData) -> DynamicImage {


    let mut result = DynamicImage::new_rgb8(image.dimensions().0, image.dimensions().1);

    for pixel in image.pixels() {
        let pixel_col = pixel.2.to_rgb();

        for color in color_data.colors.iter() {
            if eq_tolerance(pixel.2.to_rgb(), *color, color_data.tolerance) {
                result.put_pixel(pixel.0, pixel.1, color.to_rgba());
            }
        }
    }

    result
}

// get points from image
pub fn get_points(image: &DynamicImage, color_data: ColorData) -> PointData {


    // back, front, and destination points array
    //let mut points: [Option<Point>; 3] = [None, None, None];
    let points = PointData {
        back: None, front: None, dest: None
    };

    for pixel in image.pixels() {

        let pixel_col = pixel.2.to_rgb();
        // which point to choose based on color
        let mut point_ref: Option<&mut Option<Point>> = None;

        // iterate through colors, and check if corresponds to any point's color
        for i in 0..3 {
            if eq_tolerance(pixel_col, color_data.colors[i], color_data.tolerance) {
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
        points
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
    use image::io::Reader as ImageReader;
    use super::*;

    #[test]
    fn tolerance_test() {

        assert!(eq_tolerance(BLACK, WHITE, 1.));
        assert!(!eq_tolerance(BLACK, WHITE, 0.5));

        assert!(eq_tolerance(GREEN, GREEN, 0.));
        assert!(eq_tolerance(GREEN, GREEN, 0.2));


    }

    const IMG_PATH: &str = "/home/ggsvr/dev/rust/birdeye/test_image.jpg";

    fn setup_color() -> ColorData {
        ColorData {
            colors: [
                Color::from([254, 118, 128]), // back
                Color::from([142, 224, 140]), // front
                Color::from([91, 122, 239]),  // dest
            ],
            tolerance: 0.2,
        }
    }


    #[test]
    fn isolate_test() {
        let img = ImageReader::open("/home/ggsvr/dev/rust/birdeye/test_image.jpg").unwrap().decode().unwrap();

        let result = isolate(&img, setup_color());
        result.save("/home/ggsvr/dev/rust/birdeye/isolated.png").unwrap();
    }

    #[test]
    fn points_test() {
        let img = ImageReader::open(IMG_PATH).unwrap().decode().unwrap();

        let point_data = get_points(&img, setup_color());
        let points = [point_data.points[BACK].unwrap(), point_data.points[FRONT].unwrap(), point_data.points[DEST].unwrap()];
        println!("{:?}", points);

        //let mut result = DynamicImage::new_rgb8(img.dimensions().0, img.dimensions().1);


        //result.save("points.png").unwrap();

    }
}