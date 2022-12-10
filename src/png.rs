use image::{GenericImageView, DynamicImage};

fn binary_string_to_u8(str: &str) -> u8 {
    let mut val: u8 = 0;
    let itr = str.as_bytes();
    val += (itr[0] - 48) * 128;
    val += (itr[1] - 48) * 64;
    val += (itr[2] - 48) * 32;
    val += (itr[3] - 48) * 16;
    val += (itr[4] - 48) * 8;
    val += (itr[5] - 48) * 4;
    val += (itr[6] - 48) * 2;
    val += (itr[7] - 48) * 1;
    val
}


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
    
        thumb.save( &thumb_path ).expect( &error_str );
        return thumb_path
    }
    
    pub fn encode(&self, data: Vec<u8>) -> String {
        let mut new_image = image::RgbaImage::new(self.width, self.height);
        let mut bit_string: Vec<String> = Vec::new();
        let path_split: Vec<&str> = self.path.split(".").collect();
        let enc_path;

        if path_split.len() == 3 {
            enc_path = format!(".{}{}_enc.{}", path_split[0], path_split[1], path_split[2]);
        } else {
            enc_path = format!(".{}_enc.{}", path_split[0], path_split[1]);
        }

        for x in 0..data.len() {
            bit_string.push( format!("{:08b}", data[x]) );
        }

        let joined = bit_string.join("");
        let joined_as_bytes = joined.as_bytes();
        let mut i = 0 as usize;

        for (x, y, pixel) in self.image.pixels() {

            let mut new_pix = pixel.clone();

            // Perform LSB operation
            // ---------------------
            // If the bit is 1 and the pixel red value is even, we convert. If odd, leave it alone.
            // If the bit is 0 and the pixel red value is odd, we convert. If even, leave it alone.
            if joined_as_bytes[i] == 49 {
                if new_pix.0[0] % 2 == 0 {
                    if new_pix.0[0] > 0 {
                        new_pix.0[0] = new_pix.0[0] - 1;
                    } else {
                        new_pix.0[0] = new_pix.0[0] + 1;
                    }
                }
            } else {
                if new_pix.0[0] % 2 != 0 {
                    if new_pix.0[0] == 255 {
                        new_pix.0[0] = new_pix.0[0] - 1;
                    } else {
                        new_pix.0[0] = new_pix.0[0] + 1;
                    }
                }
            }

            new_image.put_pixel(x, y, new_pix);

            i += 1;

            if i >= joined_as_bytes.len() {
                i = 0
            }
        }

        new_image.save( enc_path.clone() ).expect( "Failed to save encoded image" );

        return enc_path
    }

    pub fn decode(&self) -> Vec::<u8> {
        let mut bit_string = String::new();
        let mut final_vec = Vec::new();
        let mut intermediary: Vec::<u8> = Vec::new();

        for (x, y, pixel) in self.image.pixels() {
            if pixel.0[0] % 2 == 0 {
                bit_string.push( '0' );
            } else {
                bit_string.push( '1' );
            }
        }


        for i in (0..(bit_string.len() as isize - 7) as usize).step_by(8) {
            if intermediary.len() > 0 && intermediary[intermediary.len()-1] == 61 {
                let end = check_for_end(intermediary.clone(),"======".as_bytes());
                if end {
                    final_vec = intermediary.clone();
                    
                    final_vec = final_vec[6..].to_vec();

                    for i in 0..6 {
                        final_vec.pop();
                    }
                    
                    return final_vec;
                }
            }

            let items = &bit_string[i..i + 8];
            intermediary.push( binary_string_to_u8( items ) );
        }


        return final_vec;
    }
}

fn check_for_end(mut vec: Vec<u8>, end: &[u8]) -> bool {
    let mut found = false;

    for i in 0..end.len() {
        let val = vec.pop().expect( "No more values left" );

        if val == "=".as_bytes()[0] {
            found = true;
        } else {
            found = false;
        }
    }

    found
}

pub fn create(filepath: String) -> PNG {
    return PNG::new(filepath);
}