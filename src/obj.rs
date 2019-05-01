use cgmath::{Vector3, Point3, Point2};
use std::f32;
use std::io::{BufRead, BufReader};

pub struct GfxObject<T> {
    pub faces: Vec<T>,
    pub verts: Vec<Point3<f32>>,
    pub tex_uvs: Vec<Point2<f32>>,
    pub normals: Vec<Vector3<f32>>
}

pub struct Face {
    pub verts: [Point3<f32>; 3],
    pub tex_uvs: Option<[Point2<f32>; 3]>,
    pub normals: Option<[Vector3<f32>; 3]>
}

pub struct FaceIndex<T> {
    pub verts: [T; 3],
    pub tex_uvs: Option<[T; 3]>,
    pub normals: Option<[T; 3]>
}

pub fn deindex_face(face: &FaceIndex<usize>, gfxobj: &GfxObject<FaceIndex<usize>>) -> Face {
    let verts = [gfxobj.verts[face.verts[0]], gfxobj.verts[face.verts[1]], gfxobj.verts[face.verts[2]]];
    let tex_uvs = match face.tex_uvs {
        Some(ref x) => Some([gfxobj.tex_uvs[x[0]], gfxobj.tex_uvs[x[1]], gfxobj.tex_uvs[x[2]]]),
        None => None
    };
    let normals = match face.normals {
        Some(ref x) => Some([gfxobj.normals[x[0]], gfxobj.normals[x[1]], gfxobj.normals[x[2]]]),
        None => None
    };
    Face { verts, tex_uvs, normals }
}

// triangle coords, faces
pub fn parse_obj<B>(buf: B) -> GfxObject<FaceIndex<usize>>
where
    B: BufRead
{

    let mut verts: Vec<Point3<f32>> = Vec::new();
    let mut tex_uvs: Vec<Point2<f32>> = Vec::new();
    let mut normals: Vec<Vector3<f32>> = Vec::new();

    // face indicies before normalization
    let mut faces_raw: Vec<FaceIndex<i64>> = Vec::new(); 
    
    

    
    for (i, line) in buf.lines().enumerate() {
        let line = line.unwrap();
    
        let split: Vec<_> = line.trim().split_whitespace().collect();
        if split.len() == 0 {
            continue;
        }
        
        // vertex parsing
        if split[0] == "v" {
            if split.len() != 4 {
                panic!("Warning: could not parse vertex at line number {:?}", i);
            }
            
            if let (Ok(vx), Ok(vy), Ok(vz)) = (split[1].parse::<f32>(), split[2].parse::<f32>(), split[3].parse::<f32>()) {
                verts.push(Point3::new(vx, vy, vz));
            }
            else {
                panic!("Warning: could not parse vertex at line number {:?}", i);
            }
        }
        
        // tex UV parsing
        if split[0] == "vt" {
            if split.len() > 4 {
                panic!("Warning: could not parse tex UV at line number {:?} {:?}", i, split);
            }
            
            if let (Ok(u), Ok(v)) = (split[1].parse::<f32>(), split[2].parse::<f32>()) {
                tex_uvs.push(Point2::new(u, v));
            }
            else {
                panic!("Warning: could not parse tex UV at line number {:?}", i);
            }
        }
        
        // normals parsing
        if split[0] == "vn" {
            if split.len() != 4 {
                panic!("Warning: could not parse normals at line number {:?}", i);
            }
            
            if let (Ok(vx), Ok(vy), Ok(vz)) = (split[1].parse::<f32>(), split[2].parse::<f32>(), split[3].parse::<f32>()) {
                normals.push(Vector3::new(vx, vy, vz));
            }
            else {
                panic!("Warning: could not parse normals at line number {:?}", i);
            }
        }
        
        else if split[0] == "f" {
            if split.len() != 4 {
                panic!("Warning: could not parse face at line number {:?}", i);
            }
            
            // TODO: handle cases where the face doesn't have certain vertex attributes
            
            let v0: Vec<_> = split[1].split("/").collect();
            let v1: Vec<_> = split[2].split("/").collect();
            let v2: Vec<_> = split[3].split("/").collect();
            
            let v0_v: i64 = v0[0].parse().unwrap();
            let v1_v: i64 = v1[0].parse().unwrap();
            let v2_v: i64 = v2[0].parse().unwrap();
            
            let v0_t: i64 = v0[1].parse().unwrap();
            let v1_t: i64 = v1[1].parse().unwrap();
            let v2_t: i64 = v2[1].parse().unwrap();
            
            let v0_n: i64 = v0[2].parse().unwrap();
            let v1_n: i64 = v1[2].parse().unwrap();
            let v2_n: i64 = v2[2].parse().unwrap();

            let face_verts_raw: [i64; 3] = [v0_v - 1, v1_v - 1, v2_v - 1];
            let face_tex_uvs_raw: [i64; 3] = [v0_t - 1, v1_t - 1, v2_t - 1];
            let face_normals_raw: [i64; 3] = [v0_n - 1, v1_n - 1, v2_n - 1];
            
            faces_raw.push(FaceIndex::<i64> {verts: face_verts_raw, tex_uvs: Some(face_tex_uvs_raw), normals: Some(face_normals_raw)});
        }
    }
    
    let wrap_one_index = |i: &i64, cap: usize| if *i < 0 { (*i + cap as i64) as usize } else { *i as usize };
    let wrap_indicies = |indicies: &[i64; 3], cap: usize| [wrap_one_index(&indicies[0], cap), wrap_one_index(&indicies[1], cap), wrap_one_index(&indicies[2], cap) ];
    
    let faces = 
    faces_raw.iter()
    .map(|raw_face| FaceIndex::<usize> { 
        verts: wrap_indicies(&raw_face.verts, verts.len()), 
        tex_uvs: match raw_face.tex_uvs {
            Some(ref t) => Some(wrap_indicies(t, tex_uvs.len())),
            None => None
        },
        normals: match raw_face.normals {
            Some(ref t) => Some(wrap_indicies(t, normals.len())),
            None => None
        }
    })
    .collect();
    
    GfxObject {faces, verts, tex_uvs, normals}
}