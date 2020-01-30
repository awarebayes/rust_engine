use std::fs;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32)
}
implement_vertex!(Vertex, position);

impl Vertex {
    pub fn from_vec_f32(vec: Vec<f32>) -> Vertex {
        let p0 = vec.get(0).unwrap() as &f32;
        let p1 = vec.get(1).unwrap() as &f32;
        let p2 = vec.get(2).unwrap() as &f32;
        return Vertex {position: (*p0, *p1, *p2)};
    }
}

#[derive(Copy, Clone)]
pub struct Normal{
    normal: (f32, f32, f32)
}
implement_vertex!(Normal, normal);

impl Normal {
    pub fn from_vec_f32(vec: Vec<f32>) -> Normal {
        let p0 = vec.get(0).unwrap() as &f32;
        let p1 = vec.get(1).unwrap() as &f32;
        let p2 = vec.get(2).unwrap() as &f32;
        return Normal {normal: (*p0, *p1, *p2)};
    }
}

pub struct Obj3D {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}

impl Obj3D {
    pub fn get_vertices(&self) -> &[Vertex] {
       return &(self.vertices) as &[Vertex]; 
    }

    pub fn get_normals(&self) -> &[Normal] {
        return &(self.normals) as &[Normal];
    }

    pub fn get_indices(&self) -> &[u32] {
       return &(self.indices) as &[u32];
    }

    pub fn load_file(path: &String) -> Obj3D {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();
        let mut indices: Vec<f32> = Vec::new();

        let contents = fs::read_to_string(path)
            .expect("Something went wrong with file reading");
        for line in contents.lines() {
            if line.trim().is_empty() { continue; }
            let splitted: Vec<&str> = line.trim().split(" ").collect();
            let t: &str = splitted.get(0).unwrap();
            let args: Vec<&str> = splitted[1..].to_vec();
            let args: Vec<f32> = args.iter()
                .filter_map(|s| s.parse::<f32>().ok())
                .collect::<Vec<_>>();
        
            match t {
                "v" => vertices.push(Vertex::from_vec_f32(args)),
                "vn" => normals.push(Normal::from_vec_f32(args)),
                "f" => indices.append(&mut args.iter().map(|&i| i-1.0).collect()),
                _ => ()
            }
        } 
        let indices: Vec<u32> = indices.iter().map(|&e| e as u32).collect();
        return Obj3D{vertices, indices, normals};

    }
}
