pub struct Context {}
impl Context {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct App<S, F: FnMut(&mut S, &mut Context)> {
    context: Context,
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
            draw,
            context,
        }
    }

    pub fn run(mut self) {
        loop {
            (self.draw)(&mut self.state, &mut self.context)
        }
    }
}

pub mod prelude {
    pub use crate::{Context, App};
}
