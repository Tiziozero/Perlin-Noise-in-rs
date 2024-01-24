extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn random_gradient() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=360)
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }
}

pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

pub fn lerp(t: f32, a1: f32, a2: f32) -> f32 {
    a1 + t * (a2 - a1)
}

fn fade(t: f32) -> f32 {
    ((6.0*t - 15.0)*t + 10.0)*t*t*t
}

pub fn make_permutation() -> Vec<u8> {
    let mut permutation: Vec<u8> = vec![];
    for i in 0u8..=255 {
        permutation.push(i);
    }

    {
        let mut rng = rand::thread_rng();
        permutation.shuffle(&mut rng);
    }
    for i in 0..=255 {
        permutation.push(permutation[i]);
    }
    permutation
}
pub fn noise_2d(x: f32, y: f32, permutation: &Vec<u8>) -> f32 {
    let b_x: usize = x.floor() as usize;
    let b_y: usize = y.floor() as usize;
    let xf = x - x.floor();
    let yf = y - y.floor();

    let top_right = Vector2::new(xf - 1.0, yf - 1.0);
    let top_left = Vector2::new(xf, yf - 1.0);
    let bot_right = Vector2::new(xf - 1.0, yf);
    let bot_left = Vector2::new(xf, yf);

    // let value_top_right = permutation[permutation[b_x+1] as usize + b_y + 1];
    // let value_top_left = permutation[permutation[b_x] as usize + b_y + 1];
    // let value_bot_right = permutation[permutation[b_x+1] as usize + b_y];
    // let value_bot_left = permutation[permutation[b_x] as usize + b_y];

    let value_top_right = permutation[permutation[b_x.wrapping_add(1) as usize & 255] as usize + b_y.wrapping_add(1) as usize & 255];
    let value_top_left = permutation[permutation[b_x as usize & 255] as usize + b_y.wrapping_add(1) as usize & 255];
    let value_bot_right = permutation[permutation[b_x.wrapping_add(1) as usize & 255] as usize + b_y as usize & 255];
    let value_bot_left = permutation[permutation[b_x as usize & 255] as usize + b_y as usize & 255];

    let dot_top_right = top_right.dot(&get_const_vector(value_top_right));
    let dot_top_left = top_left.dot(&get_const_vector(value_top_left));

    let dot_bot_right = bot_right.dot(&get_const_vector(value_bot_right));
    let dot_bot_left = bot_left.dot(&get_const_vector(value_bot_left));
    let u = fade(xf);
    let v = fade(yf);
    let result: f32 = lerp(u,
        lerp(v, dot_bot_left, dot_top_left),
        lerp(v, dot_bot_right, dot_top_right));
    // println!("result: {result}");
    result
}

fn get_const_vector(v: u8) -> Vector2 {
    let h = v & 3;
    if h == 0 {
        Vector2 { x: 1.0, y: 1.0 }
    } else if h == 1 {
        Vector2 { x: -1.0, y: 1.0 }
    } else if h == 2 {
        Vector2 { x: -1.0, y: -1.0 }
    } else {
        Vector2 { x: 1.0, y: -1.0 }
    }
}
