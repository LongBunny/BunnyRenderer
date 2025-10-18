use gl::types::{GLsizeiptr, GLuint};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::ffi::{c_void, CStr, CString};
use std::path::Path;
use std::ptr::{null, null_mut};
use std::time::Duration;

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
    
    let vertices: Vec<f32> = vec![
        0.5,  0.5, 0.0,  // top right
        0.5, -0.5, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  // bottom left
        -0.5,  0.5, 0.0   // top left
    ];
    
    let indices: Vec<u32> = vec![
        0, 1, 3,
        1, 2, 3,
    ];
    
    let mut ebo: u32 = 0;
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (size_of::<u32>() * indices.len()) as isize, indices.as_ptr() as *const _, gl::STATIC_DRAW);
        
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (size_of::<f32>() * vertices.len()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);
        
        gl::VertexAttribPointer(0, 3, gl::FLOAT, 0, 3 * size_of::<f32>() as i32, null());
        gl::EnableVertexAttribArray(0);
    }
    
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
                _ => {}
            }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::UseProgram(program);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, null());
        }
        
        window.gl_swap_window();
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
        gl::DeleteProgram(program);
    }
}