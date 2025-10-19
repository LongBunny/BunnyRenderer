mod renderer;

use crate::renderer::mesh::Mesh;
use crate::renderer::shader::Shader;
use glm::Vec3;
use num_traits::identities::One;
use num_traits::Zero;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use std::ffi::{c_void, CStr};
use std::path::Path;
use std::time::Duration;

fn main() {
    let mut width = 800;
    let mut height = 600;
    
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);
    
    
    let window = video_subsystem.window("Hellowo Katharina", width, height)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();
        
    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    
    let display = window.get_display().unwrap();
    let bounds = display.get_bounds();
    println!("bounds: {bounds:?}");
    
    
    let mut aspect_ratio = width as f32 / height as f32;
    
    gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s).unwrap() as *const c_void
    });
    
    unsafe {
        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL version: {}", version.to_string_lossy());
    }
    
    let mut shader = Shader::new(Path::new("res/shaders/vertex.glsl"), Path::new("res/shaders/frag.glsl")).unwrap();
    let mesh = Mesh::quad();
    
    unsafe {
        gl::ClearColor(0.1, 0.3, 0.2, 1.0);
    }
    
    let mut projection = glm::ext::perspective(70f32, aspect_ratio, 0.1, 100.0);
    let view = glm::ext::look_at(
        Vec3::new(0.0, 0.0, 2.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );
    let pv_mat = projection * view;
    let mut model = glm::Mat4::one();
    
    let mut pvm_mat = pv_mat * model;
    
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
                        width = w as u32;
                        height = h as u32;
                        aspect_ratio = w as f32 / h as f32;
                        projection = glm::ext::perspective(90f32, aspect_ratio, 0.1, 100.0);
                    }
                }
                _ => {}
            }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        shader.bind();
        
        model = glm::ext::rotate(&model, 0.05, glm::vec3(0.0, 1.0, 0.0));
        pvm_mat = pv_mat * model;
        
        let pvm_loc = shader.get_uniform_location("pvm").unwrap();
        shader.set_uniform(pvm_loc, pvm_mat);
        
        mesh.render();
        
        window.gl_swap_window();
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}