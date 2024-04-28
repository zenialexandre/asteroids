use bevy::{
    prelude::*,
    math::vec3
};

use rand::{
    rngs::ThreadRng,
    Rng
};

use bevy_rapier2d::{
    prelude::*,
    geometry::Collider
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
    BOTTOM_BORDER_POSITION,
    RIGHT_BORDER_OFFSCREEN_POSITION,
    LEFT_BORDER_OFFSCREEN_POSITION,
    TOP_BORDER_OFFSCREEN_POSITION,
    BOTTOM_BORDER_OFFSCREEN_POSITION
};

use crate::constants::asteroid_movement_values::{
    SMALL_ASTEROID_MOVEMENT_SPEED,
    SMALL_ASTEROID_MOVEMENT_SPEED_DRAG,
    SMALL_ASTEROID_ROTATION_SPEED,
    MEDIUM_ASTEROID_MOVEMENT_SPEED,
    MEDIUM_ASTEROID_MOVEMENT_SPEED_DRAG,
    MEDIUM_ASTEROID_ROTATION_SPEED,
    BIG_ASTEROID_MOVEMENT_SPEED,
    BIG_ASTEROID_MOVEMENT_SPEED_DRAG,
    BIG_ASTEROID_ROTATION_SPEED
};

use crate::constants::asteroid::{
    INITIAL_BIG_ASTEROIDS_ONSCREEN,
    SMALL_ASTEROID_COLLIDER_BALL_SIZE,
    MEDIUM_ASTEROID_COLLIDER_BALL_SIZE,
    BIG_ASTEROID_COLLIDER_BALL_SIZE
};

#[derive(Debug)]
enum BorderSide {
    Right,
    Left,
    Top,
    Bottom
}

