
extern crate num;

use std::f32;
use std::mem;
use core::ops::{Deref, DerefMut};
use image::{ImageBuffer, Pixel};
use image::{RgbImage, RgbaImage, Rgb, Rgba};

use cgmath::prelude::*;
use cgmath::{Vector3, Point3, Point2, Matrix4, Vector4};

use crate::obj::Face;

pub struct RenderCtx<'a> {
    pub framebuffer: &'a mut RgbImage,
    pub zbuf: &'a mut Vec<f32>,
    pub transform_matrix: &'a mut Matrix4<f32>
}

/*
fn safe_put_pixel<P, C>(buf: &mut ImageBuffer<P, C>, x: i32, y: i32, color: P)
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    C: Deref<Target = [P::Subpixel]> + DerefMut
{
    if buf.width() as i32 > x && buf.height() as i32 > y && x >= 0 && y >= 0 {
        buf.put_pixel(x as u32, y as u32, color);
    }
}

pub fn draw_line<P, C>(buf: &mut ImageBuffer<P, C>, x0: i32, y0: i32, x1: i32, y1: i32, color: P) 
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    C: Deref<Target = [P::Subpixel]> + DerefMut
{

    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;

    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }
    
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;
    
    for x in x0..x1+1 {
        if steep {
            safe_put_pixel(buf, y, x, color);
        }
        else {
            safe_put_pixel(buf, x, y, color);
        }
        error2 += derror2;
        if error2 > dx {
            if y1 > y0 {
                y += 1;
            }
            else {
                y -= 1;
            }
            error2 -= dx*2;
        }
    }
}
*/

pub fn viewport(ctx: &RenderCtx, xscale: f32, yscale: f32, xtrans: f32, ytrans: f32) -> Matrix4<f32> {
    let imgx = ctx.framebuffer.width();
    let imgy = ctx.framebuffer.height();
    Matrix4::new(
        
        xscale * imgx as f32 / 2., 0., 0., 0., 
        0., yscale * imgy as f32 / 2., 0., 0.,
        0., 0., 1., 0.,
        (1.+xtrans) * imgx as f32 / 2., (1.+ytrans) * imgy as f32 / 2., 0., 1.
    )
}

pub fn apply_transform(ctx: &RenderCtx, verts: [Point3<f32>; 3]) -> [Point3<f32>; 3]
{
    let mut retval = [
        verts[0].to_homogeneous(),
        verts[1].to_homogeneous(),
        verts[2].to_homogeneous()
    ];
    
    for point in retval.iter_mut() {
        *point = *ctx.transform_matrix * *point;
    }
    
    [
        Point3::<f32>::from_homogeneous(retval[0]),
        Point3::<f32>::from_homogeneous(retval[1]),
        Point3::<f32>::from_homogeneous(retval[2])
    ]
}

