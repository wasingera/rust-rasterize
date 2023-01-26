use crate::{CANVAS_WIDTH, CANVAS_HEIGHT, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, PROJ_D};

use sdl2::{pixels::Color, rect::Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use glam::*;

pub fn put_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Vec3) {
    let x = (CANVAS_WIDTH / 2) + x;
    let y = (CANVAS_HEIGHT / 2) - y;
    let color = Color::RGB(color.x as u8, color.y as u8, color.z as u8);
    let point = Point::new(x, y);

    canvas.set_draw_color(color);
    canvas.draw_point(point).unwrap();
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub v0: (Vec3, Vec3),
    pub v1: (Vec3, Vec3),
    pub v2: (Vec3, Vec3),
}

impl Triangle {
    pub fn draw(self: &Self, canvas: &mut Canvas<Window>, zbuff: &mut Vec<Vec<f32>>) {
        let (a_orig, a_color) = self.v0;
        let (b_orig, b_color) = self.v1;
        let (c_orig, c_color) = self.v2;

        let a = project_vertex(a_orig, PROJ_D);
        let b = project_vertex(b_orig, PROJ_D);
        let c = project_vertex(c_orig, PROJ_D);

        // compute bounding box
        let x_min = a.x.min(b.x).min(c.x) as i32;
        let y_min = a.y.min(b.y).min(c.y) as i32;

        let x_max = a.x.max(b.x).max(c.x) as i32;
        let y_max = a.y.max(b.y).max(c.y) as i32;

        let a_total = Self::edge_function(b, c, a);

        println!("xmax: {:?}, yman: {:?}", x_min, y_min);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let w_ab = Self::edge_function(a, b, vec2(x as f32, y as f32)) / a_total;
                let w_bc = Self::edge_function(b, c, vec2(x as f32, y as f32)) / a_total;
                let w_ca = Self::edge_function(c, a, vec2(x as f32, y as f32)) / a_total;

                let range = 0.0..=1.0;
                if range.contains(&w_ab) && range.contains(&w_bc) && range.contains(&w_ca) {
                    let color = w_ab * a_color + w_bc * b_color + w_ca * c_color;

                    let z = w_ab * a_orig.z + w_bc * b_orig.z + w_ca * c_orig.z;

                    if Self::check_zbuff(x, y, z, zbuff) {
                        put_pixel(canvas, x, y, color);
                    }
                }
            }
        }
    }

    fn edge_function(a: Vec2, b: Vec2, p: Vec2) -> f32 {
        (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
    }

    fn interpolate_z(z0: f32, z1: f32, l: f32) -> f32 {
        let inverse_z = (1.0 / z0) * (1.0 - l) + (1.0 / z1) * (1.0 - l);

        1.0 / inverse_z
    }

    fn check_zbuff(x: i32, y: i32, depth: f32, zbuff: &mut Vec<Vec<f32>>) -> bool {
        let x = CANVAS_WIDTH / 2 + x;
        let y = CANVAS_HEIGHT / 2 - y;

        if x >= CANVAS_WIDTH || x < 0 || y >= CANVAS_HEIGHT || y < 0 {
            return false;
        }

        let buff_depth = zbuff[y as usize][x as usize];

        if depth <= buff_depth && depth >= PROJ_D {
            return true;
        }

        false
    }
}

fn project_vertex(v: Vec3, d: f32) -> Vec2 {
    if v.z == 0.0 {
        println!("Can't project to 0 plane!");
        return vec2(0.0, 0.0);
    }

    let x = v.x * d / v.z;
    let y = v.y * d / v.z;
    viewport_to_canvas(vec2(x, y))
}

fn viewport_to_canvas(v: Vec2) -> Vec2 {
    let height = CANVAS_HEIGHT as f32;
    let width = CANVAS_WIDTH as f32;
    return vec2(v.x * width / VIEWPORT_WIDTH, v.y * height / VIEWPORT_HEIGHT);
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

