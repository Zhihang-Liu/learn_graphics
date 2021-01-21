use bmp::Pixel;
use glam::Vec3;


#[inline]
pub fn f2b(i: Vec3) -> bool {
    i.z > 0.0
}

#[inline]
pub fn vec3_to_pixel(i: Vec3) -> Pixel {
    Pixel::new(i.x as u8, i.y as u8, i.z as u8)
}