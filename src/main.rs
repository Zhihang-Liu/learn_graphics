use glam::*;
use bmp::{Image, consts::{BLACK, WHITE}};
use ordered_float::OrderedFloat;

type Triangle = Mat3;

#[inline]
fn f2b(i: Vec3) -> bool {
    i.z > 0.0
}

fn inside(tri: Triangle, x: f32, y: f32) -> bool {
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

fn bounding_box(tri: Triangle) -> Mat2 {
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

fn rasterization(tri: Triangle, screen: &mut Image) {
    let size = bounding_box(tri);
    for y in size.x_axis.y as u32..size.y_axis.y as u32+1 {
        for x in size.x_axis.x as u32..size.y_axis.x as u32+1 {
            if screen.get_pixel(x, y) == BLACK {
                let r = inside(tri, x as f32+0.5, y as f32+0.5);
                let r = if r { WHITE } else { BLACK };
                screen.set_pixel(x, y, r);
            };
        }
    }
}

fn main() {
    // input
    let hyper: f32 = 100.0;
    let size = uvec2(32*hyper as u32, 32*hyper as u32);
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
    let mut screen = Image::new(size.x, size.y);
    [tri, tri1]
        .iter()
        .map(|x| x.mul_scalar(hyper))
        .for_each(|tri| rasterization(tri, &mut screen));
    screen.save("out.bmp").expect("save error");
}
