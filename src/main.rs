use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowPosition, PrimaryWindow},
    winit::WinitWindows
};
use std::io::Cursor;
use winit::window::Icon;

#[derive(Component)]
struct HeroShip {
    movement_speed: f32,
    rotation_speed: f32,
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
        .add_systems(Startup, (setup, set_game_window_icon))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_2d_bundle: Camera2dBundle = Camera2dBundle {
        camera: Camera { clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() },
        ..default()
    };
    commands.spawn(camera_2d_bundle);

    let hero_ship_handle = asset_server.load("textures/sprites/ships/asteroids_hero_ship.png");
    commands.spawn((
        SpriteBundle {
            texture: hero_ship_handle,
            ..default()
        },
        HeroShip {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
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
