use bevy::prelude::*;

use crate::actions::{run_action, select_civet_by_index};
use crate::model::{
    Action, BackgroundAssets, CharacterAssets, CivetClickTarget, EnvironmentBackdrop, GameScreen,
    GameState, Helicopter, MovingCivet, ParallaxLayer, PlantationRoom, PlayerAvatar, PlayerLabel,
    PlayerShadow, PropAssets, RetroSkyBand, SuspicionGlow, UiSkinAssets, WorldActionTarget,
    WorldVisual,
};

const SKIN_STATS_PANEL: usize = 0;
const SKIN_TOOL_PANEL: usize = 1;
const SKIN_BUTTON: usize = 3;

pub fn spawn_world(commands: &mut Commands, backgrounds: &BackgroundAssets) {
    spawn_retro_sky(commands);

    for phase in 0..4 {
        commands.spawn((
            Sprite::from_atlas_image(
                backgrounds.texture.clone(),
                TextureAtlas {
                    layout: backgrounds.atlas.clone(),
                    index: phase,
                },
            ),
            Transform::from_xyz(0.0, -25.0, -34.0).with_scale(Vec3::splat(2.38)),
            EnvironmentBackdrop { phase },
        ));
    }

    for (x, y, speed, amplitude, alpha) in [
        (-470.0, 250.0, 5.5, 36.0, 0.20),
        (-70.0, 285.0, 3.7, 24.0, 0.16),
        (390.0, 232.0, 4.6, 31.0, 0.18),
    ] {
        spawn_cloud(commands, x, y, speed, amplitude, alpha);
    }

    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.12, 0.22, 0.10, 0.24),
            Vec2::new(2400.0, 90.0),
        ),
        Transform::from_xyz(0.0, -332.0, -6.0),
        ParallaxLayer {
            base: Vec3::new(0.0, -332.0, -6.0),
            speed: 1.1,
            amplitude: 18.0,
        },
    ));
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.78, 0.50, 0.23, 0.50),
            Vec2::new(2400.0, 150.0),
        ),
        Transform::from_xyz(0.0, -365.0, -5.0),
    ));
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.76, 0.05, 0.04, 0.0),
            Vec2::new(2200.0, 900.0),
        ),
        Transform::from_xyz(0.0, -55.0, 8.0),
        SuspicionGlow,
    ));
}

fn spawn_cloud(commands: &mut Commands, x: f32, y: f32, speed: f32, amplitude: f32, alpha: f32) {
    for (dx, width, height) in [(-48.0, 92.0, 30.0), (0.0, 128.0, 40.0), (58.0, 86.0, 28.0)] {
        let base = Vec3::new(x + dx, y, -14.0);
        commands.spawn((
            Sprite::from_color(
                Color::srgba(1.0, 0.93, 0.76, alpha),
                Vec2::new(width, height),
            ),
            Transform::from_translation(base),
            ParallaxLayer {
                base,
                speed,
                amplitude,
            },
        ));
    }
}

fn spawn_retro_sky(commands: &mut Commands) {
    spawn_sky_panel(
        commands,
        0.0,
        238.0,
        1480.0,
        370.0,
        Color::srgba(0.08, 0.22, 0.58, 0.42),
        -33.4,
    );
    for (y, height, color) in [
        (356.0, 24.0, Color::srgba(0.12, 0.38, 0.90, 0.24)),
        (304.0, 18.0, Color::srgba(0.46, 0.78, 1.00, 0.16)),
        (246.0, 16.0, Color::srgba(1.00, 0.58, 0.30, 0.12)),
        (193.0, 12.0, Color::srgba(0.99, 0.86, 0.42, 0.09)),
    ] {
        spawn_sky_panel(commands, 0.0, y, 1480.0, height, color, -33.0);
    }

    for (layer, y, speed, color, height) in [
        (0, 266.0, 7.0, Color::srgba(0.08, 0.11, 0.26, 0.22), 78.0),
        (1, 219.0, 12.0, Color::srgba(0.07, 0.18, 0.23, 0.18), 58.0),
        (2, 174.0, 19.0, Color::srgba(0.12, 0.28, 0.18, 0.15), 46.0),
    ] {
        for tile in -1..=1 {
            spawn_mountain_ridge(
                commands,
                tile as f32 * 620.0,
                y,
                height,
                color,
                -32.5 + layer as f32 * 0.3,
                speed,
                layer as f32 * 0.7,
            );
        }
    }

    for (x, y, speed, scale, alpha) in [
        (-520.0, 324.0, 18.0, 1.0, 0.14),
        (-80.0, 349.0, 13.0, 1.25, 0.11),
        (430.0, 294.0, 23.0, 0.88, 0.13),
    ] {
        spawn_sky_wisp(commands, x, y, speed, scale, alpha);
    }

    for y in [169.0, 181.0, 197.0] {
        spawn_sky_wisp(commands, -120.0, y, -7.0, 1.7, 0.045);
    }
}

