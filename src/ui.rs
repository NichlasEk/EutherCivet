use bevy::prelude::*;

use crate::actions::run_action;
use crate::model::*;

const PANEL_PAPER: Color = Color::srgba(0.98, 0.84, 0.70, 0.10);
const SKIN_STATS_PANEL: usize = 0;
const SKIN_PAPER_PANEL: usize = 2;
const SKIN_BUTTON: usize = 3;

pub fn spawn_ui(commands: &mut Commands, skin: &UiSkinAssets) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(14),
                    right: px(14),
                    top: px(10),
                    min_height: px(72),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    align_items: AlignItems::Center,
                    row_gap: px(6),
                    column_gap: px(10),
                    padding: UiRect::axes(px(14), px(8)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(10)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.04, 0.035, 0.025, 0.34)),
                BorderColor::all(Color::srgba(1.0, 0.78, 0.36, 0.24)),
            ))
            .with_children(|hud| {
                hud.spawn((
                    Text::new("EutherCivet"),
                    TextFont {
                        font_size: 27.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.86, 0.42)),
                ));
                hud.spawn((
                    Text::new("Fair-trade coffee. Questionable optics."),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.92, 0.86, 0.68, 0.86)),
                ));

                spawn_bar(hud, "Suspicion", StatusKind::Suspicion);
                spawn_bar(hud, "Civets", StatusKind::Happiness);
                spawn_bar(hud, "Coffee", StatusKind::CoffeePipeline);

                for kind in [
                    StatKind::Day,
                    StatKind::Fruit,
                    StatKind::Beans,
                    StatKind::Roasted,
                    StatKind::Money,
                    StatKind::Suspicion,
                    StatKind::Happiness,
                    StatKind::Reputation,
                    StatKind::Mailbox,
                    StatKind::Order,
                ] {
                    hud.spawn((
                        Text::new("..."),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.94, 0.76, 0.94)),
                        StatText(kind),
                    ));
                }
            });

            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(14),
                    top: px(92),
                    width: px(430),
                    max_height: px(150),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(8)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.025, 0.022, 0.016, 0.24)),
                BorderColor::all(Color::srgba(1.0, 0.82, 0.48, 0.15)),
            ))
            .with_children(|debug| {
                debug.spawn((
                    Text::new(""),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.95, 0.91, 0.76, 0.76)),
                    LogText,
                ));
            });

            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(145),
                    right: px(145),
                    bottom: px(14),
                    min_height: px(118),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: px(7),
                    column_gap: px(7),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(10)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.045, 0.032, 0.018, 0.30)),
                BorderColor::all(Color::srgba(1.0, 0.72, 0.32, 0.22)),
            ))
            .with_children(|buttons| {
                for (label, action) in [
                    ("Sanctuary", Action::GoSanctuary),
                    ("Coffee Field", Action::GoCoffeeField),
                    ("Roastery", Action::GoRoastery),
                    ("Paperwork Office", Action::GoPaperworkOffice),
                ] {
                    spawn_dynamic_button(buttons, skin, label, action, 132.0);
                }
                for (label, action) in [
                    ("Care", Action::ShowCareTools),
                    ("Field", Action::ShowFieldTools),
                    ("Production", Action::ShowProductionTools),
                    ("Compliance", Action::ShowComplianceTools),
                    ("Upgrades", Action::ShowUpgradeTools),
                    ("System", Action::ShowSystemTools),
                ] {
                    spawn_dynamic_button(buttons, skin, label, action, 92.0);
                }
                for (label, action, group) in [
                    ("Feed civets", Action::FeedCivets, ToolGroup::Care),
                    (
                        "Improve enclosure",
                        Action::ImproveEnclosure,
                        ToolGroup::Care,
                    ),
                    ("Plant coffee", Action::PlantCoffee, ToolGroup::Field),
                    ("Harvest fruit", Action::HarvestFruit, ToolGroup::Field),
                    ("Collect beans", Action::CollectBeans, ToolGroup::Production),
                    ("Roast coffee", Action::RoastCoffee, ToolGroup::Production),
                    ("Sell coffee", Action::SellCoffee, ToolGroup::Production),
                    ("Deliver order", Action::DeliverOrder, ToolGroup::Production),
                    (
                        "Show paperwork to authorities",
                        Action::ShowPaperwork,
                        ToolGroup::Compliance,
                    ),
                    (
                        "Build legal office",
                        Action::BuildLegalOffice,
                        ToolGroup::Upgrades,
                    ),
                    ("Hire caretaker", Action::HireCaretaker, ToolGroup::Upgrades),
                    (
                        "Build fruit sorter",
                        Action::BuildFruitSorter,
                        ToolGroup::Upgrades,
                    ),
                    (
                        "Build roasting shed",
                        Action::BuildRoastingShed,
                        ToolGroup::Upgrades,
                    ),
                    (
                        "Open tasting room",
                        Action::BuildTastingRoom,
                        ToolGroup::Upgrades,
                    ),
                    ("Save", Action::Save, ToolGroup::System),
                    ("Load", Action::Load, ToolGroup::System),
                ] {
                    spawn_grouped_dynamic_button(buttons, skin, label, action, group);
                }
                spawn_dynamic_button(
                    buttons,
                    skin,
                    "Inventory sack",
                    Action::ToggleInventory,
                    150.0,
                );
                for (label, action) in [
                    ("Give coffee fruit", Action::GiveFruitFromInventory),
                    ("Pick up beans", Action::PickUpBeansToInventory),
                    ("Tiny brush", Action::UseTinyBrush),
                    ("Ribbon collar", Action::UseRibbonCollar),
                    ("Fruit puzzle", Action::UseFruitPuzzle),
                ] {
                    spawn_inventory_button(buttons, skin, label, action);
                }
            });
        });
}

