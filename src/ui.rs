use bevy::prelude::*;

use crate::constants::borders::{
    TOP_BORDER_POSITION,
    RIGHT_BORDER_POSITION
};

#[derive(Component)]
pub struct PauseMenu;

pub fn spawn_pause_menu(
    mut commands: Commands
) {
    commands.spawn((
        TextBundle::from_section(
            "Paused!",
            TextStyle {
                font_size: 50.,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(TOP_BORDER_POSITION / 2. + 80.),
            left: Val::Px(RIGHT_BORDER_POSITION / 2. + 120.),
            ..default()
        }),
        PauseMenu,
    ));
}

pub fn erase_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>
) {
    for pause_menu_entity in &pause_menu_query {
        commands.entity(pause_menu_entity).despawn();
    }
}