fn spawn_sky_panel(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    z: f32,
) {
    commands.spawn((
        Sprite::from_color(color, Vec2::new(width, height)),
        Transform::from_xyz(x, y, z),
    ));
}

fn spawn_mountain_ridge(
    commands: &mut Commands,
    x: f32,
    y: f32,
    height: f32,
    color: Color,
    z: f32,
    speed: f32,
    phase: f32,
) {
    for (i, peak) in [0.50, 0.76, 0.58, 0.84, 0.62, 0.70, 0.52]
        .iter()
        .enumerate()
    {
        let width = 108.0 + i as f32 * 7.0;
        let x = x - 318.0 + i as f32 * 96.0;
        let y = y - height * (1.0 - peak) * 0.40;
        commands.spawn((
            Sprite::from_color(color, Vec2::new(width, height * peak)),
            Transform::from_xyz(x, y, z).with_rotation(Quat::from_rotation_z(0.70)),
            RetroSkyBand {
                base: Vec3::new(x, y, z),
                speed,
                wave: phase + i as f32 * 0.17,
                wrap_width: 1860.0,
            },
        ));
    }
}

fn spawn_sky_wisp(commands: &mut Commands, x: f32, y: f32, speed: f32, scale: f32, alpha: f32) {
    for (dx, dy, w, h) in [
        (-92.0, 0.0, 122.0, 8.0),
        (-18.0, 10.0, 168.0, 10.0),
        (86.0, 2.0, 116.0, 7.0),
        (18.0, -9.0, 198.0, 6.0),
    ] {
        let base = Vec3::new(x + dx * scale, y + dy * scale, -30.8);
        commands.spawn((
            Sprite::from_color(
                Color::srgba(1.0, 0.90, 0.72, alpha),
                Vec2::new(w * scale, h * scale),
            ),
            Transform::from_translation(base).with_rotation(Quat::from_rotation_z(-0.015)),
            RetroSkyBand {
                base,
                speed,
                wave: dx * 0.01,
                wrap_width: 1600.0,
            },
        ));
    }
}

