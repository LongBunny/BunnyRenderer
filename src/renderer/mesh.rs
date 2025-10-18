use std::ptr::null;
use glm::Vec4;
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
        vao.vertex_attrib_pointer(0, 3, 7, 0);
        vao.vertex_attrib_pointer(1, 4, 7, 3);
        
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
            Vertex {pos: glm::Vec3::new(0.5,  0.5, 0.0), col: Vec4::new(1.0, 1.0, 1.0, 1.0)},
            Vertex {pos: glm::Vec3::new(0.5, -0.5, 0.0), col: Vec4::new(1.0, 1.0, 1.0, 1.0)},
            Vertex {pos: glm::Vec3::new(-0.5, -0.5, 0.0), col: Vec4::new(1.0, 1.0, 1.0, 1.0)},
            Vertex {pos: glm::Vec3::new(-0.5,  0.5, 0.0), col: Vec4::new(1.0, 1.0, 1.0, 1.0)},
        ];
        
        let indices: Vec<u32> = vec![
            0, 1, 3,
            1, 2, 3,
        ];
        
        Mesh::new(&vertices, &indices)
    }
}