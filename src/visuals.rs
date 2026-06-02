use bevy::prelude::*;

use crate::actions::{run_action, select_civet_by_index};
use crate::model::{
    Action, CivetClickTarget, GameScreen, GameState, Helicopter, PlantationRoom, SuspicionGlow,
    WorldActionTarget, WorldVisual,
};

pub fn spawn_world(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.54, 0.72, 0.48), Vec2::new(2400.0, 1000.0)),
        Transform::from_xyz(0.0, -80.0, -10.0),
    ));
    commands.spawn((
        Sprite::from_color(
            Color::srgba(1.0, 0.80, 0.68, 0.22),
            Vec2::new(2400.0, 260.0),
        ),
        Transform::from_xyz(0.0, 250.0, -9.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.70, 0.52, 0.32), Vec2::new(2400.0, 150.0)),
        Transform::from_xyz(0.0, -360.0, -8.0),
    ));
    for i in 0..8 {
        commands.spawn((
            Sprite::from_color(
                Color::srgba(0.24, 0.46, 0.22, 0.42),
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

    spawn_room_title(&mut commands, &state);
    match state.current_room {
        PlantationRoom::Sanctuary => spawn_sanctuary_room(&mut commands, &state),
        PlantationRoom::CoffeeField => spawn_coffee_field_room(&mut commands, &state),
        PlantationRoom::Roastery => spawn_roastery_room(&mut commands, &state),
        PlantationRoom::PaperworkOffice => spawn_paperwork_office_room(&mut commands, &state),
    }

    state.dirty_visuals = false;
}

fn spawn_room_title(commands: &mut Commands, state: &GameState) {
    let (title, subtitle, color) = match state.current_room {
        PlantationRoom::Sanctuary => (
            "SANCTUARY ROOM",
            "Soft paws, snack trays, and welfare optics.",
            Color::srgb(1.0, 0.76, 0.64),
        ),
        PlantationRoom::CoffeeField => (
            "COFFEE FIELD",
            "Fruit grows fast. So do questions.",
            Color::srgb(0.78, 1.0, 0.46),
        ),
        PlantationRoom::Roastery => (
            "ROASTERY",
            "Artisanal smoke with unfortunate silhouettes.",
            Color::srgb(1.0, 0.72, 0.38),
        ),
        PlantationRoom::PaperworkOffice => (
            "PAPERWORK OFFICE",
            "Receipts, permits, hoofprints, and strategic calm.",
            Color::srgb(0.78, 0.92, 1.0),
        ),
    };

    commands.spawn((
        Text2d::new(title),
        TextFont {
            font_size: 26.0,
            ..default()
        },
        TextColor(color),
        Transform::from_xyz(-160.0, 315.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Text2d::new(subtitle),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.94, 0.86, 0.62)),
        Transform::from_xyz(-160.0, 285.0, 2.0),
        WorldVisual,
    ));
}

fn spawn_coffee_field_room(commands: &mut Commands, state: &GameState) {
    let plant_count = state.coffee_plants.min(36);
    for i in 0..plant_count {
        let x = -250.0 + (i % 12) as f32 * 42.0;
        let y = -160.0 + (i / 12) as f32 * 48.0;
        commands.spawn((
            Sprite::from_color(Color::srgb(0.16, 0.25, 0.12), Vec2::new(28.0, 8.0)),
            Transform::from_xyz(x, y - 18.0, 0.9),
            WorldVisual,
        ));
        commands
            .spawn((
                Sprite::from_color(Color::srgb(0.05, 0.48, 0.19), Vec2::new(22.0, 32.0)),
                Transform::from_xyz(x, y, 1.0),
                Pickable::default(),
                WorldActionTarget(Action::HarvestFruit),
                WorldVisual,
            ))
            .observe(world_action_on_click)
            .observe(tint_sprite_on_hover(Color::srgb(0.10, 0.64, 0.25)))
            .observe(tint_sprite_on_out(Color::srgb(0.05, 0.48, 0.19)));
        commands.spawn((
            Sprite::from_color(Color::srgb(0.88, 0.12, 0.08), Vec2::new(7.0, 7.0)),
            Transform::from_xyz(x + 6.0, y + 6.0, 2.0),
            WorldVisual,
        ));
    }

    for i in 0..5 {
        let x = -460.0 + i as f32 * 92.0;
        commands
            .spawn((
                Sprite::from_color(Color::srgb(0.58, 0.38, 0.16), Vec2::new(54.0, 30.0)),
                Transform::from_xyz(x, 105.0 + (i % 2) as f32 * 22.0, 2.0),
                Pickable::default(),
                WorldActionTarget(Action::FeedCivets),
                WorldVisual,
            ))
            .observe(world_action_on_click)
            .observe(tint_sprite_on_hover(Color::srgb(0.72, 0.48, 0.20)))
            .observe(tint_sprite_on_out(Color::srgb(0.58, 0.38, 0.16)));
        commands.spawn((
            Text2d::new("fruit"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.78, 0.52)),
            Transform::from_xyz(x, 105.0 + (i % 2) as f32 * 22.0, 3.0),
            WorldVisual,
        ));
    }

    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.28, 0.42, 0.18), Vec2::new(110.0, 46.0)),
            Transform::from_xyz(410.0, 72.0, 2.0),
            Pickable::default(),
            WorldActionTarget(Action::PlantCoffee),
            WorldVisual,
        ))
        .observe(world_action_on_click)
        .observe(tint_sprite_on_hover(Color::srgb(0.36, 0.55, 0.22)))
        .observe(tint_sprite_on_out(Color::srgb(0.28, 0.42, 0.18)));
    commands.spawn((
        Text2d::new("seedlings"),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgb(0.92, 1.0, 0.72)),
        Transform::from_xyz(410.0, 72.0, 3.0),
        WorldVisual,
    ));

    commands.spawn((
        Text2d::new(format!("Coffee fruit on hand: {:.0}", state.coffee_fruit)),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 1.0, 0.74)),
        Transform::from_xyz(235.0, 125.0, 3.0),
        WorldVisual,
    ));

    if state.goat_present {
        spawn_goat(commands, 420.0, -95.0, "field goat?");
    }

    spawn_room_hint(
        commands,
        "Best buttons here: Plant coffee, Harvest fruit, Feed civets.",
    );
}