fn ui_skin_node(skin: &UiSkinAssets, index: usize, color: Color) -> ImageNode {
    ImageNode::from_atlas_image(
        skin.texture.clone(),
        TextureAtlas {
            layout: skin.atlas.clone(),
            index,
        },
    )
    .with_color(color)
}

fn spawn_dynamic_button(
    parent: &mut ChildSpawnerCommands,
    skin: &UiSkinAssets,
    label: &str,
    action: Action,
    width: f32,
) {
    parent
        .spawn((
            Button,
            Node {
                width: px(width),
                height: px(34),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(px(8)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            ui_skin_node(skin, SKIN_BUTTON, button_base_color(action)),
            BorderColor::all(button_border_color(action)),
            ActionButton(action),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.92, 0.72)),
                DynamicButtonText(action),
            ));
        });
}

fn spawn_grouped_dynamic_button(
    parent: &mut ChildSpawnerCommands,
    skin: &UiSkinAssets,
    label: &str,
    action: Action,
    group: ToolGroup,
) {
    parent
        .spawn((
            Button,
            Node {
                width: px(210),
                height: px(34),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(px(8)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            ui_skin_node(skin, SKIN_BUTTON, button_base_color(action)),
            BorderColor::all(button_border_color(action)),
            ActionButton(action),
            ToolActionGroup(group),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.92, 0.72)),
                DynamicButtonText(action),
            ));
        });
}

