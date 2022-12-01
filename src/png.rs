use image::{GenericImageView, DynamicImage};


#[derive(Debug)]
pub struct PNG {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub alpha: bool,
    pub channels: u8,
    pub image: DynamicImage
}

impl PNG {
    pub fn new(filepath: String) -> PNG {
        let img = image::open(&filepath).expect("File not found!");
        let (w, h) = img.dimensions();

        return PNG {
            path: filepath,
            width: w,
            height: h,
            alpha: img.color().has_alpha(),
            channels: img.color().channel_count(),
            image: img,
        };
    }

    pub fn dimensions(&self) -> String {
        format!("{}W x {}H", self.width, self.height)
    }
    
    pub fn total_pixels(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn create_thumbnail(&self, w: u32, h: u32) -> String {
        let thumb = self.image.thumbnail(w, h);
    
        let path_split: Vec<&str> = self.path.split(".").collect();
        let thumb_path = format!("{}_thumb.{}", path_split[0], path_split[1]);
        let error_str = format!("Failed to save thumbnail at {:?}", thumb_path);
    
        thumb.save(&thumb_path).expect(&error_str[..]);
        return thumb_path
    }
    
    pub fn encode(&self) {
        for (x, y, pixel) in self.image.pixels() {
            let red_pix = pixel.0;
            println!("{:?}", red_pix);
        }
    }
}

pub fn create(filepath: String) -> PNG {
    return PNG::new(filepath);
}