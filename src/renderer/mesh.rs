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
        vao.vertex_attrib_pointer(1, 2, 8, 5);
        
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
        let vertices: Vec<Vertex> = vec![
            // Front face corners
            Vertex { v: Vec3::new(-0.5, -0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 0.0)},
            Vertex { v: Vec3::new( 0.5, -0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 0.0)},
            Vertex { v: Vec3::new( 0.5,  0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(1.0, 1.0)},
            Vertex { v: Vec3::new(-0.5,  0.5,  0.5), vn: Vec3::new(0.0, 0.0, 1.0), vt: Vec2::new(0.0, 1.0)},
            // Back face corners
            Vertex { v: Vec3::new(-0.5, -0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(1.0, 0.0)},
            Vertex { v: Vec3::new( 0.5, -0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(0.0, 0.0)},
            Vertex { v: Vec3::new( 0.5,  0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(0.0, 1.0)},
            Vertex { v: Vec3::new(-0.5,  0.5, -0.5), vn: Vec3::new(0.0, 0.0, -1.), vt: Vec2::new(1.0, 1.0)},
        ];
        
        let indices: Vec<u32> = vec![
            // Front
            0, 1, 2,  2, 3, 0,
            // Back
            5, 4, 7,  7, 6, 5,
            // Left
            4, 0, 3,  3, 7, 4,
            // Right
            1, 5, 6,  6, 2, 1,
            // Top
            3, 2, 6,  6, 7, 3,
            // Bottom
            4, 5, 1,  1, 0, 4,
        ];
        
        Mesh::new(&vertices, &indices)
    }
}