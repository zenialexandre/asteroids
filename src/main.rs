mod constants;
mod hero_ship;
mod asteroid;
mod projectile;
mod collision;
mod ui;

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

#[macro_use]
extern crate lazy_static;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    StartScreen,
    InGame
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum PausingState {
    Paused,
    #[default]
    Running
}

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
        .init_state::<GameState>()
        .init_state::<PausingState>()
        .init_resource::<projectile::ProjectileSpawnTimer>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (set_fps_counter, setup_main_entities))
        .add_systems(Update, ui::spawn_start_screen.run_if(in_state(GameState::StartScreen)))
        .add_systems(Update, check_for_starting_by_keyboard.run_if(in_state(GameState::StartScreen)))
        .add_systems(Update, check_for_pausing_by_keyboard.run_if(in_state(GameState::InGame)))
        .add_systems(Update,
            ui::spawn_pause_menu.run_if(in_state(PausingState::Paused).and_then(in_state(GameState::InGame)))
        )
        .add_systems(Update, (
            ui::erase_start_screen,
            ui::erase_pause_menu
        ).run_if(in_state(PausingState::Running).and_then(in_state(GameState::InGame))))
        .add_systems(FixedUpdate, (
            hero_ship::set_hero_ship_movement_and_rotation,
            hero_ship::draw_hero_ship_fire,
            hero_ship::set_hero_ship_position_after_border_outbounds,
            hero_ship::hero_ship_fire_projectile,
            projectile::set_projectile_movement,
            asteroid::set_asteroid_movement_and_rotation,
            asteroid::set_asteroid_position_after_border_outbounds,
            collision::detect_asteroid_collision
        ).run_if(in_state(PausingState::Running).and_then(in_state(GameState::InGame))
    )).run();
}

fn setup(
    commands: Commands,
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>
) {
    set_game_camera(commands);
    set_game_window_icon(windows, primary_window_query);
}

fn set_game_camera(mut commands: Commands) {
    let camera_2d_bundle: Camera2dBundle = Camera2dBundle {
        camera: Camera { clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() },
        ..default()
    };
    commands.spawn(camera_2d_bundle);
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

fn setup_main_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    hero_ship::spawn_hero_ship(commands.reborrow(), &asset_server);
    asteroid::spawn_initial_asteroids(commands.reborrow(), &asset_server);
}

fn set_fps_counter(
    mut fps_counter_state: ResMut<FpsCounter>,
    mut fps_counter_text_query: Query<&mut Text, With<FpsCounterText>>
) {
    let mut fps_counter_text: Mut<'_, Text> = fps_counter_text_query.single_mut();
    fps_counter_text.sections[0].style.font_size = 15.;
    fps_counter_state.enable();
}

fn check_for_starting_by_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    states: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if states.get() == &GameState::StartScreen {
            next_state.set(GameState::InGame);
        }
    }
}

fn check_for_pausing_by_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    states: ResMut<State<PausingState>>,
    mut next_state: ResMut<NextState<PausingState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match states.get() {
            PausingState::Paused => next_state.set(PausingState::Running),
            PausingState::Running => next_state.set(PausingState::Paused)
        }
    }
}
