use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub};

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// struct Point(Vec3);
struct Color(Vec3);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

fn write_color(pixel_color: &Color) {
    let r = pixel_color.0.x;
    let g = pixel_color.0.y;
    let b = pixel_color.0.z;

    let ir = (255.999 * r) as u32;
    let ig = (255.999 * g) as u32;
    let ib = (255.999 * b) as u32;
    println!("{ir} {ig} {ib}");
}

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {0} ", image_height - j);
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0;

            let pixel_color = Color(Vec3::new(r, g, b));
            write_color(&pixel_color);
        }
    }
    eprintln!("\rDone.")
}
