use num::{pow};
use std::time::SystemTime;

// -------------------------------------------------------------------------------------------------
// Convolution
// -------------------------------------------------------------------------------------------------

/// Take an image in the form of a Vector of rows and cols including the RGB values of each pixel
fn convolve(base: &Vec<Vec<f32>>, kernel: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut final_vec = base.clone();

    for x in 0..(base.len() - kernel.len()) {
        for y in 0..(base[x as usize].len() - kernel[0].len()) {
            let mut val: f32 = 0.0;

            for kernel_x in 0..kernel.len() {
                for kernel_y in 0..kernel[kernel_x as usize].len() {
                    let kernel_value = kernel[kernel_x as usize][kernel_y as usize];

                    val += base[(x + kernel_x) as usize][(y + kernel_y) as usize] * kernel_value;
                }
            }
            final_vec[x as usize][y as usize] = val;
        }
    }
    return final_vec;
}

// -------------------------------------------------------------------------------------------------
// Helper functions
// -------------------------------------------------------------------------------------------------

fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().into_iter().collect()
}

fn add_padding( vec: Vec<Vec<f32>>, padding: i32 ) -> Vec<Vec<f32>> {
    let mut final_vec: Vec<Vec<f32>> = vec![vec![0.0; vec[0].len() + (padding * 2) as usize]; vec.len() + (padding * 2) as usize];
    for x in padding as isize..(final_vec.len() - padding as usize) as isize {
        for y in padding as isize..(final_vec[x as usize].len() - padding as usize) as isize {
            final_vec[x as usize][y as usize] = vec[(x-padding as isize) as usize][(y-padding as isize) as usize];
        }
    }

    // Set padding pixels to the nearest-ish values
    for x in 0..final_vec.len() {
        for y in 0..final_vec[0].len() {
            if x < (padding as usize) {
                final_vec[x][y] = final_vec[2][y].clone()
            } else if x > (final_vec.len() - padding as usize) {
                final_vec[x][y] = final_vec[x-3][y].clone()
            } else if y < (padding as usize) {
                final_vec[x][y] = final_vec[x][2].clone()
            } else if y > (final_vec.len() - padding as usize) {
                final_vec[x][y] = final_vec[x][y-3].clone()
            }
        }
    }

    return final_vec;
}

fn remove_padding( mut vec: Vec<Vec<f32>>, padding: i32 ) -> Vec<Vec<f32>> {
    let mut final_vec: Vec<Vec<f32>> = vec![vec![0.0; vec[0].len() - (padding * 2) as usize]; vec.len() - (padding * 2) as usize];
    let start = padding as isize;
    let stop = (vec.len() - padding as usize) as isize;
    for x in start..stop {
        for y in start..(vec[x as usize].len() - padding as usize) as isize {
            final_vec[(x-padding as isize) as usize][(y-padding as isize) as usize] = vec[x as usize][y as usize];
        }
    }
    return final_vec;
}

fn hypot( vec_one: &Vec<Vec<f32>>, vec_two: &Vec<Vec<f32>> ) -> Vec<Vec<f32>>{
    let mut final_vec = vec![vec![0.0; vec_one[0].len() as usize]; vec_one.len() as usize];
    for x in 0..vec_one.len() {
        for y in 0..vec_one[x].len() {
            let val_1 = vec_one[x][y].clone();
            let val_2 = vec_two[x][y].clone();

            final_vec[x][y] = val_1.hypot(val_2);
        }
    }
    return final_vec;
}

fn atan( vec_one: Vec<Vec<f32>>, vec_two: Vec<Vec<f32>> ) -> Vec<Vec<f32>> {
    let mut final_vec = vec_one.clone();
    for x in 0..vec_one.len() {
        for y in 0..vec_one[x].len() {
            let val_1 = vec_one[x][y].clone();
            let va_2 = vec_two[x][y].clone();

            final_vec[x][y] = ( val_1 / va_2 ).atan();
        }
    }
    return final_vec;
}

fn div_mul(val: f32, i: f32, j: f32) -> f32 {
    val / i * j
}

fn mul_div(val: f32, i: f32, j: f32) -> f32 {
    val * i / j
}

// -------------------------------------------------------------------------------------------------
// Main processes
// -------------------------------------------------------------------------------------------------

/// Function to sharpen an image by convolution
fn sharpen(vec: &Vec<Vec<f32>> ) -> Vec<Vec<f32>> {
    let kernel = vec![
        vec![-0.5, -1.0, -0.5],
        vec![-1.0, 7.0, -1.0],
        vec![-0.5, -1.0, -0.5]
    ];

    return convolve( vec, kernel );
}

fn sobel_process( vec: &Vec<Vec<f32>>, i: f32, j: f32 ) -> Vec<Vec<f32>> {
    let mut final_vec = vec.clone();
    for x in 0..vec.len() {
        for y in 0..vec[x].len() {
            let val_1 = vec[x][y].clone();
            final_vec[x][y] = div_mul( val_1, i, j );
        }
    }
    return final_vec;
}

fn angle_process( vec: Vec<Vec<f32>>, i: f32, j: f32 ) -> Vec<Vec<f32>> {
    let mut final_vec = vec.clone();
    for x in 0..vec.len() {
        for y in 0..vec[x].len() {
            let val_1 = vec[x][y].clone();

            final_vec[x][y] = mul_div( val_1, i, j );
        }
    }
    return final_vec;
}

