use bevy::prelude::*;

use crate::model::{GameState, Helicopter, SuspicionGlow, WorldVisual};

pub fn spawn_world(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.12, 0.32, 0.20), Vec2::new(2400.0, 1000.0)),
        Transform::from_xyz(0.0, -80.0, -10.0),
    ));
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.88, 0.63, 0.25, 0.18),
            Vec2::new(2400.0, 260.0),
        ),
        Transform::from_xyz(0.0, 250.0, -9.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.42, 0.28, 0.12), Vec2::new(2400.0, 150.0)),
        Transform::from_xyz(0.0, -360.0, -8.0),
    ));
    for i in 0..8 {
        commands.spawn((
            Sprite::from_color(
                Color::srgba(0.07, 0.20, 0.10, 0.42),
                Vec2::new(2400.0, 18.0),
            ),
            Transform::from_xyz(0.0, -245.0 + i as f32 * 54.0, -7.0),
        ));
    }
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.76, 0.05, 0.04, 0.0),
            Vec2::new(2200.0, 900.0),
        ),
        Transform::from_xyz(0.0, -55.0, 8.0),
        SuspicionGlow,
    ));
    commands.spawn((
        Text2d::new("EUTHERCIVET COFFEE ESTATE"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.84, 0.42)),
        Transform::from_xyz(-160.0, 315.0, 2.0),
    ));
    commands.spawn((
        Text2d::new("Fair-trade sanctuary. Legally boring. Visually indefensible."),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.94, 0.86, 0.62)),
        Transform::from_xyz(-160.0, 285.0, 2.0),
    ));
}

pub fn animate_world(
    time: Res<Time>,
    state: Res<GameState>,
    mut helicopters: Query<(&Helicopter, &mut Transform)>,
    mut glows: Query<&mut Sprite, With<SuspicionGlow>>,
) {
    let t = time.elapsed_secs();
    let base = Vec3::new(
        315.0 + (t * 0.9).sin() * 42.0,
        240.0 + (t * 1.7).cos() * 7.0,
        3.0,
    );
    for (helicopter, mut transform) in &mut helicopters {
        transform.translation = base + helicopter.offset;
    }

    let pulse = 0.5 + 0.5 * (t * 4.0).sin();
    let alpha = if state.inspection {
        0.30 + pulse * 0.18
    } else {
        (state.suspicion / 100.0) * (0.04 + pulse * 0.10)
    };
    for mut sprite in &mut glows {
        sprite.color = Color::srgba(0.90, 0.03, 0.02, alpha);
    }
}

