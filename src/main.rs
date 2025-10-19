mod renderer;

use std::cell::RefCell;
use crate::renderer::mesh::Mesh;
use crate::renderer::shader::Shader;
use crate::renderer::model::{Model, Transform};
use glm::{cos, sin, Vec3, Vec4};
use num_traits::identities::One;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use std::ffi::{c_void, CStr};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;
use num_traits::Zero;

fn main() {
    let mut width = 1920;
    let mut height = 1080;
    
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
    
    let mut default_shader = Rc::new(RefCell::new(
        Shader::new(&PathBuf::from("res/shaders/default.vert"), &PathBuf::from("res/shaders/default.frag")).unwrap()
    ));
    
    let mut checkerboard_shader = Rc::new(RefCell::new(
        Shader::new(&PathBuf::from("res/shaders/checkerboard.vert"), &PathBuf::from("res/shaders/checkerboard.frag")).unwrap()
    ));
    
    let quad_mesh = Rc::new(RefCell::new(
        Mesh::quad()
    ));
    let cube_mesh = Rc::new(RefCell::new(
        Mesh::cube()
    ));
    
    let cube1 = Model::with_transform(
        cube_mesh.clone(), default_shader.clone(),
        Transform::new(Vec3::new(0.0, 0.0, 0.0), Vec3::one(), Vec3::zero())
    );
    let mut quad1 = Model::with_transform(
        quad_mesh.clone(), default_shader.clone(),
        Transform::new(Vec3::new(1.5, 0.75, 0.0), Vec3::one(), Vec3::zero())
    );
    let mut cube2 = Model::with_transform(
        cube_mesh.clone(), default_shader.clone(),
        Transform::new(Vec3::new(-3.0, 0.0, -5.0), Vec3::one(), Vec3::zero())
    );
    
    let floor = Model::with_transform(
        quad_mesh.clone(), checkerboard_shader.clone(),
        Transform::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(50.0, 50.0, 1.0), Vec3::new(-std::f32::consts::PI / 2.0, 0.0, 0.0))
    );
    
    // floor.set_tint(Vec4::new(50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0, 1.0));
    // quad2.set_tint(Vec4::new(0.0, 1.0, 0.0, 1.0));
    // quad3.set_tint(Vec4::new(0.0, 0.0, 1.0, 1.0));
    
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::LINE_SMOOTH);
        gl::Enable(gl::CULL_FACE);
        gl::ClearColor(0.1, 0.3, 0.2, 1.0);
    }
    
    let mut projection = glm::ext::perspective(70f32, aspect_ratio, 0.01, 100.0);
    let view = glm::ext::look_at(
        Vec3::new(0.0, 1.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );
    let pv_mat = projection * view;
    
    let mut i = 0f32;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'running,
                        Keycode::R => {
                            println!("Reloading shaders");
                            default_shader.borrow().unbind();
                            match default_shader.borrow_mut().reload() {
                                Ok(_) => { println!("default_shader reloaded!") }
                                Err(e) => { eprintln!("default_shader compilation failed: {}", e) }
                            }
                            
                            checkerboard_shader.borrow().unbind();
                            match checkerboard_shader.borrow_mut().reload() {
                                Ok(_) => { println!("shader_checkerboard reloaded!") }
                                Err(e) => { eprintln!("shader_checkerboard compilation failed: {}", e) }
                            }
                        }
                        _ => {}
                    }
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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        let old_rot = cube1.transform().rotation();
        cube1.transform_mut().set_rotation(Vec3::new(old_rot.x + 0.034, old_rot.y + 0.05, old_rot.z - 0.01));
        
        let old_rot = quad1.transform().rotation();
        quad1.transform_mut().set_rotation(Vec3::new(old_rot.x, old_rot.y + 0.02, old_rot.z));
        
        // let old_pos = floor.transform().pos();
        // floor.transform_mut().set_pos(Vec3::new(old_pos.x, old_pos.y - 0.01, old_pos.z));
        
        quad1.set_tint(Vec4::new(sin(i) * 0.5 + 0.5, 72.0 / 255.0, 213.0 / 255.0, 1.0));
        cube2.set_tint(Vec4::new(
            sin(i + 1.242 * 0.5) * 0.5 + 0.5,
            cos(i + 2.5283 * 0.2) * 0.5 + 0.5,
            (sin(i + 0.82 * 0.7) * 0.5 + 0.5 + cos(i + 0.8223 * 1.23) * 0.5 + 0.5) * 0.5,
            1.0));
        
        floor.render(pv_mat);
        cube1.render(pv_mat);
        cube2.render(pv_mat);
        quad1.render(pv_mat);
        
        window.gl_swap_window();
        
        i += 0.01;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}