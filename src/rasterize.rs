use crate::CANVAS_WIDTH;
use crate::CANVAS_HEIGHT;

use sdl2::{pixels::Color, rect::Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use glam::{Vec3, vec3};

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

pub fn draw_triangle(canvas: &mut Canvas<Window>, p0: Point, p1: Point, p2: Point, color: Vec3) {
    let e01 = |x: i32, y: i32| {
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        (x - p0.x) * dy - (y - p0.y) * dx
    };
    let e12 = |x: i32, y: i32| {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (x - p1.x) * dy - (y - p1.y) * dx
    };
    let e20 = |x: i32, y: i32| {
        let dx = p0.x - p2.x;
        let dy = p0.y - p2.y;
        (x - p2.x) * dy - (y - p2.y) * dx
    };

    for x in -(CANVAS_WIDTH / 2)..=(CANVAS_WIDTH / 2) {
        for y in -(CANVAS_HEIGHT / 2)..=(CANVAS_HEIGHT / 2) {
            // println!("Point ({}, {}): {}, {}, {}", x, y, e01(x,y), e12(x,y), e20(x,y));
            if e01(x,y) <= 0 && e12(x,y) <= 0 && e20(x,y) <= 0 {
                // println!("putting triangle color");
                put_pixel(canvas, x, y, color);
            } else {
                put_pixel(canvas, x, y, vec3(0.0,0.0,0.0));
            }
        }
    }
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

