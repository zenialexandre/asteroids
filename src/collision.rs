use bevy::{
    prelude::*,
    math::bounding::{
        Aabb2d,
        BoundingCircle,
        BoundingVolume,
        IntersectsVolume
    }
};

use crate::asteroid::{self, Asteroid};
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Collider;


#[derive(Debug)]
pub enum CollisionSide {
    Right,
    Left,
    Top,
    Bottom
}

pub fn get_projectile_and_asteroid_collision(
    mut commands: Commands,
    projectile_query: Query<&Transform, With<Projectile>>,
    collider_query: Query<(Entity, &Transform, Option<&Asteroid>), With<Collider>>
) {
    let projectile_transform: &Transform = projectile_query.single();

    for (collider_entity, collider_transform, asteroid) in &collider_query {
        let projectile_asteroid_collision = projectile_collision(
            BoundingCircle::new(
                projectile_transform.translation.truncate(),
                1.
            ),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.
            )
        );

        if let Some(projectile_asteroid_collision) = projectile_asteroid_collision {
            if asteroid.is_some() {
                println!("entrou aq");
                commands.entity(collider_entity).despawn();
            }
        }
    }
}

fn projectile_collision(
    projectile: BoundingCircle,
    bounding_box: Aabb2d
) -> Option<CollisionSide> {
    if !projectile.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(projectile.center());
    let offset = projectile.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            println!("oi");
            CollisionSide::Left
        } else {
            CollisionSide::Right
        }
    } else if offset.y > 0. {
        CollisionSide::Top
    } else {
        CollisionSide::Bottom
    };

    return Some(side);
}
