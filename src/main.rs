extern crate image;
extern crate perlin_noise;

use image::{ImageBuffer, RgbImage};
use perlin_noise::*;

fn main() {
    // Example pixel data
    let mut pixel_data: Vec<Vec<Pixel>> = vec![];

    let permutation = make_permutation();

    /*
     * will return only one color
    for i in 0..=500 {
        let mut row: Vec<Pixel> = vec![];
        for j in 0..=500 {
            let mut n: f32 = noise_2d((j as f32) * 0.01, (i as f32) * 0.01, &permutation);

            n += 1.0;
            n /= 2.0;

            let c : u8 = (255.0 * n).round() as u8;
            println!("color: {}", c);
            row.push(Pixel::new(c, c, c));
        }
        pixel_data.push(row.clone());
    }
    */

    //Generates a color heightmap with fractal brownian motion
    for i in 0..=500 {
        let mut row: Vec<Pixel> = vec![];
        for j in 0..=500 {
            let mut n = 0.0;
            let mut a = 1.0;
            let mut f = 0.005;

            for _ in 0..8 {
                let v: f32 = a * noise_2d((j as f32) * f, (i as f32) * f, &permutation);
                n += v;

                a *= 0.5;
                f *= 2.0;
            }

            n += 1.0;
            n /= 2.0;

            let c : u8 = (255.0 * n).round() as u8;
            // println!("color: {}", c);
            row.push(Pixel::new(c, c, c));
        }
        pixel_data.push(row.clone());
    }


    let imgx = pixel_data.len() as u32;
    let imgy = pixel_data[0].len() as u32;

    let mut imgbuf: RgbImage = ImageBuffer::new(imgx, imgy);

    for (x, row) in pixel_data.iter().enumerate() {
        for (y, &pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([pixel.r, pixel.g, pixel.b]));
        }
    }
    imgbuf.save("image.png").unwrap();
}

