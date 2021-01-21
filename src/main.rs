mod triangle;
mod rasterization;
mod utils;

use glam::*;
use bmp::{Image, Pixel, consts::{WHITE}};
use ordered_float::OrderedFloat;
use lazy_static::lazy_static;

use utils::*;
// use triangle::*;
use rasterization::*;

pub type Triangle = Mat3;

pub fn inside(tri: Triangle, x: f32, y: f32) -> bool {
    let p = vec3(x, y, 1.0);
    let v0: Vec3 = tri.y_axis - tri.x_axis;
    let v1: Vec3 = tri.z_axis - tri.y_axis;
    let v2: Vec3 = tri.x_axis - tri.z_axis;
    let p0: Vec3 = p - tri.x_axis;
    let p1: Vec3 = p - tri.y_axis;
    let p2: Vec3 = p - tri.z_axis;
    let a = f2b(v0.cross(p0));
    let b = f2b(v1.cross(p1));
    let c = f2b(v2.cross(p2));
    a == b && b == c
}

pub fn bounding_box(tri: Triangle) -> Mat2 {
    let vec_iter = [tri.x_axis, tri.y_axis, tri.z_axis];
    let x_list = vec_iter.iter().map(
        |v| OrderedFloat::from(v.x));
    let y_list = vec_iter.iter().map(
        |v| OrderedFloat::from(v.y));
    let x_min = x_list.clone().min().unwrap().into();
    let y_min = y_list.clone().min().unwrap().into();
    let x_max = x_list.max().unwrap().into();
    let y_max = y_list.max().unwrap().into();

    const_mat2!([x_min, y_min], [x_max, y_max])
}


fn rasterization(tri: Triangle, buffer: &mut Vec<Vec<bool>>) {
    let size = bounding_box(tri);
    for y in size.x_axis.y as usize..size.y_axis.y as usize+1 {
        for x in size.x_axis.x as usize..size.y_axis.x as usize+1 {
            let p = unsafe {
                buffer.get_unchecked_mut(y).get_unchecked_mut(x)
            };
            if *p == false {
                *p = inside(tri, x as f32+0.5, y as f32+0.5);
            };
        }
    }
}

/*
#[inline]
fn box_filter(i: Mat3) -> f32 {
    let rv: Vec3 = i.x_axis + i.y_axis + i.z_axis;
    let rv = rv.x + rv.y + rv.z;
    rv / 9.0
} */



lazy_static! {
    static ref WHITE_VEC: Vec3 = vec3(WHITE.r as f32, WHITE.g as f32, WHITE.b as f32);
}

fn sampling_unit(i: &Vec<bool>) -> Vec3 {
    let r: f32 = i.iter().map(|x|*x as u8 as f32).sum();
    *WHITE_VEC * r / i.len() as f32
}

fn sampling(i: Vec<Vec<bool>>, sampling_pipe: usize, h: usize, w: usize) -> Vec<Vec<Vec3>> {
    let mut buffer = new_2d_array(Vec3::zero(), w, h);
    for y in 0..h {
        let hy = y*sampling_pipe;
        for x in 0..w {
            let hx = x*sampling_pipe;
            let slice_i: Vec<bool> = Vec::from(&i[hy..hy+sampling_pipe])
                .iter()
                .flat_map(|x| Vec::from(&x[hx..hx+sampling_pipe]))
                .collect();
            let r = sampling_unit(&slice_i);
            let p = unsafe {
                buffer.get_unchecked_mut(y).get_unchecked_mut(x)
            };
            *p = r;
        }
    }
    buffer
}

fn init() -> Vec<Mat3> {
    let lst: [[f32; 3]; 3] = [
        [2.2, 1.3, 1.0],
        [4.4, 11.0, 1.0],
        [15.3, 8.6, 1.0]];
    let tri = Mat3::from_cols_array_2d(&lst);
    let lst: [[f32; 3]; 3] = [
        [5.8, 5.2, 1.0],
        [15.4, 20.0, 1.0],
        [25.3, 24.6, 1.0]];
    let tri1 = Mat3::from_cols_array_2d(&lst);
    vec![tri, tri1]
}

fn new_2d_array<T: Clone>(default: T, x: usize, y: usize) -> Vec<Vec<T>> {
    let line = vec![default; x];
    vec![line; y]
}

fn output(path: &str, buffer: &Vec<Vec<Vec3>>, x: u32, y: u32) {
    let mut screen = Image::new(x, y);
    for (y, l) in buffer.iter().enumerate() {
        for (x, p) in l.iter().enumerate() {
            screen.set_pixel(x as u32, y as u32, vec3_to_pixel(*p));
        }
    }
    screen.save(path).expect("save error");
}

fn main() {
    // input
    const HYPER: usize = 16;
    let sampling_pipe = 4;
    let size = uvec2(32, 32);

    let mut buffer = new_2d_array(
        false, size.x as usize*sampling_pipe*HYPER, size.y as usize*sampling_pipe*HYPER);

    init()
        .iter()
        .map(|x| x.mul_scalar(HYPER as f32 * sampling_pipe as f32))
        .for_each(|tri| rasterization(tri, &mut buffer));
    
    let buffer = sampling(buffer, sampling_pipe, size.y as usize*HYPER, size.x as usize*HYPER);

    output("out.bmp", &buffer, HYPER as u32*size.x, HYPER as u32*size.y);
}
