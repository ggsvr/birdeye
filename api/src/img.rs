#![allow(dead_code)]
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
    Rgb,
    Pixel,
};

pub use image;

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

        for color in color_data.colors.iter() {
            if eq_tolerance(pixel.2.to_rgb(), *color, color_data.tolerance) {
                result.put_pixel(pixel.0, pixel.1, color.to_rgba());
            }
        }
    }

    result
}

// get points from image
pub fn get_points(image: &DynamicImage, color_data: &ColorData) -> PointData {


    // back, front, and destination points array
    //let mut points: [Option<Point>; 3] = [None, None, None];
    let mut point_data = PointData::new();

    for pixel in image.pixels() {

        let pixel_col = pixel.2.to_rgb();
        // which point to choose based on color
        let mut point_ref: Option<&mut Pt> = None;

        // iterate through colors, and check if corresponds to any point's color

        for (i, col) in color_data.colors.iter().enumerate() {
            if eq_tolerance(pixel_col, *col, color_data.tolerance) {
                point_ref = Some(&mut point_data.points[i]);
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

    point_data
}

// compare colors with tolerance
fn eq_tolerance(col1: Color, col2: Color, tolerance: f64) -> bool {

    // tolerance should always be between 0 and 1; Like 0% and 100% tolerance
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

    const IMG_PATH: &str = "../color_img.jpg";

    fn setup_color() -> ColorData {
        ColorData::new(
            Color::from([0x00,0x11,0x69]),
            Color::from([0x02,0x4B,0x2D]),
            Color::from([0x78,0x09,0x08]),
            0.15
        )
    }


    #[test]
    fn isolate_test() {
        let img = ImageReader::open(IMG_PATH).unwrap().decode().unwrap();

        let result = isolate(&img, setup_color());
        result.save("/home/ggsvr/dev/rust/birdeye/isolated.png").unwrap();
    }

    #[test]
    fn points_test() {
        use crate::math::Point;

        let col = setup_color();
        let img = ImageReader::open(IMG_PATH).unwrap().decode().unwrap();

        let point_data = get_points(&img, &col);
        let points = [
            point_data.back().unwrap(),
            point_data.front().unwrap(),
            point_data.dest().unwrap(),
        ];
        println!("{:?}", points);

        let mut result = DynamicImage::new_rgb8(img.dimensions().0, img.dimensions().1);

        for (x, y, _) in img.pixels() {

            let pixel_pt = Point::new(x.into(), y.into());

            for (index, point) in points.iter().enumerate() {
                if point.distance2(pixel_pt) < 50. {
                    result.put_pixel(x, y, col.colors[index].to_rgba());
                }

            }
        }


        result.save("points.png").unwrap();



    }
}