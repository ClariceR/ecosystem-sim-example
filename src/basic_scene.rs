use crate::prelude::*;

pub fn basic_scene_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: &AssetServer,
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


}