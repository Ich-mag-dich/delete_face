use image::{DynamicImage, GrayImage, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rustface::{Detector, FaceInfo, ImageData, Rectangle};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    let line = "------------------------------------------";
    print!("\nInput image path: ");
    io::stdout().flush().unwrap();
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).expect("input error");
    let input_path = format!("input/{}", input_path.trim());
    println!("\n{}\n\nimage path is: {}", line, input_path);
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
    println!("\n{}\nimage count: {}\n", line, file_count);
    let mut detector: Box<dyn Detector> =
        match rustface::create_detector("./model/seeta_fd_frontal_v1.0.bin") {
            Ok(detector) => detector,
            Err(error) => {
                println!("Failed to create detector: {}", error.to_string());
                std::process::exit(1)
            }
        };
    let stdout = io::stdout();
    detector.set_min_face_size(20);
    detector.set_max_face_size(500);
    detector.set_score_thresh(0.8); // default: 2.0
    detector.set_pyramid_scale_factor(0.5); // max: 0.99
    detector.set_slide_window_step(4, 4);
    let mut before_file_name = String::from("");
    for (count, file) in files.iter().enumerate() {
        let processing = ((count + 1) as f64) / (files.len() as f64);
        let file_path = file.replace("\"", "");

        print_process(
            &count,
            &file_count,
            &file_path.as_str(),
            &processing,
            &stdout,
            &before_file_name,
        );
        let image: DynamicImage = image::open(&file_path)
            .expect(format!("Failed to read image: {}", &file_path).as_str());

        let mut rgb = image.to_rgb8();
        let faces = detect_faces(&mut *detector, &image.to_luma8());

        for face in faces {
            let bbox: &Rectangle = face.bbox();
            let rect: Rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());

            draw_filled_rect_mut(&mut rgb, rect, Rgb([255, 255, 255]));
        }

        let output_file: PathBuf =
            PathBuf::from(format!("{}", file.as_str().replace("input", "output")));

        match rgb.save(&output_file) {
            Ok(_) => {
                print_process(
                    &count,
                    &file_count,
                    &output_file.display().to_string(),
                    &processing,
                    &stdout,
                    &before_file_name,
                );
            }
            Err(message) => {
                let create_dir_name: PathBuf =
                    PathBuf::from(format!("./output/{}", input_path.replace("input/", "")));
                fs::create_dir_all(&create_dir_name).expect("Failed to create directory");
                match rgb.save(&output_file) {
                    Ok(_) => {
                        print_process(
                            &count,
                            &file_count,
                            &output_file.display().to_string(),
                            &processing,
                            &stdout,
                            &before_file_name,
                        );
                    }
                    Err(message2) => println!("Can't save file, {}, {}", message, message2),
                }
            }
        }
        before_file_name = format!(
            "   {} {} {}% {}     ",
            count + 1,
            file_count,
            processing * 100.0,
            output_file.to_str().unwrap()
        );
    }
    println!("\n\n{}\nDone!", line);
}

fn print_process(
    count: &usize,
    file_count: &usize,
    file_path: &str,
    processing: &f64,
    mut stdout: &io::Stdout,
    before_file_name: &String,
) {
    let bar = "█".repeat((processing * 20.0).ceil() as usize);
    let bar2 = " ".repeat(20 - ((processing * 20.0).ceil() as usize));
    let string_len = before_file_name.len() + bar.len() + bar2.len();
    if string_len > 1 {
        print!("\r");
        for _ in 1..=string_len {
            print!(" ");
        }
        stdout.flush().unwrap();
    }
    print!(
        "\r [{}{}] | {}/{} | {:.2}% | {} ",
        bar,
        bar2,
        count + 1,
        file_count,
        processing * 100.0,
        file_path
    );
    stdout.flush().unwrap();
}
fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
    let (width, height): (u32, u32) = gray.dimensions();
    let mut image: ImageData = ImageData::new(gray, width, height);

    let faces: Vec<FaceInfo> = detector.detect(&mut image);

    faces
}

// fn get_millis(duration: Duration) -> u64 {
//     duration.as_secs() * 1000u64 + u64::from(duration.subsec_nanos() / 1_000_000)
// }