pub fn refresh_world_visuals(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    visuals: Query<Entity, With<WorldVisual>>,
) {
    if !state.dirty_visuals {
        return;
    }
    for entity in &visuals {
        commands.entity(entity).despawn();
    }

    let plant_count = state.coffee_plants.min(36);
    for i in 0..plant_count {
        let x = -250.0 + (i % 12) as f32 * 42.0;
        let y = -160.0 + (i / 12) as f32 * 48.0;
        commands.spawn((
            Sprite::from_color(Color::srgb(0.16, 0.25, 0.12), Vec2::new(28.0, 8.0)),
            Transform::from_xyz(x, y - 18.0, 0.9),
            WorldVisual,
        ));
        commands.spawn((
            Sprite::from_color(Color::srgb(0.05, 0.48, 0.19), Vec2::new(22.0, 32.0)),
            Transform::from_xyz(x, y, 1.0),
            WorldVisual,
        ));
        commands.spawn((
            Sprite::from_color(Color::srgb(0.88, 0.12, 0.08), Vec2::new(7.0, 7.0)),
            Transform::from_xyz(x + 6.0, y + 6.0, 2.0),
            WorldVisual,
        ));
    }

    commands.spawn((
        Sprite::from_color(Color::srgb(0.67, 0.44, 0.22), Vec2::new(320.0, 8.0)),
        Transform::from_xyz(430.0, -170.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.67, 0.44, 0.22), Vec2::new(320.0, 8.0)),
        Transform::from_xyz(430.0, 2.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.67, 0.44, 0.22), Vec2::new(8.0, 180.0)),
        Transform::from_xyz(275.0, -84.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.67, 0.44, 0.22), Vec2::new(8.0, 180.0)),
        Transform::from_xyz(585.0, -84.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Text2d::new("CIVET ENCLOSURE"),
        TextFont {
            font_size: 17.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.82, 0.44)),
        Transform::from_xyz(430.0, 20.0, 4.0),
        WorldVisual,
    ));

    for i in 0..state.civets.min(10) {
        let x = 330.0 + (i % 5) as f32 * 48.0;
        let y = -120.0 + (i / 5) as f32 * 44.0;
        commands.spawn((
            Sprite::from_color(Color::srgb(0.34, 0.25, 0.18), Vec2::new(38.0, 20.0)),
            Transform::from_xyz(x, y, 3.0),
            WorldVisual,
        ));
        commands.spawn((
            Sprite::from_color(Color::srgb(0.90, 0.78, 0.56), Vec2::new(11.0, 11.0)),
            Transform::from_xyz(x + 18.0, y + 4.0, 4.0),
            WorldVisual,
        ));
        commands.spawn((
            Sprite::from_color(Color::srgb(0.12, 0.08, 0.05), Vec2::new(8.0, 8.0)),
            Transform::from_xyz(x + 27.0, y + 8.0, 5.0),
            WorldVisual,
        ));
    }

    if state.binturong_home {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.08, 0.07, 0.06), Vec2::new(58.0, 26.0)),
            Transform::from_xyz(455.0, 10.0, 3.0),
            WorldVisual,
        ));
        commands.spawn((
            Text2d::new("binturong"),
            TextFont {
                font_size: 13.0,
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.87, 0.68)),
            Transform::from_xyz(455.0, 38.0, 4.0),
            WorldVisual,
        ));
    }

    if state.goat_present {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.93, 0.89, 0.78), Vec2::new(42.0, 28.0)),
            Transform::from_xyz(190.0, 115.0, 3.0),
            WorldVisual,
        ));
        commands.spawn((
            Text2d::new("goat?"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.17, 0.12, 0.08)),
            Transform::from_xyz(190.0, 116.0, 4.0),
            WorldVisual,
        ));
    }

    commands.spawn((
        Sprite::from_color(Color::srgb(0.58, 0.39, 0.18), Vec2::new(84.0, 62.0)),
        Transform::from_xyz(-420.0, 145.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.70, 0.49, 0.24), Vec2::new(72.0, 50.0)),
        Transform::from_xyz(-345.0, 130.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.46, 0.28, 0.10), Vec2::new(110.0, 14.0)),
        Transform::from_xyz(-385.0, 100.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Text2d::new("ROASTED COFFEE"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.20, 0.12, 0.05)),
        Transform::from_xyz(-420.0, 145.0, 3.0),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.12, 0.13, 0.16), Vec2::new(86.0, 22.0)),
        Transform::from_xyz(325.0, 245.0, 3.0),
        WorldVisual,
        Helicopter {
            offset: Vec3::new(0.0, 0.0, 0.0),
        },
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.75, 0.08, 0.07), Vec2::new(125.0, 5.0)),
        Transform::from_xyz(325.0, 260.0, 4.0),
        WorldVisual,
        Helicopter {
            offset: Vec3::new(0.0, 15.0, 1.0),
        },
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.75, 0.08, 0.07), Vec2::new(5.0, 58.0)),
        Transform::from_xyz(372.0, 245.0, 4.0),
        WorldVisual,
        Helicopter {
            offset: Vec3::new(47.0, 0.0, 1.0),
        },
    ));
    commands.spawn((
        Text2d::new("police helicopter"),
        TextFont {
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.76, 0.66)),
        Transform::from_xyz(325.0, 220.0, 4.0),
        WorldVisual,
    ));

    spawn_upgrade_buildings(&mut commands, &state);

    state.dirty_visuals = false;
}

fn spawn_upgrade_buildings(commands: &mut Commands, state: &GameState) {
    let buildings = [
        (
            state.legal_office,
            -545.0,
            -255.0,
            "LEGAL",
            Color::srgb(0.16, 0.28, 0.32),
        ),
        (
            state.caretaker,
            -455.0,
            -255.0,
            "CARE",
            Color::srgb(0.24, 0.35, 0.18),
        ),
        (
            state.fruit_sorter,
            -365.0,
            -255.0,
            "SORT",
            Color::srgb(0.42, 0.31, 0.10),
        ),
        (
            state.roasting_shed,
            -275.0,
            -255.0,
            "ROAST",
            Color::srgb(0.33, 0.19, 0.11),
        ),
        (
            state.tasting_room,
            -185.0,
            -255.0,
            "TASTE",
            Color::srgb(0.38, 0.24, 0.30),
        ),
    ];

    for (enabled, x, y, label, color) in buildings {
        if !enabled {
            continue;
        }
        commands.spawn((
            Sprite::from_color(color, Vec2::new(72.0, 48.0)),
            Transform::from_xyz(x, y, 2.0),
            WorldVisual,
        ));
        commands.spawn((
            Sprite::from_color(Color::srgb(0.74, 0.57, 0.30), Vec2::new(82.0, 10.0)),
            Transform::from_xyz(x, y + 29.0, 3.0),
            WorldVisual,
        ));
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.90, 0.65)),
            Transform::from_xyz(x, y, 4.0),
            WorldVisual,
        ));
    }
}
