use std::path::PathBuf;
use num_traits::identities::One;
use std::ptr::null;
use glm::{Vec2, Vec3, Vec4};
use crate::renderer::buffer::{Buffer, EBO, VAO, VBO};
use crate::renderer::vertex::Vertex;

#[allow(dead_code)]
pub struct Mesh {
    ebo: EBO,
    vbo: VBO,
    vao: VAO,
    
    indices_len: usize,
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
        let ebo = EBO::new();
        let vbo = VBO::new();
        let vao = VAO::new();
        
        ebo.bind();
        ebo.buffer_data(&indices);
        
        vao.bind();
        vbo.bind();
        vbo.buffer_data(vertices);
        vao.vertex_attrib_pointer(0, 3, 8, 0);
        vao.vertex_attrib_pointer(1, 3, 8, 3);
        vao.vertex_attrib_pointer(2, 2, 8, 6);
        
        ebo.unbind();
        vbo.unbind();
        vao.unbind();
        
        Self {
            ebo,
            vbo,
            vao,
            indices_len: indices.len()
        }
    }
    
    pub fn from_model(path: &PathBuf) -> Result<Self, String> {
        let (vertices, indices) = load_from_obj(path)?;
        Ok(Mesh::new(&vertices, &indices))
    }
    
    pub fn render(&self) {
        self.vao.bind();
        self.ebo.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices_len as i32, gl::UNSIGNED_INT, null());
        }
    }
    
    pub fn quad() -> Self {
        let vertices: Vec<Vertex> = vec![
            Vertex { v: Vec3::new(-0.5, -0.5, 0.0), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 0.0) },
            Vertex { v: Vec3::new( 0.5, -0.5, 0.0), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 0.0) },
            Vertex { v: Vec3::new( 0.5,  0.5, 0.0), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 1.0) },
            Vertex { v: Vec3::new(-0.5,  0.5, 0.0), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 1.0) },
        ];
        
        let indices: Vec<u32> = vec![
            0, 1, 2,
            2, 3, 0,
        ];
        
        Mesh::new(&vertices, &indices)
    }
    
    pub fn cube() -> Self {
        // let vertices: Vec<Vertex> = vec![
        //     // Front face corners
        //     Vertex { v: Vec3::new(-0.5, -0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 0.0)},
        //     Vertex { v: Vec3::new( 0.5, -0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 0.0)},
        //     Vertex { v: Vec3::new( 0.5,  0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 1.0)},
        //     Vertex { v: Vec3::new(-0.5,  0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 1.0)},
        //     // Back face corners
        //     Vertex { v: Vec3::new(-0.5, -0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(1.0, 0.0)},
        //     Vertex { v: Vec3::new( 0.5, -0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(0.0, 0.0)},
        //     Vertex { v: Vec3::new( 0.5,  0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(0.0, 1.0)},
        //     Vertex { v: Vec3::new(-0.5,  0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(1.0, 1.0)},
        // ];
        //
        // let indices: Vec<u32> = vec![
        //     // Front
        //     0, 1, 2,  2, 3, 0,
        //     // Back
        //     5, 4, 7,  7, 6, 5,
        //     // Left
        //     4, 0, 3,  3, 7, 4,
        //     // Right
        //     1, 5, 6,  6, 2, 1,
        //     // Top
        //     3, 2, 6,  6, 7, 3,
        //     // Bottom
        //     4, 5, 1,  1, 0, 4,
        // ];
        
        let vertices: Vec<Vertex> = vec![
            // Front face (+Z) - Red
            Vertex { v: Vec3::new(-0.5, -0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 0.0) },
            Vertex { v: Vec3::new( 0.5, -0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 0.0) },
            Vertex { v: Vec3::new( 0.5,  0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 1.0) },
            Vertex { v: Vec3::new(-0.5,  0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 1.0) },
            
            // Back face (-Z) - Green
            Vertex { v: Vec3::new( 0.5, -0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.0), vt: Vec2::new(0.0, 0.0) },
            Vertex { v: Vec3::new(-0.5, -0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.0), vt: Vec2::new(1.0, 0.0) },
            Vertex { v: Vec3::new(-0.5,  0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.0), vt: Vec2::new(1.0, 1.0) },
            Vertex { v: Vec3::new( 0.5,  0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.0), vt: Vec2::new(0.0, 1.0) },
            
            // Top face (+Y) - Blue
            Vertex { v: Vec3::new(-0.5,  0.5,  0.5), vn: Vec3::new(0.0, 1.0, 0.0), vt: Vec2::new(0.0, 0.0) },
            Vertex { v: Vec3::new( 0.5,  0.5,  0.5), vn: Vec3::new(0.0, 1.0, 0.0), vt: Vec2::new(1.0, 0.0) },
            Vertex { v: Vec3::new( 0.5,  0.5, -0.5), vn: Vec3::new(0.0, 1.0, 0.0), vt: Vec2::new(1.0, 1.0) },
            Vertex { v: Vec3::new(-0.5,  0.5, -0.5), vn: Vec3::new(0.0, 1.0, 0.0), vt: Vec2::new(0.0, 1.0) },
            
            // Bottom face (-Y) - Yellow
            Vertex { v: Vec3::new(-0.5, -0.5, -0.5), vn: Vec3::new(0.0, -1.0, 0.0), vt: Vec2::new(0.0, 0.0) },
            Vertex { v: Vec3::new( 0.5, -0.5, -0.5), vn: Vec3::new(0.0, -1.0, 0.0), vt: Vec2::new(1.0, 0.0) },
            Vertex { v: Vec3::new( 0.5, -0.5,  0.5), vn: Vec3::new(0.0, -1.0, 0.0), vt: Vec2::new(1.0, 1.0) },
            Vertex { v: Vec3::new(-0.5, -0.5,  0.5), vn: Vec3::new(0.0, -1.0, 0.0), vt: Vec2::new(0.0, 1.0) },
            
            // Right face (+X) - Magenta
            Vertex { v: Vec3::new( 0.5, -0.5,  0.5), vn: Vec3::new(1.0, 0.0, 0.0), vt: Vec2::new(0.0, 0.0) },
            Vertex { v: Vec3::new( 0.5, -0.5, -0.5), vn: Vec3::new(1.0, 0.0, 0.0), vt: Vec2::new(1.0, 0.0) },
            Vertex { v: Vec3::new( 0.5,  0.5, -0.5), vn: Vec3::new(1.0, 0.0, 0.0), vt: Vec2::new(1.0, 1.0) },
            Vertex { v: Vec3::new( 0.5,  0.5,  0.5), vn: Vec3::new(1.0, 0.0, 0.0), vt: Vec2::new(0.0, 1.0) },
            
            // Left face (-X) - Cyan
            Vertex { v: Vec3::new(-0.5, -0.5, -0.5), vn: Vec3::new(-1.0, 0.0, 0.0), vt: Vec2::new(0.0, 0.0) },
            Vertex { v: Vec3::new(-0.5, -0.5,  0.5), vn: Vec3::new(-1.0, 0.0, 0.0), vt: Vec2::new(1.0, 0.0) },
            Vertex { v: Vec3::new(-0.5,  0.5,  0.5), vn: Vec3::new(-1.0, 0.0, 0.0), vt: Vec2::new(1.0, 1.0) },
            Vertex { v: Vec3::new(-0.5,  0.5, -0.5), vn: Vec3::new(-1.0, 0.0, 0.0), vt: Vec2::new(0.0, 1.0) },
        ];
        
        let indices: Vec<u32> = vec![
            // Front
            0, 1, 2,  2, 3, 0,
            // Back
            4, 5, 6,  6, 7, 4,
            // Top
            8, 9, 10,  10, 11, 8,
            // Bottom
            12, 13, 14,  14, 15, 12,
            // Right
            16, 17, 18,  18, 19, 16,
            // Left
            20, 21, 22,  22, 23, 20,
        ];
        
        Mesh::new(&vertices, &indices)
    }
}

fn load_from_obj(path: &PathBuf) -> Result<(Vec<Vertex>, Vec<u32>), String> {
    todo!()
}