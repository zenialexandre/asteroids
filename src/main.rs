mod constants;
mod hero_ship;
mod asteroid;

use bevy::{
    prelude::*,
    window::{EnabledButtons, PrimaryWindow, WindowPosition},
    winit::WinitWindows
};

use std::io::Cursor;
use winit::window::Icon;
use constants::image_handles::HERO_SHIP_HANDLE_IMAGE;

use constants::hero_ship_movement_values::{
    HERO_SHIP_MOVEMENT_SPEED_DRAG,
    HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED,
    HERO_SHIP_MAX_MOVEMENT_SPEED,
    HERO_SHIP_ROTATION_SPEED_DRAG,
    HERO_SHIP_INCREMENTAL_ROTATION_SPEED,
    HERO_SHIP_MAX_ROTATION_SPEED
};

#[macro_use]
extern crate lazy_static;

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
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            hero_ship::set_hero_ship_movement_and_rotation,
            hero_ship::draw_hero_ship_fire,
            hero_ship::set_hero_ship_position_after_border_outbounds,
            asteroid::set_asteroid_movement_and_rotation,
            asteroid::set_asteroid_position_after_border_outbounds
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>
) {
    set_game_camera(commands.reborrow());
    set_game_window_icon(windows, primary_window_query);
    set_game_hero_ship(commands.reborrow(), &asset_server);
    asteroid::spawn_initial_asteroids(commands, &asset_server);
}

fn set_game_camera(mut commands: Commands) {
    let camera_2d_bundle: Camera2dBundle = Camera2dBundle {
        camera: Camera { clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() },
        ..default()
    };
    commands.spawn(camera_2d_bundle);
}

fn set_game_hero_ship(mut commands: Commands, asset_server: &Res<AssetServer>) {
    let hero_ship_handle: Handle<Image> = asset_server.load(HERO_SHIP_HANDLE_IMAGE);

    commands.spawn((
        SpriteBundle {
            texture: hero_ship_handle,
            ..default()
        },
        hero_ship::HeroShip {
            movement_speed: 0.,
            movement_speed_drag: HERO_SHIP_MOVEMENT_SPEED_DRAG,
            movement_speed_incrementation: HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED,
            movement_speed_maximum: HERO_SHIP_MAX_MOVEMENT_SPEED,
            rotation_speed: 0.,
            rotation_speed_drag: HERO_SHIP_ROTATION_SPEED_DRAG,
            rotation_speed_incrementation: HERO_SHIP_INCREMENTAL_ROTATION_SPEED,
            rotation_speed_maximum: HERO_SHIP_MAX_ROTATION_SPEED
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
