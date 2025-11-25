
#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    prelude::*,
};

mod sun_file;
mod planet_file;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            WireframePlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                sun_file::animate_suns,
                planet_file::animate_planets,
                #[cfg(not(target_arch = "wasm32"))]
                toggle_wireframe,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let solar_system_center: Entity= commands.spawn(
        (Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ), Visibility::Inherited)
    ).id();
    
    sun_file::make_sun (
        &mut commands,
        &asset_server,
        &mut meshes,
        //&mut images,
        &mut materials,
        solar_system_center,
    );

    planet_file::make_planets (
        &mut commands,
        //&asset_server,
        &mut meshes,
        &mut images,
        &mut materials,
        solar_system_center,
    );

    /* Disabling Ground Plane
    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
    */

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 50., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}