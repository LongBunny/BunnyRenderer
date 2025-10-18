use std::ptr::null;
use crate::renderer::buffer::{Buffer, EBO, VAO, VBO};

#[allow(dead_code)]
pub struct Mesh {
    ebo: EBO,
    vbo: VBO,
    vao: VAO,
    
    indices_len: usize,
}

impl Mesh {
    pub fn new(vertices: &Vec<f32>, indices: &Vec<u32>) -> Self {
        let ebo = EBO::new();
        let vbo = VBO::new();
        let vao = VAO::new();
        
        ebo.bind();
        ebo.buffer_data(&indices);
        
        vao.bind();
        vbo.bind();
        vbo.buffer_data(vertices);
        vao.vertex_attrib_pointer(0, 3, 3);
        
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
}