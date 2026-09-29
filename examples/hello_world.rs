use bevy::prelude::*;
use dvox::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DVoxPlugin)
        .run();
}
