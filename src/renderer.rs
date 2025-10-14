use glutin::prelude::GlSurface;
use glow::HasContext;
use glutin::context::PossiblyCurrentContext;
use glutin::surface::{Surface, WindowSurface};

pub struct Renderer {
    gl: glow::Context,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

impl Renderer {
    pub fn new(gl: glow::Context, gl_surface: Surface<WindowSurface>, gl_context: PossiblyCurrentContext) -> Self {
        Self {
            gl,
            gl_surface,
            gl_context,
        }
    }
    
    pub fn render(&mut self) {
        
        let (gl, surface, context) = (&self.gl, &self.gl_surface, &self.gl_context);
        
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            
        }
        surface.swap_buffers(context).unwrap();
    }
}