fn spawn_sanctuary_room(commands: &mut Commands, state: &GameState) {
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
        commands
            .spawn((
                Sprite::from_color(Color::srgb(0.34, 0.25, 0.18), Vec2::new(38.0, 20.0)),
                Transform::from_xyz(x, y, 3.0),
                Pickable::default(),
                CivetClickTarget { index: i as usize },
                WorldVisual,
            ))
            .observe(select_civet_on_click)
            .observe(tint_civet_on_hover(Color::srgb(0.44, 0.33, 0.24)))
            .observe(tint_civet_on_out(Color::srgb(0.34, 0.25, 0.18)));
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
        if let Some(profile) = state.civet_profiles.get(i as usize) {
            let selected = state.selected_civet == Some(i as usize);
            let label = if selected {
                format!("{}  mood {:.0}%", profile.name, profile.mood)
            } else {
                profile.name.clone()
            };
            commands.spawn((
                Text2d::new(label),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(if selected {
                    Color::srgb(1.0, 0.95, 0.46)
                } else {
                    Color::srgb(1.0, 0.86, 0.64)
                }),
                Transform::from_xyz(x, y - 21.0, 5.0),
                WorldVisual,
            ));
        }
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
        spawn_goat(commands, 190.0, 115.0, "goat?");
    }

    commands.spawn((
        Sprite::from_color(Color::srgb(0.95, 0.58, 0.48), Vec2::new(118.0, 58.0)),
        Transform::from_xyz(-285.0, -130.0, 2.0),
        WorldVisual,
    ));
    commands.spawn((
        Text2d::new("snack trays"),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgb(0.22, 0.11, 0.08)),
        Transform::from_xyz(-285.0, -130.0, 3.0),
        WorldVisual,
    ));

    spawn_room_hint(
        commands,
        "Click a civet to feed, pet, inspect notes, and build affection.",
    );
}

