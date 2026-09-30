use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

pub struct Context {
    window: Window,
}
impl Context {
    pub fn new(window: Window) -> Self {
        Self { window }
    }
}

impl<S: 'static, FS: FnMut(&mut Context) -> S, F: FnMut(&mut S, &mut Context)> ApplicationHandler
    for App<S, FS, F>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Unable to create window!");
        self.context = Some(Context::new(window));
        self.state = Some((self.setup)(self.context.as_mut().expect("setup was called without context")));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => (self.draw)(
                self.state.as_mut().expect("draw was called without state"),
                self.context
                    .as_mut()
                    .expect("draw was called without context"),
            ),
            _ => (),
        }
    }
}

pub struct App<S, FS: FnMut(&mut Context) -> S, F: FnMut(&mut S, &mut Context)> {
    context: Option<Context>,
    state: Option<S>,
    setup: FS,
    draw: F,
}

impl<S: 'static, FS: FnMut(&mut Context) -> S, F: FnMut(&mut S, &mut Context)> App<S, FS, F> {
    pub fn new(setup: FS, draw: F) -> Self
    where
        FS: FnOnce(&mut Context) -> S,
    {
        Self {
            state: None,
            context: None,
            setup,
            draw,
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().expect("Unable to build event loop!");
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop
            .run_app(&mut self)
            .expect("An unexpected error occured!");
    }
}

pub mod prelude {
    pub use crate::{App, Context};
}
