use opencv::{
    core::{self, UMat, UMatUsageFlags, KeyPoint},
    features2d::{SimpleBlobDetector, SimpleBlobDetector_Params},
    imgproc,
    prelude::*,
    types::VectorOfUMat
};

use api::data::*;

pub fn mask_and_get_points(mat: &mut UMat, points: &ColorData) -> () {
    //let detector_params = SimpleBlobDetector_Params::default().unwrap();
    //let detector = SimpleBlobDetector::create(detector_params).unwrap();

    //let mut keypoints: Vec<KeyPoint> = Vec::new();
    //detector.detect(&mat, &mut keypoints, )
}

pub fn mask_red(mat: &UMat, tolerace: u8) -> () {
//    let mut channels: Vec<UMat> = Vec::new();
    let mut channels: VectorOfUMat = VectorOfUMat::new();
    core::split(&mat, &mut channels).unwrap();
    //imgproc::cvt_color(&mat, &mut img, imgproc::COLOR_RGB2HSV, 0).unwrap();
}