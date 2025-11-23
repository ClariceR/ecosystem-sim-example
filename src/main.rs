use bevy::{camera::ScalingMode, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

//set up a simple 3d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Chicken Model
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/animated/chicken2.glb")),
    ));

    // Rock
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/nature/Rock_3_H_Color1.glb"),
        )),
        Transform::from_xyz(1.5, 0.2, 1.5)
            .with_scale(Vec3::new(0.5, 0.5, 0.5))
    ));

    // Grass
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/nature/grass1and2.glb"),
        )),
        Transform::from_xyz(0.8, 0.0, 1.6)
            //.with_scale(Vec3::new(0.5, 0.5, 0.5))
    ));

    // Grass 2
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/nature/grass4.glb"),
        )),
        Transform::from_xyz(2.0, 0.0, 1.9)
            .with_scale(Vec3::new(0.7, 0.7, 0.7))
    ));

    // Bush
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/nature/bush1default.glb"),
        )),
        Transform::from_xyz(-1.2, 0.0, -1.6)
            //.with_scale(Vec3::new(0.6, 0.6, 0.6))
    ));

    // Tree
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/nature/treeBdefault.glb"),
        )),
        Transform::from_xyz(1.2, 0.0, -1.6)
            //.with_scale(Vec3::new(0.6, 0.6, 0.6))
    ));


    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(2.0, 4.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));

    //Orthographic Camera
        // camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 4.5,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));


}
