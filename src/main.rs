extern crate sdl2;

use objects::Object;
use sdl2::keyboard::Keycode;
use sdl2::{pixels::Color, rect::Point};
use sdl2::event::Event;

use glam::*;

use tobj::Model;

use crate::rasterize::*;

pub const CANVAS_WIDTH: i32 = 800;
pub const CANVAS_HEIGHT: i32 = 600;

pub const VIEWPORT_WIDTH: f32 = 1.0;
pub const VIEWPORT_HEIGHT: f32 = 1.0;

pub const PROJ_D: f32 = 1.0;

pub mod rasterize;
pub mod objects;
pub mod shaders;

fn load_obj(fname: &str) -> Vec<Model> {
    let load_options = tobj::LoadOptions {
        triangulate: true,
        ..tobj::LoadOptions::default()
    };

    let (models, materials) =
        tobj::load_obj(
            fname,
            &load_options
            )
        .expect("Failed to OBJ load file");

    // let materials = materials.expect("Failed to load MTL file");
    println!("Number of models = {:?}", models.len());
    // println!("Number of materials = {}", materials.len());

    models
}

fn create_triangles(model: &Model) -> Vec<Triangle> {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut colors: Vec<Vec3> = Vec::new();

    let mut triangles: Vec<Triangle> = Vec::new();

    let mesh = &model.mesh;

    for vtx in 0..mesh.positions.len() / 3 {
        vertices.push(vec3(
            mesh.positions[vtx*3],
            mesh.positions[vtx*3 + 1],
            mesh.positions[vtx*3 + 2],
            ));
        colors.push(vec3(203.0, 195.0, 227.0));
        // colors.push(vec3(
        //     mesh.vertex_color[vtx*3],
        //     mesh.vertex_color[vtx*3 + 1],
        //     mesh.vertex_color[vtx*3 + 2],
        //     ));
    }
    // println!("INDICES LEN: {:?}", mesh.indices.len());
    // println!("INDICES: {:?}", mesh.indices);
    // println!("Vertices: {:?}", vertices);
    // println!("VERTICES LEN: {:?}", vertices.len());
    // println!("INDICES LEN: {:?}", mesh.indices.len());
    // println!("FACE COUNT: {:?}", mesh.face_arities.len());

    for i in 0..mesh.indices.len() / 3 {
        let i0 = mesh.indices[i*3] as usize;
        let i1 = mesh.indices[i*3+1] as usize;
        let i2 = mesh.indices[i*3+2] as usize;

        let v0 = (vertices[i0], colors[i0]);
        let v1 = (vertices[i1], colors[i1]);
        let v2 = (vertices[i2], colors[i2]);
        triangles.push(Triangle {v0, v1, v2});
    }

    triangles
}

fn create_objects(models: Vec<Model>) -> Vec<Object> {
    let mut objects: Vec<Object> = Vec::new();

    for model in models.iter() {
        let triangles = create_triangles(model);
        objects.push(Object {triangles});
    }

    objects
}

fn main() {
    // load in obj file
    let models = load_obj("objs/teapot.obj");
    let mut objects = create_objects(models);
    println!("TOTAL OBJECTS: {:?}", objects.len());
    // let triangles = create_triangles(models);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rasterizer", CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut zbuff = vec!(vec!(1000.0 as f32; CANVAS_WIDTH as usize); CANVAS_HEIGHT as usize);

    let mut x_rot = 0.0;
    let mut y_rot = 0.0;
    let mut z_rot = 0.0;
    let rot_speed: f32 = 0.1;
    let mut zoom: f32 = 10.0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                Event::KeyDown {keycode, ..} => {
                    let keycode = keycode.unwrap();
                    match keycode {
                        Keycode::W => x_rot += rot_speed,
                        Keycode::S => x_rot -= rot_speed,
                        Keycode::A => y_rot += rot_speed,
                        Keycode::D => y_rot -= rot_speed,
                        Keycode::Equals => zoom -= 0.2,
                        Keycode::Minus => zoom += 0.2,

                        _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for object in objects.iter_mut() {
            let vertex_shader = Mat4::from_translation(vec3(0.0, 0.0, zoom)) * Mat4::from_rotation_x(x_rot) * Mat4::from_rotation_y(y_rot) * Mat4::from_rotation_z(z_rot);
            let object = object.apply_vertex_shader(vertex_shader);
            // object.draw(&mut canvas, &mut zbuff);
            object.draw_wireframe(&mut canvas, &mut zbuff);
        }

        canvas.present();
    }
}
