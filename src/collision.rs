use bevy::prelude::*;
use bevy_rapier2d::pipeline::*;

use crate::asteroid::Asteroid;
use crate::projectile::Projectile;

pub fn detect_asteroid_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<Entity, With<Projectile>>,
    asteroid_query: Query<Entity, With<Asteroid>>
) {
    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                for asteroid_entity in &asteroid_query {
                    for projectile_entity in &projectile_query {
                        if 
                            (asteroid_entity == first_entity || asteroid_entity == second_entity) &&
                            (projectile_entity == first_entity || projectile_entity == second_entity)
                        {
                            commands.entity(first_entity).despawn();
                            commands.entity(second_entity).despawn();
                        }
                    }
                }
            },
            CollisionEvent::Stopped(_, _, _) => ()
        }
    }
}
