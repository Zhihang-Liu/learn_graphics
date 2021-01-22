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
            width: Cell::from(0),
            height: Cell::from(0),
            sampling_pipe: Cell::from(2),
            frame_buf: RefCell::from(vec![]),
            depth_buf: RefCell::from(vec![]),
        }
    }

    pub fn set_size(&mut self, w: usize, h: usize) {
        self.width.set(w);
        self.height.set(h);
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
        // /*
        let sp = self.sampling_pipe.get() as f32;
        tris
        .iter_mut()
        .for_each(|x|
            x.apply_transform(&const_mat4!(
                [sp, 0., 0., 0.],
                [0., sp, 0., 0.],
                [0., 0., sp, 0.],
                [0., 0., 0., 1.])));
        // */
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

    pub fn sampling_range(&self, x: usize, y: usize) -> Vec3 {
        let sp = self.sampling_pipe.get();
        // let h = self.height.get();
        let w = self.width.get();
        let hx = x*sp;
        let hy = y*sp;
        
        /*
        let mut buf = vec![];
        for y in hy..hy+sp {
            for x in hx..hx+sp {
                let ind = point2index(w, x, y);
                println!("x: {}, y: {}, ind: {}", x, y, ind);
                let r = self.frame_buf.try_borrow().unwrap()[ind];
                buf.push(r)
            }
        }
        // */
        // /*
        let buf = (hy..hy+sp)
                .flat_map(|y| (hx..hx+sp).map(move |x|(x, y)))
                // .map(|v| {println!("out: {:?}", v); v})
                .map(|(x, y)| point2index(w, x, y))
                // .map(|v| {println!("out: {}", v); v})
                .map(|ind| self.frame_buf.try_borrow().unwrap()[ind])
                .collect();
        //  */
        return sampling_unit(&buf);
    }

    
    fn sampling(&self) -> Vec<Vec3> {
        // todo!("有bug");
        let h = self.height.get();
        let w = self.width.get();
        /*
        let mut buf = vec![];
        for y in 0..h {
            for x in 0..w {
                buf.push(self.sampling_range(x, y));
            }
        }
        // */
        // /*
        let buf: Vec<Vec3> = (0..h)
            .flat_map(|y| (0..w).map(move |x| (x, y)))
            // .map(|v| { println!("out: {:?}", v); v})
            .map(|(x, y)| self.sampling_range(x, y))
            .collect();
        //  */
        return buf;
    }

    pub fn draw(&self) {
        // For reading and opening files
        use std::path::Path;
        use std::fs::File;
        use std::io::BufWriter;

        let path = Path::new(r"out.png");
        let file = File::create(path).unwrap();
        let ref mut wr = BufWriter::new(file);

        let w = self.width.get();
        let h = self.height.get();

        let mut encoder = png::Encoder::new(wr, w as u32, h as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::RGB);
        // encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let data: Vec<f32> = self.sampling()
            .iter()
            .flat_map(|v| vec![v.x, v.x, v.x])
            .collect();

        let data: Vec<u8> = data.iter().map(|v| *v as u8).collect();

        writer.write_image_data(&data).unwrap();
    }
}
