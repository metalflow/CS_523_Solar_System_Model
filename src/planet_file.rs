use std::f32::consts::PI;
use rand::{Rng, rng};

///bevy libraries
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

/// new marker component for Planet Objects
#[derive(Component)]
pub struct Planet {
    //should be a value from +2PI to -2PI
    rotation_speed:f32,
    //should be a value from +2PI to -2PI
    orbital_speed:f32,
}

/// orbital speed of zero should be disallowed, thus I need to make this little function to avoid it
fn compute_orbital_speed(min:i8,max:i8) -> f32 {
    let mut rng = rng();
    let mut output:f32 = 0.0;
    while output == 0.0 {
        output = PI * rng.random_range((min as f32)..=(max as f32))
    }
    return output;
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

pub fn make_planets(commands: &mut Commands,
    //asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    solar_system_center: Entity,
    radius: u32,
) -> u32 {
    let mut new_radius:f32=radius as f32;
    let mut scaling_factor: f32=1.0;
    let mut rng = rng();
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let num_planets: u8 =rng.random_range(2..=9);
    let mut planets= Vec::new();

    for current_planet in 1..=num_planets {
        if current_planet % 2 == 0 {
            planets.push(meshes.add(Sphere::default().mesh().ico(5).unwrap()));
        } else {
            planets.push(meshes.add(Sphere::default().mesh().uv(32, 18)));
        }
    }

    for (i, shape) in planets.into_iter().enumerate() {
        scaling_factor+=1.0;
        //inital radius growth of scaling factor to place center of new planet
        new_radius+=scaling_factor;
        commands.spawn((
            ChildOf(solar_system_center),
            Mesh3d(shape),
            MeshMaterial3d(debug_material.clone()),
            Transform::from_xyz(
                new_radius,
                0.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(-PI * (i as f32)/ 4. ))
            .with_scale(
                Vec3::new(
                    1.0 + scaling_factor,
                    1.0 + scaling_factor,
                    1.0 + scaling_factor,
                )
            ),
            Planet {
                orbital_speed: compute_orbital_speed(-1,1),
                rotation_speed: PI * rng.random_range(-2.0..2.0),
            },
        ));
        //final radius growth of scaling factor to account for the rest of the planet
        new_radius+=scaling_factor;
    }

    return new_radius as u32;
}

pub fn animate_planets(mut query: Query<(&mut Transform, &Planet)>, time: Res<Time>) {
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