use bevy::{
    prelude::*,
    math::vec3
};

use crate::constants::image_handles::PROJECTILE_HANDLE_IMAGE;
use crate::constants::projectile_movement_values::PROJECTILE_MOVEMENT_SPEED;

#[derive(Component)]
pub struct Projectile {
    pub translation: Vec3,
    pub direction: Vec3,
    pub movement_speed: f32
}

impl Default for Projectile {
    fn default() -> Self {
        return Self {
            translation: vec3(0., 0., 0.),
            direction: vec3(0., 0., 0.),
            movement_speed: PROJECTILE_MOVEMENT_SPEED
        };
    }
}

impl Projectile {
    pub fn spawn_projectile(
        self,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let projectile_handle_image: Handle<Image> = asset_server.load(PROJECTILE_HANDLE_IMAGE);

        commands.spawn((
            SpriteBundle {
                texture: projectile_handle_image,
                transform: Transform {
                    translation: self.translation,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            self
        ));
    }
}

pub fn set_projectile_movement(
    time: Res<Time>,
    mut projectile_query: Query<(&mut Projectile, &mut Transform, &mut Visibility)>
) {
    let mut movement_distance: f32;

    for (mut projectile_entity, mut projectile_transform, mut projectile_visibility) in &mut projectile_query {
        movement_distance = projectile_entity.movement_speed * time.delta_seconds();
        projectile_transform.translation += movement_distance;
        *projectile_visibility = Visibility::Visible;
    }
}
