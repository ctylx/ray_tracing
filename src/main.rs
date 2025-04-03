use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct Point(Vec3);

#[derive(Debug, Clone)]
pub struct Color(Vec3);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
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
        &self - &rhs
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for u32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self as f32 * rhs.x,
            y: self as f32 * rhs.y,
            z: self as f32 * rhs.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
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

#[derive(Debug)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn at(self, t: f32) -> Point {
        Point(&self.orig.0 + &(t * &self.dir))
    }

    pub fn origin(&self) -> &Point {
        &self.orig
    }
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0))
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
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point(Vec3::new(0.0, 0.0, 0.0));

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = &viewport_u / image_width as f32;
    let pixel_delta_v = &viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center.0
        - &Vec3::new(0.0, 0.0, focal_length)
        - &viewport_u / 2.0
        - &viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {0} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = &pixel00_loc + (i * &pixel_delta_u) + (j * &pixel_delta_v);
            let ray_direction = &pixel_center - &camera_center.0;
            let r = Ray::new(camera_center.clone(), ray_direction);
            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }

    eprintln!("\rDone.");
}
