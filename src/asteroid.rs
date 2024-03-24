use bevy::prelude::*;

use crate::constants::image_handles::{
    SMALL_ASTEROID_HANDLE_IMAGE,
    MEDIUM_ASTEROID_HANDLE_IMAGE,
    BIG_ASTEROID_HANDLE_IMAGE
};

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
}

impl AsteroidType {
    fn value(&self) -> &str {
        match self {
            AsteroidType::SMALL => SMALL_ASTEROID_HANDLE_IMAGE,
            AsteroidType::MEDIUM => MEDIUM_ASTEROID_HANDLE_IMAGE,
            AsteroidType::BIG => BIG_ASTEROID_HANDLE_IMAGE
        }
    }
}

pub fn spawn_asteroids_on_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load(BIG_ASTEROID_HANDLE_IMAGE),
            ..default()
        },
    );
}
