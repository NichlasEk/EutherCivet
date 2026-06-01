use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const SAVE_PATH: &str = "euther_civet_save.json";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.39, 0.32, 0.19)))
        .insert_resource(GameState::load().unwrap_or_default())
        .insert_resource(GameTick(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(EventTick(Timer::from_seconds(9.0, TimerMode::Repeating)))
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
                update_stats,
                update_status_bars,
                update_log,
                animate_world,
                refresh_world_visuals,
                refresh_inspection_modal,
            ),
        )
        .run();
}

#[derive(Resource, Serialize, Deserialize, Clone)]
struct GameState {
    coffee_plants: u32,
    civets: u32,
    coffee_fruit: f32,
    civet_feed: f32,
    processed_beans: f32,
    roasted_coffee: f32,
    money: i32,
    suspicion: f32,
    civet_happiness: f32,
    reputation: i32,
    enclosure_level: u32,
    paperwork_level: u32,
    binturong_home: bool,
    goat_present: bool,
    inspection: bool,
    rng_seed: u64,
    log: Vec<String>,
    dirty_visuals: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            coffee_plants: 6,
            civets: 3,
            coffee_fruit: 8.0,
            civet_feed: 0.0,
            processed_beans: 0.0,
            roasted_coffee: 0.0,
            money: 160,
            suspicion: 18.0,
            civet_happiness: 72.0,
            reputation: 8,
            enclosure_level: 1,
            paperwork_level: 1,
            binturong_home: true,
            goat_present: true,
            inspection: false,
            rng_seed: 0xC1FE_CAFE_BA5E_BA11,
            log: vec![
                "Welcome to EutherCivet: fair-trade coffee, suspicious silhouettes.".to_string(),
                "Reminder: no narcotics. Only fruit, civets, beans, and bureaucracy.".to_string(),
            ],
            dirty_visuals: true,
        }
    }
}

impl GameState {
    fn load() -> Option<Self> {
        let text = fs::read_to_string(SAVE_PATH).ok()?;
        let mut state: Self = serde_json::from_str(&text).ok()?;
        state.log_line("Loaded plantation ledger from disk.");
        state.dirty_visuals = true;
        Some(state)
    }

    fn save(&mut self) {
        match serde_json::to_string_pretty(self) {
            Ok(text) => {
                if fs::write(SAVE_PATH, text).is_ok() {
                    self.log_line("Saved an alarmingly neat plantation ledger.");
                } else {
                    self.log_line("Save failed. The paperwork drawer jammed.");
                }
            }
            _ => self.log_line("Save failed. The paperwork drawer jammed."),
        }
    }

    fn log_line(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
        while self.log.len() > 8 {
            self.log.remove(0);
        }
    }

    fn clamp(&mut self) {
        self.suspicion = self.suspicion.clamp(0.0, 100.0);
        self.civet_happiness = self.civet_happiness.clamp(0.0, 100.0);
        if self.suspicion >= 100.0 {
            self.inspection = true;
            self.suspicion = 100.0;
            self.log_line("Operation Bitter Bean begins.");
        }
    }

    fn rand_index(&mut self, max: usize) -> usize {
        self.rng_seed = self
            .rng_seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.rng_seed >> 32) as usize) % max
    }
}

#[derive(Resource)]
struct GameTick(Timer);

#[derive(Resource)]
struct EventTick(Timer);

#[derive(Component)]
struct StatText(StatKind);

#[derive(Component)]
struct StatusBar(StatusKind);

#[derive(Component)]
struct LogText;

#[derive(Component)]
struct WorldVisual;

#[derive(Component)]
struct Helicopter {
    offset: Vec3,
}

#[derive(Component)]
struct SuspicionGlow;

#[derive(Component)]
struct InspectionModal;

#[derive(Component, Clone, Copy)]
struct ActionButton(Action);

#[derive(Clone, Copy)]
enum StatKind {
    Plants,
    Civets,
    Fruit,
    Feed,
    Beans,
    Roasted,
    Money,
    Suspicion,
    Happiness,
    Reputation,
    Paperwork,
}

#[derive(Clone, Copy)]
enum StatusKind {
    Suspicion,
    Happiness,
    CoffeePipeline,
}