pub fn animate_world(
    time: Res<Time>,
    state: Res<GameState>,
    mut backdrops: Query<
        (&EnvironmentBackdrop, &mut Sprite),
        (Without<SuspicionGlow>, Without<PlayerAvatar>),
    >,
    mut parallax: Query<
        (&ParallaxLayer, &mut Transform),
        (
            Without<Helicopter>,
            Without<MovingCivet>,
            Without<PlayerAvatar>,
            Without<PlayerShadow>,
            Without<EnvironmentBackdrop>,
        ),
    >,
    mut sky: Query<
        (&RetroSkyBand, &mut Transform),
        (
            Without<ParallaxLayer>,
            Without<Helicopter>,
            Without<MovingCivet>,
            Without<PlayerAvatar>,
            Without<PlayerShadow>,
        ),
    >,
    mut helicopters: Query<
        (&Helicopter, &mut Transform),
        (
            Without<PlayerAvatar>,
            Without<PlayerShadow>,
            Without<MovingCivet>,
            Without<ParallaxLayer>,
        ),
    >,
    mut civets: Query<
        (&MovingCivet, &mut Transform),
        (
            Without<Helicopter>,
            Without<PlayerAvatar>,
            Without<PlayerShadow>,
            Without<ParallaxLayer>,
        ),
    >,
    mut players: Query<
        (&PlayerAvatar, &mut Transform),
        (
            Without<PlayerShadow>,
            Without<SuspicionGlow>,
            Without<EnvironmentBackdrop>,
            Without<Helicopter>,
            Without<MovingCivet>,
            Without<ParallaxLayer>,
        ),
    >,
    mut player_shadows: Query<
        &mut Transform,
        (
            With<PlayerShadow>,
            Without<PlayerAvatar>,
            Without<Helicopter>,
            Without<MovingCivet>,
            Without<ParallaxLayer>,
        ),
    >,
    mut glows: Query<
        &mut Sprite,
        (
            With<SuspicionGlow>,
            Without<PlayerAvatar>,
            Without<EnvironmentBackdrop>,
        ),
    >,
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

    let cycle = (t / 96.0).fract();
    for (backdrop, mut sprite) in &mut backdrops {
        let alpha = backdrop_alpha(cycle, backdrop.phase);
        sprite.color = Color::srgba(1.0, 1.0, 1.0, alpha);
    }

    for (layer, mut transform) in &mut parallax {
        let drift = (t * layer.speed * 0.04).sin() * layer.amplitude;
        transform.translation.x = layer.base.x + drift;
        transform.translation.y = layer.base.y + (t * layer.speed * 0.025).cos() * 3.0;
    }

    for (band, mut transform) in &mut sky {
        let offset = (t * band.speed + band.wave * 79.0).rem_euclid(band.wrap_width);
        transform.translation.x = band.base.x + offset - band.wrap_width * 0.5;
        transform.translation.y = band.base.y + (t * 0.55 + band.wave).sin() * 2.5;
    }

    for (civet, mut transform) in &mut civets {
        let walk = (t * 1.35 + civet.phase).sin();
        let bob = (t * 2.7 + civet.phase).cos();
        transform.translation.x = civet.base.x + walk * 18.0;
        transform.translation.y = civet.base.y + bob * 4.0;
        transform.translation.z = ground_z(transform.translation.y) + 0.2;
        transform.rotation = Quat::from_rotation_z(walk * 0.035);
    }

    for (avatar, mut transform) in &mut players {
        let stride = (t * 10.0).sin();
        let lift = if avatar.moving {
            stride.abs() * 7.0
        } else {
            0.0
        };
        let squash = if avatar.moving {
            stride.abs() * 0.015
        } else {
            0.0
        };
        transform.translation.y = state.player_y + lift;
        transform.translation.z = ground_z(state.player_y) + 0.8;
        transform.rotation =
            Quat::from_rotation_z(if avatar.moving { stride * 0.025 } else { 0.0 });
        transform.scale = Vec3::new(0.26 * avatar.facing, 0.26 + squash, 0.26);
    }

    for mut transform in &mut player_shadows {
        transform.translation.x = state.player_x;
        transform.translation.y = state.player_y - 68.0;
        transform.translation.z = ground_z(state.player_y) - 0.3;
        let width = if state.player_y < -185.0 { 1.10 } else { 0.96 };
        transform.scale = Vec3::new(width, 0.78, 1.0);
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

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut players: Query<
        (&mut Transform, &mut PlayerAvatar),
        (Without<PlayerLabel>, Without<PlayerShadow>),
    >,
    mut labels: Query<
        &mut Transform,
        (
            With<PlayerLabel>,
            Without<PlayerAvatar>,
            Without<PlayerShadow>,
        ),
    >,
    mut shadows: Query<
        &mut Transform,
        (
            With<PlayerShadow>,
            Without<PlayerAvatar>,
            Without<PlayerLabel>,
        ),
    >,
) {
    if state.screen != GameScreen::Playing
        || state.inspection
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
        for (_, mut avatar) in &mut players {
            avatar.moving = false;
        }
        return;
    }

    let mut delta = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
        delta.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
        delta.y -= 1.0;
    }

    if delta == Vec2::ZERO {
        for (_, mut avatar) in &mut players {
            avatar.moving = false;
        }
        return;
    }

    let speed = 205.0;
    let step = delta.normalize() * speed * time.delta_secs();
    state.player_x += step.x;
    state.player_y += step.y;

    if state.player_x < -525.0 {
        let next_room = exit_left(state.current_room);
        walk_to_room(&mut state, next_room, 500.0);
    } else if state.player_x > 525.0 {
        let next_room = exit_right(state.current_room);
        walk_to_room(&mut state, next_room, -500.0);
    } else {
        state.player_x = state.player_x.clamp(-525.0, 525.0);
    }
    let floor = walkable_floor(state.current_room);
    state.player_y = state.player_y.clamp(floor.min_y, floor.max_y);
    state.player_x = state.player_x.clamp(floor.min_x, floor.max_x);

    for (mut transform, mut avatar) in &mut players {
        transform.translation.x = state.player_x;
        transform.translation.y = state.player_y;
        avatar.facing = if step.x < -0.1 {
            1.0
        } else if step.x > 0.1 {
            -1.0
        } else {
            avatar.facing
        };
        avatar.moving = true;
    }
    for mut transform in &mut labels {
        transform.translation.x = state.player_x;
        transform.translation.y = state.player_y - 94.0;
    }
    for mut transform in &mut shadows {
        transform.translation.x = state.player_x;
        transform.translation.y = state.player_y - 68.0;
    }
}

