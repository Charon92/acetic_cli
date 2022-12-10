mod crypt;
mod phash;
mod png;
mod jpeg;
mod edge_detection;

use clap::Parser;
use chrono;
use image::{Luma, Rgb, Rgb32FImage, GrayImage};
use std::env;
use std::str;
use std::string::{String};
use std::thread::available_parallelism;
use std::time::SystemTime;

const START: &str = "######";
const END: &str = "======";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    process: String,
    filepath: String,
    data: Option<String>,
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

fn encode( path: String, data: Vec<u8> ) -> String {
    let mut encoded: String = String::new();
    let ext = path.split( "." ).last().unwrap();
    if ext.contains( "png" ) {
        let image = png::create( path );
        encoded = image.encode( data );
    } else if ext.contains( ".jpg" ) {
        let image = jpeg::create( path );
        encoded = image.encode( data );
    }

    return encoded;
}

fn decode( path: String, secret_key: String ) -> String {
    let mut decoded: Vec::<u8> = Vec::new();
    if path.split( "." ).last().unwrap().contains( "png" ) {
        let image = png::create( path );
        decoded = image.decode();
    }
    let plain_text = crypt::decrypt( secret_key, decoded.as_slice());
    return String::from_utf8( plain_text ).unwrap();
}

fn main() {
    let start = SystemTime::now();

    let secret_key = env::var( "SEC_K" ).unwrap_or( String::from( "Super_Secret_Key_To_Protect_You__" ) );

    let args: Args = Args::parse();
    let path = args.filepath.clone();
    let process = args.process;
    let mut output = String::new();
    let mut encrypted= Vec::new();
    let mut data = String::new();

    let mut start_sequence: Vec<u8> = START.as_bytes().to_vec();
    let mut end_sequence: Vec<u8> = END.as_bytes().to_vec();

    if process.as_str() == "encode" {
        data = args.data.expect( "No data found to encode. Aborting." );
        if data.len() > 0 {
            encrypted = crypt::encrypt( secret_key.clone(), data );
        }
        println!("Found: {:?}", encrypted);
    }

    start_sequence.append( &mut encrypted );
    start_sequence.append( &mut end_sequence );

    // let default_parallelism_approx = available_parallelism().unwrap().get();
    // println!("Available cores (approx.): {}", default_parallelism_approx);

    match process.as_str() {
        "edge" => output = edge( path ),
        "phash" => output = phash::phash( path ),
        "encode" => output = encode( path, start_sequence ),
        "decode" => output = decode( path, secret_key ),
         _ => println!( "No matched process found. Aborting." )
    }

    println!("Output: {}", output);

    let end = SystemTime::now();
    let duration = end.duration_since( start ).unwrap();
    println!("It took {} milliseconds", duration.as_millis());

}
