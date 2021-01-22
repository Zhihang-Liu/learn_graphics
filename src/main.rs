mod triangle;
mod rasterization;
mod utils;

use std::cell::Cell;

use glam::*;
use bmp::consts::{RED, WHITE};


use utils::*;
use triangle::*;
use rasterization::*;

/*
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


fn output(path: &str, buffer: &Vec<Vec<Vec3>>, x: u32, y: u32) {
    let mut screen = Image::new(x, y);
    for (y, l) in buffer.iter().enumerate() {
        for (x, p) in l.iter().enumerate() {
            screen.set_pixel(x as u32, y as u32, vec3_to_pixel(*p));
        }
    }
    screen.save(path).expect("save error");
}

 */

fn init() -> Vec<Triangle> {
    let tri = Triangle {
        v: Cell::from([
            const_vec3!([2.2, 1.3, 1.0]),
            const_vec3!([4.4, 11.0, 1.0]),
            const_vec3!([15.3, 8.6, 1.0])
        ]),
        color: pixel_to_vec3(WHITE),
    };
    let tri1 = Triangle {
        v: Cell::from([
            const_vec3!([5.8, 5.2, 1.0]),
            const_vec3!([15.4, 20.0, 1.0]),
            const_vec3!([25.3, 24.6, 1.0])
        ]),
        color: pixel_to_vec3(RED),

    };
    vec![tri1]
}

fn main() {
    let mut raster = Rasterizer::new();
    raster.set_size(32, 32);
    raster.rasterization(&mut init());
    // raster.sampling_range(0, 0);
    raster.draw();
}