fn walk_to_room(state: &mut GameState, room: PlantationRoom, player_x: f32) {
    if state.current_room == room {
        state.player_x = player_x;
        return;
    }
    state.current_room = room;
    state.player_x = player_x;
    state.player_y = -145.0;
    state.selected_civet = None;
    state.dirty_visuals = true;
    state.log_line(format!("Walked to {}.", room_name(room)));
}

fn exit_left(room: PlantationRoom) -> PlantationRoom {
    match room {
        PlantationRoom::Sanctuary => PlantationRoom::PaperworkOffice,
        PlantationRoom::CoffeeField => PlantationRoom::Sanctuary,
        PlantationRoom::Roastery => PlantationRoom::CoffeeField,
        PlantationRoom::PaperworkOffice => PlantationRoom::Roastery,
    }
}

fn exit_right(room: PlantationRoom) -> PlantationRoom {
    match room {
        PlantationRoom::Sanctuary => PlantationRoom::CoffeeField,
        PlantationRoom::CoffeeField => PlantationRoom::Roastery,
        PlantationRoom::Roastery => PlantationRoom::PaperworkOffice,
        PlantationRoom::PaperworkOffice => PlantationRoom::Sanctuary,
    }
}

fn room_name(room: PlantationRoom) -> &'static str {
    match room {
        PlantationRoom::Sanctuary => "Sanctuary",
        PlantationRoom::CoffeeField => "Coffee Field",
        PlantationRoom::Roastery => "Roastery",
        PlantationRoom::PaperworkOffice => "Paperwork Office",
    }
}

