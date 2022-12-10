use std::fs::File;
use std::io::prelude::*;

use image::{GenericImageView};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JPEG {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub alpha: bool,
    pub channels: u8
}

impl JPEG {
    pub fn new(filepath: String) -> JPEG {
        let img = image::open(&filepath).expect("File not found!");
        let (w, h) = img.dimensions();

        return JPEG {
            path: filepath,
            width: w,
            height: h,
            alpha: img.color().has_alpha(),
            channels: img.color().channel_count()
        };
    }

    pub fn dimensions(&self) -> String {
        format!("{}W x {}H", self.width, self.height)
    }
    
    pub fn total_pixels(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn create_thumbnail(&self, w: u32, h: u32) -> String {
        let img = image::open(self.path.clone()).expect("File not found!");
        let thumb = img.thumbnail(w, h);
    
        let path_split: Vec<&str> = self.path.split(".").collect();
        let thumb_path = format!("{}_thumb.{}", path_split[0], path_split[1]);
        let error_str = format!("Failed to save thumbnail at {:?}", thumb_path);
    
        thumb.save(&thumb_path).expect(&error_str[..]);
        return thumb_path
    }

    pub fn encode(&self,  data: Vec<u8>) -> String {
        let mut file = match File::open(&self.path) {
            Err(why) => panic!("couldn't open {}: {}", self.path, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", self.path, why),
            Ok(_) => print!("{} contains:\n{}", self.path, s),
        }

        return String::from(self.path.clone());
    }
}

pub fn create(filepath: String) -> JPEG {
    return JPEG::new(filepath);
}