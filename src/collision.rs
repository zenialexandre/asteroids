use bevy::prelude::*;
use bevy_rapier2d::pipeline::*;

use crate::{
    hero_ship::HeroShip,
    projectile::Projectile,
    asteroid::{
        Asteroid,
        spawn_asteroids_after_collision
    }
};

pub fn detect_asteroid_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    hero_ship_query: Query<(Entity, &HeroShip)>,
    projectile_query: Query<(Entity, &Projectile)>,
    asteroid_query: Query<(Entity, &Asteroid, &Transform)>
) {
    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                let (hero_ship_entity, _) = hero_ship_query.single();

                for (asteroid_entity, asteroid_component, asteroid_transform) in &asteroid_query {
                    if
                        (asteroid_entity == first_entity || asteroid_entity == second_entity) &&
                        (hero_ship_entity == first_entity || hero_ship_entity == second_entity)
                    {
                        commands.entity(hero_ship_entity).despawn();
                    }

                    for (projectile_entity, _) in &projectile_query {
                        if 
                            (asteroid_entity == first_entity || asteroid_entity == second_entity) &&
                            (projectile_entity == first_entity || projectile_entity == second_entity)
                        {
                            commands.entity(projectile_entity).despawn();
                            commands.entity(asteroid_entity).despawn();
                            spawn_asteroids_after_collision(
                                commands.reborrow(),
                                &asset_server,
                                asteroid_component,
                                asteroid_transform
                            );
                        }
                    }
                }
            },
            CollisionEvent::Stopped(_, _, _) => ()
        }
    }
}
