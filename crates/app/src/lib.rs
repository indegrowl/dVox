use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::{Window, WindowAttributes},
};

pub struct Context {
    window: Option<Window>,
}
impl Context {
    pub fn new() -> Self {
        Self { window: None }
    }
}

impl<S: 'static, F: FnMut(&mut S, &mut Context)> ApplicationHandler for App<S, F> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.context.window = Some(
            event_loop
                .create_window(WindowAttributes::default())
                .expect("Unable to create window!"),
        );
    }

    fn window_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.should_close = true;
            }
            WindowEvent::RedrawRequested => (self.draw)(&mut self.state, &mut self.context),
            _ => (),
        }
    }
}

pub struct App<S, F: FnMut(&mut S, &mut Context)> {
    context: Context,
    should_close: bool,
    state: S,
    draw: F,
}

impl<S: 'static, F: FnMut(&mut S, &mut Context)> App<S, F> {
    pub fn new<FS>(setup: FS, draw: F) -> Self
    where
        FS: FnOnce(&mut Context) -> S,
    {
        let mut context = Context::new();
        Self {
            state: setup(&mut context),
            should_close: false,
            draw,
            context,
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().expect("Unable to build event loop!");
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut self).expect("An unexpected error occured!");
    }
}

pub mod prelude {
    pub use crate::{App, Context};
}
