use rscam::{Camera};
use std::fs;
use std::io::Write;

const CAM_FORMAT: &[u8] = b"MJPG";
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut cam = rscam::new("/dev/video0")?;

    let res = get_highest_res(&cam, CAM_FORMAT)?;

    let cam_config = &rscam::Config {
        interval: (1, 30), // 30 fps
        resolution: res,
        format: CAM_FORMAT,
        ..Default::default()
    };

    cam.start(cam_config)?;

    let frame = cam.capture()?;
    let mut file = fs::File::create("frame.jpg")?;
    file.write_all(&frame[..])?;

    Ok(())
}

// get highest camera resolution to use
fn get_highest_res(camera: &Camera, format: &[u8]) -> rscam::Result<(u32, u32)> {
    use rscam::ResolutionInfo;

    let res_info = camera.resolutions(format)?; // get resolutions
    let mut resolutions: Vec<(u32, u32)>;

    // get resolutions in vector
    // TODO: Implement Stepwise resolutions
    if let ResolutionInfo::Discretes(vec) = res_info {
        resolutions = vec;
    }
    else {
        eprintln!("Stepwise resolutions not implemented.");
        return Err(rscam::Error::BadResolution);
    }


    resolutions.sort_unstable(); // get highest resolution, handling empty vector
    match resolutions.last() {
        Some(res) => Ok(*res),
        None => {
            eprintln!("No resolutions, somehow.");
            Err(rscam::Error::BadResolution)
        }
    }
}