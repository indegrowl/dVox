use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    mesh::MeshTag,
    prelude::*,
};
use dvox::{DVoxWorld, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FreeCameraPlugin)
        .add_plugins(DVoxPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    dvox_world: Res<DVoxWorld>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));
    let cube = meshes.add(Cuboid::new(1., 1., 1.));
    let material = materials.add(StandardMaterial::default());
    let scale = Vec3::ONE
        / Vec3::new(
            dvox_world.dimentions.x as f32,
            dvox_world.dimentions.y as f32,
            dvox_world.dimentions.z as f32,
        );
    for i in 0u32..dvox_world.voxels.len() as u32 {
        let x = i % dvox_world.dimentions.x;
        let y = (i / dvox_world.dimentions.x) % dvox_world.dimentions.y;
        let z = (i / (dvox_world.dimentions.x * dvox_world.dimentions.y)) % dvox_world.dimentions.z;
        let translation = Vec3::new(x as f32, y as f32, z as f32) * scale;
        let translation = translation - scale / 2.0;
        if dvox_world.voxels[i as usize].color > 0 {
            commands.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(material.clone()),
                MeshTag(i as u32),
                Transform::from_translation(translation).with_scale(scale),
            ));
        }
    }
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
