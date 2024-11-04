use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn as_vector(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.0,
        }
    }

    pub fn as_point(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    pub fn to_color(&self) -> Color {
        Color {
            r: (self.x * 255.0).clamp(0.0, 255.0) as u8,
            g: (self.y * 255.0).clamp(0.0, 255.0) as u8,
            b: (self.z * 255.0).clamp(0.0, 255.0) as u8,
            a: (self.w * 255.0).clamp(0.0, 255.0) as u8,
        }
    }

    pub fn det_2d(&self, other: &Vec4) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[derive(Debug)]
pub struct Mesh<'a> {
    positions: &'a mut [Vec3],
    vertex_count: u32,
    color: Vec4,
}

#[derive(Debug)]
pub struct DrawCommand<'a> {
    mesh: Mesh<'a>,
}

#[derive(Debug)]
pub struct ImageView {
    pixels: *mut [Color],
    width: u32,
    height: u32,
}

impl ImageView {
    pub fn from_pixel_buffer(pixel_buffer: &mut [u8], width: u32, height: u32) -> ImageView {
        let pixel_ptr = pixel_buffer.as_mut_ptr() as *mut Color;
        let len = (width * height) as usize;
        let raw_pixels = unsafe { std::slice::from_raw_parts_mut(pixel_ptr, len) };

        ImageView {
            pixels: raw_pixels,
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: Vec4) {
        let rgba = color.to_color();
        let rgba_u32 = u32::from_ne_bytes([rgba.r, rgba.g, rgba.b, rgba.a]);

        let len = (self.width * self.height) as usize;

        let pixels_u32 =
            unsafe { std::slice::from_raw_parts_mut(self.pixels as *mut Color as *mut u32, len) };
        pixels_u32.fill(rgba_u32);
    }

    pub fn at(&self, x: usize, y: usize) -> *mut Color {
        unsafe { self.pixels.add(x + y * self.width as usize) as *mut Color }
    }
}
