use std::path::{Path};
use std::f32::consts::PI;
use rand::{Rng, rng};

///bevy libraries
use bevy::{
    prelude::*,
    //asset::LoadedFolder,
    asset::AssetServer,
};



/// new marker component for Sun objects
#[derive(Component)]
pub struct Sun {
    //should be a value from +2PI to -2PI
    rotation_speed:f32,
}

pub fn make_sun(commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    //images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    solar_system_center: Entity,
    //loaded_folders: &mut Res<Assets<LoadedFolder>>,
) -> u32 {
    let mut solar_radius: u32 = 0;
    let mut rng = rng();
    let texture_path = Path::new("textures/sun/");
    
    /*pre-load assets into asset handler */
    let sun_texture:Vec<Handle<Image>> = vec![
        asset_server.load(texture_path.join(Path::new("Solarsystemscope_texture_2k_sun.jpg"))),
        asset_server.load(texture_path.join(Path::new("8k_sun.jpg"))),
    ];
    //commands.insert_resource(MyTextureHandle(sun_texture));

    let sun_material = materials.add(StandardMaterial {
        //base_color: Color::srgb(1.0,1.0,1.0),
        emissive_texture: Some(sun_texture[0].clone()),
        //base_color_texture: Some(sun_texture),
        emissive: LinearRgba {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
            alpha: 1000.0,
        },
        alpha_mode:AlphaMode::Opaque,
        ..default()
    });

    let sun_mesh = meshes.add(Sphere::default().mesh().uv(32, 18));

    /*Spawn in a Sun at 0,0,0 
    with pi/2 rotation so that poles are in the Z axis
    Scale it to a large size*/
    let id = commands.spawn((
        ChildOf(solar_system_center),
        Mesh3d(sun_mesh),
        MeshMaterial3d(sun_material),
        Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        )
        .with_rotation(
            Quat::from_rotation_x(PI/2.0)
        )
        .with_scale(
            //Transform::from_scale(
                Vec3::new(
                    10.0,
                    10.0,
                    10.0,
                )
            //)
        ),
        Sun {
            rotation_speed: PI * rng.random_range(-2.0..2.0),
        },
    )).id();

    //just a fixed set here for now until I figure out how to dynamically assign this
    solar_radius+=10;

    //moved the point light inside the Sun to mimic light emission
    commands.spawn((
        ChildOf(id),
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 1000.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    solar_radius
}

pub fn animate_suns(mut query: Query<(&mut Transform, &Sun)>, time: Res<Time>) {
    for (mut transform, this_sun) in &mut query {
        transform.rotate_y(time.delta_secs() * this_sun.rotation_speed);
    }
}
