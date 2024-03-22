use bevy::{
    prelude::*, transform, window::{EnabledButtons, PrimaryWindow, WindowPosition}, winit::WinitWindows
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
        .add_systems(FixedUpdate, (set_hero_ship_movement, set_new_hero_position_after_border_collision))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    set_game_camera(commands.reborrow());
    set_game_hero_ship(commands, asset_server)
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
        asset_server.load("textures/sprites/ships/asteroids_hero_ship.png");
    commands.spawn((
        SpriteBundle {
            texture: hero_ship_handle,
            ..default()
        },
        HeroShip {
            movement_speed: 400.0,
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

fn set_hero_ship_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&HeroShip, &mut Transform)>,
) {
    let (hero_ship, mut hero_ship_transform) = query.single_mut();
    let mut rotation_factor: f32 = 0.0;
    let mut movement_factor: f32 = 0.0;

    if
        keyboard_input.pressed(KeyCode::ArrowLeft) ||
        keyboard_input.pressed(KeyCode::KeyA)
    {
        rotation_factor += 1.0;
    }

    if 
        keyboard_input.pressed(KeyCode::ArrowRight) ||
        keyboard_input.pressed(KeyCode::KeyD)
    {
        rotation_factor -= 1.0;
    }

    if 
        keyboard_input.pressed(KeyCode::ArrowUp) ||
        keyboard_input.pressed(KeyCode::KeyW)
    {
        movement_factor += 1.0;
    }

    hero_ship_transform.rotate_z(
        rotation_factor * hero_ship.rotation_speed * time.delta_seconds()
    );

    let movement_direction: Vec3 = hero_ship_transform.rotation * Vec3::Y;
    let movement_distance: f32 = movement_factor * hero_ship.movement_speed * time.delta_seconds();
    let translation_delta: Vec3 = movement_direction * movement_distance;
    hero_ship_transform.translation += translation_delta;
}

/*
TODO: Gonna need to set Sprite Bundles for the borders
fn set_new_hero_position_after_border_collision(
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    hero_ship_query: Query<(&HeroShip, &mut Transform)>
) {
    let primary_window_entity: &Window = window_query.single();
    let primary_window_width: u16 = primary_window_entity.width() as u16;
    let primary_window_height: u16 = primary_window_entity.height() as u16;

    println!("Window W: {}", primary_window_width);
    println!("Window H: {}", primary_window_height);

    let (_, hero_ship_transform) = hero_ship_query.single();
    let hero_ship_x_position: u16 = hero_ship_transform.translation.x as u16;
    let hero_ship_y_position: u16 = hero_ship_transform.translation.y as u16;

    println!("Hero Ship X: {}", hero_ship_x_position);
    println!("Hero Ship Y: {}", hero_ship_y_position);
}
*/
