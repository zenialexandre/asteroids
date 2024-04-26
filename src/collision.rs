use bevy::prelude::*;
use bevy_rapier2d::pipeline::*;

use crate::hero_ship::HeroShip;
use crate::projectile::Projectile;
use crate::asteroid::Asteroid;

pub fn detect_asteroid_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    hero_ship_query: Query<Entity, With<HeroShip>>,
    projectile_query: Query<Entity, With<Projectile>>,
    asteroid_query: Query<Entity, With<Asteroid>>
) {
    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                let hero_ship_entity: Entity = hero_ship_query.single();

                for asteroid_entity in &asteroid_query {
                    if
                        (asteroid_entity == first_entity || asteroid_entity == second_entity) &&
                        (hero_ship_entity == first_entity || hero_ship_entity == second_entity)
                    {
                        commands.entity(hero_ship_entity).despawn();
                    }

                    for projectile_entity in &projectile_query {
                        if 
                            (asteroid_entity == first_entity || asteroid_entity == second_entity) &&
                            (projectile_entity == first_entity || projectile_entity == second_entity)
                        {
                            commands.entity(projectile_entity).despawn();
                            commands.entity(asteroid_entity).despawn();
                        }
                    }
                }
            },
            CollisionEvent::Stopped(_, _, _) => ()
        }
    }
}
