use bevy::prelude::*;

use crate::constants::ui_values::{
    TOP_TEXT_VAL_PX,
    RIGHT_TEXT_VAL_PX
};

#[derive(Component)]
pub struct StartScreenMenu;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct EndGameMenu;

#[derive(Component, Clone, Copy)]
pub struct Scoreboard;

#[derive(Resource, Clone, Copy)]
pub struct ScoreboardScore {
    pub score: usize
}

impl Default for ScoreboardScore {
    fn default() -> Self {
        return Self { score: 0 };
    }
}

pub fn spawn_start_screen_menu(
    mut commands: Commands
) {
    commands.spawn((
        TextBundle::from_section(
            "Asteroids\nPress Enter to start the game!",
            TextStyle {
                font_size: 40.,
                color: Color::SALMON,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(TOP_TEXT_VAL_PX),
            right: Val::Px(RIGHT_TEXT_VAL_PX - 220.),
            ..default()
        }),
        StartScreenMenu,
    ));
}

pub fn erase_start_screen_menu(
    mut commands: Commands,
    start_screen_query: Query<Entity, With<StartScreenMenu>>
) {
    for start_screen_entity in &start_screen_query {
        commands.entity(start_screen_entity).despawn();
    }
}

pub fn spawn_pause_menu(
    mut commands: Commands
) {
    commands.spawn((
        TextBundle::from_section(
            "Paused!",
            TextStyle {
                font_size: 50.,
                color: Color::SALMON,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(TOP_TEXT_VAL_PX),
            left: Val::Px(RIGHT_TEXT_VAL_PX),
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

pub fn spawn_end_game_menu(
    mut commands: Commands
) {
    commands.spawn((
        TextBundle::from_section(
            "You crashed! :(\nPress Enter to restart the game!",
            TextStyle {
                font_size: 40.,
                color: Color::RED,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(TOP_TEXT_VAL_PX),
            right: Val::Px(RIGHT_TEXT_VAL_PX - 240.),
            ..default()
        }),
        EndGameMenu,
    ));
}

pub fn erase_end_game_menu(
    mut commands: Commands,
    start_screen_query: Query<Entity, With<EndGameMenu>>
) {
    for start_screen_entity in &start_screen_query {
        commands.entity(start_screen_entity).despawn();
    }
}

pub fn spawn_scoreboard(
    mut commands: Commands
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 30.,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 30.,
                color: Color::GREEN,
                ..default()
            }),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(3.),
            right: Val::Px(350.),
            ..default()
        }),
        Scoreboard
    ));
}

pub fn update_scoreboard_score(
    scoreboard_score: Res<ScoreboardScore>,
    mut scoreboard_query: Query<&mut Text, With<Scoreboard>>
) {
    let mut scoreboard_text: Mut<Text> = scoreboard_query.single_mut();
    scoreboard_text.sections[1].value = scoreboard_score.score.to_string();
}
