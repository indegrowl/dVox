use bevy::prelude::*;

pub struct DVoxPlugin;
impl Plugin for DVoxPlugin {
    fn build(&self, _app: &mut App) {}
}

pub mod prelude {
    pub use crate::DVoxPlugin;
}