#[derive(Clone, Copy)]
enum Action {
    PlantCoffee,
    HarvestFruit,
    FeedCivets,
    CollectBeans,
    RoastCoffee,
    SellCoffee,
    ImproveEnclosure,
    ShowPaperwork,
    Save,
    Load,
    InspectPaperwork,
    InspectTasting,
    InspectGoat,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_world(&mut commands);
    spawn_ui(&mut commands);
}

fn spawn_world(commands: &mut Commands) {
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

fn spawn_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(16)),
                column_gap: px(14),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: px(360),
                    height: percent(100),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(10),
                    padding: UiRect::all(px(16)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.06, 0.05, 0.035, 0.92)),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("EutherCivet"),
                    TextFont {
                        font_size: 34.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.86, 0.42)),
                ));
                panel.spawn((
                    Text::new("Premium coffee. Exotic mammals. Unhelpful optics."),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.88, 0.82, 0.66)),
                ));

                spawn_bar(panel, "Suspicion", StatusKind::Suspicion);
                spawn_bar(panel, "Civet happiness", StatusKind::Happiness);
                spawn_bar(panel, "Coffee pipeline", StatusKind::CoffeePipeline);

                for kind in [
                    StatKind::Plants,
                    StatKind::Civets,
                    StatKind::Fruit,
                    StatKind::Feed,
                    StatKind::Beans,
                    StatKind::Roasted,
                    StatKind::Money,
                    StatKind::Suspicion,
                    StatKind::Happiness,
                    StatKind::Reputation,
                    StatKind::Paperwork,
                ] {
                    panel.spawn((
                        Text::new("..."),
                        TextFont {
                            font_size: 17.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        StatText(kind),
                    ));
                }

                panel.spawn((
                    Text::new(""),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.92, 0.88, 0.76)),
                    LogText,
                ));
            });

            root.spawn((
                Node {
                    flex_grow: 1.0,
                    height: percent(100),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .with_children(|right| {
                right
                    .spawn((
                        Node {
                            width: px(460),
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            row_gap: px(8),
                            column_gap: px(8),
                            padding: UiRect::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.10, 0.065, 0.04, 0.92)),
                    ))
                    .with_children(|buttons| {
                        for (label, action) in [
                            ("Plant coffee", Action::PlantCoffee),
                            ("Harvest fruit", Action::HarvestFruit),
                            ("Feed civets", Action::FeedCivets),
                            ("Collect beans", Action::CollectBeans),
                            ("Roast coffee", Action::RoastCoffee),
                            ("Sell coffee", Action::SellCoffee),
                            ("Improve enclosure", Action::ImproveEnclosure),
                            ("Show paperwork to authorities", Action::ShowPaperwork),
                            ("Save", Action::Save),
                            ("Load", Action::Load),
                        ] {
                            spawn_button(buttons, label, action);
                        }
                    });
            });
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, action: Action) {
    parent
        .spawn((
            Button,
            Node {
                width: px(218),
                height: px(42),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(px(8)),
                ..default()
            },
            BackgroundColor(button_base_color(action)),
            ActionButton(action),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.92, 0.72)),
            ));
        });
}

fn spawn_bar(parent: &mut ChildSpawnerCommands, label: &str, kind: StatusKind) {
    parent
        .spawn(Node {
            width: percent(100),
            height: px(40),
            flex_direction: FlexDirection::Column,
            row_gap: px(4),
            ..default()
        })
        .with_children(|bar| {
            bar.spawn((
                Text::new(label),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.88, 0.82, 0.66)),
            ));
            bar.spawn((
                Node {
                    width: percent(100),
                    height: px(13),
                    padding: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            ))
            .with_children(|track| {
                track.spawn((
                    Node {
                        width: percent(10),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.60, 0.20, 0.12)),
                    StatusBar(kind),
                ));
            });
        });
}

fn button_base_color(action: Action) -> Color {
    match action {
        Action::ShowPaperwork | Action::InspectPaperwork => Color::srgb(0.18, 0.34, 0.28),
        Action::SellCoffee | Action::RoastCoffee | Action::InspectTasting => {
            Color::srgb(0.48, 0.31, 0.10)
        }
        Action::Save | Action::Load => Color::srgb(0.18, 0.18, 0.18),
        Action::InspectGoat => Color::srgb(0.48, 0.16, 0.12),
        _ => Color::srgb(0.33, 0.21, 0.10),
    }
}

