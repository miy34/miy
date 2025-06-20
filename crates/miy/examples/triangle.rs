use std::{mem::MaybeUninit, rc::Rc};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

use miy::gl;

const WINDOW_TITLE: &str = "Miy Example: Triangle";

pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    winit::event_loop::EventLoop::new()
        .expect("failed to create event loop")
        .run_app(&mut App::default())
        .unwrap()
}

#[derive(Default)]
pub struct App {
    systems: Option<Systems>,
}

pub struct Systems {
    window: Rc<Window>,
    renderer: Renderer,
}

pub struct Renderer {
    gl: gl::Bindings,
    context: gl::Context,

    dummy_vao: gl::VertexArray,
    program: gl::Program,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default().with_title(WINDOW_TITLE))
            .unwrap();

        let window = Rc::new(window);
        let PhysicalSize { width, height } = window.inner_size();
        let surface_size = gl::SurfaceSize::new(width, height).unwrap();

        let renderer = Renderer::new(surface_size, window.clone());

        self.systems = Some(Systems { window, renderer });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(systems) = self.systems.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            // no need to resize context every time, just before rendering is fine
            WindowEvent::Resized(_) => {
                systems.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let PhysicalSize { width, height } = systems.window.inner_size();
                let Some(surface_size) = gl::SurfaceSize::new(width, height) else {
                    return;
                };
                systems.renderer.context.resize_surface(surface_size);
                systems.renderer.context.make_current();
                systems.renderer.draw();
                systems.window.pre_present_notify();
                systems.renderer.context.swap_buffers();
                systems.window.request_redraw();
            }
            _ => (),
        }
    }
}

impl Renderer {
    pub fn new(surface_size: gl::SurfaceSize, window: impl gl::ContextWindow) -> Self {
        let context = gl::Context::new(gl::Version::GL33, gl::Profile::Core, surface_size, window)
            .expect("failed to create OpenGL context");
        let gl = gl::Bindings::with_context(&context).expect("failed to load OpenGL bindings");

        let dummy_vao = unsafe {
            let mut dummy_vao = MaybeUninit::uninit();
            gl.gen_vertex_arrays(1, dummy_vao.as_mut_ptr());
            let dummy_vao = dummy_vao.assume_init();
            gl.bind_vertex_array(dummy_vao);
            dummy_vao
        };

        let program = gl::util::create_program_with_sources(&gl, VS, FS).unwrap();

        unsafe {
            gl.clear_color(0.0, 0.0, 1.0, 1.0);
        }

        Self {
            gl,
            context,
            dummy_vao,
            program,
        }
    }

    pub fn draw(&self) {
        let gl = &self.gl;
        let (surface_width, surface_height) = self.context.surface_size().as_int();

        unsafe {
            gl.clear(gl::ClearMask::COLOR);
            gl.viewport(0, 0, surface_width, surface_height);

            gl.bind_vertex_array(self.dummy_vao);
            gl.use_program(self.program);
            gl.draw_arrays(gl::Primitive::TRIANGLES, gl::Start(0), gl::Count(3));
        }
    }
}

const VS: &str = "#version 430
void main() {
    const vec2 vertices[3] = {
        vec2(-0.5,-0.5), // bottom left
        vec2( 0.0, 0.5), // top center
        vec2( 0.5,-0.5), // bottom right
    };
    gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
}";

const FS: &str = "#version 430
out vec4 frag_color;

void main() {
   frag_color = vec4(0.2, 0.0, 0.3, 1.0);
}";
