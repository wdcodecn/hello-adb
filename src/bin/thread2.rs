use std::cell::RefCell;
use std::ops::Deref;
use std::process::Command;
use std::sync::{Arc, Mutex, RwLock};
use std::{thread, time};

use opencv::{core, imgcodecs, imgproc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Copy, Clone)]
struct Pic {
    path: &'static str,
    offset: (i32, i32),
}
//
// impl Clone for Box<String> {
//     fn clone(&self) -> Box<String> {
//         Box::new(String::from(self))
//     }
// }

// tokio
#[tokio::main]
async fn main() {
    let out = Command::new("adb")
        .arg("devices")
        .stdout(std::process::Stdio::piped())
        .output()
        .unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);

    println!("out_str: {}", out_str);

    let out_str = out_str.split_whitespace().collect::<Vec<&str>>();
    let device_id = out_str[0];
    println!("out_str: {:?}", out_str);
    println!("device_id: {}", device_id);

    // let image1 = imgcodecs::imread("screencap.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();
    Command::new("adb")
        .arg("shell")
        .arg("am")
        .arg("start")
        .arg("air.jp.co.cygames.worldflipper/air.jp.co.cygames.worldflipper.AppEntry")
        .output()
        .unwrap();


    let json_str = r#"[
        { "path": "image/tpl1680104663877.png", "offset": [25,30] },
        { "path": "image/tpl1680105642722.png", "offset": [25,30] },
        { "path": "image/tpl1680105795315.png", "offset": [25,30] },
        { "path": "image/tpl1680106603490.png", "offset": [25,30] },
        { "path": "image/tpl1680166390912.png", "offset": [25,30] },
        { "path": "image/tpl1680185473330.png", "offset": [25,30] },
        { "path": "image/tpl1680185488804.png", "offset": [25,30] },
        { "path": "image/tpl1680185500004.png", "offset": [25,30] },
        { "path": "image/tpl1680185537772.png", "offset": [25,30] },
        { "path": "image/tpl1680188480504.png", "offset": [25,30] },
        { "path": "image/tpl1680188636629.png", "offset": [25,30] },
        { "path": "image/tpl1680189704651.png", "offset": [25,30] },
        { "path": "image/tpl1680263002789.png", "offset": [25,30] },
        { "path": "image/tpl1680264200820.png", "offset": [25,30] },
        { "path": "image/tpl1680289207272.png", "offset": [25,30] },
        { "path": "image/tpl1680289285545.png", "offset": [25,30] },
        { "path": "image/tpl1680342907575.png", "offset": [25,30] },
        { "path": "image/tpl1680344146420.png", "offset": [25,30] }
    ]"#;
    // let json_str = r#"[
    //     { "path": "image/tpl1680104663877.png", "offset": [25,30] }
    // ]"#;

    let mut _pic: Vec<Pic> = serde_json::from_str(json_str).unwrap();

    // todo 多线程循环 _pic
    let mut handles = vec![];

    let mut ready = false;
    // let lock = Arc::new(RwLock::new(ready));
    let lock = Arc::new(Mutex::new(ready));

    for i in 0.._pic.len() {
        let pic = RefCell::new(_pic[i]);
        let c_lock = Arc::clone(&lock);

        // 启动线程
        let mut handle = thread::spawn(move || {
            let pic = pic.borrow();

            loop {
                let _now = time::Instant::now();
                // println!("c_lock read {:?} , ready {}", _now.elapsed(), ready);
                //
                // if !*ready {
                //     println!("thread::sleep 100 ms, {:?}", i);
                //
                //     thread::sleep(std::time::Duration::from_millis(1000));
                //     continue;
                // }

                // println!("This is thread number {} {:?}", i, pic);

                thread::sleep(std::time::Duration::from_millis(100));

                let lock = c_lock.lock().unwrap();

                let image1 =
                    imgcodecs::imread("screencap.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();

                drop(lock);

                let image2 = imgcodecs::imread(pic.path, imgcodecs::IMREAD_GRAYSCALE).unwrap();

                let mut result = core::Mat::default();

                imgproc::match_template(
                    &image1,
                    &image2,
                    &mut result,
                    imgproc::TM_CCOEFF_NORMED,
                    &core::no_array(),
                )
                    .unwrap();

                let mut min_val = 0.0;
                let mut max_val = 0.0;
                let mut min_loc = core::Point::new(0, 0);
                let mut max_loc = core::Point::new(0, 0);

                core::min_max_loc(
                    &result,
                    Some(&mut min_val),
                    Some(&mut max_val),
                    Some(&mut min_loc),
                    Some(&mut max_loc),
                    &core::no_array(),
                )
                    .unwrap();

                let threshold = 0.8;

                println!("Image {} match found with score: {}", pic.path, max_val);
                if max_val > threshold {
                    let px = max_loc.x + pic.offset.0;
                    let py = max_loc.y + pic.offset.1;

                    println!("Image {} match found with point: {},{}", pic.path, px, py);

                    // Command::new("adb")
                    //     .arg("shell")
                    //     .arg("input")
                    //     .arg("tap")
                    //     .arg((px).to_string())
                    //     .arg((py).to_string())
                    //     .output()
                    //     .unwrap();
                    thread::sleep(std::time::Duration::from_millis(200));
                }
            }
        });
        handles.push(handle);
    }

    loop {
        let c_lock = Arc::clone(&lock);

        let _now = time::Instant::now();
        Command::new("adb")
            .arg("shell")
            .arg("screencap")
            .arg("-p")
            .arg("/sdcard/screencap.png")
            .output()
            .unwrap();
        println!("screencap took {:?}", _now.elapsed());

        let _now = time::Instant::now();

        let lock = c_lock.lock().unwrap();
        Command::new("adb")
            .arg("pull")
            .arg("/sdcard/screencap.png")
            // .stdout(std::process::Stdio::piped())
            .output()
            .unwrap();
        drop(lock);

        println!("pull took {:?}", _now.elapsed());
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // println!("_pic: {:?}", _pic);
}
