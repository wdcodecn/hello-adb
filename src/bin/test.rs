use opencv::{core, imgcodecs, imgproc};

use std::time;

fn main() {
    let _now = time::Instant::now();

    // Load the images to match
    let image1 = imgcodecs::imread("screencap1.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();
    println!("screencap took {:?}", _now.elapsed());

    let image2 = imgcodecs::imread("screencap2.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();
    println!("screencap took {:?}", _now.elapsed());

    for _i in 0..10 {
        // Match the images using template matching
        let mut result = core::Mat::default();

        let _now1 = time::Instant::now();

        imgproc::match_template(
            &image1,
            &image2,
            &mut result,
            imgproc::TM_CCOEFF_NORMED,
            &core::no_array(),
        )
        .unwrap();

        println!("match_template took {:?}", _now1.elapsed());

        println!("screencap took {:?}", _now.elapsed());

        let mut min_val: f64 = 0.0;
        let mut max_val: f64 = 0.0;
        let mut min_loc: core::Point = core::Point::new(0, 0);
        let mut max_loc: core::Point = core::Point::new(0, 0);

        core::min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &core::no_array(),
        )
        .unwrap();
        println!("screencap took {:?}", _now.elapsed());

        // Set a threshold for the match value
        let threshold = 0.8;

        // Check if the maximum match value exceeds the threshold
        if max_val > threshold {
            println!("Image match found with score: {}", max_val);
            println!("Location of match: {:?}", max_loc);
        } else {
            println!("No match found");
        }
        println!("screencap took {:?}", _now.elapsed());
    }
}
