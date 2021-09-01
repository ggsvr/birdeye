use opencv::{
    prelude::*,
    core::{self, UMat, UMatUsageFlags},
    highgui,
    videoio
};

use api::data::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = "Birdye Client";

    let tolerance_bar = "tolerance_trackbar";

    let mut color_data = ColorData::new(
        Color::black(),
        Color::black(),
        Color::black(),
    );


    highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

        let mut tol = 0;

        highgui::create_trackbar(
            tolerance_bar,
            window,
            &mut tol,
            100,
            None
        )?;

//        highgui::create_button

    loop {
        let mut frame = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {

            highgui::imshow(window, &frame)?;

        }
        let key = highgui::poll_key()?;
        if key != -1 {
            println!("key pressed: {}", key);
            break;
        }

    }

    Ok(())
}