fn spawn_roastery_room(commands: &mut Commands, state: &GameState) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.22, 0.13, 0.08), Vec2::new(360.0, 145.0)),
        Transform::from_xyz(-175.0, -92.0, 2.0),
        WorldVisual,
    ));
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.56, 0.30, 0.12), Vec2::new(185.0, 78.0)),
            Transform::from_xyz(-175.0, -60.0, 3.0),
            Pickable::default(),
            WorldActionTarget(Action::RoastCoffee),
            WorldVisual,
        ))
        .observe(world_action_on_click)
        .observe(tint_sprite_on_hover(Color::srgb(0.70, 0.39, 0.16)))
        .observe(tint_sprite_on_out(Color::srgb(0.56, 0.30, 0.12)));
    commands.spawn((
        Text2d::new("ROASTER"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.78, 0.45)),
        Transform::from_xyz(-175.0, -60.0, 4.0),
        WorldVisual,
    ));
    for i in 0..6 {
        let x = 140.0 + (i % 3) as f32 * 76.0;
        let y = -140.0 + (i / 3) as f32 * 74.0;
        commands
            .spawn((
                Sprite::from_color(Color::srgb(0.58, 0.39, 0.18), Vec2::new(62.0, 48.0)),
                Transform::from_xyz(x, y, 2.0),
                Pickable::default(),
                WorldActionTarget(Action::SellCoffee),
                WorldVisual,
            ))
            .observe(world_action_on_click)
            .observe(tint_sprite_on_hover(Color::srgb(0.72, 0.49, 0.24)))
            .observe(tint_sprite_on_out(Color::srgb(0.58, 0.39, 0.18)));
        commands.spawn((
            Text2d::new("coffee"),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::srgb(0.22, 0.12, 0.05)),
            Transform::from_xyz(x, y, 3.0),
            WorldVisual,
        ));
    }
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.20, 0.12, 0.07), Vec2::new(116.0, 54.0)),
            Transform::from_xyz(-390.0, 45.0, 2.0),
            Pickable::default(),
            WorldActionTarget(Action::CollectBeans),
            WorldVisual,
        ))
        .observe(world_action_on_click)
        .observe(tint_sprite_on_hover(Color::srgb(0.28, 0.16, 0.09)))
        .observe(tint_sprite_on_out(Color::srgb(0.20, 0.12, 0.07)));
    commands.spawn((
        Text2d::new("bean crate"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.78, 0.54)),
        Transform::from_xyz(-390.0, 45.0, 3.0),
        WorldVisual,
    ));

    commands.spawn((
        Text2d::new(format!(
            "Processed beans {:.1}  |  Roasted bags {:.1}",
            state.processed_beans, state.roasted_coffee
        )),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.84, 0.58)),
        Transform::from_xyz(100.0, 95.0, 3.0),
        WorldVisual,
    ));

    spawn_room_hint(
        commands,
        "Best buttons here: Collect beans, Roast coffee, Sell coffee.",
    );
}

