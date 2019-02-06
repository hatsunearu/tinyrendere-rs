

use std::cmp;
use std::mem;
use core::ops::{Deref, DerefMut};
use image::{ImageBuffer, Pixel};
    


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

pub fn draw_tri<P, C>(buf: &mut ImageBuffer<P, C>, points: [[i32; 2]; 3], color: P)
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    C: Deref<Target = [P::Subpixel]> + DerefMut
{
    let mut bboxmin: [i32; 2] = [(buf.width() - 1) as i32, (buf.height() - 1) as i32];
    let mut bboxmax: [i32; 2] = [0, 0];
    let clamp: [i32; 2] = [(buf.width() - 1) as i32, (buf.height() - 1) as i32];
    
    for p in &points {
        for (c, min, max, clamp) in izip!(p, &mut bboxmin, &mut bboxmax, &clamp) {
            *min = cmp::max(0, cmp::min(*min, *c));
            *max = cmp::min(*clamp, cmp::max(*max, *c));
        }
    }
    
    for x in bboxmin[0]..bboxmax[0]+1 {
        for y in bboxmin[0]..bboxmax[1]+1 {
            let bary_v = barycentric([x, y], points);
            if (bary_v[0] > 0. && bary_v[1] > 0. && bary_v[2] > 0.) {
                safe_put_pixel(buf, x, y, color);
            }
        }
    }
    
    
}

// barycentric basis of the input point `p` on the triangle defined by `points`
fn barycentric(p: [i32; 2], points: [[i32; 2]; 3]) -> [f32; 3] {
    let acx = points[2][0] - points[0][0];
    let abx = points[1][0] - points[0][0];
    let pax = points[0][0] - p[0];
    
    let acy = points[2][1] - points[0][1];
    let aby = points[1][1] - points[0][1];
    let pay = points[0][1] - p[1];
    
    let u: [f32; 3] = [
        (abx * pay - pax * aby) as f32,
        (pax * acy - acx * pay) as f32,
        (acx * aby - abx * acy) as f32
    ];
    
    // degenerate case
    if u[2].abs() < 1.0 {
        return [-1., 1., 1.];
    }
    
    [1. - (u[0] + u[1]) / u[2], u[1] / u[2], u[0] / u[2]]
}
