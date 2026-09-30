use dvox::prelude::*;

fn setup(_: &mut Context) {
    println!("Setup");
}

fn draw(_: &mut (), _: &mut Context) {
    println!("Draw");
}

fn main() {
    App::new(setup, draw).run();
}
