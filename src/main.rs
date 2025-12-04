use std::path::{Path};

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy::{
    prelude::*,
    asset::LoadedFolder,
    //transform::TransformSystems,
};

mod sun_file;
mod planet_file;

//adding in AppStates to allow for assets to preload in a successul manner
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    PreLoad,
    InGame,
}

//making a resource so that I don't have to keep lading the folder to get its handle
#[derive(Resource)]
struct AssetHandles {
    texture_folder_handle: Handle<LoadedFolder>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            WireframePlugin::default(),
            PanOrbitCameraPlugin,
        ))
        .insert_state(AppState::PreLoad)
        .add_systems(PreStartup, preload_assets)
        .add_systems(Update, check_assets_loaded.run_if(in_state(AppState::PreLoad)))
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(
            Update,
            (
                sun_file::animate_suns,
                planet_file::animate_planets,
                #[cfg(not(target_arch = "wasm32"))]
                toggle_wireframe,
                //limit_pan_system, //disabled due to improper behavior
            ).run_if(in_state(AppState::InGame)),
        )
        //.add_systems(Update, limit_pan_system.after(TransformSystems::Propagate)) //disabled due to improper behavior
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut loaded_folders: Res<Assets<LoadedFolder>>,
) {
    let solar_system_center: Entity= commands.spawn(
        (Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ), Visibility::Inherited)
    ).id();
    
    let solar_radius = sun_file::make_sun (
        &mut commands,
        &asset_server,
        &mut meshes,
        //&mut images,
        &mut materials,
        solar_system_center,
        //&mut loaded_folders,
    );

    let system_radius = planet_file::make_planets (
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut images,
        &mut materials,
        solar_system_center,
        solar_radius,
    );

    /* Disabling Ground Plane
    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
    */

    /* replacing old camera with a movable camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 50., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
    */
    let _camera = commands.spawn((
        Transform::from_xyz(10.0, 50., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        PanOrbitCamera{
            zoom_lower_limit: solar_radius as f32+10.0,
            zoom_upper_limit: Some((system_radius as f32)+10.0),
            ..default()
        }
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

/*This function doesn't appear to actually update the transform for the camera
 it will pop the camera to an appropriate value, but when you resume moving,
 it pops back to the original bad value and moves relative to that.  There
 must be some stored value in the move/pan function that is still holding
 the bad value.  GitHub issue added.
 https://github.com/Plonq/bevy_panorbit_camera/issues/133
fn limit_pan_system(mut query: Query<&mut Transform, With <PanOrbitCamera>>) {
    for mut camera in query.iter_mut() {     
        const MIN: f32 = -100.0;
        const MAX: f32 = 100.0;
        camera.translation.x=camera.translation.x.clamp(MIN, MAX);
        camera.translation.y=camera.translation.y.clamp(MIN, MAX);
        camera.translation.z=camera.translation.z.clamp(MIN, MAX);
    }
}
*/

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn preload_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_path = Path::new("textures/");
    let texture_folder_handle = asset_server.load_folder(texture_path);
    //storing our handle in AssetHandles Resource so I don't have to keep recreting it
    commands.insert_resource(AssetHandles { texture_folder_handle });
}

fn check_assets_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    asset_handles: Res<AssetHandles>,
) {
    //let texture_path = Path::new("textures/");
    //let texture_folder_handle = asset_server.load_folder(texture_path);
    if asset_server.is_loaded_with_dependencies(asset_handles.texture_folder_handle.id()) {
        info!("All assets loaded! Transitioning to InGame state.");
        next_state.set(AppState::InGame);
    } else {
         println!("folder load state {:?}", asset_server.get_recursive_dependency_load_state(asset_handles.texture_folder_handle.id()));
    }
}