fn handle_buttons(
    mut interactions: Query<
        (&Interaction, &ActionButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<GameState>,
) {
    for (interaction, button, mut color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.78, 0.32, 0.18));
                run_action(&mut state, button.0);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.62, 0.39, 0.15));
            }
            Interaction::None => {
                *color = BackgroundColor(button_base_color(button.0));
            }
        }
    }
}

fn run_action(state: &mut GameState, action: Action) {
    if state.inspection {
        match action {
            Action::InspectPaperwork => {
                let reduction = 46.0 + state.paperwork_level as f32 * 5.0;
                state.money -= 18;
                state.suspicion -= reduction;
                state.reputation += 3;
                state.inspection = false;
                state.log_line("Authorities read the paperwork and become visibly tired.");
            }
            Action::InspectTasting => {
                if state.roasted_coffee >= 4.0 {
                    state.roasted_coffee -= 4.0;
                    state.suspicion -= 34.0;
                    state.reputation += 6;
                    state.money -= 8;
                    state.log_line(
                        "Coffee tasting successful. One inspector detects notes of panic.",
                    );
                } else {
                    state.suspicion -= 12.0;
                    state.reputation -= 2;
                    state.log_line(
                        "There was not enough roasted coffee. The tasting was mostly spoons.",
                    );
                }
                state.inspection = false;
            }
            Action::InspectGoat => {
                state.suspicion -= 24.0;
                state.reputation -= 4;
                state.civet_happiness -= 6.0;
                state.goat_present = false;
                state.inspection = false;
                state.dirty_visuals = true;
                state.log_line("The goat accepts no blame but leaves under legal advice.");
            }
            _ => state.log_line("Normal work is paused during Operation Bitter Bean."),
        }
        state.clamp();
        return;
    }

    match action {
        Action::PlantCoffee => {
            if state.money >= 14 {
                state.money -= 14;
                state.coffee_plants += 1;
                state.suspicion += if state.coffee_plants > 18 { 3.5 } else { 1.2 };
                state.log_line(
                    "A coffee shrub is planted in a formation lawyers called unfortunate.",
                );
                state.dirty_visuals = true;
            } else {
                state.log_line("Not enough money for another coffee plant.");
            }
        }
        Action::HarvestFruit => {
            let gained = state.coffee_plants as f32 * 1.6;
            state.coffee_fruit += gained;
            state.civet_happiness -= 0.8;
            state.log_line(format!("Harvested {gained:.0} coffee fruit."));
        }
        Action::FeedCivets => {
            let wanted = state.civets as f32 * 5.0;
            let fed = state.coffee_fruit.min(wanted);
            if fed > 0.0 {
                state.coffee_fruit -= fed;
                state.civet_feed += fed;
                state.civet_happiness += 8.0 + fed * 0.25;
                state.suspicion -= 1.0;
                state.log_line("Civets receive fruit. Morale improves. Optics remain complex.");
            } else {
                state.civet_happiness -= 5.0;
                state.suspicion += 3.0;
                state.log_line("No fruit to feed the civets. They file a silent complaint.");
            }
        }
        Action::CollectBeans => {
            let found = 1.0 + state.civets as f32 * 0.35;
            state.processed_beans += found;
            state.suspicion += 0.7;
            state.log_line(format!(
                "Collected {found:.1} processed beans from the civet area."
            ));
        }
        Action::RoastCoffee => {
            let batch = state.processed_beans.min(8.0);
            if batch >= 1.0 {
                state.processed_beans -= batch;
                state.roasted_coffee += batch * 0.82;
                state.money -= 2;
                state.suspicion += 0.8;
                state.log_line("Roasted a premium batch. Smoke plume described as theatrical.");
            } else {
                state.log_line("Not enough processed beans to roast.");
            }
        }
        Action::SellCoffee => {
            let sold = state.roasted_coffee.min(8.0);
            if sold >= 1.0 {
                let earned = (sold * (13.0 + state.reputation as f32 * 0.7)).round() as i32;
                state.roasted_coffee -= sold;
                state.money += earned;
                state.reputation += 1 + (sold / 5.0) as i32;
                state.suspicion += if sold > 6.0 { 4.0 } else { 1.2 };
                state.log_line(format!(
                    "Sold {sold:.1} bags of civet coffee for ${earned}."
                ));
            } else {
                state.log_line("No roasted coffee ready to sell.");
            }
        }
        Action::ImproveEnclosure => {
            let cost = 45 + state.enclosure_level as i32 * 20;
            if state.money >= cost {
                state.money -= cost;
                state.enclosure_level += 1;
                state.civet_happiness += 18.0;
                state.suspicion -= 8.0;
                state.reputation += 2;
                state.log_line("Enclosure improved. Inspectors dislike how wholesome it is.");
            } else {
                state.log_line(format!("Enclosure upgrade needs ${cost}."));
            }
        }
        Action::ShowPaperwork => {
            let cost = 16 + state.paperwork_level as i32 * 3;
            if state.money >= cost {
                state.money -= cost;
                state.paperwork_level += 1;
                state.suspicion -= 18.0 + state.paperwork_level as f32;
                state.reputation += 1;
                state.log_line(
                    "Presented receipts, permits, civet dental charts, and bean custody forms.",
                );
            } else {
                state.log_line("Not enough money to print the paperwork annex.");
            }
        }
        Action::Save => state.save(),
        Action::Load => {
            if let Some(loaded) = GameState::load() {
                *state = loaded;
            } else {
                state.log_line("No save file found.");
            }
        }
        Action::InspectPaperwork | Action::InspectTasting | Action::InspectGoat => {}
    }

    if state.civet_happiness < 35.0 {
        state.suspicion += 2.0;
        state.reputation -= 1;
    }
    state.clamp();
}

