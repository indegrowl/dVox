use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct Voxel {
    pub color: u32,
}

#[derive(Resource)]
pub struct DVoxWorld {
    pub dimentions: UVec3,
    pub voxels: Vec<Voxel>,
}
impl Default for DVoxWorld {
    fn default() -> Self {
        Self {
            dimentions: UVec3::new(16, 16, 16),
            voxels: vec![Voxel { color: 0xF3F3F3FF }; 16 * 16 * 16],
        }
    }
}

pub struct DVoxPlugin;
impl Plugin for DVoxPlugin {
    fn build(&self, app: &mut App) {
        let mut world = DVoxWorld::default();
        world.voxels.iter_mut().enumerate().for_each(|(i, v)| {
            if i % 2 == 0 {
                v.color = 0;
            }
        });
        app.insert_resource(world);
    }
}

pub mod prelude {
    pub use crate::DVoxPlugin;
}