impl BorderSide {
    fn get_randomic_border_side(mut thread_rng: ThreadRng) -> Self {
        let randomic_border_side: u32 = thread_rng.gen_range(1..=4);

        match randomic_border_side {
            1 => return Self::Right,
            2 => return Self::Left,
            3 => return Self::Top,
            4 => return Self::Bottom,
            _ => panic!("Invalid randomic number.")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AsteroidType {
    Small,
    Medium,
    Big
}

#[derive(Component, Clone, Debug)]
pub struct Asteroid {
    asteroid_type: AsteroidType,
    collider_ball_size: f32,
    pub handle_image: Option<Handle<Image>>,
    pub movement_speed: Option<f32>,
    pub movement_speed_drag: Option<f32>,
    pub movement_direction: Vec3,
    pub rotation_speed: Option<f32>,
    pub rotation_direction: f32
}

impl Default for Asteroid {
    fn default() -> Self {
        let mut thread_rng: ThreadRng = rand::thread_rng();

        return Self {
            asteroid_type: AsteroidType::Big,
            collider_ball_size: BIG_ASTEROID_COLLIDER_BALL_SIZE,
            handle_image: None,
            movement_speed: None,
            movement_speed_drag: None,
            movement_direction: vec3(thread_rng.gen_range(-1.0..=1.0), thread_rng.gen_range(-1.0..=1.0), 0.),
            rotation_speed: None,
            rotation_direction: thread_rng.gen_range(-1.0..=1.0)
        };
    }
}

impl Asteroid {
    fn initialize_asteroid_based_on_type(
        asset_server: &Res<AssetServer>,
        asteroid_type: AsteroidType
    ) -> Self {
        let mut asteroid_struct: Asteroid = Asteroid::default();

        match asteroid_type {
            AsteroidType::Small => {
                asteroid_struct.asteroid_type = asteroid_type;
                asteroid_struct.collider_ball_size = SMALL_ASTEROID_COLLIDER_BALL_SIZE;
                asteroid_struct.handle_image = Some(asset_server.load(SMALL_ASTEROID_HANDLE_IMAGE));
                asteroid_struct.movement_speed = Some(SMALL_ASTEROID_MOVEMENT_SPEED);
                asteroid_struct.movement_speed_drag = Some(SMALL_ASTEROID_MOVEMENT_SPEED_DRAG);
                asteroid_struct.rotation_speed = Some(f32::to_radians(SMALL_ASTEROID_ROTATION_SPEED));
            },
            AsteroidType::Medium => {
                asteroid_struct.asteroid_type = asteroid_type;
                asteroid_struct.collider_ball_size = MEDIUM_ASTEROID_COLLIDER_BALL_SIZE;
                asteroid_struct.handle_image = Some(asset_server.load(MEDIUM_ASTEROID_HANDLE_IMAGE));
                asteroid_struct.movement_speed = Some(MEDIUM_ASTEROID_MOVEMENT_SPEED);
                asteroid_struct.movement_speed_drag = Some(MEDIUM_ASTEROID_MOVEMENT_SPEED_DRAG);
                asteroid_struct.rotation_speed = Some(f32::to_radians(MEDIUM_ASTEROID_ROTATION_SPEED));
            },
            AsteroidType::Big => {
                asteroid_struct.asteroid_type = asteroid_type;
                asteroid_struct.collider_ball_size = BIG_ASTEROID_COLLIDER_BALL_SIZE;
                asteroid_struct.handle_image = Some(asset_server.load(BIG_ASTEROID_HANDLE_IMAGE));
                asteroid_struct.movement_speed = Some(BIG_ASTEROID_MOVEMENT_SPEED);
                asteroid_struct.movement_speed_drag = Some(BIG_ASTEROID_MOVEMENT_SPEED_DRAG);
                asteroid_struct.rotation_speed = Some(f32::to_radians(BIG_ASTEROID_ROTATION_SPEED));
            }
        }
        return asteroid_struct;
    }
}

pub fn spawn_initial_asteroids(
    mut commands: Commands,
    asset_server: &Res<AssetServer>
) {
    for _ in 1..=INITIAL_BIG_ASTEROIDS_ONSCREEN {
        spawn_asteroids(
            commands.reborrow(),
            asset_server,
            AsteroidType::Big,
            get_randomic_asteroid_onscreen_position(LEFT_BORDER_POSITION, RIGHT_BORDER_POSITION),
            get_randomic_asteroid_onscreen_position(BOTTOM_BORDER_POSITION, TOP_BORDER_POSITION)
        );
    }
}

pub fn spawn_asteroids_after_collision(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    asteroid: &Asteroid,
    asteroid_transform: &Transform
) {
    match asteroid.asteroid_type {
        AsteroidType::Big => {
            for _ in 1..=3 {
                spawn_asteroids(
                    commands.reborrow(),
                    asset_server,
                    AsteroidType::Medium,
                    asteroid_transform.translation.x,
                    asteroid_transform.translation.y
                );
            }
        },
        AsteroidType::Medium => { 
            for _ in 1..=5 {
                spawn_asteroids(
                    commands.reborrow(),
                    asset_server,
                    AsteroidType::Small,
                    asteroid_transform.translation.x,
                    asteroid_transform.translation.y
                );
            }
        },
        AsteroidType::Small => {}
    }
}

pub fn spawn_asteroids(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    asteroid_type: AsteroidType,
    position_x: f32,
    position_y: f32
) {
    let asteroid_struct: Asteroid = Asteroid::initialize_asteroid_based_on_type(asset_server, asteroid_type);

    commands.spawn((
        SpriteBundle {
            texture: asteroid_struct.clone().handle_image.unwrap(),
            transform: Transform {
                translation: vec3(
                    position_x,
                    position_y,
                    0.
                ),
                ..default()
            },
            ..default()
        },
        asteroid_struct.clone(),
    ))
    .insert(Name::new("Asteroid"))
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(asteroid_struct.collider_ball_size))
    .insert(GravityScale(0.))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_10))
    .insert(Dominance::group(10));
}

pub fn set_asteroid_movement_and_rotation(
    time: Res<Time>,
    mut asteroid_query: Query<(&Asteroid, &mut Transform)>
) {
    let mut movement_distance: f32;

    for (asteroid, mut asteroid_transform) in &mut asteroid_query {
        asteroid_transform.rotate_z(
            asteroid.rotation_direction * asteroid.rotation_speed.unwrap() * time.delta_seconds()
        );
        movement_distance = asteroid.movement_speed.unwrap() * time.delta_seconds();
        asteroid_transform.translation += asteroid.movement_direction * movement_distance;

        apply_drag_on_asteroid_movement_speed(
            &time,
            asteroid,
            asteroid_transform
        );
    }
}

fn apply_drag_on_asteroid_movement_speed(
    time: &Res<Time>,
    asteroid: &Asteroid,
    mut asteroid_transform: Mut<'_, Transform>
) {
    asteroid_transform.translation -= asteroid.movement_speed_drag.unwrap() * time.delta_seconds();
}