struct WalkableFloor {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

fn walkable_floor(room: PlantationRoom) -> WalkableFloor {
    match room {
        PlantationRoom::Sanctuary => WalkableFloor {
            min_x: -525.0,
            max_x: 525.0,
            min_y: -268.0,
            max_y: 48.0,
        },
        PlantationRoom::CoffeeField => WalkableFloor {
            min_x: -525.0,
            max_x: 525.0,
            min_y: -272.0,
            max_y: 128.0,
        },
        PlantationRoom::Roastery => WalkableFloor {
            min_x: -525.0,
            max_x: 525.0,
            min_y: -270.0,
            max_y: 88.0,
        },
        PlantationRoom::PaperworkOffice => WalkableFloor {
            min_x: -525.0,
            max_x: 525.0,
            min_y: -270.0,
            max_y: 58.0,
        },
    }
}

fn ground_z(y: f32) -> f32 {
    3.0 + (-y + 280.0) * 0.006
}

fn backdrop_alpha(cycle: f32, phase: usize) -> f32 {
    let centers = [0.08, 0.34, 0.62, 0.86];
    let mut weights = [0.0; 4];
    for (index, center) in centers.iter().enumerate() {
        let distance = circular_distance(cycle, *center);
        let weight = (1.0_f32 - distance / 0.28).clamp(0.0, 1.0);
        weights[index] = weight * weight;
    }
    let total: f32 = weights.iter().sum();
    if total <= 0.0 {
        if phase == 1 { 1.0 } else { 0.0 }
    } else {
        weights[phase] / total
    }
}

fn circular_distance(a: f32, b: f32) -> f32 {
    let distance = (a - b).abs();
    distance.min(1.0 - distance)
}

pub fn refresh_world_visuals(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    characters: Res<CharacterAssets>,
    props: Res<PropAssets>,
    skin: Res<UiSkinAssets>,
    visuals: Query<Entity, With<WorldVisual>>,
) {
    if !state.dirty_visuals {
        return;
    }
    for entity in &visuals {
        commands.entity(entity).despawn();
    }

    spawn_room_title(&mut commands, &state);
    spawn_walkable_floor(&mut commands, state.current_room);
    spawn_room_exits(&mut commands, &state, &skin);
    spawn_player(&mut commands, &characters, &state);
    match state.current_room {
        PlantationRoom::Sanctuary => {
            spawn_sanctuary_room(&mut commands, &state, &characters, &props, &skin)
        }
        PlantationRoom::CoffeeField => spawn_coffee_field_room(&mut commands, &state, &props),
        PlantationRoom::Roastery => spawn_roastery_room(&mut commands, &state, &props, &skin),
        PlantationRoom::PaperworkOffice => {
            spawn_paperwork_office_room(&mut commands, &state, &props, &skin)
        }
    }

    state.dirty_visuals = false;
}

fn prop_sprite(props: &PropAssets, index: usize) -> Sprite {
    Sprite::from_atlas_image(
        props.texture.clone(),
        TextureAtlas {
            layout: props.atlas.clone(),
            index,
        },
    )
}

fn skin_sprite(skin: &UiSkinAssets, index: usize, color: Color) -> Sprite {
    let mut sprite = Sprite::from_atlas_image(
        skin.texture.clone(),
        TextureAtlas {
            layout: skin.atlas.clone(),
            index,
        },
    );
    sprite.color = color;
    sprite
}

fn spawn_player(commands: &mut Commands, characters: &CharacterAssets, state: &GameState) {
    commands.spawn((
        Sprite::from_color(Color::srgba(0.05, 0.035, 0.02, 0.26), Vec2::new(86.0, 28.0)),
        Transform::from_xyz(state.player_x, state.player_y - 68.0, 4.0),
        PlayerShadow,
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_atlas_image(
            characters.texture.clone(),
            TextureAtlas {
                layout: characters.atlas.clone(),
                index: 3,
            },
        ),
        Transform::from_xyz(state.player_x, state.player_y, 5.0).with_scale(Vec3::splat(0.26)),
        PlayerAvatar {
            facing: 1.0,
            moving: false,
        },
        WorldVisual,
    ));
    commands.spawn((
        Text2d::new("plantation owner"),
        TextFont {
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.88, 0.62)),
        Transform::from_xyz(state.player_x, state.player_y - 94.0, 6.0),
        PlayerLabel,
        WorldVisual,
    ));
}

