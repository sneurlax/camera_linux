// src/lib.rs

use opencv::{
    core, imgcodecs, highgui, prelude::*, videoio,
};
use std::time::Instant;

#[no_mangle]
pub extern "C" fn initialize_camera() -> bool {
    let window = "Video Capture";
    if let Err(_) = highgui::named_window(window, highgui::WINDOW_AUTOSIZE) {
        return false;
    }

    let mut cam = match videoio::VideoCapture::new(0, videoio::CAP_ANY) {
        Ok(cam) => cam,
        Err(_) => return false,
    };

    if !videoio::VideoCapture::is_opened(&cam).unwrap_or(false) {
        return false;
    }

    let mut img = core::Mat::default();
    let start = Instant::now();

    loop {
        if let Err(_) = cam.read(&mut img) {
            return false;
        }

        if img.size().unwrap().width > 0 {
            if let Err(_) = highgui::imshow(window, &img) {
                return false;
            }

            let filename = format!("frame_{}.jpg", start.elapsed().as_secs());
            if let Err(_) = imgcodecs::imwrite(&filename, &img, &opencv::types::VectorOfint::new()) {
                return false;
            }
        }

        if highgui::wait_key(10).unwrap_or(-1) != -1 {
            break;
        }
    }

    true
}
