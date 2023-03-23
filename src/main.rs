use image::{DynamicImage, GrayImage, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rustface::{Detector, FaceInfo, ImageData, Rectangle};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
fn main() {
    let line = "------------------------------------------";
    print!("Input image path: ");
    io::stdout().flush().unwrap();
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).expect("input error");
    let input_path = format!("input/{}", input_path.trim());
    println!("\n\n{}\nimage path is: {}", line, input_path);
    let files: Vec<String> = Path::new(&input_path)
        .read_dir()
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() || path.file_name()?.to_str()? == ".DS_Store" {
                None
            } else {
                Some(path.to_str()?.to_owned())
            }
        })
        .collect::<Vec<String>>();
    let file_count = files.len();
    println!("{}\nimage count: {}", line, file_count);
    let mut detector: Box<dyn Detector> =
        match rustface::create_detector("./model/seeta_fd_frontal_v1.0.bin") {
            Ok(detector) => detector,
            Err(error) => {
                println!("Failed to create detector: {}", error.to_string());
                std::process::exit(1)
            }
        };
    detector.set_min_face_size(20);
    detector.set_max_face_size(500);
    detector.set_score_thresh(0.8); // default: 2.0
    detector.set_pyramid_scale_factor(0.9); // max: 0.99
    detector.set_slide_window_step(4, 4);
    for (count, file) in files.iter().enumerate() {
        let file_path = file.replace("\"", "");
        println!("\n{}/{}\n{}", count + 1, file_count, file_path);
        let image: DynamicImage = image::open(&file_path)
            .expect(format!("Failed to read image: {}", &file_path).as_str());

        let mut rgb = image.to_rgb8();
        let faces = detect_faces(&mut *detector, &image.to_luma8());

        for face in faces {
            let bbox: &Rectangle = face.bbox();
            let rect: Rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());

            draw_filled_rect_mut(&mut rgb, rect, Rgb([255, 255, 255]));
        }
        let output_file: PathBuf = PathBuf::from(format!(
            "./output/{}DF_{}",
            input_path.replace("input/", ""),
            file.as_str()
                .replace(&input_path.replace("input/", ""), "")
                .replace("input/", "")
        ));
        match rgb.save(&output_file) {
            Ok(_) => println!("Saved result to {}", output_file.display()),
            Err(message) => {
                let create_dir_name: PathBuf =
                    PathBuf::from(format!("./output/{}", input_path.replace("input/", "")));
                fs::create_dir_all(&create_dir_name).expect("Failed to create directory");
                match rgb.save(&output_file) {
                    Ok(_) => println!("Saved result to {}", output_file.display()),
                    Err(message2) => println!("Can't save file, {}, {}", message, message2),
                }
            }
        }
    }
    println!("\n{}\nDone!", line);
}

fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
    let (width, height): (u32, u32) = gray.dimensions();
    let mut image: ImageData = ImageData::new(gray, width, height);
    let now: Instant = Instant::now();
    let faces: Vec<FaceInfo> = detector.detect(&mut image);
    println!(
        "Found {} faces in {} ms",
        faces.len(),
        get_millis(now.elapsed())
    );
    faces
}

fn get_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000u64 + u64::from(duration.subsec_nanos() / 1_000_000)
}