pub fn draw_tri(ctx: &mut RenderCtx, face: &Face, tex_img: &RgbaImage, light_vec: Vector3<f32>)
{
    let mut bboxmin: [f32; 2] = [f32::INFINITY, f32::INFINITY];
    let mut bboxmax: [f32; 2] = [f32::NEG_INFINITY, f32::NEG_INFINITY];
    let clamp: [f32; 2] = [(ctx.framebuffer.width() - 1) as f32, (ctx.framebuffer.height() - 1) as f32];
 
    // convert to screen space verts
    let verts = apply_transform(&ctx, face.verts);
    
    //println!("points: {:?}", points);
    
    for p in &verts {
        // add perspective projection here?
        let x = p.x;
        let y = p.y; 
        
        for (c, min, max, clmp) in izip!([x, y].iter(), &mut bboxmin, &mut bboxmax, &clamp) {
            *min = 0f32.max(min.min(*c));
            *max = clmp.min(max.max(*c));
        }
    }
    
    let bboxmin: [u32; 2] = [bboxmin[0] as u32, bboxmin[1].ceil() as u32];
    let bboxmax: [u32; 2] = [bboxmax[0] as u32, bboxmax[1].ceil() as u32];
    
    //println!("bboxmin: {:?}, bboxmax: {:?}", bboxmin, bboxmax);
    
    
    /*
    let bboxmin: [i32; 2] = [0, 0];
    let bboxmax: [i32; 2] = [buf.width() as i32, buf.height() as i32];
    */
    
    for x in bboxmin[0]..=bboxmax[0] {
        for y in bboxmin[1]..=bboxmax[1] {
            let bary_v = barycentric([x as f32, y as f32], &verts);
            
            if bary_v.x < 0. || bary_v.y < 0. || bary_v.z < 0. {
                continue;
            }
            
            let z = &verts.iter().map(|p| p.z).sum::<f32>() + bary_v.sum();
            //buf.put_pixel(x as u32, y as u32, color);

            if ctx.zbuf[(x + y*ctx.framebuffer.width()) as usize ] < z {
                
                let tex_color = if let &Some(tex_uvs) = &face.tex_uvs {
                    interpolate_texel(&bary_v, &tex_uvs, tex_img)
                }
                else {
                    Rgba{ data: [255, 255, 255, 0] }
                };
                
                // get normal vector (phong shading)
                let normal_v = if let &Some(normals) = &face.normals {
                    (normals[0] * -bary_v[0] + normals[1] * -bary_v[1] + normals[2] * -bary_v[2]).normalize()
                }
                // fallback
                else {
                    (face.verts[2] - face.verts[0]).cross(face.verts[1] - face.verts[0]).normalize()
                };
                
                let brightness = light_vec.dot(normal_v);
                
                if brightness < 0. {
                    continue;
                }
                
                
                if tex_color[3] == 255 {
                    ctx.zbuf[(x + y*ctx.framebuffer.width()) as usize] = z;
                    ctx.framebuffer.put_pixel(x as u32, y as u32, 
                    Rgb{ data: [
                        (tex_color.data[0] as f32 * brightness) as u8, 
                        (tex_color.data[1] as f32 * brightness) as u8, 
                        (tex_color.data[2] as f32 * brightness) as u8
                    ]});
                }
                else {
                    let old_pixel = ctx.framebuffer.get_pixel(x as u32, y as u32);
                    let trans = 1. - (tex_color[3] as f32) / 255.;
                    let new_pixel = Rgb{ data: [
                    num::clamp(tex_color[0] as f32 * brightness + old_pixel[0] as f32 * trans, 0., 255.) as u8, 
                    num::clamp(tex_color[1] as f32 * brightness+ old_pixel[1] as f32 * trans, 0., 255.) as u8, 
                    num::clamp(tex_color[2] as f32 * brightness+ old_pixel[2] as f32 * trans, 0., 255.) as u8
                    ]};

                    ctx.framebuffer.put_pixel(x as u32, y as u32, new_pixel);
                }
                

            }

        }
    }
    
    
}

// barycentric basis of the input point `p` on the triangle defined by `points`
fn barycentric(p: [f32; 2], points: &[Point3<f32>; 3]) -> Vector3<f32> {


    let a = Vector3::new(
    (points[2].x - points[0].x) as f32,
    (points[1].x - points[0].x) as f32,
    (points[0].x - p[0]) as f32
    );
    
    let b = Vector3::new(
    (points[2].y - points[0].y) as f32,
    (points[1].y - points[0].y) as f32,
    (points[0].y - p[1]) as f32
    );
    
    let u = a.cross(b);
    
    // degenerate case
    if u.z.abs() < 1e-2 {
        return Vector3::new(-1., 0., 0.);
    }
    
    Vector3::new(1. - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
}

fn interpolate_texel(coord_bary: &Vector3<f32>, uvs: &[Point2<f32>; 3], tex: &RgbaImage) -> Rgba<u8>
{
    let u = uvs[0].x * coord_bary.x + uvs[1].x * coord_bary.y + uvs[2].x * coord_bary.z;
    let v = uvs[0].y * coord_bary.x + uvs[1].y * coord_bary.y + uvs[2].y * coord_bary.z;
    
    let u_max = tex.width() as f32;
    let v_max = tex.height() as f32;
    
    let x_texel = (u_max * u).round() as u32;
    let y_texel = (v_max * (1.-v)).round() as u32;
    
    tex.get_pixel(x_texel, y_texel).clone()
}
