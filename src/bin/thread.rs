// use opencv::core::Mat;
// use opencv::{core, imgcodecs, imgproc};
// use std::cell::RefCell;
// use std::cmp::Ordering;
// use std::io::Write;
// use std::ops::Deref;
// use std::process::Command;
// use std::sync::{Arc, Mutex, RwLock};
// use std::thread;
// use tokio::time;
//
// fn main() {
//     let image_names = vec![
//         "image/tpl1680104663877.png",
//         "image/tpl1680105642722.png",
//         "image/tpl1680264200820.png",
//         "image/tpl1680105795315.png",
//         "image/tpl1680106603490.png",
//         "image/tpl1680166390912.png",
//         "image/tpl1680185500004.png",
//         "image/tpl1680185537772.png",
//         "image/tpl1680188480504.png",
//         "image/tpl1680188636629.png",
//         "image/tpl1680189704651.png",
//         "image/tpl1680263002789.png",
//         // "image/tpl1680264200820.png",
//         // "image/tpl1680289207272.png", // 返回
//         "image/tpl1680185488804.png",
//         // "image/tpl1680289285545.png", // 完成
//         "image/tpl1680342907575.png",
//         // "image/tpl1680344146420.png", // 返回
//         "image/tpl1680185473330.png",
//     ];
//     let image_points = vec![
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(80, -160),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//         core::Point::new(30, 20),
//     ];
//
//     let image_names_lock = Arc::new(RwLock::new(image_names.clone()));
//     let image_points_lock = Arc::new(RwLock::new(image_points.clone()));
//
//     unsafe {
//         loop {
//             let _now = time::Instant::now();
//
//             Command::new("adb")
//                 .arg("shell")
//                 .arg("screencap")
//                 .arg("-p")
//                 .arg("/sdcard/screencap.png")
//                 .output()
//                 .unwrap();
//             println!("screencap took {:?}", _now.elapsed());
//
//             let _now = time::Instant::now();
//
//             Command::new("adb")
//                 .arg("pull")
//                 .arg("/sdcard/screencap.png")
//                 // .stdout(std::process::Stdio::piped())
//                 .output()
//                 .unwrap();
//
//             println!("pull took {:?}", _now.elapsed());
//
//             // TODO match image
//
//             let _now = time::Instant::now();
//             let image1 = imgcodecs::imread("screencap.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();
//
//             let image1_lock = Arc::new(Mutex::new(&image1));
//
//             for i in 0..*&image_names.len() {
//                 let image_names_lock1 = image_names_lock.clone();
//                 let image_points_lock1 = image_points_lock.clone();
//                 let image1_lock1 = image1_lock.clone();
//
//                 thread::spawn(move || {
//                     println!("thread: {}", i);
//                     println!("image_names_lock: {:?}", image_names_lock1);
//                     let image_name: &str = image_names_lock1.read().unwrap().get(i).unwrap();
//
//                     let image2 =
//                         imgcodecs::imread(image_name, imgcodecs::IMREAD_GRAYSCALE).unwrap();
//
//                     let mut result = core::Mat::default();
//                     let image = image1_lock1.lock().unwrap().deref().deref();
//
//                     imgproc::match_template(
//                         image,
//                         &image2,
//                         &mut result,
//                         imgproc::TM_CCOEFF_NORMED,
//                         &core::no_array(),
//                     )
//                     .unwrap();
//
//                     let mut min_val: f64 = 0.0;
//                     let mut max_val: f64 = 0.0;
//                     let mut min_loc: core::Point = core::Point::new(0, 0);
//                     let mut max_loc: core::Point = core::Point::new(0, 0);
//
//                     core::min_max_loc(
//                         &result,
//                         Some(&mut min_val),
//                         Some(&mut max_val),
//                         Some(&mut min_loc),
//                         Some(&mut max_loc),
//                         &core::no_array(),
//                     )
//                     .unwrap();
//
//                     let threshold = 0.8;
//
//                     println!("Image match found with score: {}", max_val);
//                     if max_val > threshold {
//                         let image_point =
//                             image_points_lock1.read().unwrap().get(i).unwrap().clone();
//
//                         // println!("Location of match: {:?} {:?}", max_loc, image_point);
//                         Command::new("adb")
//                             .arg("shell")
//                             .arg("input")
//                             .arg("tap")
//                             .arg((max_loc.x + image_point.x).to_string())
//                             .arg((max_loc.y + image_point.y).to_string())
//                             .output()
//                             .unwrap();
//                         // break;
//                     } else {
//                         // println!("No match found");
//                     }
//                     println!("screencap took {:?}", _now.elapsed());
//                 });
//             }
//
//             loop {
//                 thread::sleep(time::Duration::from_millis(1000));
//                 println!("sleep");
//             }
//         }
//     }
// }

fn main() {}
