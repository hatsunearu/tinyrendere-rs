extern crate image;
extern crate rand;
#[macro_use]
extern crate itertools;

use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use image::{ImageBuffer, Rgb};
use rand::prelude::*;
use cgmath::prelude::*;

use cgmath::{Vector3, Point3};


mod gfx;
use crate::gfx::*;

fn main(){

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {:?} [obj_name]", args[0]);
        return;
    }
    
    
    let file = File::open(&args[1]).unwrap();
    let mut buf = BufReader::new(file);
    
    let (verts, faces) = parse_obj(&mut buf);
    //println!("{:?}", verts);

    let imgx = 500;
    let imgy = 500;

    let mut imgbuf: ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::new(imgx, imgy);
    let white = image::Rgb{ data: [255, 255, 255] };
    let red = image::Rgb{ data: [255, 0, 0] };
    let green = image::Rgb{ data: [0, 255, 0] };
    let blue = image::Rgb{ data: [0, 0, 255] };

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
    
    if args.len() >= 4 {
        xbias = args[2].parse().unwrap();
        ybias = args[3].parse().unwrap();
    }
    
    for face in faces {
    
        let mut ind0 = face[0];
        let mut ind1 = face[1];
        let mut ind2 = face[2];

        let v0 = verts[ind0];
        let v1 = verts[ind1];
        let v2 = verts[ind2];
        
        //let mut normal_v = crossp(&sub_v(&v2, &v0), &sub_v(&v1, &v0));
        //normalize(&mut normal_v);
        //println!("{:?} {:?} {:?}", v0, v1, v2);
        let mut normal_v = (v2 - v0).cross(v1 - v0).normalize();
        
        let intensity: f32 = normal_v.dot(light_dir);
        
        let v0 = convert_coords(v0, xbias, ybias, imgx, imgy);
        let v1 = convert_coords(v1, xbias, ybias, imgx, imgy);
        let v2 = convert_coords(v2, xbias, ybias, imgx, imgy);
    
        
        if intensity > 0. { 
    
            draw_tri(&mut imgbuf, [v0, v1, v2], 
            image::Rgb{ data: [cmp::min(255, (intensity * 255.0) as u8); 3] });
    
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

fn convert_coords(vertex: Point3<f32>, xbias: f32, ybias: f32, imgx: u32, imgy: u32) -> [i32; 2] {
    let x0 = ((vertex.x+xbias) * imgx as f32/2. ) as i32; 
    let y0 = ((vertex.y+ybias) * imgy as f32/2. ) as i32; 
    [x0, y0]
}


fn parse_obj<B>(buf: B) -> (Vec<Point3<f32>>, Vec<[usize; 3]>) 
where
    B: BufRead
{
    
    let mut verticies: Vec<Point3<f32>> = Vec::new();
    let mut face_vert_inds: Vec<_> = Vec::new();
    
    for (i, line) in buf.lines().enumerate() {
        let line = line.unwrap();
    
        let split: Vec<_> = line.trim().split_whitespace().collect();
        if split.len() == 0 {
            continue;
        }
        
        if split[0] == "v" {
            if split.len() != 4 {
                panic!("Warning: could not parse vertex at line number {:?}", i);
            }
            
            if let (Ok(vx), Ok(vy), Ok(vz)) = (split[1].parse::<f32>(), split[2].parse::<f32>(), split[3].parse::<f32>()) {
                let v = Point3::new(vx, vy, vz);
                verticies.push(v);
            }
            else {
                panic!("Warning: could not parse vertex at line number {:?}", i);
            }
        }
        
        else if split[0] == "f" {
            if split.len() != 4 {
                panic!("Warning: could not parse face at line number {:?}", i);
            }
            
            let v0: Vec<_> = split[1].split("/").collect();
            let v1: Vec<_> = split[2].split("/").collect();
            let v2: Vec<_> = split[3].split("/").collect();
            
            let v0_v: i32 = v0[0].parse().unwrap();
            let v1_v: i32 = v1[0].parse().unwrap();
            let v2_v: i32 = v2[0].parse().unwrap();

            let inds: [i32; 3] = [v0_v - 1, v1_v - 1, v2_v - 1];
            
            face_vert_inds.push(inds);
        }
    }
    
    let wrap_index = |i: &i32| if *i < 0 { (*i + verticies.len() as i32) as usize } else { *i as usize };
    
    let face_vert_inds: Vec<[usize; 3]> = 
    face_vert_inds.iter()
    .map(|[i0, i1, i2]| [wrap_index(i0), wrap_index(i1), wrap_index(i2)] )
    .collect();

    (verticies, face_vert_inds)
}