fn spawn_walkable_floor(commands: &mut Commands, room: PlantationRoom) {
    let (main, edge, path, y, h) = match room {
        PlantationRoom::Sanctuary => (
            Color::srgba(0.47, 0.33, 0.18, 0.72),
            Color::srgba(0.23, 0.14, 0.07, 0.48),
            Color::srgba(0.72, 0.47, 0.24, 0.34),
            -150.0,
            305.0,
        ),
        PlantationRoom::CoffeeField => (
            Color::srgba(0.31, 0.52, 0.21, 0.56),
            Color::srgba(0.13, 0.24, 0.10, 0.36),
            Color::srgba(0.77, 0.52, 0.25, 0.28),
            -145.0,
            355.0,
        ),
        PlantationRoom::Roastery => (
            Color::srgba(0.38, 0.22, 0.13, 0.64),
            Color::srgba(0.17, 0.09, 0.05, 0.44),
            Color::srgba(0.75, 0.49, 0.26, 0.23),
            -153.0,
            270.0,
        ),
        PlantationRoom::PaperworkOffice => (
            Color::srgba(0.39, 0.27, 0.17, 0.62),
            Color::srgba(0.17, 0.10, 0.06, 0.42),
            Color::srgba(0.89, 0.70, 0.44, 0.22),
            -150.0,
            285.0,
        ),
    };

    commands.spawn((
        Sprite::from_color(main, Vec2::new(1180.0, h)),
        Transform::from_xyz(40.0, y, -4.2),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(edge, Vec2::new(1180.0, 20.0)),
        Transform::from_xyz(40.0, y + h * 0.5, -3.8),
        WorldVisual,
    ));
    commands.spawn((
        Sprite::from_color(path, Vec2::new(760.0, 72.0)),
        Transform::from_xyz(25.0, y - h * 0.10, -3.6).with_rotation(Quat::from_rotation_z(-0.04)),
        WorldVisual,
    ));

    for (x, stripe_y, alpha) in [
        (-360.0, y - h * 0.34, 0.16),
        (70.0, y - h * 0.12, 0.13),
        (360.0, y + h * 0.18, 0.10),
    ] {
        commands.spawn((
            Sprite::from_color(
                Color::srgba(0.06, 0.04, 0.02, alpha),
                Vec2::new(330.0, 12.0),
            ),
            Transform::from_xyz(x, stripe_y, -3.4),
            WorldVisual,
        ));
    }
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

fn spawn_room_exits(commands: &mut Commands, state: &GameState, skin: &UiSkinAssets) {
    let left = exit_left(state.current_room);
    let right = exit_right(state.current_room);
    spawn_exit_sign(
        commands,
        skin,
        -455.0,
        -260.0,
        format!("< {}", room_name(left)),
        room_action(left),
    );
    spawn_exit_sign(
        commands,
        skin,
        430.0,
        -260.0,
        format!("{} >", room_name(right)),
        room_action(right),
    );
}

fn spawn_exit_sign(
    commands: &mut Commands,
    skin: &UiSkinAssets,
    x: f32,
    y: f32,
    label: String,
    action: Action,
) {
    commands
        .spawn((
            skin_sprite(skin, SKIN_BUTTON, Color::srgba(1.0, 0.80, 0.50, 0.90)),
            Transform::from_xyz(x, y, 3.0).with_scale(Vec3::splat(0.34)),
            Pickable::default(),
            WorldActionTarget(action),
            WorldVisual,
        ))
        .observe(world_action_on_click)
        .observe(tint_sprite_on_hover(Color::srgb(1.0, 0.86, 0.56)))
        .observe(tint_sprite_on_out(Color::WHITE));
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.92, 0.70)),
        Transform::from_xyz(x, y, 4.0),
        WorldVisual,
    ));
}

fn room_action(room: PlantationRoom) -> Action {
    match room {
        PlantationRoom::Sanctuary => Action::GoSanctuary,
        PlantationRoom::CoffeeField => Action::GoCoffeeField,
        PlantationRoom::Roastery => Action::GoRoastery,
        PlantationRoom::PaperworkOffice => Action::GoPaperworkOffice,
    }
}

