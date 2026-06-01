use bevy::prelude::*;

use crate::actions::run_action;
use crate::model::*;

pub fn spawn_ui(commands: &mut Commands) {
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
                    StatKind::Day,
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
        Action::ContinueDay => Color::srgb(0.22, 0.38, 0.22),
        Action::EventOptionA | Action::EventOptionB | Action::EventOptionC => {
            Color::srgb(0.26, 0.25, 0.13)
        }
        Action::InspectGoat => Color::srgb(0.48, 0.16, 0.12),
        _ => Color::srgb(0.33, 0.21, 0.10),
    }
}

pub fn handle_buttons(
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

pub fn update_stats(
    state: Res<GameState>,
    mut stats: Query<(&StatText, &mut Text, &mut TextColor)>,
) {
    if !state.is_changed() {
        return;
    }
    for (stat, mut text, mut color) in &mut stats {
        let value = match stat.0 {
            StatKind::Day => {
                if state.game_result.is_some() {
                    format!("Week status: final report")
                } else {
                    format!("Day: {} / 7", state.day)
                }
            }
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

pub fn update_status_bars(
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

pub fn update_log(state: Res<GameState>, mut logs: Query<&mut Text, With<LogText>>) {
    if !state.is_changed() {
        return;
    }
    for mut text in &mut logs {
        **text = format!("\n{}", state.log.join("\n"));
    }
}

pub fn refresh_inspection_modal(
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

pub fn refresh_day_modal(
    mut commands: Commands,
    state: Res<GameState>,
    modal: Query<Entity, With<DayModal>>,
) {
    let should_show = state.day_report.is_some() || state.game_result.is_some();
    let exists = !modal.is_empty();

    if should_show && !exists {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(28),
                    top: percent(17),
                    width: percent(44),
                    padding: UiRect::all(px(22)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.08, 0.055, 0.97)),
                GlobalZIndex(9),
                DayModal,
            ))
            .with_children(|modal| {
                if let Some(report) = &state.day_report {
                    modal.spawn((
                        Text::new(report.title.clone()),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.84, 0.42)),
                    ));
                    modal.spawn((
                        Text::new(report.summary.clone()),
                        TextFont {
                            font_size: 17.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.91, 0.78)),
                    ));
                    modal.spawn((
                        Text::new(format!(
                            "Upkeep charged: ${}. Official memo: all beans remain legally beans.",
                            report.upkeep
                        )),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.78, 0.88, 0.70)),
                    ));

                    if state.game_result.is_none() {
                        spawn_button(modal, "Begin next day", Action::ContinueDay);
                    } else {
                        spawn_button(modal, "View final verdict", Action::ContinueDay);
                    }
                } else if let Some(result) = &state.game_result {
                    let (title, body, color) = match result {
                        GameResult::Won(body) => (
                            "Weekly Verdict: Operationally Legitimate",
                            body,
                            Color::srgb(0.46, 1.0, 0.48),
                        ),
                        GameResult::Failed(body) => (
                            "Weekly Verdict: Board-Level Concern",
                            body,
                            Color::srgb(1.0, 0.26, 0.18),
                        ),
                    };
                    modal.spawn((
                        Text::new(title),
                        TextFont {
                            font_size: 31.0,
                            ..default()
                        },
                        TextColor(color),
                    ));
                    modal.spawn((
                        Text::new(body.clone()),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.91, 0.78)),
                    ));
                    modal.spawn((
                        Text::new("Save the run or start a new one from a clean save file."),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.78, 0.88, 0.70)),
                    ));
                }
            });
    } else if !should_show && exists {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    } else if should_show && exists && state.is_changed() {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    }
}

pub fn refresh_event_modal(
    mut commands: Commands,
    state: Res<GameState>,
    modal: Query<Entity, With<EventModal>>,
) {
    let should_show = state.event.is_some();
    let exists = !modal.is_empty();

    if should_show && !exists {
        let event = state.event.as_ref().expect("event checked above");
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(24),
                    top: percent(15),
                    width: percent(52),
                    padding: UiRect::all(px(22)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.12, 0.075, 0.035, 0.97)),
                GlobalZIndex(8),
                EventModal,
            ))
            .with_children(|modal| {
                modal.spawn((
                    Text::new(event.title.clone()),
                    TextFont {
                        font_size: 31.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.82, 0.40)),
                ));
                modal.spawn((
                    Text::new(event.body.clone()),
                    TextFont {
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.94, 0.88, 0.72)),
                ));

                let (a, b, c) = event_option_labels(event.kind);
                spawn_button(modal, a, Action::EventOptionA);
                spawn_button(modal, b, Action::EventOptionB);
                spawn_button(modal, c, Action::EventOptionC);
            });
    } else if !should_show && exists {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    } else if should_show && exists && state.is_changed() {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    }
}

fn event_option_labels(kind: RandomEventKind) -> (&'static str, &'static str, &'static str) {
    match kind {
        RandomEventKind::PoliceVisit => (
            "Show bean paperwork",
            "Offer coffee tasting",
            "Answer vaguely",
        ),
        RandomEventKind::JournalistQuestions => (
            "Invite full civet tour",
            "Control the tour route",
            "No comment",
        ),
        RandomEventKind::WelfareInspection => (
            "Buy enrichment now",
            "Open every enclosure",
            "Reschedule politely",
        ),
        RandomEventKind::HelicopterOverhead => {
            ("Deploy coffee tarps", "Wave cheerfully", "Hide everyone")
        }
        RandomEventKind::BinturongEscape => ("Hire caretaker", "Let fame happen", "Send the goat"),
        RandomEventKind::PickyCivet => (
            "Serve best fruit",
            "Import better fruit",
            "Insist it is fine",
        ),
        RandomEventKind::GoatAppearance => (
            "Put goat on payroll",
            "Remove goat quietly",
            "Blame goat early",
        ),
    }
}
