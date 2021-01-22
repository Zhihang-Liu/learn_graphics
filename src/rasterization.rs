use std::cell::{Cell, RefCell};


use bmp::Image;
use glam::*;

use crate::{
    triangle::*,
    utils::*};

#[derive(Debug)]
pub struct Rasterizer {
    pub model: Cell<Mat4>,
    pub view: Cell<Mat4>,
    pub projection: Cell<Mat4>,
    pub width: Cell<usize>,
    pub height: Cell<usize>,
    pub sampling_pipe: Cell<usize>,
    pub frame_buf: RefCell<Vec<Vec3>>,
    pub depth_buf: RefCell<Vec<f32>>,
}

impl Rasterizer {
    pub fn new() -> Self {
        Rasterizer {
            model: Cell::default(),
            view: Cell::default(),
            projection: Cell::default(),
            width: Cell::from(32),
            height: Cell::from(32),
            sampling_pipe: Cell::from(4),
            frame_buf: RefCell::from(vec![Vec3::default(); 32*32]),
            depth_buf: RefCell::from(vec![0.; 32*32]),
        }
    }

    pub fn set_size(&mut self, w: usize, h: usize) {
        self.width.set(w);
        self.width.set(h);
        let sp = self.sampling_pipe.get();
        self.frame_buf.get_mut().resize(w*sp*h*sp, Vec3::default());
        self.depth_buf.get_mut().resize(w*sp*h*sp, f32::default());
    }

    fn set_pixel(&mut self, point: &UVec3, color: &Vec3) {
        let ind = point2index(
            self.width.get(),
            point.x as usize,
            point.y as usize);
        self.frame_buf.get_mut()[ind] = *color;
    }


    pub fn rasterization(&mut self, tris: &mut [Triangle]) {
        let sp = self.sampling_pipe.get() as f32;
        tris
        .iter_mut()
        .for_each(|x|
            x.apply_transform(&const_mat4!(
                [sp, 0., 0., 0.],
                [0., sp, 0., 0.],
                [0., 0., sp, 0.],
                [0., 0., 0., 1.])));
        tris.iter()
        .for_each(|tri| self.rasterization_triangle(tri));
    }

    fn rasterization_triangle(&mut self, tri: &Triangle) {
        let size = tri.bounding_box();
        for y in size.x_axis.y as usize..size.y_axis.y as usize + 1 {
            for x in size.x_axis.x as usize..size.y_axis.x as usize + 1 {
                if tri.inside(x, y) {
                    self.set_pixel(&uvec3(x as u32, y as u32, 1), &tri.color);
                }
            }
        }
    }

    fn sampling_range(&self, x: usize, y: usize) -> Vec3 {
        let sp = self.sampling_pipe.get();
        // let h = self.height.get();
        let w = self.width.get();
        let hx = x*sp;
        let hy = y*sp;

        let slice_i = (hy..hy+sp)
                .flat_map(|y| (hx..hx+sp).map(move |x|(x, y)))
                .map(|(x, y)| point2index(w, x, y))
                .map(|ind| self.frame_buf.try_borrow().unwrap()[ind])
                .collect();
        return sampling_unit(&slice_i);
    }

    fn sampling(&self) -> Vec<Vec3> {
        let h = self.height.get();
        let w = self.width.get();
        let r: Vec<Vec3> = (0..h)
            .flat_map(|y| (0..w).map(move |x| (x, y)))
            .map(|(x, y)| self.sampling_range(x, y))
            .collect();
        return r;
    }

    pub fn draw(&self) {
        let w = self.width.get();
        let h = self.height.get();
        let buf = self.sampling();
        let mut screen = Image::new(w as u32, h as u32);
        for (ind, p) in buf.iter().enumerate() {
            let (x, y) = index2point(w, ind);
            screen.set_pixel(x as u32, y as u32, vec3_to_pixel(*p));
        }
        screen.save("out.bmp").expect("save error");
    }
}
