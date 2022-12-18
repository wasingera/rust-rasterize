extern crate sdl2;

use sdl2::{pixels::Color, rect::Point};
use sdl2::event::Event;

use glam::{Vec3, vec3, vec2};

use tobj::Model;

use crate::rasterize::*;

pub const CANVAS_WIDTH: i32 = 800;
pub const CANVAS_HEIGHT: i32 = 600;

pub const VIEWPORT_WIDTH: f32 = 1.0;
pub const VIEWPORT_HEIGHT: f32 = 1.0;

pub const PROJ_D: f32 = 1.0;

pub mod rasterize;

fn load_obj() -> Vec<Model> {
    let (models, _materials) =
        tobj::load_obj(
            "./objs/triangle.obj",
            &tobj::LoadOptions::default()
            )
        .expect("Failed to OBJ load file");

    println!("Number of models = {:?}", models.len());

    models
}

fn create_triangles(models: Vec<Model>) -> Vec<Triangle> {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut colors: Vec<Vec3> = Vec::new();

    let mut triangles: Vec<Triangle> = Vec::new();

    for model in models.iter() {
        let mesh = &model.mesh;

        for vtx in 0..mesh.positions.len() / 3 {
            vertices.push(vec3(
                mesh.positions[vtx*3],
                mesh.positions[vtx*3 + 1],
                mesh.positions[vtx*3 + 2],
                ));
            colors.push(vec3(
                mesh.vertex_color[vtx*3],
                mesh.vertex_color[vtx*3 + 1],
                mesh.vertex_color[vtx*3 + 2],
                ));
        }

        for i in 0..mesh.indices.len() / 3 {
            let v0 = (vertices[i], colors[i]);
            let v1 = (vertices[i+1], colors[i+1]);
            let v2 = (vertices[i+2], colors[i+2]);
            triangles.push(Triangle {v0, v1, v2});
        }
    }

    triangles
}

fn main() {
    // load in obj file
    let models = load_obj();

    let triangles = create_triangles(models);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rasterizer", CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for triangle in triangles.iter() {
        triangle.draw(&mut canvas);
    }

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
