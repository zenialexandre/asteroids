mod constants;
mod hero_ship;
mod asteroid;
mod projectile;
mod collision;

use bevy::{
    prelude::*,
    window::{EnabledButtons, PrimaryWindow, WindowPosition},
    winit::WinitWindows
};

use bevy_fps_counter::{
    FpsCounter,
    FpsCounterText,
    FpsCounterPlugin
};

use bevy_rapier2d::prelude::*;
use std::io::Cursor;
use winit::window::Icon;
use constants::image_handles::HERO_SHIP_HANDLE_IMAGE;

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
        .add_plugins(FpsCounterPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<projectile::ProjectileSpawnTimer>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, set_fps_counter)
        .add_systems(FixedUpdate, (
            hero_ship::set_hero_ship_movement_and_rotation,
            hero_ship::draw_hero_ship_fire,
            hero_ship::set_hero_ship_position_after_border_outbounds,
            hero_ship::hero_ship_fire_projectile,
            projectile::set_projectile_movement,
            asteroid::set_asteroid_movement_and_rotation,
            asteroid::set_asteroid_position_after_border_outbounds,
            collision::detect_asteroid_collision
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

fn set_game_hero_ship(
    mut commands: Commands,
    asset_server: &Res<AssetServer>
) {
    let hero_ship_handle: Handle<Image> = asset_server.load(HERO_SHIP_HANDLE_IMAGE);

    commands.spawn((
        SpriteBundle {
            texture: hero_ship_handle,
            ..default()
        },
        hero_ship::HeroShip::default(),
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

fn set_fps_counter(
    mut fps_counter_state: ResMut<FpsCounter>,
    mut fps_counter_text_query: Query<&mut Text, With<FpsCounterText>>
) {
    let mut fps_counter_text: Mut<'_, Text> = fps_counter_text_query.single_mut();
    fps_counter_text.sections[0].style.font_size = 15.;
    fps_counter_state.enable();
}
