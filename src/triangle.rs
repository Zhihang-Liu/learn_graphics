use std::convert::TryInto;
use glam::*;
use ordered_float::OrderedFloat;

use crate::utils::f2b;


#[derive(Debug, Default)]
pub struct Triangle {
    pub v: [Vec3; 3],
    pub color: Vec3,
    // pub tex_coords: [Vec2; 3],
    // pub normal: [Vec3; 3],
}

impl Triangle {
    pub fn to_vec4(&self) -> [Vec4; 3] {
        self.v.iter()
            .map(|x| x.extend(1.))
            .collect::<Vec<Vec4>>()
            .try_into().unwrap()
    }

    pub fn inside(&self, x: usize, y: usize) -> bool {
        let p = vec3(x as f32 + 0.5, y as f32 + 0.5, 1.0);
        let v0: Vec3 = self.v[1] - self.v[0];
        let v1: Vec3 = self.v[2] - self.v[1];
        let v2: Vec3 = self.v[0] - self.v[2];
        let p0: Vec3 = p - self.v[0];
        let p1: Vec3 = p - self.v[1];
        let p2: Vec3 = p - self.v[2];
        let a = f2b(v0.cross(p0));
        let b = f2b(v1.cross(p1));
        let c = f2b(v2.cross(p2));
        a == b && b == c
    }

    pub fn bounding_box(&self) -> Mat2 {
        let vec_iter = [self.v[0], self.v[1], self.v[2]];
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
}