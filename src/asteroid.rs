use bevy::{
    prelude::*,
    math::vec3
};

use rand::{
    rngs::ThreadRng,
    Rng
};

use crate::constants::image_handles::{
    SMALL_ASTEROID_HANDLE_IMAGE,
    MEDIUM_ASTEROID_HANDLE_IMAGE,
    BIG_ASTEROID_HANDLE_IMAGE
};

use crate::constants::borders::{
    RIGHT_BORDER_POSITION,
    LEFT_BORDER_POSITION,
    TOP_BORDER_POSITION,
    BOTTOM_BORDER_POSITION,
    RIGHT_BORDER_OFFSCREEN_POSITION,
    LEFT_BORDER_OFFSCREEN_POSITION,
    TOP_BORDER_OFFSCREEN_POSITION,
    BOTTOM_BORDER_OFFSCREEN_POSITION
};

use crate::constants::asteroid::BIG_ASTEROID_ON_SCREEN_MAX_VALUE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
enum AsteroidType {
    SMALL,
    MEDIUM,
    BIG
}

#[derive(Component, Clone)]
pub struct Asteroid {
    asteroid_type: AsteroidType,
    handle_image: Option<Handle<Image>>,
    movement_speed: Option<f32>,
    movement_direction: Vec3,
    rotation_speed: Option<f32>,
    rotation_direction: f32
}

impl Asteroid {
    fn initialize_asteroid_based_on_type(
        asset_server: &Res<AssetServer>,
        asteroid_type_external: AsteroidType
    ) -> Self {
        let mut thread_rng: ThreadRng = rand::thread_rng();
        let mut asteroid_struct: Asteroid = Asteroid {
            asteroid_type: asteroid_type_external,
            handle_image: None,
            movement_speed: None,
            movement_direction: vec3(thread_rng.gen_range(-1.1..1.1), thread_rng.gen_range(-1.1..1.1), 0.) * Vec3::Y,
            rotation_speed: None,
            rotation_direction: thread_rng.gen_range(-1.0..1.0)
        };

        match asteroid_struct.asteroid_type {
            AsteroidType::SMALL => {
                asteroid_struct.handle_image = Some(asset_server.load(SMALL_ASTEROID_HANDLE_IMAGE));
                asteroid_struct.movement_speed = Some(320.);
                asteroid_struct.rotation_speed = Some(f32::to_radians(150.));
            },
            AsteroidType::MEDIUM => {
                asteroid_struct.handle_image = Some(asset_server.load(MEDIUM_ASTEROID_HANDLE_IMAGE));
                asteroid_struct.movement_speed = Some(160.);
                asteroid_struct.rotation_speed = Some(f32::to_radians(100.));
            },
            AsteroidType::BIG => {
                asteroid_struct.handle_image = Some(asset_server.load(BIG_ASTEROID_HANDLE_IMAGE));
                asteroid_struct.movement_speed = Some(80.);
                asteroid_struct.rotation_speed = Some(f32::to_radians(50.));
            }
        }
        return asteroid_struct;
    }
}

pub fn spawn_initial_asteroids(
    mut commands: Commands,
    asset_server: &Res<AssetServer>
) {
    let mut random_asteroid_position_x: f32;
    let mut random_asteroid_position_y: f32;

    for _ in 1..=BIG_ASTEROID_ON_SCREEN_MAX_VALUE {
        let asteroid_struct: Asteroid = Asteroid::initialize_asteroid_based_on_type(asset_server, AsteroidType::BIG);
        random_asteroid_position_x = get_randomic_asteroid_onscreen_position(LEFT_BORDER_POSITION, RIGHT_BORDER_POSITION);
        random_asteroid_position_y = get_randomic_asteroid_onscreen_position(BOTTOM_BORDER_POSITION, TOP_BORDER_POSITION);

        commands.spawn((
            SpriteBundle {
                texture: asteroid_struct.clone().handle_image.unwrap(),
                transform: Transform {
                    translation: vec3(
                        random_asteroid_position_x,
                        random_asteroid_position_y,
                        0.
                    ),
                    ..default()
                },
                ..default()
            },
            asteroid_struct
        ));
    }
}

pub fn set_asteroid_movement_and_rotation(
    time: Res<Time>,
    mut asteroid_query: Query<(&Asteroid, &mut Transform)>
) {
    let mut movement_distance: f32;

    for (asteroid_entity, mut asteroid_transform) in &mut asteroid_query {
        asteroid_transform.rotate_z(
            asteroid_entity.rotation_direction * asteroid_entity.rotation_speed.unwrap() * time.delta_seconds()
        );
        movement_distance = asteroid_entity.movement_speed.unwrap() * time.delta_seconds();
        asteroid_transform.translation += asteroid_entity.movement_direction * movement_distance;
    }
}

pub fn set_asteroid_position_after_border_outbounds(
    mut asteroid_query: Query<(&Asteroid, &mut Transform)>
) {
    for (_, mut asteroid_transform) in &mut asteroid_query {
        let asteroid_position_x: f32 = asteroid_transform.translation.x;
        let asteroid_position_y: f32 = asteroid_transform.translation.y;

        if
            asteroid_position_x >= RIGHT_BORDER_OFFSCREEN_POSITION &&
            asteroid_position_y <= LEFT_BORDER_OFFSCREEN_POSITION &&
            asteroid_position_x >= TOP_BORDER_OFFSCREEN_POSITION &&
            asteroid_position_y <= BOTTOM_BORDER_OFFSCREEN_POSITION
        {
            println!("saiu da tela");
            //asteroid_transform.translation.x = get_randomic_asteroid_position(-420..420.);
            //asteroid_transform.translation.y = get_randomic_asteroid_position(-280., 280.);
        }
    }
}

fn get_randomic_asteroid_onscreen_position(
    position_initial_range: f32,
    position_final_range: f32
) -> f32 {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let mut is_randomic_position_ok: bool = false;
    let mut randomic_asteroid_position: f32 = 0.;

    while !is_randomic_position_ok {
        randomic_asteroid_position = thread_rng.gen_range(position_initial_range..=position_final_range + 1.);

        if
            randomic_asteroid_position >= 25. ||
            randomic_asteroid_position <= -25.
        {
            is_randomic_position_ok = true;
        }
    }
    return randomic_asteroid_position;
}
