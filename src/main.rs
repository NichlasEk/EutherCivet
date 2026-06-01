use bevy::prelude::*;

mod actions;
mod model;
mod simulation;
mod ui;
mod visuals;

use model::*;
use simulation::{advance_day, tick_game, trigger_random_events};
use ui::{
    handle_buttons, refresh_day_modal, refresh_inspection_modal, spawn_ui, update_log,
    update_stats, update_status_bars,
};
use visuals::{animate_world, refresh_world_visuals, spawn_world};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.39, 0.32, 0.19)))
        .insert_resource(GameState::load().unwrap_or_default())
        .insert_resource(GameTick(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(EventTick(Timer::from_seconds(9.0, TimerMode::Repeating)))
        .insert_resource(DayTick(Timer::from_seconds(45.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "EutherCivet".to_string(),
                resolution: (1440, 820).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_buttons,
                tick_game,
                trigger_random_events,
                advance_day,
                update_stats,
                update_status_bars,
                update_log,
                animate_world,
                refresh_world_visuals,
                refresh_inspection_modal,
                refresh_day_modal,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_world(&mut commands);
    spawn_ui(&mut commands);
}
