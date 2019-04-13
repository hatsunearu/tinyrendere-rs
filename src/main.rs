extern crate image;
extern crate rand;
#[macro_use]
extern crate itertools;

use std::f32;
use std::env;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use image::{ImageBuffer, Rgb};
//use rand::prelude::*;
use cgmath::prelude::*;

use cgmath::{Vector3, Point3, Point2};


mod gfx;
mod obj;
use crate::gfx::*;
use crate::obj::*;

fn main(){

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {:?} [obj_name] [tex_name]", args[0]);
        return;
    }
    
    
    let obj_file = File::open(&args[1]).unwrap();
    let mut obj_buf = BufReader::new(obj_file);
    
    let gfxobj = obj::parse_obj(&mut obj_buf);
    //println!("{:?}", verts);
    
    let tex_file = File::open(&args[2]).unwrap();
    let mut tex_buf = BufReader::new(tex_file);
    
    let tex_img = image::load(tex_buf, image::ImageFormat::PNG).unwrap().to_rgba();

    let imgx = 800;
    let imgy = 600;

    let mut imgbuf: ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::new(imgx, imgy);
    let mut zbuf: Vec<f32> = vec![f32::NEG_INFINITY; imgx as usize * imgy as usize];
    /*
    let white = image::Rgb{ data: [255, 255, 255] };
    let red = image::Rgb{ data: [255, 0, 0] };
    let green = image::Rgb{ data: [0, 255, 0] };
    let blue = image::Rgb{ data: [0, 0, 255] };
    */
    /*
    draw_line(&mut imgbuf, 250, 250, 300, 400, red);
    draw_line(&mut imgbuf, 250, 250, 200, 400, blue);
    draw_line(&mut imgbuf, 250, 250, 100, 300, green);
    draw_line(&mut imgbuf, 250, 250, 100, 200, white);
    draw_line(&mut imgbuf, 250, 250, 200, 100, red);
    draw_line(&mut imgbuf, 250, 250, 300, 100, blue);
    draw_line(&mut imgbuf, 250, 250, 400, 200, green);
    */
    
    //draw_tri(&mut imgbuf, [[300,100], [300, 110], [310, 100]], red);
    
    let light_dir = Vector3::new(0., 0., -1.);
    
    
    
    let mut xbias: f32 = 0.;
    let mut ybias: f32 = 0.;
    
    if args.len() >= 5 {
        xbias = args[3].parse().unwrap();
        ybias = args[4].parse().unwrap();
    }
    
    for face in &gfxobj.faces {
    
        let mut face = obj::deindex_face(&face, &gfxobj);
        //let mut normal_v = crossp(&sub_v(&v2, &v0), &sub_v(&v1, &v0));
        //normalize(&mut normal_v);
        //println!("{:?} {:?} {:?}", v0, v1, v2);
        let normal_v = (face.verts[2] - face.verts[0]).cross(face.verts[1] - face.verts[0]).normalize();
        
        let intensity: f32 = normal_v.dot(light_dir);
        
        face.verts[0] = convert_coords(face.verts[0], xbias, ybias, imgx, imgy);
        face.verts[1] = convert_coords(face.verts[1], xbias, ybias, imgx, imgy);
        face.verts[2] = convert_coords(face.verts[2], xbias, ybias, imgx, imgy);
        
        if intensity > 0. { 
    
            draw_tri(&mut imgbuf, &mut zbuf, &face, &tex_img,
            image::Rgba{ data: [cmp::min(255, (intensity * 255.0) as u8); 4] });
    
        }
    /*
        for i in 0..3 {
            let mut ind0 = face[i];
            let mut ind1 = face[(i+1) % 3];
            
            let v0 = verts[ind0];
            let v1 = verts[ind1];
            
            let x0 = ((v0[0]+xbias) * imgx as f32/2. ) as i32; 
            let y0 = ((v0[1]+ybias) * imgy as f32/2. ) as i32; 
            let x1 = ((v1[0]+xbias) * imgx as f32/2. ) as i32; 
            let y1 = ((v1[1]+ybias) * imgy as f32/2. ) as i32; 
            //draw_line(&mut imgbuf, x0, y0, x1, y1, white); 
        }
        */
    }
    
    
    image::imageops::flip_vertical(&imgbuf).save("render.png").unwrap();
    
}

fn convert_coords(vertex: Point3<f32>, xbias: f32, ybias: f32, imgx: u32, imgy: u32) -> Point3<f32> {
    let x0 = (vertex.x+xbias) * imgx as f32/2.; 
    let y0 = (vertex.y+ybias) * imgy as f32/2.; 
    Point3::new(x0, y0, vertex.z)
}



/*
struct Face<T> {
    coords: [T; 3],
    tex_uv: Option<[T; 3]>,
    normals: Option<[T; 3]>
}


struct GfxObject {
    faces: Vec<Face<usize>>,
    verts: Vec<Point3<f32>>,
    tex_uvs: Vec<Point2<f32>>,
    normals: Vec<Vector3<f32>>
}
*/