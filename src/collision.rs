use bevy::prelude::*;
use bevy_rapier2d::pipeline::*;

use bevy::audio::{
    PlaybackMode::Despawn,
    Volume
};

use crate::projectile::Projectile;
use crate::ui::ScoreboardScore;
use crate::GameState;

use crate::hero_ship::{
    HeroShip,
    HeroShipDestroyedSound
};

use crate::asteroid::{
    Asteroid,
    AsteroidType,
    spawn_asteroids_after_collision
};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct AsteroidDestroyedSound(pub Handle<AudioSource>);

pub fn detect_asteroid_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut scoreboard_score: ResMut<ScoreboardScore>,
    hero_ship_destroyed_sound: Res<HeroShipDestroyedSound>,
    asteroid_destroyed_sound: Res<AsteroidDestroyedSound>,
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
                        next_state.set(GameState::EndGame);

                        commands.spawn(AudioBundle {
                            source: hero_ship_destroyed_sound.clone(),
                            settings: PlaybackSettings {
                                mode: Despawn,
                                volume: Volume::new(2.5),
                                ..default()
                            }
                        });
                    }

                    for (projectile_entity, _) in &projectile_query {
                        if 
                            (asteroid_entity == first_entity || asteroid_entity == second_entity) &&
                            (projectile_entity == first_entity || projectile_entity == second_entity)
                        {
                            commands.entity(projectile_entity).despawn();
                            commands.entity(asteroid_entity).despawn();

                            commands.spawn(AudioBundle {
                                source: asteroid_destroyed_sound.clone(),
                                settings: PlaybackSettings::DESPAWN
                            });

                            spawn_asteroids_after_collision(
                                commands.reborrow(),
                                &asset_server,
                                asteroid_component,
                                asteroid_transform
                            );

                            increase_score_based_on_asteroid_type(
                                asteroid_component,
                                &mut scoreboard_score
                            );
                        }
                    }
                }
            },
            CollisionEvent::Stopped(_, _, _) => ()
        }
    }
}

fn increase_score_based_on_asteroid_type(
    asteroid_component: &Asteroid,
    scoreboard_score: &mut ResMut<ScoreboardScore>
) {
    match asteroid_component.asteroid_type {
        AsteroidType::Small => { scoreboard_score.score += 50; },
        AsteroidType::Medium => { scoreboard_score.score += 30; },
        AsteroidType::Big => { scoreboard_score.score += 10; }
    }
}
