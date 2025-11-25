use std::f32::consts::PI;
use std::path::{Path};
use rand::{Rng, rng};

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    asset::RenderAssetUsages,
    //color::palettes::basic::SILVER,   //disabled as its only used by extrusions at the moment
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

/// new marker component for Sun objects
#[derive(Component)]
struct Sun {
    //should be a value from +2PI to -2PI
    rotation_speed:i8,
}

/// new marker component for Planet Objects
#[derive(Component)]
struct Planet {
    //should be a value from +2PI to -2PI
    rotation_speed:i8,
    //should be a value from +2PI to -2PI
    orbital_speed:i8,
}

/// orbital speed of zero should be disallowed, thus I need to make this little function to avoid it
fn compute_orbital_speed(min:i8,max:i8) -> i8 {
    let mut rng = rng();
    let mut output = 0;
    while (output == 0) {
        output = (PI as i8) * rng.random_range(min..=max)
    }
    return output;
}

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
                animate_suns,
                animate_planets,
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
    //get a local RNG for randomizing
    let mut rng = rng();

    let solar_system_center: Entity= commands.spawn(
        (Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ), Visibility::Inherited)
    ).id();
    let texture_path = Path::new("textures/");
    /*pre-load assets into asset handler */
    let sun_texture = asset_server.load(texture_path.join(Path::new("Solarsystemscope_texture_2k_sun.jpg")));
    //commands.insert_resource(MyTextureHandle(sun_texture));

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let sun_material = materials.add(StandardMaterial {
        //base_color: Color::srgb(1.0,1.0,1.0),
        emissive_texture: Some(sun_texture),
        //base_color_texture: Some(sun_texture),
        emissive: LinearRgba {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
            alpha: 1000.0,
        },
        alpha_mode:AlphaMode::Add,
        ..default()
    });

    let planets = [
        //meshes.add(Cuboid::default()),
        //meshes.add(Tetrahedron::default()),
        //meshes.add(Capsule3d::default()),
        //meshes.add(Torus::default()),
        //meshes.add(Cylinder::default()),
        //meshes.add(Cone::default()),
        //meshes.add(ConicalFrustum::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
        //meshes.add(Segment3d::default()),
        //meshes.add(Polyline3d::new(vec![
        //    Vec3::new(-0.5, 0.0, 0.0),
        //    Vec3::new(0.5, 0.0, 0.0),
        //    Vec3::new(0.0, 0.5, 0.0),
        //])),
    ];

    let sun_mesh = meshes.add(Sphere::default().mesh().uv(32, 18));

    /* Disabling extrusions
    let extrusions = [
        meshes.add(Extrusion::new(Rectangle::default(), 1.)),
        meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
        meshes.add(Extrusion::new(Annulus::default(), 1.)),
        meshes.add(Extrusion::new(Circle::default(), 1.)),
        meshes.add(Extrusion::new(Ellipse::default(), 1.)),
        meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
        meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
    ];
    */

    /*Spawn in a Sun at 0,0,0 
        with pi/2 rotation so that poles are in the Z axis
        Scale it to a large size*/
    let sun = commands.spawn((
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
            rotation_speed: (PI as i8) * rng.random_range(-2..2),
        },
    )).id();
    //moved the point light inside the Sun to mimic light emission
    commands.spawn((
        ChildOf(sun),
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 1000.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    for (i, shape) in planets.into_iter().enumerate() {
        commands.spawn((
            ChildOf(solar_system_center),
            Mesh3d(shape),
            MeshMaterial3d(debug_material.clone()),
            Transform::from_xyz(
                6.0+(i as f32*2.0),
                0.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(-PI * (i as f32)/ 4. ))
            .with_scale(
                Vec3::new(
                    1.0 + (i as f32),
                    1.0 + (i as f32),
                    1.0 + (i as f32),
                )
            ),
            Planet {
                orbital_speed: compute_orbital_speed(-1,1),
                rotation_speed: (PI as i8) * rng.random_range(-2..2),
            },
        ));
    }

    /* Disabling extrusions
    let num_extrusions = extrusions.len();

    for (i, shape) in extrusions.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(debug_material.clone()),
            Transform::from_xyz(
                -EXTRUSION_X_EXTENT / 2.
                    + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
                2.0,
                -Z_EXTENT / 2.,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            Shape,
        ));
    }
    */

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

fn animate_suns(mut query: Query<(&mut Transform, &Sun)>, time: Res<Time>) {
    for (mut transform, this_sun) in &mut query {
        transform.rotate_y(time.delta_secs() * (this_sun.rotation_speed as f32));
    }
}

fn animate_planets(mut query: Query<(&mut Transform, &Planet)>, time: Res<Time>) {
    for (mut transform, this_planet) in &mut query {
        transform.rotate_y(time.delta_secs() * (this_planet.rotation_speed as f32));
        transform.rotate_around(
            Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ), 
            Quat::from_rotation_y(time.delta_secs() * (this_planet.orbital_speed as f32)));
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
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