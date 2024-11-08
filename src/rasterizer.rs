use std::ops::{Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub enum CullMode {
    None,
    Counterclockwise,
    Clockwise,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

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
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
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

#[derive(Debug, Copy, Clone)]
pub struct Mat4x4 {
    pub values: [f32; 16],
}

impl Mat4x4 {
    pub fn new(values: [f32; 16]) -> Mat4x4 {
        Mat4x4 { values }
    }

    pub fn identity() -> Mat4x4 {
        Mat4x4 {
            values: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self.values[0] * rhs.x
                + self.values[1] * rhs.y
                + self.values[2] * rhs.z
                + self.values[3] * rhs.w,
            y: self.values[4] * rhs.x
                + self.values[5] * rhs.y
                + self.values[6] * rhs.z
                + self.values[7] * rhs.w,
            z: self.values[8] * rhs.x
                + self.values[9] * rhs.y
                + self.values[10] * rhs.z
                + self.values[11] * rhs.w,
            w: self.values[12] * rhs.x
                + self.values[13] * rhs.y
                + self.values[14] * rhs.z
                + self.values[15] * rhs.w,
        }
    }
}

#[derive(Debug)]
pub struct Mesh<'a> {
    pub positions: &'a mut [Vec3],
    pub color: Vec4,
}

#[derive(Debug)]
pub struct DrawCommand<'a> {
    pub mesh: Mesh<'a>,
    pub cull_mode: CullMode,
    pub transform: Mat4x4,
}

#[derive(Debug)]
pub struct ImageView<'a> {
    pixels: &'a mut [Color],
    width: u32,
    height: u32,
}

impl ImageView<'_> {
    pub fn from_pixel_buffer(pixel_buffer: &mut [u8], width: u32, height: u32) -> ImageView {
        // we always have a whole number of pixels in the buffer
        let length = width as usize * height as usize;
        let pixels = unsafe {
            std::slice::from_raw_parts_mut(pixel_buffer.as_mut_ptr() as *mut Color, length)
        };

        ImageView {
            pixels,
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: Vec4) {
        let rgba = color.to_color();
        let rgba_u32 = u32::from_ne_bytes([rgba.r, rgba.g, rgba.b, rgba.a]);

        let length = (self.width * self.height) as usize;

        // a Color is equivalent to u32 and there are a whole number of Colors in the buffer
        let pixels_u32 =
            unsafe { std::slice::from_raw_parts_mut(self.pixels.as_mut_ptr() as *mut u32, length) };
        pixels_u32.fill(rgba_u32);
    }

    pub fn at(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[x + y * self.width as usize]
    }

    pub fn draw(&mut self, command: &DrawCommand) {
        for vertices in command.mesh.positions.chunks_exact(3) {
            let (mut v0, mut v1, v2) = (
                command.transform * vertices[0].as_point(),
                command.transform * vertices[1].as_point(),
                command.transform * vertices[2].as_point(),
            );

            let mut det_012 = (v1 - v0).det_2d(&(v2 - v0));
            let counterclockwise = det_012 < 0.0;

            match command.cull_mode {
                CullMode::None => {}
                CullMode::Clockwise => {
                    if !counterclockwise {
                        continue;
                    }
                }
                CullMode::Counterclockwise => {
                    if counterclockwise {
                        continue;
                    }
                }
            }

            if counterclockwise {
                std::mem::swap(&mut v0, &mut v1);
                det_012 = -det_012;
            }

            let xmin = 0.max(
                (v0.x.floor() as usize)
                    .min(v1.x.floor() as usize)
                    .min(v2.x.floor() as usize),
            );
            let xmax = (self.width as usize).min(
                (v0.x.ceil() as usize)
                    .max(v1.x.ceil() as usize)
                    .max(v2.x.ceil() as usize),
            );
            let ymin = 0.max(
                (v0.y.floor() as usize)
                    .min(v1.y.floor() as usize)
                    .min(v2.y.floor() as usize),
            );
            let ymax = (self.height as usize).min(
                (v0.y.ceil() as usize)
                    .max(v1.y.ceil() as usize)
                    .max(v2.y.ceil() as usize),
            );

            for y in ymin..ymax {
                for x in xmin..xmax {
                    let p = Vec4 {
                        x: x as f32 + 0.5,
                        y: y as f32 + 0.5,
                        z: 0.0,
                        w: 0.0,
                    };

                    let det_01_p = (v1 - v0).det_2d(&(p - v0));
                    let det_12_p = (v2 - v1).det_2d(&(p - v1));
                    let det_20_p = (v0 - v2).det_2d(&(p - v2));

                    if det_01_p >= 0.0 && det_12_p >= 0.0 && det_20_p >= 0.0 {
                        *self.at(x, y) = command.mesh.color.to_color();
                    }
                }
            }
        }
    }
}