fn spawn_inventory_button(
    parent: &mut ChildSpawnerCommands,
    skin: &UiSkinAssets,
    label: &str,
    action: Action,
) {
    parent
        .spawn((
            Button,
            Node {
                display: Display::None,
                width: px(138),
                height: px(34),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(px(8)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            ui_skin_node(skin, SKIN_BUTTON, button_base_color(action)),
            BorderColor::all(button_border_color(action)),
            ActionButton(action),
            InventoryAction,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.92, 0.72)),
                DynamicButtonText(action),
            ));
        });
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    skin: &UiSkinAssets,
    label: &str,
    action: Action,
) {
    parent
        .spawn((
            Button,
            Node {
                width: px(218),
                height: px(42),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(px(8)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            ui_skin_node(skin, SKIN_BUTTON, button_base_color(action)),
            BorderColor::all(button_border_color(action)),
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
            width: px(128),
            height: px(34),
            flex_direction: FlexDirection::Column,
            row_gap: px(3),
            ..default()
        })
        .with_children(|bar| {
            bar.spawn((
                Text::new(label),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(Color::srgb(0.88, 0.82, 0.66)),
            ));
            bar.spawn((
                Node {
                    width: percent(100),
                    height: px(10),
                    padding: UiRect::all(px(2)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(5)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.02, 0.025, 0.015, 0.62)),
                BorderColor::all(Color::srgba(1.0, 0.83, 0.42, 0.25)),
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
        Action::FeedSelectedCivet | Action::PetSelectedCivet | Action::InspectSelectedCivet => {
            Color::srgb(0.24, 0.34, 0.18)
        }
        Action::UseTinyBrush | Action::UseRibbonCollar | Action::UseFruitPuzzle => {
            Color::srgb(0.32, 0.28, 0.16)
        }
        Action::CloseAnimalPanel => Color::srgb(0.18, 0.18, 0.14),
        Action::StartGame
        | Action::ShowIntro
        | Action::ShowAnimalBook
        | Action::BackToMenu
        | Action::ContinueDay => Color::srgb(0.22, 0.38, 0.22),
        Action::GoSanctuary
        | Action::GoCoffeeField
        | Action::GoRoastery
        | Action::GoPaperworkOffice => Color::srgb(0.28, 0.18, 0.24),
        Action::ShowCareTools
        | Action::ShowFieldTools
        | Action::ShowProductionTools
        | Action::ShowComplianceTools
        | Action::ShowUpgradeTools
        | Action::ShowSystemTools => Color::srgb(0.16, 0.22, 0.16),
        Action::DeliverOrder | Action::AcceptOrder => Color::srgb(0.33, 0.34, 0.12),
        Action::GiveFruitFromInventory
        | Action::PickUpBeansToInventory
        | Action::ToggleInventory => Color::srgb(0.36, 0.25, 0.11),
        Action::DeclineOrder => Color::srgb(0.32, 0.16, 0.12),
        Action::BuildLegalOffice
        | Action::HireCaretaker
        | Action::BuildFruitSorter
        | Action::BuildRoastingShed
        | Action::BuildTastingRoom => Color::srgb(0.20, 0.30, 0.18),
        Action::EventOptionA | Action::EventOptionB | Action::EventOptionC => {
            Color::srgb(0.26, 0.25, 0.13)
        }
        Action::InspectGoat => Color::srgb(0.48, 0.16, 0.12),
        _ => Color::srgb(0.33, 0.21, 0.10),
    }
}

fn button_border_color(action: Action) -> Color {
    match action {
        Action::ShowPaperwork | Action::InspectPaperwork => Color::srgba(0.56, 0.86, 0.72, 0.70),
        Action::GoSanctuary
        | Action::GoCoffeeField
        | Action::GoRoastery
        | Action::GoPaperworkOffice => Color::srgba(1.0, 0.70, 0.82, 0.56),
        Action::ShowCareTools
        | Action::ShowFieldTools
        | Action::ShowProductionTools
        | Action::ShowComplianceTools
        | Action::ShowUpgradeTools
        | Action::ShowSystemTools => Color::srgba(0.78, 1.0, 0.64, 0.45),
        Action::Save | Action::Load => Color::srgba(0.88, 0.88, 0.78, 0.30),
        _ => Color::srgba(1.0, 0.76, 0.42, 0.50),
    }
}

fn button_lit_color(action: Action) -> Color {
    match action {
        Action::ShowPaperwork | Action::InspectPaperwork => Color::srgb(0.28, 0.52, 0.43),
        Action::SellCoffee | Action::RoastCoffee | Action::InspectTasting => {
            Color::srgb(0.72, 0.43, 0.16)
        }
        Action::GoSanctuary
        | Action::GoCoffeeField
        | Action::GoRoastery
        | Action::GoPaperworkOffice => Color::srgb(0.46, 0.26, 0.38),
        Action::ShowCareTools
        | Action::ShowFieldTools
        | Action::ShowProductionTools
        | Action::ShowComplianceTools
        | Action::ShowUpgradeTools
        | Action::ShowSystemTools => Color::srgb(0.28, 0.43, 0.27),
        _ => Color::srgb(0.62, 0.39, 0.15),
    }
}

fn button_phase(action: Action) -> f32 {
    match action {
        Action::GoSanctuary => 0.0,
        Action::GoCoffeeField => 0.7,
        Action::GoRoastery => 1.4,
        Action::GoPaperworkOffice => 2.1,
        Action::ShowCareTools => 0.3,
        Action::ShowFieldTools => 0.9,
        Action::ShowProductionTools => 1.5,
        Action::ShowComplianceTools => 2.1,
        Action::ShowUpgradeTools => 2.7,
        Action::ShowSystemTools => 3.3,
        Action::PlantCoffee => 0.2,
        Action::HarvestFruit => 0.8,
        Action::FeedCivets => 1.2,
        Action::CollectBeans => 1.8,
        Action::RoastCoffee => 2.4,
        Action::SellCoffee => 3.0,
        _ => 0.5,
    }
}

fn mix_color(a: Color, b: Color, amount: f32) -> Color {
    let a = a.to_srgba();
    let b = b.to_srgba();
    let t = amount.clamp(0.0, 1.0);
    Color::srgba(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
        a.alpha + (b.alpha - a.alpha) * t,
    )
}

pub fn animate_buttons(
    time: Res<Time>,
    state: Res<GameState>,
    mut buttons: Query<
        (
            &ActionButton,
            &Interaction,
            &mut ImageNode,
            &mut BorderColor,
            &mut Node,
        ),
        With<Button>,
    >,
) {
    let t = time.elapsed_secs();
    for (button, interaction, mut image, mut border, mut node) in &mut buttons {
        let available = can_run(button.0, &state);
        let pulse = 0.5 + 0.5 * (t * 2.8 + button_phase(button.0)).sin();

        if !available {
            image.color = Color::srgba(0.44, 0.40, 0.34, 0.72);
            *border = BorderColor::all(Color::srgba(0.48, 0.42, 0.32, 0.22));
            node.border = UiRect::all(px(1));
            continue;
        }

        let base = button_base_color(button.0);
        let lit = button_lit_color(button.0);
        let active = is_active_group_action(button.0, &state);
        let amount = match *interaction {
            Interaction::Pressed => 0.92,
            Interaction::Hovered => 0.55 + pulse * 0.18,
            Interaction::None if active => 0.32 + pulse * 0.18,
            Interaction::None => 0.08 + pulse * 0.04,
        };

        image.color = mix_color(base, lit, amount);
        *border = BorderColor::all(mix_color(
            button_border_color(button.0),
            Color::srgba(1.0, 0.92, 0.62, 0.92),
            if active {
                0.55 + pulse * 0.25
            } else {
                amount * 0.55
            },
        ));
        node.border = UiRect::all(px(
            if active || matches!(*interaction, Interaction::Hovered) {
                2
            } else {
                1
            },
        ));
    }
}

pub fn handle_buttons(
    interactions: Query<(&Interaction, &ActionButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<GameState>,
) {
    for (interaction, button) in &interactions {
        if !can_run(button.0, &state) {
            if matches!(*interaction, Interaction::Pressed) {
                let reason = unavailable_reason(button.0, &state);
                state.log_line(reason);
            }
            continue;
        }
        match *interaction {
            Interaction::Pressed => {
                run_action(&mut state, button.0);
            }
            Interaction::Hovered | Interaction::None => {}
        }
    }
}

pub fn update_button_labels(
    state: Res<GameState>,
    mut labels: Query<(&DynamicButtonText, &mut Text)>,
    mut grouped_actions: Query<(&ToolActionGroup, &mut Node)>,
    mut inventory_actions: Query<&mut Node, (With<InventoryAction>, Without<ToolActionGroup>)>,
) {
    if !state.is_changed() {
        return;
    }

    for (dynamic, mut text) in &mut labels {
        **text = action_label(dynamic.0, &state);
    }

    for (group, mut node) in &mut grouped_actions {
        node.display = if group.0 == state.active_tool_group {
            Display::Flex
        } else {
            Display::None
        };
    }

    for mut node in &mut inventory_actions {
        node.display = if state.inventory_open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn is_active_group_action(action: Action, state: &GameState) -> bool {
    matches!(
        (action, state.active_tool_group),
        (Action::ShowCareTools, ToolGroup::Care)
            | (Action::ShowFieldTools, ToolGroup::Field)
            | (Action::ShowProductionTools, ToolGroup::Production)
            | (Action::ShowComplianceTools, ToolGroup::Compliance)
            | (Action::ShowUpgradeTools, ToolGroup::Upgrades)
            | (Action::ShowSystemTools, ToolGroup::System)
    )
}

fn action_label(action: Action, state: &GameState) -> String {
    match action {
        Action::PlantCoffee => "Plant coffee ($14)".to_string(),
        Action::HarvestFruit => format!("Harvest fruit (+{:.0})", state.coffee_plants as f32 * 1.6),
        Action::FeedCivets => format!("Feed civets (needs fruit)"),
        Action::CollectBeans => "Collect beans".to_string(),
        Action::RoastCoffee => {
            let rate = if state.roasting_shed { "96%" } else { "82%" };
            format!("Roast coffee ({rate} yield)")
        }
        Action::SellCoffee => {
            let bonus = if state.tasting_room { "+ tasting" } else { "" };
            format!("Sell coffee {bonus}")
        }
        Action::DeliverOrder => "Deliver order".to_string(),
        Action::ToggleInventory => {
            if state.inventory_open {
                "Close sack".to_string()
            } else {
                format!(
                    "Inventory sack ({:.0} fruit, {:.1} beans)",
                    state.coffee_fruit, state.processed_beans
                )
            }
        }
        Action::GiveFruitFromInventory => format!("Give fruit ({:.0})", state.coffee_fruit),
        Action::PickUpBeansToInventory => "Pick up beans".to_string(),
        Action::ImproveEnclosure => {
            format!(
                "Improve enclosure (${})",
                45 + state.enclosure_level as i32 * 20
            )
        }
        Action::ShowPaperwork => {
            let cost = if state.legal_office {
                8 + state.paperwork_level as i32 * 2
            } else {
                16 + state.paperwork_level as i32 * 3
            };
            format!("Show paperwork (${cost})")
        }
        Action::BuildLegalOffice => upgrade_label("Legal office", 110, state.legal_office),
        Action::HireCaretaker => upgrade_label("Caretaker", 85, state.caretaker),
        Action::BuildFruitSorter => upgrade_label("Fruit sorter", 95, state.fruit_sorter),
        Action::BuildRoastingShed => upgrade_label("Roasting shed", 125, state.roasting_shed),
        Action::BuildTastingRoom => upgrade_label("Tasting room", 140, state.tasting_room),
        Action::Save => "Save".to_string(),
        Action::Load => "Load".to_string(),
        Action::FeedSelectedCivet => "Feed fruit tray".to_string(),
        Action::PetSelectedCivet => "Pet gently".to_string(),
        Action::InspectSelectedCivet => "Inspect notes".to_string(),
        Action::UseTinyBrush => "Tiny brush".to_string(),
        Action::UseRibbonCollar => "Ribbon collar".to_string(),
        Action::UseFruitPuzzle => "Fruit puzzle".to_string(),
        Action::GoSanctuary => room_label("Sanctuary", PlantationRoom::Sanctuary, state),
        Action::GoCoffeeField => room_label("Coffee Field", PlantationRoom::CoffeeField, state),
        Action::GoRoastery => room_label("Roastery", PlantationRoom::Roastery, state),
        Action::GoPaperworkOffice => {
            room_label("Paperwork Office", PlantationRoom::PaperworkOffice, state)
        }
        Action::ShowCareTools => group_label("Care", ToolGroup::Care, state),
        Action::ShowFieldTools => group_label("Field", ToolGroup::Field, state),
        Action::ShowProductionTools => group_label("Production", ToolGroup::Production, state),
        Action::ShowComplianceTools => group_label("Compliance", ToolGroup::Compliance, state),
        Action::ShowUpgradeTools => group_label("Upgrades", ToolGroup::Upgrades, state),
        Action::ShowSystemTools => group_label("System", ToolGroup::System, state),
        _ => "Action".to_string(),
    }
}

fn room_label(name: &str, room: PlantationRoom, state: &GameState) -> String {
    if state.current_room == room {
        format!("{name} *")
    } else {
        name.to_string()
    }
}

fn group_label(name: &str, group: ToolGroup, state: &GameState) -> String {
    if state.active_tool_group == group {
        format!("v {name}")
    } else {
        format!("> {name}")
    }
}

fn upgrade_label(name: &str, cost: i32, bought: bool) -> String {
    if bought {
        format!("{name} (built)")
    } else {
        format!("{name} (${cost})")
    }
}

fn can_run(action: Action, state: &GameState) -> bool {
    match action {
        Action::PlantCoffee => state.money >= 14,
        Action::HarvestFruit => state.coffee_plants > 0,
        Action::FeedCivets => state.coffee_fruit > 0.0,
        Action::CollectBeans => true,
        Action::RoastCoffee => state.processed_beans >= 1.0,
        Action::SellCoffee => state.roasted_coffee >= 1.0,
        Action::DeliverOrder => state
            .active_order
            .as_ref()
            .is_some_and(|order| state.roasted_coffee >= order.bags),
        Action::ToggleInventory => true,
        Action::GiveFruitFromInventory => {
            state.current_room == PlantationRoom::Sanctuary && state.coffee_fruit >= 1.0
        }
        Action::PickUpBeansToInventory => state.current_room == PlantationRoom::Sanctuary,
        Action::ImproveEnclosure => state.money >= 45 + state.enclosure_level as i32 * 20,
        Action::ShowPaperwork => {
            let cost = if state.legal_office {
                8 + state.paperwork_level as i32 * 2
            } else {
                16 + state.paperwork_level as i32 * 3
            };
            state.money >= cost
        }
        Action::BuildLegalOffice => !state.legal_office && state.money >= 110,
        Action::HireCaretaker => !state.caretaker && state.money >= 85,
        Action::BuildFruitSorter => !state.fruit_sorter && state.money >= 95,
        Action::BuildRoastingShed => !state.roasting_shed && state.money >= 125,
        Action::BuildTastingRoom => !state.tasting_room && state.money >= 140,
        Action::FeedSelectedCivet => state.selected_civet.is_some() && state.coffee_fruit >= 2.0,
        Action::PetSelectedCivet
        | Action::InspectSelectedCivet
        | Action::UseTinyBrush
        | Action::UseRibbonCollar
        | Action::UseFruitPuzzle
        | Action::CloseAnimalPanel => state.selected_civet.is_some(),
        Action::GoSanctuary
        | Action::GoCoffeeField
        | Action::GoRoastery
        | Action::GoPaperworkOffice => state.screen == GameScreen::Playing,
        Action::ShowCareTools
        | Action::ShowFieldTools
        | Action::ShowProductionTools
        | Action::ShowComplianceTools
        | Action::ShowUpgradeTools
        | Action::ShowSystemTools => state.screen == GameScreen::Playing,
        Action::AcceptOrder | Action::DeclineOrder => state.pending_order.is_some(),
        Action::EventOptionA | Action::EventOptionB | Action::EventOptionC => state.event.is_some(),
        Action::InspectPaperwork | Action::InspectTasting | Action::InspectGoat => state.inspection,
        _ => true,
    }
}

fn unavailable_reason(action: Action, state: &GameState) -> &'static str {
    match action {
        Action::PlantCoffee => "Not enough money to plant coffee.",
        Action::FeedCivets => "No coffee fruit available for feeding.",
        Action::RoastCoffee => "Not enough processed beans to roast.",
        Action::SellCoffee => "No roasted coffee ready to sell.",
        Action::DeliverOrder if state.active_order.is_none() => "No active order to deliver.",
        Action::DeliverOrder => "Not enough roasted coffee for the active order.",
        Action::GiveFruitFromInventory => "Walk to the civets with coffee fruit in the sack.",
        Action::PickUpBeansToInventory => "Walk to the civet enclosure work area.",
        Action::ShowPaperwork => "Not enough money for paperwork.",
        Action::ImproveEnclosure => "Not enough money for enclosure work.",
        Action::BuildLegalOffice
        | Action::HireCaretaker
        | Action::BuildFruitSorter
        | Action::BuildRoastingShed
        | Action::BuildTastingRoom => "Upgrade is unavailable or already built.",
        Action::FeedSelectedCivet if state.selected_civet.is_none() => "Select a civet first.",
        Action::FeedSelectedCivet => "A personal fruit tray needs 2 coffee fruit.",
        Action::PetSelectedCivet | Action::InspectSelectedCivet | Action::CloseAnimalPanel => {
            "Select a civet first."
        }
        Action::UseTinyBrush | Action::UseRibbonCollar | Action::UseFruitPuzzle => {
            "Select a civet first."
        }
        _ => "That action is unavailable right now.",
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
                    "Final report".to_string()
                } else {
                    format!("Day {}/7", state.day)
                }
            }
            StatKind::Plants => format!("Plants {}", state.coffee_plants),
            StatKind::Civets => format!("Civets {}", state.civets),
            StatKind::Fruit => format!("Fruit {:.0}", state.coffee_fruit),
            StatKind::Feed => format!("Feed {:.0}", state.civet_feed),
            StatKind::Beans => format!("Beans {:.1}", state.processed_beans),
            StatKind::Roasted => format!("Roast {:.1}", state.roasted_coffee),
            StatKind::Money => format!("${}", state.money),
            StatKind::Suspicion => format!("Susp {:.0}%", state.suspicion),
            StatKind::Happiness => format!("Happy {:.0}%", state.civet_happiness),
            StatKind::Reputation => format!("Rep {}", state.reputation),
            StatKind::Paperwork => format!("Paper {}", state.paperwork_level),
            StatKind::Mailbox => mailbox_summary(&state),
            StatKind::Upgrades => format!("Upgrades: {}", upgrade_summary(&state)),
            StatKind::Order => order_summary(&state),
        };
        **text = value;
        color.0 = match stat.0 {
            StatKind::Suspicion if state.suspicion >= 75.0 => Color::srgb(1.0, 0.18, 0.12),
            StatKind::Suspicion if state.suspicion >= 45.0 => Color::srgb(1.0, 0.58, 0.24),
            StatKind::Happiness if state.civet_happiness < 40.0 => Color::srgb(1.0, 0.28, 0.18),
            StatKind::Money if state.money < 20 => Color::srgb(1.0, 0.38, 0.22),
            StatKind::Mailbox if state.event.is_some() || state.pending_order.is_some() => {
                Color::srgb(1.0, 0.82, 0.42)
            }
            _ => Color::srgb(0.97, 0.92, 0.78),
        };
    }
}

fn mailbox_summary(state: &GameState) -> String {
    let mut letters = 0;
    if state.event.is_some() {
        letters += 1;
    }
    if state.pending_order.is_some() {
        letters += 1;
    }
    if letters == 0 {
        "Mail 0".to_string()
    } else {
        format!("Mail {letters}")
    }
}

fn order_summary(state: &GameState) -> String {
    if let Some(order) = &state.active_order {
        format!("Order {:.1} by d{}", order.bags, order.due_day)
    } else if state.pending_order.is_some() {
        "Order offer".to_string()
    } else {
        "Order none".to_string()
    }
}

fn upgrade_summary(state: &GameState) -> String {
    let mut names = Vec::new();
    if state.legal_office {
        names.push("legal");
    }
    if state.caretaker {
        names.push("caretaker");
    }
    if state.fruit_sorter {
        names.push("sorter");
    }
    if state.roasting_shed {
        names.push("roaster");
    }
    if state.tasting_room {
        names.push("tasting");
    }

    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(", ")
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
    skin: Res<UiSkinAssets>,
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
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(14)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.42, 0.03, 0.02, 0.90)),
                ui_skin_node(&skin, SKIN_STATS_PANEL, Color::srgba(1.0, 0.64, 0.54, 0.85)),
                BorderColor::all(Color::srgba(1.0, 0.68, 0.40, 0.72)),
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
                spawn_button(modal, &skin, "Show paperwork", Action::InspectPaperwork);
                spawn_button(modal, &skin, "Offer coffee tasting", Action::InspectTasting);
                spawn_button(modal, &skin, "Blame the goat", Action::InspectGoat);
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
    skin: Res<UiSkinAssets>,
    modal: Query<(Entity, &DayModalKind), With<DayModal>>,
) {
    let should_show = state.day_report.is_some() || state.game_result.is_some();
    let desired_kind = if state.day_report.is_some() {
        Some(DayModalKind::DayReport)
    } else if state.game_result.is_some() {
        Some(DayModalKind::FinalVerdict)
    } else {
        None
    };
    let mut exists = false;

    if should_show {
        let desired_kind = desired_kind.expect("modal kind checked by should_show");
        for (entity, kind) in &modal {
            exists = true;
            if *kind != desired_kind {
                commands.entity(entity).despawn();
                exists = false;
            }
        }
        if exists {
            return;
        }
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
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(14)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.08, 0.055, 0.91)),
                ui_skin_node(&skin, SKIN_STATS_PANEL, Color::srgba(0.88, 1.0, 0.74, 0.82)),
                BorderColor::all(Color::srgba(0.84, 1.0, 0.62, 0.55)),
                GlobalZIndex(9),
                DayModal,
                desired_kind,
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
                        spawn_button(modal, &skin, "Begin next day", Action::ContinueDay);
                    } else {
                        spawn_button(modal, &skin, "View final verdict", Action::ContinueDay);
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
    } else {
        for (entity, _) in &modal {
            commands.entity(entity).despawn();
        }
    }
}

