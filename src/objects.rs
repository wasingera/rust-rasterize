use crate::Triangle;

use sdl2::video::Window;
use sdl2::render::Canvas;

use glam::*;

#[derive(Debug)]
pub struct Object {
    pub triangles: Vec<Triangle>
}

impl Object {
    pub fn draw(self: &Self, canvas: &mut Canvas<Window>, zbuff: &mut Vec<Vec<f32>>) {
        for triangle in self.triangles.iter() {
            triangle.draw(canvas, zbuff);
            // println!("Drawing {:?}", triangle);
        }
        // println!();
    }

    pub fn draw_wireframe(self: &Self, canvas: &mut Canvas<Window>, zbuff: &mut Vec<Vec<f32>>) {
        for triangle in self.triangles.iter() {
            triangle.draw_wireframe(canvas, zbuff);
        }
    }

    pub fn apply_vertex_shader(self: &mut Self, shader: Mat4) -> Object {
        let mut new_triangles: Vec<Triangle> = Vec::new();

        for triangle in self.triangles.iter() {
            let mut new = Triangle {..*triangle};

            new.v0.0 = shader.transform_point3(triangle.v0.0);
            new.v1.0 = shader.transform_point3(triangle.v1.0);
            new.v2.0 = shader.transform_point3(triangle.v2.0);

            new_triangles.push(new);
        }

        Object{triangles: new_triangles}
    }
}
