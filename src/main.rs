extern crate image;
extern crate perlin_noise;

use image::{ImageBuffer, RgbImage};
use perlin_noise::*;

use std::io;

fn main() {
    let mut pixel_data: Vec<Vec<Pixel>> = vec![];

    let permutation = make_permutation();

    println!("Enter file name for map (don't add '.png'): ");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Error reading line");
    let trimmed_file_name = path.trim();
    let mut file_name = trimmed_file_name.to_owned();
    file_name.push_str(".png");

    println!("Enter width of desired map: ");
    let mut w_size_string = String::new();
    io::stdin().read_line(&mut w_size_string).expect("Error reading line");
    let w_size: u32 = w_size_string.trim().parse().expect("Error in parcing string");

    println!("Enter height of desired map: ");
    let mut h_size_string = String::new();
    io::stdin().read_line(&mut h_size_string).expect("Error reading line");
    let h_size: u32 = h_size_string.trim().parse().expect("Error in parcing string");

    println!("Creating perlin noise map with fractal brownian motion of size {}, {} as {}", w_size, h_size, file_name);

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
    for i in 0..=h_size {
        let mut row: Vec<Pixel> = vec![];
        for j in 0..=w_size {
            let mut n = 0.0;
            let mut a = 1.0;
            let mut f = 0.005;
            let octaves = 8;

            for _ in 0..octaves {
                let v: f32 = a * noise_2d((j as f32) * f, (i as f32) * f, &permutation);
                n += v;

                a *= 0.5;
                f *= 2.0;
            }

            n += 1.0;
            n /= 2.0;

            let c : u8 = (255.0 * n).round() as u8;
            // println!("color: {}", c);
            if n < 0.25 {
                row.push(Pixel::new(0, 0, 153));
            } else if n < 0.5 {
                row.push(Pixel::new(0, 0, 255));
            } else if n < 0.75 {
                row.push(Pixel::new(0, 255, c));
            } else {
                row.push(Pixel::new(c, c, c));
            }
        }
        pixel_data.push(row.clone());
    }


    let imgy = pixel_data.len() as u32;
    let imgx = pixel_data[0].len() as u32;

    let mut imgbuf: RgbImage = ImageBuffer::new(imgx, imgy);

    for (y, row) in pixel_data.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([pixel.r, pixel.g, pixel.b]));
        }
    }
    imgbuf.save(file_name).unwrap();
}