fn tick_game(time: Res<Time>, mut timer: ResMut<GameTick>, mut state: ResMut<GameState>) {
    if !timer.0.tick(time.delta()).just_finished() || state.inspection {
        return;
    }

    let fruit_growth = state.coffee_plants as f32 * 0.42;
    state.coffee_fruit += fruit_growth;

    let appetite = state.civets as f32 * 0.65;
    let eaten = state.civet_feed.min(appetite);
    if eaten > 0.0 {
        state.civet_feed -= eaten;
        let happiness_bonus = (state.civet_happiness / 100.0).max(0.2);
        let enclosure_bonus = 1.0 + state.enclosure_level as f32 * 0.08;
        state.processed_beans += eaten * 0.32 * happiness_bonus * enclosure_bonus;
        state.civet_happiness += 0.25;
    } else {
        state.civet_happiness -= 1.4;
        if state.civet_happiness < 45.0 {
            state.suspicion += 0.7;
        }
    }

    if state.coffee_plants > 24 {
        state.suspicion += 0.25;
    }
    if state.reputation < 0 {
        state.suspicion += 0.2;
    }
    state.clamp();
}

fn trigger_random_events(
    time: Res<Time>,
    mut timer: ResMut<EventTick>,
    mut state: ResMut<GameState>,
) {
    if !timer.0.tick(time.delta()).just_finished() || state.inspection {
        return;
    }

    match state.rand_index(7) {
        0 => {
            state.suspicion += 7.0;
            state.log_line("Local police visit. They admire the beans with tactical suspicion.");
        }
        1 => {
            state.suspicion += 5.0;
            state.reputation += 1;
            state.log_line("A journalist asks why every invoice says 'totally beans'.");
        }
        2 => {
            if state.civet_happiness >= 60.0 {
                state.reputation += 3;
                state.suspicion -= 5.0;
                state.log_line("Animal welfare inspection passes. Civets look smug.");
            } else {
                state.reputation -= 4;
                state.suspicion += 11.0;
                state.log_line("Animal welfare inspection finds disappointed civets.");
            }
        }
        3 => {
            state.suspicion += 10.0;
            state.log_line("A police helicopter flies overhead. It circles the coffee bags twice.");
        }
        4 => {
            state.binturong_home = false;
            state.suspicion += 8.0;
            state.civet_happiness -= 4.0;
            state.dirty_visuals = true;
            state.log_line("The binturong escapes and naps inside a government vehicle.");
        }
        5 => {
            state.civet_happiness -= 6.0;
            state.log_line("A civet refuses low-quality fruit with devastating eye contact.");
        }
        _ => {
            state.goat_present = true;
            state.suspicion += 3.0;
            state.dirty_visuals = true;
            state.log_line("A goat appears for no clear reason. Legal recommends silence.");
        }
    }
    state.clamp();
}