pub fn refresh_event_modal(
    mut commands: Commands,
    state: Res<GameState>,
    skin: Res<UiSkinAssets>,
    modal: Query<Entity, With<EventModal>>,
) {
    let should_show = state.screen == GameScreen::Playing
        && state.current_room == PlantationRoom::PaperworkOffice
        && state.event.is_some();
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
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(14)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.12, 0.075, 0.035, 0.91)),
                ui_skin_node(&skin, SKIN_PAPER_PANEL, Color::srgba(1.0, 0.90, 0.72, 0.90)),
                BorderColor::all(Color::srgba(1.0, 0.76, 0.42, 0.56)),
                GlobalZIndex(8),
                EventModal,
            ))
            .with_children(|modal| {
                modal.spawn((
                    Text::new(format!("Mailbox: {}", event.title)),
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
                spawn_button(modal, &skin, a, Action::EventOptionA);
                spawn_button(modal, &skin, b, Action::EventOptionB);
                spawn_button(modal, &skin, c, Action::EventOptionC);
            });
    } else if !should_show && exists {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    }
}

pub fn refresh_order_modal(
    mut commands: Commands,
    state: Res<GameState>,
    skin: Res<UiSkinAssets>,
    modal: Query<Entity, With<OrderModal>>,
) {
    let should_show = state.screen == GameScreen::Playing
        && state.current_room == PlantationRoom::PaperworkOffice
        && state.pending_order.is_some();
    let exists = !modal.is_empty();

    if should_show && !exists {
        let order = state.pending_order.as_ref().expect("order checked above");
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(28),
                    top: percent(47),
                    width: percent(48),
                    padding: UiRect::all(px(22)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(14)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.06, 0.09, 0.055, 0.91)),
                ui_skin_node(&skin, SKIN_PAPER_PANEL, Color::srgba(0.88, 1.0, 0.76, 0.88)),
                BorderColor::all(Color::srgba(0.76, 1.0, 0.48, 0.56)),
                GlobalZIndex(7),
                OrderModal,
            ))
            .with_children(|modal| {
                modal.spawn((
                    Text::new("Mailbox: Premium Coffee Contract"),
                    TextFont {
                        font_size: 31.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.86, 1.0, 0.48)),
                ));
                modal.spawn((
                    Text::new(format!(
                        "{} wants {:.1} roasted bags by day {}. Payout ${}, reputation +{}, suspicion +{:.1}%.\nThe letter waits in the mailbox until its due day.",
                        order.client,
                        order.bags,
                        order.due_day,
                        order.payout,
                        order.reputation_reward,
                        order.suspicion_risk
                    )),
                    TextFont {
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.94, 0.91, 0.75)),
                ));
                modal.spawn((
                    Text::new("The contract is legitimate. The word 'discreet' appears seven times."),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.78, 0.88, 0.70)),
                ));
                spawn_button(modal, &skin, "Accept contract", Action::AcceptOrder);
                spawn_button(modal, &skin, "Decline politely", Action::DeclineOrder);
            });
    } else if !should_show && exists {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    }
}

