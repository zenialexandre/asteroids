use bevy::{
    prelude::*,
    window::{EnabledButtons, PrimaryWindow, WindowPosition},
    winit::WinitWindows
};
use bevy_vector_shapes::prelude::*;
use std::io::Cursor;
use std::sync::Mutex;
use winit::window::Icon;

#[macro_use]
extern crate lazy_static;

#[derive(Component)]
struct HeroShip {
    movement_speed: f32,
    rotation_speed: f32,
}

const HERO_SHIP_MOVEMENT_SPEED_DRAG: f32 = 100.;
const HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED: f32 = 5.;
const HERO_SHIP_MAX_MOVEMENT_SPEED: f32 = 320.;
const HERO_SHIP_ROTATION_SPEED_DRAG: f32 = 150.;
const HERO_SHIP_INCREMENTAL_ROTATION_SPEED: f32 = 15.;
const HERO_SHIP_MAX_ROTATION_SPEED: f32 = 360.;
const BOTTOM_BORDER_POSITION: f32 = -260.;
const TOP_BORDER_POSITION: f32 = 260.;
const LEFT_BORDER_POSITION: f32 = -400.;
const RIGHT_BORDER_POSITION: f32 = 400.;

lazy_static! { static ref HERO_SHIP_ROTATION_FACTOR: Mutex<f32> = Mutex::new(0.); }

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Asteroids".into(),
                    resizable: false,
                    enabled_buttons: EnabledButtons{ minimize: true, maximize: false, close: true },
                    focused: true,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: (800., 500.).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(Shape2dPlugin::default())
        .add_systems(Startup, (setup, set_game_window_icon))
        .add_systems(FixedUpdate, (
            set_hero_ship_movement,
            draw_hero_ship_fire,
            set_hero_ship_position_after_border_outbounds
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    set_game_camera(commands.reborrow());
    set_game_hero_ship(commands.reborrow(), asset_server);
}

fn set_game_camera(mut commands: Commands) {
    let camera_2d_bundle: Camera2dBundle = Camera2dBundle {
        camera: Camera { clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() },
        ..default()
    };
    commands.spawn(camera_2d_bundle);
}

fn set_game_hero_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hero_ship_handle: Handle<Image> =
        asset_server.load("textures/sprites/ships/asteroids_hero_ship_24x24.png");
    commands.spawn((
        SpriteBundle {
            texture: hero_ship_handle,
            ..default()
        },
        HeroShip {
            movement_speed: 0.,
            rotation_speed: 0.,
        },
    ));
}

fn set_game_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>
) {
    let primary_window_entity: Entity = primary_window_query.single();
    let Some(primary_window) = windows.get_window(primary_window_entity) else {
        return;
    };
    let icon_buffer = Cursor::new(include_bytes!(
        "../assets/icons/asteroids_icon.png"
    ));

    if let Ok(image) = image::load(icon_buffer, image::ImageFormat::Png) {
        let icon_image = image.into_rgba8();
        let (icon_width, icon_height) = icon_image.dimensions();
        let icon_rgba: Vec<u8> = icon_image.into_raw();
        let asteroids_game_icon: Icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        primary_window.set_window_icon(Some(asteroids_game_icon));
    };
}