fn spawn_paperwork_office_room(commands: &mut Commands, state: &GameState) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.15, 0.18, 0.18), Vec2::new(620.0, 245.0)),
        Transform::from_xyz(25.0, -45.0, 1.5),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.45, 0.32, 0.18), Vec2::new(410.0, 92.0)),
        Transform::from_xyz(-120.0, -160.0, 2.0),
        WorldVisual,
    ));
    for i in 0..7 {
        let x = -290.0 + i as f32 * 54.0;
        commands
            .spawn((
                Sprite::from_color(Color::srgb(0.92, 0.86, 0.70), Vec2::new(34.0, 44.0)),
                Transform::from_xyz(x, -135.0 + (i % 2) as f32 * 10.0, 3.0),
                Pickable::default(),
                WorldActionTarget(Action::ShowPaperwork),
                WorldVisual,
            ))
            .observe(world_action_on_click)
            .observe(tint_sprite_on_hover(Color::srgb(1.0, 0.96, 0.78)))
            .observe(tint_sprite_on_out(Color::srgb(0.92, 0.86, 0.70)));
    }
    commands.spawn((
        Text2d::new(format!(
            "Paperwork level {}  |  Suspicion {:.0}%",
            state.paperwork_level, state.suspicion
        )),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.82, 0.96, 1.0)),
        Transform::from_xyz(-90.0, 15.0, 3.0),
        WorldVisual,
    ));

    spawn_upgrade_buildings(commands, state);
    spawn_helicopter(commands);
    if state.goat_present {
        spawn_goat(commands, 360.0, -150.0, "witness");
    }

    spawn_room_hint(
        commands,
        "Best buttons here: Show paperwork, build office upgrades, stay calm.",
    );
}

fn spawn_helicopter(commands: &mut Commands) {
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
}

fn spawn_goat(commands: &mut Commands, x: f32, y: f32, label: &str) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.93, 0.89, 0.78), Vec2::new(42.0, 28.0)),
        Transform::from_xyz(x, y, 3.0),
        WorldVisual,
    ));
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.17, 0.12, 0.08)),
        Transform::from_xyz(x, y + 1.0, 4.0),
        WorldVisual,
    ));
}

fn spawn_room_hint(commands: &mut Commands, text: &str) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.90, 0.66)),
        Transform::from_xyz(-20.0, -315.0, 3.0),
        WorldVisual,
    ));
}

fn select_civet_on_click(
    click: On<Pointer<Click>>,
    targets: Query<&CivetClickTarget>,
    mut state: ResMut<GameState>,
) {
    if state.screen != GameScreen::Playing
        || state.inspection
        || state.event.is_some()
        || state.pending_order.is_some()
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
        return;
    }

    let Ok(target) = targets.get(click.event_target()) else {
        return;
    };
    select_civet_by_index(&mut state, target.index);
    state.dirty_visuals = true;
}

fn world_action_on_click(
    click: On<Pointer<Click>>,
    targets: Query<&WorldActionTarget>,
    mut state: ResMut<GameState>,
) {
    if state.screen != GameScreen::Playing
        || state.inspection
        || state.event.is_some()
        || state.pending_order.is_some()
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
        return;
    }

    let Ok(target) = targets.get(click.event_target()) else {
        return;
    };
    run_action(&mut state, target.0);
}

fn tint_sprite_on_hover(color: Color) -> impl Fn(On<Pointer<Over>>, Query<&mut Sprite>) {
    move |event, mut sprites| {
        if let Ok(mut sprite) = sprites.get_mut(event.event_target()) {
            sprite.color = color;
        }
    }
}

fn tint_sprite_on_out(color: Color) -> impl Fn(On<Pointer<Out>>, Query<&mut Sprite>) {
    move |event, mut sprites| {
        if let Ok(mut sprite) = sprites.get_mut(event.event_target()) {
            sprite.color = color;
        }
    }
}

fn tint_civet_on_hover(color: Color) -> impl Fn(On<Pointer<Over>>, Query<&mut Sprite>) {
    move |event, mut sprites| {
        if let Ok(mut sprite) = sprites.get_mut(event.event_target()) {
            sprite.color = color;
        }
    }
}

fn tint_civet_on_out(color: Color) -> impl Fn(On<Pointer<Out>>, Query<&mut Sprite>) {
    move |event, mut sprites| {
        if let Ok(mut sprite) = sprites.get_mut(event.event_target()) {
            sprite.color = color;
        }
    }
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
