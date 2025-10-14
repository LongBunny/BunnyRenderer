mod renderer;

use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;
use glow::HasContext;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::{GlConfig, GlDisplay, GlSurface, NotCurrentGlContext};
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
    renderer: Option<renderer::Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attribs = Some(Window::default_attributes().with_title("Hello Katharina"));
        
        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(window_attribs);
        
        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs.reduce(|accum, cfg| {
                    if cfg.num_samples() > accum.num_samples() {
                        cfg
                    } else {
                        accum
                    }
                }).unwrap()
            }).unwrap();
        
        let window = window.unwrap();
        let raw_window_handle = window.raw_window_handle().unwrap();
        
        let context_attribs = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));
        
        let not_current = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attribs)
                .unwrap()
        };
        
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new()
            .build(
                window.raw_window_handle().unwrap(),
                NonZeroU32::new(800).unwrap(), NonZeroU32::new(600).unwrap(),
            );
        
        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };
        
        let context = not_current.make_current(&surface).unwrap();
        
        let gl = unsafe {
            glow::Context::from_loader_function(|s| gl_config.display().get_proc_address(CString::new(s).unwrap().as_c_str()))
        };
        
        self.window = Some(window);
        self.renderer = Some(renderer::Renderer::new(gl, surface, context));
    }
    
    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        
        if self.renderer.is_none() {
            return;
        }
        
        let renderer = self.renderer.as_mut().unwrap();
        
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                
                renderer.render();
                
                // Ask for another frame
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}


fn main() -> ExitCode {
    let mut event_loop = EventLoop::new().unwrap();
    
    let mut app = App::default();
    
    loop {
        let timeout = Some(Duration::ZERO);
        let status = event_loop.pump_app_events(timeout, &mut app);
        
        if let PumpStatus::Exit(exit_code) = status {
            break ExitCode::from(exit_code as u8);
        }
        
        sleep(Duration::from_millis(16));
    }
}
