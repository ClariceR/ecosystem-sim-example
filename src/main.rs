//use bevy::{camera::ScalingMode, prelude::*};

mod basic_scene;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::camera::ScalingMode;
    pub use crate::basic_scene::*;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

//set up a simple 3d scene
fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    basic_scene_setup(
        &mut commands,
        meshes,
        materials,
        &asset_server,
    );


    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 4.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    //Orthographic Camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Projection::from(OrthographicProjection {
    //         // 6 world units per pixel of window height.
    //         scaling_mode: ScalingMode::FixedVertical {
    //             viewport_height: 4.5,
    //         },
    //         ..OrthographicProjection::default_3d()
    //     }),
    //     Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));

}
