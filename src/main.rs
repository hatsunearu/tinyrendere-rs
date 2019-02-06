extern crate image;
#[macro_use]
extern crate itertools;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use image::{ImageBuffer, Rgb};

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
    
    draw_tri(&mut imgbuf, [[10,10], [100, 30], [190, 160]], red);
    
    let mut xbias: f32 = 0.;
    let mut ybias: f32 = 0.;
    
    if args.len() >= 4 {
        xbias = args[2].parse().unwrap();
        ybias = args[3].parse().unwrap();
    }
    
    for face in faces {
    
        for i in 0..3 {
            let mut ind0 = face[i];
            let mut ind1 = face[(i+1) % 3];
            
            if ind0 < 0 {
                ind0 += verts.len() as i32;
            }
            if ind1 < 0 {
                ind1 += verts.len() as i32;
            }
            
            let v0 = verts[ind0 as usize];
            let v1 = verts[ind1 as usize];
            
            let x0 = ((v0[0]+xbias) * imgx as f32/2. ) as i32; 
            let y0 = ((v0[1]+ybias) * imgy as f32/2. ) as i32; 
            let x1 = ((v1[0]+xbias) * imgx as f32/2. ) as i32; 
            let y1 = ((v1[1]+ybias) * imgy as f32/2. ) as i32; 
            draw_line(&mut imgbuf, x0, y0, x1, y1, white); 
        }
        
    }
    
    
    image::imageops::flip_vertical(&imgbuf).save("render.png").unwrap();

}


fn parse_obj<B>(buf: B) -> (Vec<[f32; 3]>, Vec<[i32; 3]>) 
where
    B: BufRead
{

    
    let mut verticies: Vec<[f32; 3]> = Vec::new();
    let mut face_vert: Vec<[i32; 3]> = Vec::new();
    
    for (i, line) in buf.lines().enumerate() {
        let line = line.unwrap();
    
        let split: Vec<_> = line.trim().split_whitespace().collect();
        if split.len() == 0 {
            continue;
        }
        
        if split[0] == "v" {
            if split.len() != 4 {
                panic!("vertex length is not equal to 4: {:?}: {:?}", i, &line);
            }
            
            let v: [f32; 3] = [split[1].parse().unwrap(), split[2].parse().unwrap(), split[3].parse().unwrap()];
            
            verticies.push(v);
        }
        
        else if split[0] == "f" {
            if split.len() != 4 {
                panic!("face length is not equal to 4: {:?}: {:?}", i, &line);
            }
            
            let v0: Vec<_> = split[1].split("/").collect();
            let v1: Vec<_> = split[2].split("/").collect();
            let v2: Vec<_> = split[3].split("/").collect();
            
            let v0_0: i32 = v0[0].parse().unwrap();
            let v1_0: i32 = v1[0].parse().unwrap();
            let v2_0: i32 = v2[0].parse().unwrap();

            let inds: [i32; 3] = [v0_0 - 1, v1_0 - 1, v2_0 - 1];
            
            face_vert.push(inds);
        }
    }
    
    (verticies, face_vert)
}