#[doc = "Needs Refactoring."]
fn set_hero_ship_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hero_ship_query: Query<(&mut HeroShip, &mut Transform)>
) {
    let mut hero_ship_rotation_factor: f32;
    let mut movement_direction: Vec3;
    let mut movement_distance: f32;
    let mut translation_delta: Vec3;

    for (mut hero_ship_entity, mut hero_ship_transform) in &mut hero_ship_query {
        if
            keyboard_input.pressed(KeyCode::ArrowLeft) ||
            keyboard_input.pressed(KeyCode::KeyA)
        {
            if hero_ship_entity.rotation_speed < f32::to_radians(HERO_SHIP_MAX_ROTATION_SPEED) {
                hero_ship_entity.rotation_speed += f32::to_radians(HERO_SHIP_INCREMENTAL_ROTATION_SPEED);
            }
            *HERO_SHIP_ROTATION_FACTOR.lock().unwrap() = 1.;
        }

        if 
            keyboard_input.pressed(KeyCode::ArrowRight) ||
            keyboard_input.pressed(KeyCode::KeyD)
        {
            if hero_ship_entity.rotation_speed < f32::to_radians(HERO_SHIP_MAX_ROTATION_SPEED) {
                hero_ship_entity.rotation_speed += f32::to_radians(HERO_SHIP_INCREMENTAL_ROTATION_SPEED);
            }
            *HERO_SHIP_ROTATION_FACTOR.lock().unwrap() = -1.;
        }

        if 
            keyboard_input.pressed(KeyCode::ArrowUp) ||
            keyboard_input.pressed(KeyCode::KeyW)
        {
            if hero_ship_entity.movement_speed < HERO_SHIP_MAX_MOVEMENT_SPEED {
                hero_ship_entity.movement_speed += HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED;
            }
        }

        if 
            keyboard_input.pressed(KeyCode::ArrowDown) ||
            keyboard_input.pressed(KeyCode::KeyS)
        {
            if hero_ship_entity.movement_speed > 0. {
                hero_ship_entity.movement_speed -= HERO_SHIP_MOVEMENT_SPEED_DRAG * time.delta_seconds();
            }
        }

        hero_ship_rotation_factor = *HERO_SHIP_ROTATION_FACTOR.lock().unwrap();

        hero_ship_transform.rotate_z(
            hero_ship_rotation_factor * hero_ship_entity.rotation_speed * time.delta_seconds()
        );
        movement_direction = hero_ship_transform.rotation * Vec3::Y;
        movement_distance = hero_ship_entity.movement_speed * time.delta_seconds();
        translation_delta = movement_direction * movement_distance;
        hero_ship_transform.translation += translation_delta;

        if
            (
                !keyboard_input.pressed(KeyCode::ArrowUp) ||
                !keyboard_input.pressed(KeyCode::KeyW)
            ) &&
            hero_ship_entity.movement_speed > 0.
        {
            hero_ship_entity.movement_speed -= HERO_SHIP_MOVEMENT_SPEED_DRAG * time.delta_seconds();
        }

        if
            (
                !keyboard_input.pressed(KeyCode::ArrowLeft) ||
                !keyboard_input.pressed(KeyCode::KeyA)
            ) &&
            (
                !keyboard_input.pressed(KeyCode::ArrowRight) ||
                !keyboard_input.pressed(KeyCode::KeyD)
            ) &&
            hero_ship_entity.rotation_speed > 0.
        {
            hero_ship_entity.rotation_speed -= f32::to_radians(HERO_SHIP_ROTATION_SPEED_DRAG) * time.delta_seconds();
        }
    }
}

#[doc = "Needs Revision / Not Working Yet."]
fn draw_hero_ship_fire(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    hero_ship_query: Query<(&HeroShip, &Transform)>,
    mut shape_painter: ShapePainter
) {
    let mut hero_ship_transform_translation: Vec3;
    let mut hero_ship_transform_translation_start: Vec3;
    let mut hero_ship_transform_translation_end: Vec3;

    for (_, hero_ship_transform) in &hero_ship_query {
        if
            keyboard_input.pressed(KeyCode::ArrowUp) ||
            keyboard_input.pressed(KeyCode::KeyW)
        {
            hero_ship_transform_translation = hero_ship_transform.translation;
            hero_ship_transform_translation_start = hero_ship_transform_translation;
            hero_ship_transform_translation_end = hero_ship_transform_translation;
            hero_ship_transform_translation_start.x = 0.;
            hero_ship_transform_translation_start.y -= 8.;
            hero_ship_transform_translation_end.x -= 2.;
            hero_ship_transform_translation_end.y -= 15.;
            shape_painter.color = Color::WHITE;
            shape_painter.alignment = Alignment::Flat;
            shape_painter.thickness = 0.1;
            shape_painter.line(hero_ship_transform_translation_start, hero_ship_transform_translation_end);
        }
    }
}

fn set_hero_ship_position_after_border_outbounds(
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
