mod phash;
mod png;
mod jpeg;
mod edge_detection;

use clap::Parser;
use chrono;
use clap::builder::Str;
use image::{Luma, ImageBuffer, RgbImage, Rgb, Rgb32FImage, GrayImage};
use std::string::{String};
use std::thread::available_parallelism;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filepath: String,
    process: String
}

fn edge(path: String) -> String {
    let mut out_path  = path.split("/").collect::<Vec<&str>>();
    let file_name = out_path.pop().unwrap();
    let output_name = format!("{}/{}_{}", out_path.join("/"), chrono::offset::Utc::now(), file_name);

    let img = png::create(path);

    // All operations use a 2D Vec of F32 and image `.to_vec()` outputs a 1D Vec so instead we
    // create an intermediary image instance in RGB F32 and add those values to `new_img`.
    let mut new_img: Vec<Vec<f32>> = vec![vec![0.0;img.height as usize];img.width as usize];
    let temp_image = img.image.into_rgb32f();

    let stop_x = img.width as usize;
    let stop_y = img.height as usize;

    for x in 0..stop_x {
        for y in 0..stop_y {
            let gray = (0.2126 * temp_image.get_pixel(x as u32, y as u32)[0]) + (0.7152 * temp_image.get_pixel(x as u32, y as u32)[1]) + (0.0722 * temp_image.get_pixel(x as u32, y as u32)[2]);
            new_img[x][y] = gray;
        }
    }

    let edge = edge_detection::detect(new_img);
    let mut altered_image = GrayImage::new(img.width, img.height);

    for x in 0..stop_x {
        for y in 0..stop_y {
            let pix_arr = edge[x][y].clone();
            let data = [(pix_arr * 255.0) as u8];
            altered_image.put_pixel(x as u32, y as u32, Luma(data));
        }
    }

    println!("Saving file to {output_name}");

    match altered_image.save(&output_name) {
        Ok(..) => println!("Saved file"),
        Err(e) => println!("{e}")
    };

    return output_name
}

fn main() {
    let start = SystemTime::now();

    let args: Args = Args::parse();
    let path = args.filepath.clone();
    let process = args.process;
    let mut output = String::new();

    let default_parallelism_approx = available_parallelism().unwrap().get();
    println!("Available cores (approx.): {}", default_parallelism_approx);

    match process.as_str() {
        "edge" => output = edge( path ),
        "phash" => output = phash::phash( path ),
         _ => println!( "No matched process found. Aborting." )
    }

    println!("Output: {}", output);

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("It took {} milliseconds", duration.as_millis());

}
