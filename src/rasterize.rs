use crate::CANVAS_WIDTH;
use crate::CANVAS_HEIGHT;

use sdl2::{pixels::Color, rect::Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::cmp::{min, max};

use glam::*;

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub v0: (Point, Vec3),
    pub v1: (Point, Vec3),
    pub v2: (Point, Vec3),
}

impl Triangle {
    pub fn draw_outline(self: &Self, canvas: &mut Canvas<Window>) {
        let v0 = &self.v0;
        let v1 = &self.v1;
        let v2 = &self.v2;

        draw_line(canvas, v0.0, v1.0, v0.1, v1.1);
        draw_line(canvas, v1.0, v2.0, v1.1, v2.1);
        draw_line(canvas, v2.0, v0.0, v2.1, v0.1);
    }
}

pub fn put_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Vec3) {
    let x = (CANVAS_WIDTH / 2) + x;
    let y = (CANVAS_HEIGHT / 2) - y;
    let color = Color::RGB(color.x as u8, color.y as u8, color.z as u8);
    let point = Point::new(x, y);

    canvas.set_draw_color(color);
    canvas.draw_point(point).unwrap();
}

pub fn draw_triangle(canvas: &mut Canvas<Window>, v0: (Vec2, Vec3), v1: (Vec2, Vec3), v2: (Vec2, Vec3)) {
    let (a, a_color) = v0;
    let (b, b_color) = v1;
    let (c, c_color) = v2;

    // compute bounding box
    let x_min = a.x.min(b.x);
    let x_min = x_min.min(c.x) as i32;
    let y_min = a.y.min(b.y);
    let y_min = y_min.min(c.y) as i32;

    let x_max = a.x.max(b.x);
    let x_max = x_max.max(c.x) as i32;
    let y_max = a.y.max(b.y);
    let y_max = y_max.max(c.y) as i32;

    let a_total = edge_function(b, c, a);

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let w_ab = edge_function(a, b, vec2(x as f32, y as f32)) / a_total;
            let w_bc = edge_function(b, c, vec2(x as f32, y as f32)) / a_total;
            let w_ca = edge_function(c, a, vec2(x as f32, y as f32)) / a_total;

            let range = 0.0..=1.0;
            if range.contains(&w_ab) && range.contains(&w_bc) && range.contains(&w_ca) {
                let color = w_ab * a_color + w_bc * b_color + w_ca * c_color;
                put_pixel(canvas, x, y, color);
            } else {
                put_pixel(canvas, x, y, vec3(0.0, 0.0, 0.0));
            }
        }
    }
}

fn edge_function(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

pub fn draw_line(canvas: &mut Canvas<Window>, p0: Point, p1: Point, color0: Vec3, color1: Vec3) {
    let line = interpolate_line(p0, p1);
    let colors = interpolate_color(color0, color1, line.len());

    for i in 0..line.len() {
        let point = line[i];
        let color = colors[i];
        put_pixel(canvas, point.x, point.y, color)
    }
}

pub fn draw_triangle_outline(canvas: &mut Canvas<Window>, p0: Point, p1: Point, p2: Point,
                             color0: Vec3, color1: Vec3, color2: Vec3) {
    draw_line(canvas, p0, p1, color0, color1);
    draw_line(canvas, p1, p2, color1, color2);
    draw_line(canvas, p2, p0, color2, color0);
}

pub fn interpolate_color(c0: Vec3, c1: Vec3, count: usize) -> Vec<Vec3> {
    let mut colors: Vec<Vec3> = Vec::new();
    for i in 0..=count {
        let c_weight = (i as f32) / count as f32;
        let color = c0 * (1.0 - c_weight) + c1 * c_weight;
        colors.push(color);
    }

    colors
}

pub fn interpolate_line(p0: Point, p1: Point) -> Vec<Point> {
    // Bresenham's Line Algorithm
    let mut x0 = p0.x;
    let mut y0 = p0.y;

    let x1 = p1.x;
    let y1 = p1.y;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };

    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;


    let mut values: Vec<Point> = Vec::new();
    loop {
        values.push(Point::new(x0, y0));

        if x0 == x1 && y0 == y1 { break }

        let e2 = 2 * error;
        if e2 >= dy {
            if x0 == x1 { break }
            error += dy;
            x0 += sx;
        }

        if e2 <= dx {
            if y0 == y1 { break }
            error += dx;
            y0 += sy;
        }
    }

    values
}