fn sobel_filters(vec: Vec<Vec<f32>>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>){
    let kx: Vec<Vec<f32>> = vec![
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 0.0, 2.0],
        vec![-1.0, 0.0, 1.0]
    ];
    let ky: Vec<Vec<f32>> = vec![
        vec![1.0, 2.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![-1.0, -2.0, -1.0]
    ];

    let hor_vec = convolve(&vec, kx);
    let vert_vec = convolve(&vec, ky);

    let hypot_vec = hypot(&hor_vec, &vert_vec);
    let max = flatten(hypot_vec.clone()).iter().cloned().fold(0./0., f32::max);
    let processed = sobel_process(&hypot_vec, max, 1.0 );
    let theta = atan(hor_vec.clone(), vert_vec.clone());

    return (processed, theta);
}

/// Generate a Gaussian kernel for applying to image vectors.
fn gaussian_kernel( radius: i32 ) -> Vec<Vec<f32>> {
    let sigma = std::cmp::max( radius / 2, 1);
    let kernel_width = (2 * radius) + 1;
    let mut kernel: Vec<Vec<f32>> = vec![vec![0.0;kernel_width as usize];kernel_width as usize];
    let denominator = (2 * sigma * sigma) as f32;
    let pi: f32 = std::f64::consts::PI as f32;
    let mut sum = 0.0;

    for x in -radius..radius {
        for y in -radius..radius {
            let numerator = -( x * x + y * y ) as f32;
            let kernel_val = ( (numerator / denominator).exp() ) / (pi * denominator);
            kernel[(x + radius) as usize][(y + radius) as usize] = kernel_val;
            sum += kernel_val;
        }
    };

    for i in 0..kernel_width as usize {
        for j in 0..kernel_width as usize {
            kernel[i][j] /= sum;
        }
    }

    return kernel;
}

fn non_max_suppression( vec: Vec<Vec<f32>>, theta: Vec<Vec<f32>> ) -> Vec<Vec<f32>> {
    let mut final_vec = vec.clone();
    let mut angle = angle_process( theta, 180.0, std::f32::consts::PI );

    for i in 1..vec.len()-1 {
        for j in 1..vec[i].len()-1 {
            let mut q = 255.0;
            let mut r = 255.0;

            if i > vec.len() || j > vec[i].len() {
                final_vec[i][j] = 0.0
            } else {
                if angle[i][j] < 0.0 {
                    angle[i][j] += 180.0
                }

                if angle[i][j] < 22.5 && angle[i][j] >= 0.0 || angle[i][j] <= 180.0 && angle[i][j] >= 157.5 {
                    q = vec[i][j + 1];
                    r = vec[i][j - 1];
                    // angle 45
                } else if angle[i][j] >= 22.5 && angle[i][j] < 67.5 {
                    q = vec[i + 1][j - 1];
                    r = vec[i - 1][j + 1];
                    // angle 90
                } else if angle[i][j] >= 67.5 && angle[i][j] < 112.5 {
                    q = vec[i + 1][j];
                    r = vec[i - 1][j];
                    // angle 135
                } else if angle[i][j] >= 112.5 && angle[i][j] < 157.5 {
                    q = vec[i - 1][j - 1];
                    r = vec[i + 1][j + 1];
                }

                if vec[i][j] >= q && vec[i][j] >= r {
                    final_vec[i][j] = vec[i][j]
                } else {
                    final_vec[i][j] = 0.0
                }
            }
        }
    }
    return final_vec;
}

fn threshold( mut vec: Vec<Vec<f32>> ) -> Vec<Vec<f32>> {
    let low_threshold = 0.05;
    let high_threshold = 0.75;

    for i in 1..vec.len() {
        for j in 1..vec[i].len() {
            if vec[i][j] >= high_threshold {
                vec[i][j] = 1.0;
            } else if vec[i][j] >= low_threshold && vec[i][j] < high_threshold {
                vec[i][j] = 0.3
            } else if vec[i][j] < low_threshold {
                vec[i][j] = 0.0
            }
        }
    }

    return vec;
}

fn hysteresis( mut vec: Vec<Vec<f32>> ) -> Vec<Vec<f32>> {
    let weak: f32 = 0.3;
    let strong: f32 = 1.0;

    for i in 1..vec.len()-1 {
        for j in 1..vec[i].len()-1 {
            if vec[i][j] == weak {
                if (vec[i+1][j-1] == strong) || (vec[i+1][j] == strong) || (vec[i+1][j+1] == strong) || (vec[i][j-1] == strong) || (vec[i][j+1] == strong) || (vec[i-1][j-1] == strong) || (vec[i-1][j] == strong) || (vec[i-1][j+1] == strong) {
                    vec[i][j] = strong
                } else {
                    vec[i][j] = strong
                }
            }
        }
    }

    return vec;
}

// -------------------------------------------------------------------------------------------------
// Utilised function
// -------------------------------------------------------------------------------------------------

pub fn detect( vec: Vec<Vec<f32>> ) -> Vec<Vec<f32>> {
    let radius = 2;
    let padded_vec = add_padding( vec, radius );

    let kernel = gaussian_kernel( radius );

    let new_image_vec = convolve( &padded_vec, kernel );
    let gradient_theta = sobel_filters( new_image_vec );
    let suppressed = non_max_suppression( gradient_theta.0, gradient_theta.1 );
    let filtered = threshold( suppressed );
    let leveled = hysteresis( filtered );
    let unpadded_vec = remove_padding( leveled, radius );

    return unpadded_vec;
}
// -------------------------------------------------------------------------------------------------