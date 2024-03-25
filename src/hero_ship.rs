use bevy::prelude::*;
use std::sync::Mutex;

use crate::constants::image_handles::{
    HERO_SHIP_HANDLE_IMAGE,
    HERO_SHIP_FIRE_HANDLE_IMAGE
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

lazy_static! { static ref HERO_SHIP_ROTATION_FACTOR: Mutex<f32> = Mutex::new(0.); }

pub fn set_hero_ship_movement_and_rotation(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hero_ship_query: Query<(&mut HeroShip, &mut Transform)>
) {
    let mut movement_direction: Vec3;
    let mut movement_distance: f32;

    for (mut hero_ship_entity, mut hero_ship_transform) in &mut hero_ship_query {
        set_hero_ship_rotation_factor(&keyboard_input, hero_ship_entity.reborrow());
        increase_hero_ship_movement_speed(&keyboard_input, hero_ship_entity.reborrow());
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
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut hero_ship_entity: Mut<'_, HeroShip>
) {
    if 
        keyboard_input.pressed(KeyCode::ArrowUp) ||
        keyboard_input.pressed(KeyCode::KeyW)
    {
        if hero_ship_entity.movement_speed < hero_ship_entity.movement_speed_maximum {
            hero_ship_entity.movement_speed += hero_ship_entity.movement_speed_incrementation;
        }
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
