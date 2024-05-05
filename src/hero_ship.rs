use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::Mutex;

use bevy::audio::{
    PlaybackMode::Despawn,
    Volume
};

use crate::projectile::{self, Projectile};
use crate::ui::ScoreboardScore;

use crate::constants::image_handles::{
    HERO_SHIP_HANDLE_IMAGE,
    HERO_SHIP_FIRE_HANDLE_IMAGE,
    HERO_SHIP_DESTROYED_HANDLE_IMAGE,
    HERO_SHIP_THREE_LIVES_HANDLE_IMAGE,
    HERO_SHIP_TWO_LIVES_HANDLE_IMAGE,
    HERO_SHIP_ONE_LIVE_HANDLE_IMAGE
};

use crate::constants::hero_ship_movement_values::{
    HERO_SHIP_MOVEMENT_SPEED_DRAG,
    HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED,
    HERO_SHIP_MAX_MOVEMENT_SPEED,
    HERO_SHIP_ROTATION_SPEED_DRAG,
    HERO_SHIP_INCREMENTAL_ROTATION_SPEED,
    HERO_SHIP_MAX_ROTATION_SPEED
};

use crate::constants::borders::{
    RIGHT_BORDER_POSITION,
    LEFT_BORDER_POSITION,
    TOP_BORDER_POSITION,
    BOTTOM_BORDER_POSITION
};

#[derive(Component)]
pub struct HeroShip {
    pub movement_speed: f32,
    pub movement_speed_drag: f32,
    pub movement_speed_incrementation: f32,
    pub movement_speed_maximum: f32,
    pub rotation_speed: f32,
    pub rotation_speed_drag: f32,
    pub rotation_speed_incrementation: f32,
    pub rotation_speed_maximum: f32
}

impl Default for HeroShip {
    fn default() -> Self {
        return Self {
            movement_speed: 0.,
            movement_speed_drag: HERO_SHIP_MOVEMENT_SPEED_DRAG,
            movement_speed_incrementation: HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED,
            movement_speed_maximum: HERO_SHIP_MAX_MOVEMENT_SPEED,
            rotation_speed: 0.,
            rotation_speed_drag: HERO_SHIP_ROTATION_SPEED_DRAG,
            rotation_speed_incrementation: HERO_SHIP_INCREMENTAL_ROTATION_SPEED,
            rotation_speed_maximum: HERO_SHIP_MAX_ROTATION_SPEED
        };
    }
}

#[derive(Component)]
pub struct HeroShipAnimationIndices {
    pub first: usize,
    pub last: usize
}

#[derive(Component, Deref, DerefMut)]
pub struct HeroShipAnimationTimer(pub Timer);

impl Default for HeroShipAnimationTimer {
    fn default() -> Self {
        return Self(Timer::from_seconds(0.1, TimerMode::Repeating));
    }
}

#[derive(Component, Clone)]
pub struct HeroShipLivesSprite {
    pub handle_image: Handle<Image>
}

impl HeroShipLivesSprite {
    fn initialize_on_setup(
        asset_server: &Res<AssetServer>
    ) -> Self {
        return Self { handle_image: asset_server.load(HERO_SHIP_THREE_LIVES_HANDLE_IMAGE) };
    }
}

#[derive(Resource)]
pub struct HeroShipRemainingLives {
    pub lives_remaining: usize
}