pub fn set_asteroid_position_after_border_outbounds(
    time: Res<Time>,
    mut asteroid_query: Query<(&mut Asteroid, &mut Transform)>
) {
    let thread_rng: ThreadRng = rand::thread_rng();

    for (asteroid_entity, mut asteroid_transform) in &mut asteroid_query {
        let randomic_asteroid_offscreen_position: Vec3;
        let asteroid_position_x: f32 = asteroid_transform.translation.x;
        let asteroid_position_y: f32 = asteroid_transform.translation.y;

        if
            asteroid_position_x >= RIGHT_BORDER_OFFSCREEN_POSITION ||
            asteroid_position_x <= LEFT_BORDER_OFFSCREEN_POSITION ||
            asteroid_position_y >= TOP_BORDER_OFFSCREEN_POSITION ||
            asteroid_position_y <= BOTTOM_BORDER_OFFSCREEN_POSITION
        {
            randomic_asteroid_offscreen_position = get_randomic_asteroid_offscreen_position(thread_rng.clone());
            asteroid_transform.translation.x = randomic_asteroid_offscreen_position.x;
            asteroid_transform.translation.y = randomic_asteroid_offscreen_position.y;

            set_asteroid_movement_direction_after_border_outbounds(
                &time,
                asteroid_entity,
                asteroid_transform,
                thread_rng.clone()
            );
        }
    }
}

fn get_randomic_asteroid_onscreen_position(
    position_initial_range: f32,
    position_final_range: f32
) -> f32 {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let mut is_randomic_position_ok: bool = false;
    let mut randomic_asteroid_position: f32 = 0.;

    while !is_randomic_position_ok {
        randomic_asteroid_position = thread_rng.gen_range(position_initial_range..=position_final_range);

        if
            randomic_asteroid_position >= 50. ||
            randomic_asteroid_position <= -50.
        {
            is_randomic_position_ok = true;
        }
    }
    return randomic_asteroid_position;
}

fn get_randomic_asteroid_offscreen_position(
    thread_rng: ThreadRng
) -> Vec3 {
    let randomic_offscreen_border_side: BorderSide = BorderSide::get_randomic_border_side(thread_rng.clone());

    match randomic_offscreen_border_side {
        BorderSide::Right => {
            return vec3(
                RIGHT_BORDER_OFFSCREEN_POSITION - 10.,
                get_randomic_asteroid_offscreen_y_position(thread_rng.clone()),
                0.
            );
        },
        BorderSide::Left => {
            return vec3(
                LEFT_BORDER_OFFSCREEN_POSITION + 10.,
                get_randomic_asteroid_offscreen_y_position(thread_rng.clone()),
                0.
            );
        },
        BorderSide::Top => {
            return vec3(
                get_randomic_asteroid_offscreen_x_position(thread_rng.clone()),
                TOP_BORDER_OFFSCREEN_POSITION - 10.,
                0.
            );
        },
        BorderSide::Bottom => {
            return vec3(
                get_randomic_asteroid_offscreen_x_position(thread_rng.clone()),
                BOTTOM_BORDER_OFFSCREEN_POSITION + 10.,
                0.
            );
        }
    }
}

fn get_randomic_asteroid_offscreen_x_position(
    mut thread_rng: ThreadRng
) -> f32 {
    return thread_rng.gen_range(LEFT_BORDER_POSITION..=RIGHT_BORDER_POSITION);
}

fn get_randomic_asteroid_offscreen_y_position(
    mut thread_rng: ThreadRng
) -> f32 {
    return thread_rng.gen_range(BOTTOM_BORDER_POSITION..=TOP_BORDER_POSITION);
}

fn set_asteroid_movement_direction_after_border_outbounds(
    time: &Res<Time>,
    mut asteroid_entity: Mut<'_, Asteroid>,
    asteroid_transform: Mut<'_, Transform>,
    mut thread_rng: ThreadRng
) {
    if asteroid_transform.translation.x <= RIGHT_BORDER_OFFSCREEN_POSITION - 10. {
        asteroid_entity.movement_direction = vec3(
            -1.,
            thread_rng.gen_range(BOTTOM_BORDER_POSITION..=TOP_BORDER_POSITION),
            0.
        ) * time.delta_seconds();
    }

    if asteroid_transform.translation.x >= LEFT_BORDER_OFFSCREEN_POSITION + 10. {
        asteroid_entity.movement_direction = vec3(
            1.,
            thread_rng.gen_range(BOTTOM_BORDER_POSITION..=TOP_BORDER_POSITION),
            0.
        ) * time.delta_seconds();
    }

    if asteroid_transform.translation.y <= TOP_BORDER_OFFSCREEN_POSITION - 10. {
        asteroid_entity.movement_direction = vec3(
            thread_rng.gen_range(LEFT_BORDER_POSITION..=RIGHT_BORDER_POSITION),
            -1.,
            0.
        ) * time.delta_seconds();
    }

    if asteroid_transform.translation.y >= BOTTOM_BORDER_OFFSCREEN_POSITION + 10. {
        asteroid_entity.movement_direction = vec3(
            thread_rng.gen_range(LEFT_BORDER_POSITION..=RIGHT_BORDER_POSITION),
            1.,
            0.
        ) * time.delta_seconds();
    }
}
