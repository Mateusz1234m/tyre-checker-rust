use std::f32::consts::PI;
use std::time::Instant;
use image::RgbImage;
// use opencv::prelude::*;

/// Function converts the image from cartesian coordinates to polar coordinates. This can be interpreted as unscrewing the image.
fn cartesian_to_polar(img: &RgbImage) -> (std::time::Duration, RgbImage) {
    
    // start point for duration measurement
    let start_time = Instant::now();
    
    // get image widht and height
    let (img_width, img_height): (u32, u32) = img.dimensions();
    // println!("Width: {img_width}, Height: {img_height}");

    // panic if widht and height are not equal
    if img_width != img_height {
        panic!("Image width and height should be equal")
    }

    // calculate result image width (alpha) and height (radius)
    let radius: u32 = if img_height > img_width {img_width / 2} else {img_height / 2};
    let alpha: u32 = (2.0 * PI * (radius as f32)) as u32;
    
    // println!("Polar alpha: {alpha}, Polar radius: {radius}");

    // create empty result image
    let mut img_polar: RgbImage = RgbImage::new(alpha, radius);

    // iterate over result image width (alpha)
    for i in 0..alpha {

        // calculate sine and cosine value for current iteration
        // they are the dependent only on current angle and alpha, so they are calculated here for optimization
        let angle: f32 = 2.0 * PI * (i as f32 / alpha as f32);
        let sine_value = f32::sin(angle);
        let cosine_value = f32::cos(angle);

        // iterate over result image height (radius)
        for j in 0..radius {

            // calculate cartesian coordinates coresponding to current polar coordinates
            let x = ((j as f32 * sine_value) as i32 + (img_width as i32 / 2)) as u32;
            let y = ((j as f32 * cosine_value) as i32 + (img_height as i32 / 2)) as u32;

            // get the pixel value from the image in cartesian coordinates
            let pixel_value = img.get_pixel(x, y);

            // put the pixel value into the image in polar coordinates
            img_polar.put_pixel(alpha -  i - 1, radius - j - 1, *pixel_value);
        }
    }

    // calculate duration time
    let duration_time = start_time.elapsed();

    // return duration time and the image in polar coordinates
    (duration_time, img_polar)
}

/// Function loads the image and measures loading time.
fn load_image(path: &String) -> (std::time::Duration, RgbImage){
    
    // start point for duration measurement
    let image_opening_start = Instant::now();

    // load image
    let img: RgbImage = image::open(&path).unwrap().to_rgb8();

    // calculate duration
    let image_opening_duration = image_opening_start.elapsed();

    // return duration and image
    (image_opening_duration, img)
}

/// Function saves image and measures saving time.
fn save_image(img: &RgbImage, path: &String) -> std::time::Duration{
    
    // start point for duration measurement
    let image_saving_start = Instant::now();

    // save image
    img.save(path).expect("Error saving image");

    // calculate duration
    let image_saving_duration = image_saving_start.elapsed();

    // return duration
    image_saving_duration
}

fn main() {

    // define path to source image
    let path = "data/images/107_1000.jpg".to_string();

    // define path to save result image
    let save_path = "data/images_out/107_1000.jpg".to_string();

    // start point for duration measurement
    let total_start = Instant::now();

    // load image and print loading time
    let (duration_time, img): (std::time::Duration, RgbImage) = load_image(&path);
    println!("Image loading time: {duration_time:?}");

    // convert image from cartesian to polar coordinates and print convertion time
    let (image_convertion_time, img_polar): (std::time::Duration, RgbImage) = cartesian_to_polar(&img);
    println!("Image convertion time: {image_convertion_time:?}");

    // save result image and print saving time
    let image_saving_duration = save_image(&img_polar, &save_path);
    println!("Image saving time: {image_saving_duration:?}");

    // measure and print total duration time 
    let total_duration = total_start.elapsed();
    println!("Total elapsed time: {total_duration:?}");

}
