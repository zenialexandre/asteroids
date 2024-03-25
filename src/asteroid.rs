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
    BOTTOM_BORDER_POSITION
};

use crate::constants::asteroid::BIG_ASTEROID_ON_SCREEN_MAX_VALUE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
enum AsteroidType {
    SMALL,
    MEDIUM,
    #[default]
    BIG
}

#[derive(Component)]
struct Asteroid {
    asteroid_type: AsteroidType,
    asteroid_movement_speed: f32,
    asteroid_movement_direction: Vec3,
    asteroid_rotation_speed: f32,
    asteroid_rotation_direction: Vec3
}

impl AsteroidType {
    fn get_asteroid_movement_speed(&self) -> f32 {
        match self {
            AsteroidType::SMALL => 320.,
            AsteroidType::MEDIUM => 160.,
            AsteroidType::BIG => 80.
        }
    }

    fn get_asteroid_movement_direction(&self) -> Vec3 {
        let mut thread_rng: ThreadRng = rand::thread_rng();
        return vec3(thread_rng.gen(), thread_rng.gen(), 0.);
    }

    fn get_asteroid_rotation_speed(&self) -> f32 {
        match self {
            AsteroidType::SMALL => 150.,
            AsteroidType::MEDIUM => 100.,
            AsteroidType::BIG => 50.
        }
    }

    fn get_asteroid_rotation_direction(&self) -> Vec3 {
        return vec3(1., 0., 0.);
    }
}

pub fn spawn_initial_asteroids(
    mut commands: Commands,
    asset_server: &Res<AssetServer>
) {
    let default_asteroid_type: AsteroidType = AsteroidType::BIG;
    let mut random_asteroid_position_x: f32;
    let mut random_asteroid_position_y: f32;

    for _ in 1..=BIG_ASTEROID_ON_SCREEN_MAX_VALUE {
        random_asteroid_position_x = get_randomic_asteroid_position(LEFT_BORDER_POSITION, RIGHT_BORDER_POSITION);
        random_asteroid_position_y = get_randomic_asteroid_position(BOTTOM_BORDER_POSITION, TOP_BORDER_POSITION);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(BIG_ASTEROID_HANDLE_IMAGE),
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
            Asteroid {
                asteroid_type: default_asteroid_type,
                asteroid_movement_speed: default_asteroid_type.get_asteroid_movement_speed(),
                asteroid_movement_direction: default_asteroid_type.get_asteroid_movement_direction(),
                asteroid_rotation_speed: default_asteroid_type.get_asteroid_rotation_speed(),
                asteroid_rotation_direction: default_asteroid_type.get_asteroid_rotation_direction(),
            },
        ));
    }
}

fn get_randomic_asteroid_position(
    position_initial_range: f32,
    position_final_range: f32
) -> f32 {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let mut is_randomic_position_ok: bool = false;
    let mut randomic_asteroid_position: f32 = 0.;

    while !is_randomic_position_ok {
        randomic_asteroid_position = thread_rng.gen_range(position_initial_range..=position_final_range);

        if 
            randomic_asteroid_position >= 25. ||
            randomic_asteroid_position <= -25.
        {
            is_randomic_position_ok = true;
        }
    }
    return randomic_asteroid_position;
}
