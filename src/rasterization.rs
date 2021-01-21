use std::{cell::{Cell, RefCell}, default};


use glam::*;


use crate::triangle::Triangle;

#[derive(Debug)]
struct Rasterizer {
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
    fn new() -> Self {
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

    fn set_size(&mut self, w: usize, h: usize) {
        self.frame_buf.get_mut().resize(w*h, Vec3::default());
        self.depth_buf.get_mut().resize(w*h, f32::default());
    }

    fn set_pixel(&mut self, point: &UVec3, color: &Vec3) {
        let ind = (self.height.get()-1-point.y as usize)*self.width.get() + point.x as usize;
        self.frame_buf.get_mut()[ind] = *color;
    }

    fn rasterization(tris: &[Triangle]) {
        
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
}