impl Default for HeroShipRemainingLives {
    fn default() -> Self {
        return Self { lives_remaining: 3 };
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct HeroShipRespawnTimer(pub Timer);

impl Default for HeroShipRespawnTimer {
    fn default() -> Self {
        return Self(Timer::from_seconds(2., TimerMode::Repeating));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct HeroShipStillAliveTimer(pub Timer);

impl Default for HeroShipStillAliveTimer {
    fn default() -> Self {
        return Self(Timer::from_seconds(10., TimerMode::Repeating));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HeroShipLaunchingSound(pub Handle<AudioSource>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HeroShipDestroyedSound(pub Handle<AudioSource>);

lazy_static! { static ref HERO_SHIP_ROTATION_FACTOR: Mutex<f32> = Mutex::new(0.); }

pub fn spawn_hero_ship(
    mut commands: Commands,
    asset_server: &Res<AssetServer>
) {
    let hero_ship_handle: Handle<Image> = asset_server.load(HERO_SHIP_HANDLE_IMAGE);

    commands.spawn((
        SpriteBundle {
            texture: hero_ship_handle,
            visibility: Visibility::Visible,
            ..default()
        },
        HeroShip::default(),
    ))
    .insert(Name::new("Hero Ship"))
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(5.))
    .insert(GravityScale(0.))
    .insert(CollisionGroups::new(Group::GROUP_10, Group::GROUP_1));
}

pub fn spawn_hero_ship_lives(
    mut commands: Commands,
    asset_server: &Res<AssetServer>
) {
    let hero_ship_lives_sprite: HeroShipLivesSprite =
        HeroShipLivesSprite::initialize_on_setup(&asset_server);

    commands.spawn((
        SpriteBundle {
            texture: hero_ship_lives_sprite.handle_image.clone_weak(),
            transform: Transform {
                translation: Vec3::new(320., 235., 0.),
                ..default()
            },
            ..default()
        },
        hero_ship_lives_sprite
    ));
}

pub fn check_for_hero_ship_lives(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hero_ship_remaining_lives: Res<HeroShipRemainingLives>,
    mut hero_ship_lives_sprite_query: Query<(Entity, &HeroShipLivesSprite, &mut Handle<Image>)>
) {
    for (hero_ship_lives_sprite_entity, _, mut hero_ship_lives_sprite_handle_image) in &mut hero_ship_lives_sprite_query {
        match hero_ship_remaining_lives.lives_remaining {
            3 => { *hero_ship_lives_sprite_handle_image = asset_server.load(HERO_SHIP_THREE_LIVES_HANDLE_IMAGE); },
            2 => { *hero_ship_lives_sprite_handle_image = asset_server.load(HERO_SHIP_TWO_LIVES_HANDLE_IMAGE); },
            1 => { *hero_ship_lives_sprite_handle_image = asset_server.load(HERO_SHIP_ONE_LIVE_HANDLE_IMAGE); },
            0 => { commands.entity(hero_ship_lives_sprite_entity).despawn_recursive(); },
            _ => {}
        }
    }
}

pub fn respawn_hero_ship_on_demand(
    time: Res<Time>,
    mut hero_ship_respawn_timer: ResMut<HeroShipRespawnTimer>,
    mut hero_ship_query: Query<(&HeroShip, &mut Transform, &mut Visibility, &mut CollisionGroups)>,
) {
    let (_, mut hero_ship_transform, mut hero_ship_visibility, mut hero_ship_collision_groups) = hero_ship_query.single_mut();

    if *hero_ship_visibility == Visibility::Hidden {
        hero_ship_respawn_timer.0.tick(time.delta());

        if hero_ship_respawn_timer.just_finished() {
            hero_ship_transform.translation = Vec3::new(0., 0., 0.);
            *hero_ship_visibility = Visibility::Visible;
            *hero_ship_collision_groups = CollisionGroups::new(Group::GROUP_10, Group::GROUP_1);
        }
    }
}

pub fn spawn_hero_ship_destroyed_spritesheet(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    hero_ship_last_position: Vec3
) {
    let texture: Handle<Image> = asset_server.load(HERO_SHIP_DESTROYED_HANDLE_IMAGE);
    let texture_atlas_layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(
        Vec2::new(24., 24.),
        7,
        1,
        None,
        None
    );
    let texture_atlas_layout_handle: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(texture_atlas_layout);
    let hero_ship_animation_indices: HeroShipAnimationIndices = HeroShipAnimationIndices { first: 1, last: 6 };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout_handle,
                index: hero_ship_animation_indices.first
            },
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: hero_ship_last_position,
                ..default()
            },
            ..default()
        },
        hero_ship_animation_indices,
        HeroShipAnimationTimer::default()
    ));
}

pub fn animate_hero_ship_destroyed_spritesheet(
    time: Res<Time>,
    mut animation_query: Query<(&HeroShipAnimationIndices, &mut HeroShipAnimationTimer, &mut TextureAtlas)>
) {
    for (indices, mut timer, mut texture_atlas) in &mut animation_query {
        timer.tick(time.delta());

        if timer.just_finished() {
            texture_atlas.index = if texture_atlas.index == indices.last {
                continue
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

pub fn set_hero_ship_movement_and_rotation(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    hero_ship_launching_sound: Res<HeroShipLaunchingSound>,
    mut hero_ship_query: Query<(&mut HeroShip, &mut Transform)>
) {
    let mut movement_direction: Vec3;
    let mut movement_distance: f32;

    for (mut hero_ship_entity, mut hero_ship_transform) in &mut hero_ship_query {
        set_hero_ship_rotation_factor(&keyboard_input, hero_ship_entity.reborrow());
        increase_hero_ship_movement_speed(commands.reborrow(), &keyboard_input, &hero_ship_launching_sound, hero_ship_entity.reborrow());
        apply_brake_on_hero_ship_movement_speed(&time, &keyboard_input, hero_ship_entity.reborrow());

        hero_ship_transform.rotate_z(
            *HERO_SHIP_ROTATION_FACTOR.lock().unwrap() * hero_ship_entity.rotation_speed * time.delta_seconds()
        );
        movement_direction = hero_ship_transform.rotation * Vec3::Y;
        movement_distance = hero_ship_entity.movement_speed * time.delta_seconds();
        hero_ship_transform.translation += movement_direction * movement_distance;

        apply_drag_on_hero_ship_movement_speed(&time, &keyboard_input, hero_ship_entity.reborrow());
        apply_drag_on_hero_ship_rotation_speed(&time, &keyboard_input, hero_ship_entity);
    }
}

fn set_hero_ship_rotation_factor(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut hero_ship_entity: Mut<'_, HeroShip>
) {
    if
        keyboard_input.pressed(KeyCode::ArrowLeft) ||
        keyboard_input.pressed(KeyCode::KeyA)
    {
        if hero_ship_entity.rotation_speed < f32::to_radians(hero_ship_entity.rotation_speed_maximum) {
            hero_ship_entity.rotation_speed += f32::to_radians(hero_ship_entity.rotation_speed_incrementation);
        }
        *HERO_SHIP_ROTATION_FACTOR.lock().unwrap() = 1.;
    }

    if 
        keyboard_input.pressed(KeyCode::ArrowRight) ||
        keyboard_input.pressed(KeyCode::KeyD)
    {
        if hero_ship_entity.rotation_speed < f32::to_radians(hero_ship_entity.rotation_speed_maximum) {
            hero_ship_entity.rotation_speed += f32::to_radians(hero_ship_entity.rotation_speed_incrementation);
        }
        *HERO_SHIP_ROTATION_FACTOR.lock().unwrap() = -1.;
    }
}

fn increase_hero_ship_movement_speed(
    mut commands: Commands,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    hero_ship_launching_sound: &Res<HeroShipLaunchingSound>,
    mut hero_ship_entity: Mut<'_, HeroShip>
) {
    if 
        keyboard_input.pressed(KeyCode::ArrowUp) ||
        keyboard_input.pressed(KeyCode::KeyW)
    {
        if hero_ship_entity.movement_speed < hero_ship_entity.movement_speed_maximum {
            hero_ship_entity.movement_speed += hero_ship_entity.movement_speed_incrementation;
        }

        commands.spawn(AudioBundle {
            source: hero_ship_launching_sound.0.clone(),
            settings: PlaybackSettings {
                mode: Despawn,
                volume: Volume::new(0.25),
                ..default()
            }
        });
    }
}

fn apply_brake_on_hero_ship_movement_speed(
    time: &Res<Time>,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut hero_ship_entity: Mut<'_, HeroShip>
) {
    if
        keyboard_input.pressed(KeyCode::ArrowDown) ||
        keyboard_input.pressed(KeyCode::KeyS)
    {
        if hero_ship_entity.movement_speed > 0. {
            hero_ship_entity.movement_speed -= hero_ship_entity.movement_speed_drag * time.delta_seconds();
        }
    }
}

fn apply_drag_on_hero_ship_movement_speed(
    time: &Res<Time>,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut hero_ship_entity: Mut<'_, HeroShip>
) {
    if
        (
            !keyboard_input.pressed(KeyCode::ArrowUp) ||
            !keyboard_input.pressed(KeyCode::KeyW)
        ) && hero_ship_entity.movement_speed > 0.
    {
        hero_ship_entity.movement_speed -= hero_ship_entity.movement_speed_drag * time.delta_seconds();
    }
}

fn apply_drag_on_hero_ship_rotation_speed(
    time: &Res<Time>,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut hero_ship_entity: Mut<'_, HeroShip>
) {
    if
        (
            !keyboard_input.pressed(KeyCode::ArrowLeft) ||
            !keyboard_input.pressed(KeyCode::KeyA)
        ) &&
        (
            !keyboard_input.pressed(KeyCode::ArrowRight) ||
            !keyboard_input.pressed(KeyCode::KeyD)
        ) && hero_ship_entity.rotation_speed > 0.
    {
        hero_ship_entity.rotation_speed -= f32::to_radians(hero_ship_entity.rotation_speed_drag) * time.delta_seconds();
    }
}

pub fn draw_hero_ship_fire(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hero_ship_query: Query<(&HeroShip, &mut Handle<Image>)>
) {
    for (_, mut hero_ship_handle_image) in &mut hero_ship_query {
        if
            keyboard_input.pressed(KeyCode::ArrowUp) ||
            keyboard_input.pressed(KeyCode::KeyW)
        {
            *hero_ship_handle_image = asset_server.load(HERO_SHIP_FIRE_HANDLE_IMAGE);
        } else {
            *hero_ship_handle_image = asset_server.load(HERO_SHIP_HANDLE_IMAGE);
        }
    }
}

pub fn set_hero_ship_position_after_border_outbounds(
    mut hero_ship_query: Query<(&HeroShip, &mut Transform)>
) {
    let (_, mut hero_ship_transform) = hero_ship_query.single_mut();
    let hero_ship_position_x: f32 = hero_ship_transform.translation.x;
    let hero_ship_position_y: f32 = hero_ship_transform.translation.y;

    if hero_ship_position_x >= RIGHT_BORDER_POSITION {
        hero_ship_transform.translation.x = LEFT_BORDER_POSITION;
        hero_ship_transform.translation.y -= 10.;
    } else if hero_ship_position_x <= LEFT_BORDER_POSITION {
        hero_ship_transform.translation.x = RIGHT_BORDER_POSITION;
        hero_ship_transform.translation.y += 10.;
    }

    if hero_ship_position_y >= TOP_BORDER_POSITION {
        hero_ship_transform.translation.y = BOTTOM_BORDER_POSITION;
        hero_ship_transform.translation.x -= 10.;
    } else if hero_ship_position_y <= BOTTOM_BORDER_POSITION {
        hero_ship_transform.translation.y = TOP_BORDER_POSITION;
        hero_ship_transform.translation.x += 10.;
    }
}

pub fn hero_ship_fire_projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut projectile_spawn_timer: ResMut<projectile::ProjectileSpawnTimer>,
    projectile_spawn_sound: Res<projectile::ProjectileSpawnSound>,
    hero_ship_query: Query<(&HeroShip, &Transform, &Visibility)>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let (_, hero_ship_transform, hero_ship_visibility) = hero_ship_query.single();
        let mut projectile_entity: Projectile = Projectile::default();
        projectile_spawn_timer.0.tick(time.delta());

        if projectile_spawn_timer.0.just_finished() && hero_ship_visibility == &Visibility::Visible {
            projectile_entity.translation = hero_ship_transform.translation;
            projectile_entity.direction = hero_ship_transform.rotation * Vec3::Y;
            Projectile::spawn_projectile(projectile_entity, commands.reborrow(), asset_server);
            commands.spawn(AudioBundle {
                source: projectile_spawn_sound.clone(),
                settings: PlaybackSettings {
                    mode: Despawn,
                    volume: Volume::new(0.5),
                    ..default()
                }
            });
        }
    }
}

pub fn dynamic_hero_ship_still_alive_check(
    time: Res<Time>,
    mut hero_ship_still_alive_timer: ResMut<HeroShipStillAliveTimer>,
    mut scoreboard_score: ResMut<ScoreboardScore>
) {
    hero_ship_still_alive_timer.0.tick(time.delta());

    if hero_ship_still_alive_timer.0.just_finished() {
        scoreboard_score.score += 5;
    }
}
