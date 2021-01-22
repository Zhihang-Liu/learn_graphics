use bmp::Pixel;
use glam::{Vec3, vec3};


#[inline]
pub fn f2b(i: Vec3) -> bool {
    i.z > 0.0
}

#[inline]
pub fn vec3_to_pixel(i: Vec3) -> Pixel {
    Pixel::new(i.x as u8, i.y as u8, i.z as u8)
}

#[inline]
pub fn pixel_to_vec3(i: Pixel) -> Vec3 {
    vec3(i.r as f32, i.g as f32, i.b as f32)
}

/*
#[inline]
pub fn new_2d_array<T: Clone>(default: T, x: usize, y: usize) -> Vec<Vec<T>> {
    let line = vec![default; x];
    vec![line; y]
} */

#[inline]
pub fn sampling_unit(i: &Vec<Vec3>) -> Vec3 {
    let r: Vec3 = i.iter().sum();
    r / i.len() as f32
}

#[inline]
pub fn point2index(w: usize, x: usize, y: usize) -> usize {
    y*w+x
}

#[inline]
pub fn index2point(w: usize, ind: usize) -> (usize, usize) {
    let y = ind / w;
    let x = ind - y*w;
    (x, y)
}