fn spawn_coffee_field_room(commands: &mut Commands, state: &GameState, props: &PropAssets) {
    let plant_count = state.coffee_plants.min(36);
    for i in 0..plant_count {
        let x = -250.0 + (i % 12) as f32 * 42.0;
        let y = -160.0 + (i / 12) as f32 * 48.0;
        commands
            .spawn((
                prop_sprite(props, 1),
                Transform::from_xyz(x, y, 1.0).with_scale(Vec3::splat(0.18)),
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
                prop_sprite(props, 2),
                Transform::from_xyz(x, 105.0 + (i % 2) as f32 * 22.0, 2.0)
                    .with_scale(Vec3::splat(0.30)),
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
            prop_sprite(props, 0),
            Transform::from_xyz(410.0, 72.0, 2.0).with_scale(Vec3::splat(0.34)),
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

    spawn_prop(commands, props, 15, 525.0, 80.0, 0.42, 2.0);
    if state.goat_present {
        spawn_goat(commands, props, 420.0, -95.0, "field goat?");
    }

    spawn_room_hint(
        commands,
        "Best buttons here: Plant coffee, Harvest fruit, Feed civets.",
    );
}

fn spawn_sanctuary_room(
    commands: &mut Commands,
    state: &GameState,
    characters: &CharacterAssets,
    props: &PropAssets,
    skin: &UiSkinAssets,
) {
    commands.spawn((
        skin_sprite(skin, SKIN_TOOL_PANEL, Color::srgba(1.0, 0.86, 0.66, 0.86)),
        Transform::from_xyz(60.0, -84.0, 1.0),
        WorldVisual,
    ));
    for i in 0..9 {
        let x = -110.0 + i as f32 * 43.0;
        spawn_prop(
            commands,
            props,
            15,
            x,
            4.0 + (i % 2) as f32 * 8.0,
            0.20,
            2.0,
        );
        spawn_prop(
            commands,
            props,
            15,
            x,
            -174.0 + (i % 2) as f32 * 7.0,
            0.18,
            2.0,
        );
    }
    for i in 0..4 {
        let y = -152.0 + i as f32 * 48.0;
        spawn_prop(commands, props, 0, -116.0, y, 0.13, 2.0);
        spawn_prop(commands, props, 0, 218.0, y + 8.0, 0.13, 2.0);
    }
    commands.spawn((
        Text2d::new("CIVET ENCLOSURE"),
        TextFont {
            font_size: 17.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.82, 0.44)),
        Transform::from_xyz(60.0, 20.0, 4.0),
        WorldVisual,
    ));

    for i in 0..state.civets.min(10) {
        let x = -40.0 + (i % 5) as f32 * 48.0;
        let y = -120.0 + (i / 5) as f32 * 44.0;
        spawn_contact_shadow(commands, x, y - 13.0, 45.0, 14.0, 2.7);
        commands
            .spawn((
                Sprite::from_atlas_image(
                    characters.texture.clone(),
                    TextureAtlas {
                        layout: characters.atlas.clone(),
                        index: i as usize % 3,
                    },
                ),
                Transform::from_xyz(x, y + 2.0, 3.0).with_scale(Vec3::splat(0.105)),
                Pickable::default(),
                CivetClickTarget { index: i as usize },
                MovingCivet {
                    base: Vec3::new(x, y + 2.0, 3.0),
                    phase: i as f32 * 1.7,
                },
                WorldVisual,
            ))
            .observe(select_civet_on_click)
            .observe(tint_sprite_on_hover(Color::srgb(1.0, 0.92, 0.74)))
            .observe(tint_sprite_on_out(Color::WHITE));
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
        spawn_contact_shadow(commands, 85.0, 0.0, 88.0, 24.0, 2.7);
        spawn_prop(commands, props, 11, 85.0, 10.0, 0.28, 3.0);
        commands.spawn((
            Text2d::new("binturong"),
            TextFont {
                font_size: 13.0,
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.87, 0.68)),
            Transform::from_xyz(85.0, 38.0, 4.0),
            WorldVisual,
        ));
    }

    if state.goat_present {
        spawn_goat(commands, props, -175.0, 115.0, "goat?");
    }

    spawn_prop(commands, props, 12, -285.0, -130.0, 0.42, 2.0);
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

fn spawn_roastery_room(
    commands: &mut Commands,
    state: &GameState,
    props: &PropAssets,
    skin: &UiSkinAssets,
) {
    commands.spawn((
        skin_sprite(skin, SKIN_BUTTON, Color::srgba(1.0, 0.80, 0.55, 0.86)),
        Transform::from_xyz(-175.0, -102.0, 1.0),
        WorldVisual,
    ));
    spawn_prop(commands, props, 4, -325.0, -115.0, 0.23, 2.0);
    spawn_prop(commands, props, 3, -35.0, -120.0, 0.21, 2.0);
    spawn_prop(commands, props, 14, -18.0, -42.0, 0.26, 2.0);
    commands
        .spawn((
            prop_sprite(props, 6),
            Transform::from_xyz(-175.0, -60.0, 3.0).with_scale(Vec3::splat(0.48)),
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
                prop_sprite(props, 5),
                Transform::from_xyz(x, y, 2.0).with_scale(Vec3::splat(0.27)),
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
            prop_sprite(props, 3),
            Transform::from_xyz(-390.0, 45.0, 2.0).with_scale(Vec3::splat(0.34)),
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

fn spawn_paperwork_office_room(
    commands: &mut Commands,
    state: &GameState,
    props: &PropAssets,
    skin: &UiSkinAssets,
) {
    commands.spawn((
        skin_sprite(skin, SKIN_STATS_PANEL, Color::srgba(0.74, 0.92, 0.94, 0.82)),
        Transform::from_xyz(25.0, -45.0, 1.5),
        WorldVisual,
    ));
    commands.spawn((
        skin_sprite(skin, SKIN_BUTTON, Color::srgba(0.92, 0.72, 0.50, 0.88)),
        Transform::from_xyz(-120.0, -160.0, 2.0),
        WorldVisual,
    ));
    spawn_prop(commands, props, 15, -335.0, -34.0, 0.30, 2.0);
    spawn_prop(commands, props, 15, 270.0, -34.0, 0.30, 2.0);
    spawn_prop(commands, props, 13, 320.0, -158.0, 0.23, 3.0);
    for i in 0..7 {
        let x = -290.0 + i as f32 * 54.0;
        commands
            .spawn((
                prop_sprite(props, 7),
                Transform::from_xyz(x, -135.0 + (i % 2) as f32 * 10.0, 3.0)
                    .with_scale(Vec3::splat(0.22)),
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

    spawn_prop(commands, props, 8, 165.0, -140.0, 0.32, 3.0);
    spawn_upgrade_buildings(commands, state);
    spawn_helicopter(commands, props);
    if state.goat_present {
        spawn_goat(commands, props, 360.0, -150.0, "witness");
    }

    spawn_room_hint(
        commands,
        "Best buttons here: Show paperwork, build office upgrades, stay calm.",
    );
}

fn spawn_helicopter(commands: &mut Commands, props: &PropAssets) {
    commands.spawn((
        prop_sprite(props, 9),
        Transform::from_xyz(325.0, 245.0, 3.0).with_scale(Vec3::splat(0.38)),
        Helicopter {
            offset: Vec3::new(0.0, 0.0, 0.0),
        },
        WorldVisual,
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

fn spawn_prop(
    commands: &mut Commands,
    props: &PropAssets,
    index: usize,
    x: f32,
    y: f32,
    scale: f32,
    z: f32,
) {
    commands.spawn((
        prop_sprite(props, index),
        Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
        WorldVisual,
    ));
}

fn spawn_contact_shadow(commands: &mut Commands, x: f32, y: f32, width: f32, height: f32, z: f32) {
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.04, 0.028, 0.015, 0.24),
            Vec2::new(width, height),
        ),
        Transform::from_xyz(x, y, z),
        WorldVisual,
    ));
}

fn spawn_goat(commands: &mut Commands, props: &PropAssets, x: f32, y: f32, label: &str) {
    spawn_contact_shadow(commands, x, y - 18.0, 66.0, 18.0, 2.7);
    spawn_prop(commands, props, 10, x, y, 0.25, 3.0);
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