pub fn refresh_animal_panel(
    mut commands: Commands,
    state: Res<GameState>,
    skin: Res<UiSkinAssets>,
    panel: Query<Entity, With<AnimalPanel>>,
) {
    let should_show = state.screen == GameScreen::Playing
        && state.selected_civet.is_some()
        && !state.inspection
        && state.day_report.is_none()
        && state.game_result.is_none();
    let exists = !panel.is_empty();

    if should_show && !exists {
        let index = state.selected_civet.expect("selected checked above");
        let Some(profile) = state.civet_profiles.get(index) else {
            return;
        };

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: px(360),
                    top: px(48),
                    width: px(300),
                    padding: UiRect::all(px(16)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(9),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(14)),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 0.78, 0.72, 0.90)),
                ui_skin_node(&skin, SKIN_PAPER_PANEL, Color::srgba(1.0, 0.88, 0.82, 0.93)),
                BorderColor::all(Color::srgba(0.55, 0.24, 0.16, 0.50)),
                GlobalZIndex(5),
                AnimalPanel,
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new(profile.name.clone()),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.25, 0.15, 0.10)),
                ));
                panel.spawn((
                    Text::new(format!("{}.", profile.note)),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.36, 0.23, 0.18)),
                ));
                panel.spawn((
                    Text::new(format!(
                        "Mood {:.0}%  Hunger {:.0}%\nFavorite: {}",
                        profile.mood, profile.hunger, profile.favorite_fruit
                    )),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.30, 0.20, 0.14)),
                ));

                spawn_button(panel, &skin, "Feed fruit tray", Action::FeedSelectedCivet);
                spawn_button(panel, &skin, "Pet gently", Action::PetSelectedCivet);
                spawn_button(panel, &skin, "Inspect notes", Action::InspectSelectedCivet);
                spawn_button(panel, &skin, "Tiny brush", Action::UseTinyBrush);
                spawn_button(panel, &skin, "Ribbon collar", Action::UseRibbonCollar);
                spawn_button(panel, &skin, "Fruit puzzle", Action::UseFruitPuzzle);
                spawn_button(panel, &skin, "Close", Action::CloseAnimalPanel);
            });
    } else if !should_show && exists {
        for entity in &panel {
            commands.entity(entity).despawn();
        }
    }
}