fn update_stats(state: Res<GameState>, mut stats: Query<(&StatText, &mut Text, &mut TextColor)>) {
    if !state.is_changed() {
        return;
    }
    for (stat, mut text, mut color) in &mut stats {
        let value = match stat.0 {
            StatKind::Plants => format!("Coffee plants: {}", state.coffee_plants),
            StatKind::Civets => format!("Civets: {}", state.civets),
            StatKind::Fruit => format!("Coffee fruit: {:.0}", state.coffee_fruit),
            StatKind::Feed => format!("Fruit in civet feeders: {:.0}", state.civet_feed),
            StatKind::Beans => format!("Processed beans: {:.1}", state.processed_beans),
            StatKind::Roasted => format!("Roasted coffee bags: {:.1}", state.roasted_coffee),
            StatKind::Money => format!("Money: ${}", state.money),
            StatKind::Suspicion => format!("Suspicion: {:.0}%", state.suspicion),
            StatKind::Happiness => format!("Civet happiness: {:.0}%", state.civet_happiness),
            StatKind::Reputation => format!("Reputation: {}", state.reputation),
            StatKind::Paperwork => format!("Paperwork level: {}", state.paperwork_level),
        };
        **text = value;
        color.0 = match stat.0 {
            StatKind::Suspicion if state.suspicion >= 75.0 => Color::srgb(1.0, 0.18, 0.12),
            StatKind::Suspicion if state.suspicion >= 45.0 => Color::srgb(1.0, 0.58, 0.24),
            StatKind::Happiness if state.civet_happiness < 40.0 => Color::srgb(1.0, 0.28, 0.18),
            StatKind::Money if state.money < 20 => Color::srgb(1.0, 0.38, 0.22),
            _ => Color::srgb(0.97, 0.92, 0.78),
        };
    }
}

fn update_status_bars(
    state: Res<GameState>,
    mut bars: Query<(&StatusBar, &mut Node, &mut BackgroundColor)>,
) {
    if !state.is_changed() {
        return;
    }

    for (bar, mut node, mut color) in &mut bars {
        let value = match bar.0 {
            StatusKind::Suspicion => state.suspicion,
            StatusKind::Happiness => state.civet_happiness,
            StatusKind::CoffeePipeline => {
                let stock = state.coffee_fruit
                    + state.civet_feed
                    + state.processed_beans * 2.0
                    + state.roasted_coffee * 4.0;
                (stock / 2.4).clamp(0.0, 100.0)
            }
        };

        node.width = percent(value.max(2.0));
        color.0 = match bar.0 {
            StatusKind::Suspicion if value >= 80.0 => Color::srgb(1.0, 0.10, 0.06),
            StatusKind::Suspicion if value >= 50.0 => Color::srgb(1.0, 0.48, 0.14),
            StatusKind::Suspicion => Color::srgb(0.68, 0.24, 0.12),
            StatusKind::Happiness if value < 35.0 => Color::srgb(0.90, 0.15, 0.10),
            StatusKind::Happiness => Color::srgb(0.20, 0.74, 0.35),
            StatusKind::CoffeePipeline => Color::srgb(0.86, 0.62, 0.20),
        };
    }
}

fn update_log(state: Res<GameState>, mut logs: Query<&mut Text, With<LogText>>) {
    if !state.is_changed() {
        return;
    }
    for mut text in &mut logs {
        **text = format!("\n{}", state.log.join("\n"));
    }
}

fn animate_world(
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

fn refresh_world_visuals(
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

    state.dirty_visuals = false;
}

fn refresh_inspection_modal(
    mut commands: Commands,
    state: Res<GameState>,
    modal: Query<Entity, With<InspectionModal>>,
) {
    let exists = !modal.is_empty();
    if state.inspection && !exists {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(23),
                    top: percent(14),
                    width: percent(54),
                    padding: UiRect::all(px(22)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.42, 0.03, 0.02, 0.96)),
                GlobalZIndex(10),
                InspectionModal,
            ))
            .with_children(|modal| {
                modal.spawn((
                    Text::new("Operation Bitter Bean"),
                    TextFont {
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.88, 0.56)),
                ));
                modal.spawn((
                    Text::new(
                        "Authorities raid the plantation expecting narcotics. They find coffee, civets, extremely detailed paperwork, and one suspicious goat.",
                    ),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                spawn_button(modal, "Show paperwork", Action::InspectPaperwork);
                spawn_button(modal, "Offer coffee tasting", Action::InspectTasting);
                spawn_button(modal, "Blame the goat", Action::InspectGoat);
            });
    } else if !state.inspection && exists {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    }
}
