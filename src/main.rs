mod renderer;

use crate::renderer::mesh::Mesh;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use std::ffi::{c_void, CStr, CString};
use std::path::Path;
use std::ptr::{null, null_mut};
use std::time::Duration;
use crate::renderer::vertex::Vertex;

enum ShaderType {
    Vertex,
    Fragment,
}

fn create_shader(shader_type: ShaderType, path: &Path) -> Result<u32, String> {
    let shader_src = std::fs::read_to_string(path).unwrap();
    let shader_src = CString::new(shader_src).unwrap();
    unsafe {
        let shader_type = match shader_type {
            ShaderType::Vertex => {gl::VERTEX_SHADER}
            ShaderType::Fragment => {gl::FRAGMENT_SHADER}
        };
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &shader_src.as_ptr(), null());
        gl::CompileShader(shader);
        
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != 1 {
            const LOG_SIZE: usize = 512;
            let mut log = [0i8; LOG_SIZE];
            gl::GetShaderInfoLog(shader, LOG_SIZE as i32, null_mut(), log.as_mut_ptr());
            let log_str = std::ffi::CStr::from_ptr(log.as_ptr()).to_string_lossy();
            return Err(std::format!("Could not compile shader: {}", log_str));
        }
        
        Ok(shader)
    }
}

fn create_program(vertex: u32, fragment: u32) -> Result<u32, String> {
    unsafe {
        let program = gl::CreateProgram();
        
        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);
        gl::LinkProgram(program);
        
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != 1 {
            const LOG_SIZE: usize = 512;
            let mut log = [0i8; LOG_SIZE];
            gl::GetProgramInfoLog(program, LOG_SIZE as i32, null_mut(), log.as_mut_ptr());
            let log_str = std::ffi::CStr::from_ptr(log.as_ptr()).to_string_lossy();
            return Err(std::format!("Could not link program: {}", log_str));
        }
        
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);
        
        Ok(program)
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("Hellowo Katharina", 800, 600)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    
    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    
    gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s).unwrap() as *const c_void
    });
    
    unsafe {
        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL version: {}", version.to_string_lossy());
    }
    
    
    let vertex_shader = create_shader(ShaderType::Vertex, Path::new("res/shaders/vertex.glsl")).unwrap();
    let fragment_shader = create_shader(ShaderType::Fragment, Path::new("res/shaders/frag.glsl")).unwrap();
    let program = create_program(vertex_shader, fragment_shader).unwrap();
    
    let mesh = Mesh::quad();
    
    unsafe {
        gl::ClearColor(0.1, 0.3, 0.2, 1.0);
    }
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::Window {win_event: WindowEvent::Resized(w, h), ..} => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                }
                _ => {}
            }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(program);
        }
        
        mesh.render();
        
        window.gl_swap_window();
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}