pub fn refresh_screen_modal(
    mut commands: Commands,
    state: Res<GameState>,
    skin: Res<UiSkinAssets>,
    modal: Query<Entity, With<ScreenModal>>,
) {
    let should_show = state.screen != GameScreen::Playing;
    let exists = !modal.is_empty();

    if should_show && exists && state.is_changed() {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
        return;
    }

    if should_show && !exists {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(18),
                    top: percent(10),
                    width: percent(64),
                    padding: UiRect::all(px(24)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(14),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(16)),
                    ..default()
                },
                BackgroundColor(PANEL_PAPER),
                ui_skin_node(&skin, SKIN_PAPER_PANEL, Color::srgba(1.0, 0.92, 0.82, 0.94)),
                BorderColor::all(Color::srgba(0.48, 0.23, 0.14, 0.42)),
                GlobalZIndex(20),
                ScreenModal,
            ))
            .with_children(|modal| match state.screen {
                GameScreen::MainMenu => spawn_main_menu(modal, &skin),
                GameScreen::Intro => spawn_intro(modal, &skin),
                GameScreen::AnimalBook => spawn_animal_book(modal, &skin, &state),
                GameScreen::Playing => {}
            });
    } else if !should_show && exists {
        for entity in &modal {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_main_menu(parent: &mut ChildSpawnerCommands, skin: &UiSkinAssets) {
    parent.spawn((
        Text::new("EutherCivet"),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(0.25, 0.18, 0.11)),
    ));
    parent.spawn((
        Text::new("A cute civet coffee sanctuary with excellent beans, soft paws, and extremely suspicious paperwork."),
        TextFont {
            font_size: 19.0,
            ..default()
        },
        TextColor(Color::srgb(0.36, 0.23, 0.18)),
    ));
    spawn_button(parent, skin, "Start plantation", Action::StartGame);
    spawn_button(parent, skin, "What is this company?", Action::ShowIntro);
    spawn_button(parent, skin, "Meet the animals", Action::ShowAnimalBook);
}

fn spawn_intro(parent: &mut ChildSpawnerCommands, skin: &UiSkinAssets) {
    parent.spawn((
        Text::new("What EutherCivet Stands For"),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.25, 0.18, 0.11)),
    ));
    parent.spawn((
        Text::new(
            "You run a fair-trade palm civet coffee plantation. The mission is simple: grow coffee fruit, care for the animals, collect processed beans, roast premium coffee, and prove every day that a sweet wildlife sanctuary is not an international criminal enterprise.",
        ),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.36, 0.23, 0.18)),
    ));
    parent.spawn((
        Text::new(
            "The tone is gentle on the animals, dry about bureaucracy, and deadly serious about good coffee.",
        ),
        TextFont {
            font_size: 17.0,
            ..default()
        },
        TextColor(Color::srgb(0.40, 0.28, 0.20)),
    ));
    spawn_button(parent, skin, "Start plantation", Action::StartGame);
    spawn_button(parent, skin, "Meet the animals", Action::ShowAnimalBook);
    spawn_button(parent, skin, "Back to menu", Action::BackToMenu);
}

fn spawn_animal_book(parent: &mut ChildSpawnerCommands, skin: &UiSkinAssets, state: &GameState) {
    parent.spawn((
        Text::new("Meet the Animals"),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.25, 0.18, 0.11)),
    ));
    let profiles = if state.civet_profiles.is_empty() {
        default_civet_profiles()
    } else {
        state.civet_profiles.clone()
    };
    for profile in profiles.iter() {
        parent.spawn((
            Text::new(format!(
                "{}: {}, favorite {}",
                profile.name, profile.note, profile.favorite_fruit
            )),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.36, 0.23, 0.18)),
        ));
    }
    parent.spawn((
        Text::new("Binturong: sleeps like a board member. Goat: appears without portfolio."),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.40, 0.28, 0.20)),
    ));
    spawn_button(parent, skin, "Start plantation", Action::StartGame);
    spawn_button(parent, skin, "Company mission", Action::ShowIntro);
    spawn_button(parent, skin, "Back to menu", Action::BackToMenu);
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
