use image;

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn vector_as_u8_array(vector: &Vec<u8>) -> [i32;64] {
    let mut arr = [0i32;64];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element as i32;
    }
    arr
}

pub fn phash(filepath: String) -> String {
    let img = image::open(filepath).unwrap();
    let thumb = img.thumbnail(8, 8).into_luma_alpha8().into_raw();
    let avg_val = average(&vector_as_u8_array(&thumb));
    let mut hash: Vec<char> = vec![];

    for pix in thumb.iter() {
        if *pix as f32 >= avg_val.floor() {
            hash.push('1')
        } else {
            hash.push('0')
        }
    }

    return hash.into_iter().collect()

}