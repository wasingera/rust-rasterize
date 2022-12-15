extern crate sdl2;

use sdl2::{pixels::Color, rect::Point};
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;

use glam::{Vec3, vec3};

const CANVAS_WIDTH: i32 = 800;
const CANVAS_HEIGHT: i32 = 600;

fn put_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Vec3) {
    let x = (CANVAS_WIDTH / 2) + x;
    let y = (CANVAS_HEIGHT / 2) - y;
    let color = Color::RGB(color.x as u8, color.y as u8, color.z as u8);
    let point = Point::new(x, y);

    canvas.set_draw_color(color);
    canvas.draw_point(point).unwrap();
}

fn draw_line(canvas: &mut Canvas<Window>, p0: Point, p1: Point, color: Vec3) {
    let line = interpolate_line(p0, p1);

    for point in line {
        put_pixel(canvas, point.x, point.y, color);
    }
}

fn interpolate_line(p0: Point, p1: Point) -> Vec<Point> {
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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rasterizer", CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let p0 = Point::new(-200, -100);
    let p1 = Point::new(240, 120);
    draw_line(&mut canvas, p0, p1, vec3(255.0, 255.0, 255.0));
    let p0 = Point::new(-50, -200);
    let p1 = Point::new(60, 240);
    draw_line(&mut canvas, p0, p1, vec3(255.0, 255.0, 255.0));

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                _ => {}
            }
        }
    }
}
