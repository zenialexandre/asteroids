use bevy::{
    prelude::*,
    audio::{
        PlaybackMode::Despawn,
        Volume
    }
};

use bevy_rapier2d::prelude::*;

use crate::projectile::Projectile;
use crate::ui::ScoreboardScore;
use crate::PausingState;
use crate::GameState;

use crate::hero_ship::{
    HeroShip,
    HeroShipDestroyedSound,
    HeroShipRemainingLives,
    spawn_hero_ship_destroyed_spritesheet
};

use crate::asteroid::{
    Asteroid,
    AsteroidType,
    AsteroidDestroyedSound,
    spawn_asteroids_after_collision
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(
        &self,
        app: &mut App
    ) {
        app.add_systems(FixedUpdate, (
            detect_asteroid_projectile_collision,
            detect_asteroid_hero_ship_collision
        ).run_if(in_state(PausingState::Running).and_then(in_state(GameState::InGame))));
    }
}

pub fn detect_asteroid_projectile_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut scoreboard_score: ResMut<ScoreboardScore>,
    asteroid_destroyed_sound: Res<AsteroidDestroyedSound>,
    projectile_query: Query<(Entity, &Projectile)>,
    asteroid_query: Query<(Entity, &Asteroid, &Transform), Without<HeroShip>>
) {
    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                for (asteroid_entity, asteroid_component, asteroid_transform) in &asteroid_query {
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

pub fn detect_asteroid_hero_ship_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut hero_ship_remaining_lives: ResMut<HeroShipRemainingLives>,
    mut hero_ship_destroyed_sound: Res<HeroShipDestroyedSound>,
    mut hero_ship_query: Query<(Entity, &HeroShip, &mut Transform, &mut Visibility, &mut CollisionGroups), Without<Asteroid>>,
    asteroid_query: Query<Entity, With<Asteroid>>
) {
    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                let (
                    hero_ship_entity,
                    _,
                    hero_ship_transform,
                    mut hero_ship_visibility,
                    mut hero_ship_collision_groups
                ) = hero_ship_query.single_mut();

                for asteroid_entity in &asteroid_query {
                    if
                        (asteroid_entity == first_entity || asteroid_entity == second_entity) &&
                        (hero_ship_entity == first_entity || hero_ship_entity == second_entity)
                    {
                        if hero_ship_remaining_lives.lives_remaining > 0 {
                            hero_ship_remaining_lives.lives_remaining -= 1;
                        }

                        if hero_ship_remaining_lives.lives_remaining == 0 {
                            next_state.set(GameState::EndGame);
                            spawn_hero_ship_destroyed(
                                commands.reborrow(),
                                &asset_server,
                                &mut texture_atlas_layouts,
                                &mut hero_ship_destroyed_sound,
                                hero_ship_transform.clone()
                            );
                        } else {
                            *hero_ship_visibility = Visibility::Hidden;
                            *hero_ship_collision_groups = CollisionGroups::new(Group::GROUP_1, Group::all());

                            spawn_hero_ship_destroyed(
                                commands.reborrow(),
                                &asset_server,
                                &mut texture_atlas_layouts,
                                &mut hero_ship_destroyed_sound,
                                hero_ship_transform.clone()
                            );
                        }
                    }
                }
            },
            CollisionEvent::Stopped(_, _, _) => ()
        }
    }
}

fn spawn_hero_ship_destroyed(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    hero_ship_destroyed_sound: &mut Res<HeroShipDestroyedSound>,
    hero_ship_transform: Transform
) {
    commands.spawn(AudioBundle {
        source: hero_ship_destroyed_sound.clone(),
        settings: PlaybackSettings {
            mode: Despawn,
            volume: Volume::new(2.5),
            ..default()
        }
    });

    spawn_hero_ship_destroyed_spritesheet(
        commands.reborrow(),
        &asset_server,
        texture_atlas_layouts,
        hero_ship_transform.translation
    );
}
