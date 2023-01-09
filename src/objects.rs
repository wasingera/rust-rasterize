use crate::Triangle;

use sdl2::video::Window;
use sdl2::render::Canvas;

use glam::*;

pub struct Object {
    pub triangles: Vec<Triangle>
}

impl Object {
    pub fn draw(self: &Self, canvas: &mut Canvas<Window>) {
        for triangle in self.triangles.iter() {
            triangle.draw(canvas);
        }
    }

    pub fn apply_vertex_shader(self: &mut Self, shader: Mat4) {
        for triangle in self.triangles.iter_mut() {
            triangle.v0.0 = shader.transform_point3(triangle.v0.0);
            triangle.v1.0 = shader.transform_point3(triangle.v1.0);
            triangle.v2.0 = shader.transform_point3(triangle.v2.0);
        }
    }
}
