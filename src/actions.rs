use crate::model::{Action, GameState, PlantationRoom, RandomEventKind, ToolGroup};

pub fn run_action(state: &mut GameState, action: Action) {
    match action {
        Action::StartGame => {
            state.screen = crate::model::GameScreen::Playing;
            state.log_line(
                "The plantation opens for business. Everyone looks adorable and audited.",
            );
            return;
        }
        Action::ShowIntro => {
            state.screen = crate::model::GameScreen::Intro;
            return;
        }
        Action::ShowAnimalBook => {
            state.screen = crate::model::GameScreen::AnimalBook;
            return;
        }
        Action::BackToMenu => {
            state.screen = crate::model::GameScreen::MainMenu;
            return;
        }
        Action::GoSanctuary
        | Action::GoCoffeeField
        | Action::GoRoastery
        | Action::GoPaperworkOffice => {
            switch_room(state, action);
            return;
        }
        Action::ShowCareTools
        | Action::ShowFieldTools
        | Action::ShowProductionTools
        | Action::ShowComplianceTools
        | Action::ShowUpgradeTools
        | Action::ShowSystemTools => {
            switch_tool_group(state, action);
            return;
        }
        _ => {}
    }

    if matches!(action, Action::ContinueDay) {
        state.day_report = None;
        if state.game_result.is_none() {
            state.log_line(format!("Day {} begins. The ledgers look awake.", state.day));
        }
        return;
    }

    if matches!(action, Action::Save) {
        state.save();
        return;
    }

    if matches!(action, Action::Load) {
        if let Some(loaded) = GameState::load() {
            *state = loaded;
        } else {
            state.log_line("No save file found.");
        }
        return;
    }

    if matches!(action, Action::CloseAnimalPanel) {
        state.selected_civet = None;
        state.dirty_visuals = true;
        state.log_line("Animal care clipboard closed.");
        return;
    }

    if matches!(
        action,
        Action::EventOptionA | Action::EventOptionB | Action::EventOptionC
    ) {
        resolve_event(state, action);
        return;
    }

    if matches!(action, Action::AcceptOrder | Action::DeclineOrder) {
        resolve_pending_order(state, action);
        return;
    }

    if state.day_report.is_some() || state.game_result.is_some() {
        state.log_line("The day report is waiting for acknowledgement.");
        return;
    }

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
            state.dirty_visuals = true;
            state.log_line(format!("Harvested {gained:.0} coffee fruit."));
        }
        Action::FeedCivets => {
            let wanted = state.civets as f32 * 5.0;
            let fed = state.coffee_fruit.min(wanted);
            if fed > 0.0 {
                state.coffee_fruit -= fed;
                state.civet_feed += fed;
                let sorter_bonus = if state.fruit_sorter { 4.0 } else { 0.0 };
                state.civet_happiness += 8.0 + sorter_bonus + fed * 0.25;
                state.suspicion -= 1.0;
                state.dirty_visuals = true;
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
            state.dirty_visuals = true;
            state.log_line(format!(
                "Collected {found:.1} processed beans from the civet area."
            ));
        }
        Action::RoastCoffee => {
            let batch = state.processed_beans.min(8.0);
            if batch >= 1.0 {
                state.processed_beans -= batch;
                let yield_rate = if state.roasting_shed { 0.96 } else { 0.82 };
                state.roasted_coffee += batch * yield_rate;
                state.money -= if state.roasting_shed { 1 } else { 2 };
                state.suspicion += if state.roasting_shed { 0.4 } else { 0.8 };
                state.dirty_visuals = true;
                state.log_line("Roasted a premium batch. Smoke plume described as theatrical.");
            } else {
                state.log_line("Not enough processed beans to roast.");
            }
        }
        Action::SellCoffee => {
            let sold = state.roasted_coffee.min(8.0);
            if sold >= 1.0 {
                let tasting_bonus = if state.tasting_room { 5.0 } else { 0.0 };
                let earned =
                    (sold * (13.0 + tasting_bonus + state.reputation as f32 * 0.7)).round() as i32;
                state.roasted_coffee -= sold;
                state.money += earned;
                state.daily_sales += earned;
                state.reputation += 1 + (sold / 5.0) as i32 + i32::from(state.tasting_room);
                state.suspicion += if sold > 6.0 {
                    if state.tasting_room { 2.5 } else { 4.0 }
                } else {
                    1.2
                };
                state.dirty_visuals = true;
                state.log_line(format!(
                    "Sold {sold:.1} bags of civet coffee for ${earned}."
                ));
            } else {
                state.log_line("No roasted coffee ready to sell.");
            }
        }
        Action::DeliverOrder => deliver_order(state),
        Action::FeedSelectedCivet => feed_selected_civet(state),
        Action::PetSelectedCivet => pet_selected_civet(state),
        Action::InspectSelectedCivet => inspect_selected_civet(state),
        Action::UseTinyBrush => use_inventory_item(state, crate::model::InventoryItem::TinyBrush),
        Action::UseRibbonCollar => {
            use_inventory_item(state, crate::model::InventoryItem::RibbonCollar)
        }
        Action::UseFruitPuzzle => {
            use_inventory_item(state, crate::model::InventoryItem::FruitPuzzle)
        }
        Action::ImproveEnclosure => {
            let cost = 45 + state.enclosure_level as i32 * 20;
            if state.money >= cost {
                state.money -= cost;
                state.daily_expenses += cost;
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
            let cost = if state.legal_office {
                8 + state.paperwork_level as i32 * 2
            } else {
                16 + state.paperwork_level as i32 * 3
            };
            if state.money >= cost {
                state.money -= cost;
                state.daily_expenses += cost;
                state.paperwork_level += 1;
                let legal_bonus = if state.legal_office { 8.0 } else { 0.0 };
                state.suspicion -= 18.0 + legal_bonus + state.paperwork_level as f32;
                state.reputation += 1;
                state.dirty_visuals = true;
                state.log_line(
                    "Presented receipts, permits, civet dental charts, and bean custody forms.",
                );
            } else {
                state.log_line("Not enough money to print the paperwork annex.");
            }
        }
        Action::BuildLegalOffice => buy_upgrade(
            state,
            110,
            |state| state.legal_office,
            |state| state.legal_office = true,
            "Built Legal Office. Suspicion now has to wait in reception.",
        ),
        Action::HireCaretaker => buy_upgrade(
            state,
            85,
            |state| state.caretaker,
            |state| state.caretaker = true,
            "Hired caretaker. Civets receive professional attention and fewer dramatic sighs.",
        ),
        Action::BuildFruitSorter => buy_upgrade(
            state,
            95,
            |state| state.fruit_sorter,
            |state| state.fruit_sorter = true,
            "Installed fruit sorter. Low-quality fruit is now rejected before the civets can judge you.",
        ),
        Action::BuildRoastingShed => buy_upgrade(
            state,
            125,
            |state| state.roasting_shed,
            |state| state.roasting_shed = true,
            "Built roasting shed. Smoke is now artisanal instead of incriminating.",
        ),
        Action::BuildTastingRoom => buy_upgrade(
            state,
            140,
            |state| state.tasting_room,
            |state| state.tasting_room = true,
            "Opened tasting room. Guests pay extra to misunderstand the business in person.",
        ),
        Action::Save => state.save(),
        Action::Load => {
            if let Some(loaded) = GameState::load() {
                *state = loaded;
            } else {
                state.log_line("No save file found.");
            }
        }
        Action::InspectPaperwork | Action::InspectTasting | Action::InspectGoat => {}
        Action::EventOptionA | Action::EventOptionB | Action::EventOptionC => {}
        Action::AcceptOrder | Action::DeclineOrder => {}
        Action::CloseAnimalPanel => {}
        Action::GoSanctuary
        | Action::GoCoffeeField
        | Action::GoRoastery
        | Action::GoPaperworkOffice => {}
        Action::ShowCareTools
        | Action::ShowFieldTools
        | Action::ShowProductionTools
        | Action::ShowComplianceTools
        | Action::ShowUpgradeTools
        | Action::ShowSystemTools => {}
        Action::StartGame | Action::ShowIntro | Action::ShowAnimalBook | Action::BackToMenu => {}
        Action::ContinueDay => {}
    }

    if state.civet_happiness < 35.0 {
        state.suspicion += 2.0;
        state.reputation -= 1;
    }
    state.clamp();
}

fn switch_room(state: &mut GameState, action: Action) {
    let (room, label) = match action {
        Action::GoSanctuary => (PlantationRoom::Sanctuary, "Sanctuary"),
        Action::GoCoffeeField => (PlantationRoom::CoffeeField, "Coffee Field"),
        Action::GoRoastery => (PlantationRoom::Roastery, "Roastery"),
        Action::GoPaperworkOffice => (PlantationRoom::PaperworkOffice, "Paperwork Office"),
        _ => return,
    };

    state.current_room = room;
    state.active_tool_group = match room {
        PlantationRoom::Sanctuary => ToolGroup::Care,
        PlantationRoom::CoffeeField => ToolGroup::Field,
        PlantationRoom::Roastery => ToolGroup::Production,
        PlantationRoom::PaperworkOffice => ToolGroup::Compliance,
    };
    if room != PlantationRoom::Sanctuary {
        state.selected_civet = None;
    }
    state.dirty_visuals = true;
    state.log_line(format!("Moved to {label}. Everything is still legal."));
}

fn switch_tool_group(state: &mut GameState, action: Action) {
    let group = match action {
        Action::ShowCareTools => ToolGroup::Care,
        Action::ShowFieldTools => ToolGroup::Field,
        Action::ShowProductionTools => ToolGroup::Production,
        Action::ShowComplianceTools => ToolGroup::Compliance,
        Action::ShowUpgradeTools => ToolGroup::Upgrades,
        Action::ShowSystemTools => ToolGroup::System,
        _ => return,
    };
    state.active_tool_group = group;
}

pub fn select_civet_by_index(state: &mut GameState, index: usize) {
    state.ensure_civet_profiles();
    if index >= state.civet_profiles.len() {
        return;
    }

    state.selected_civet = Some(index);
    let profile = &state.civet_profiles[index];
    state.log_line(format!(
        "{} trots over. Mood {:.0}%, hunger {:.0}%.",
        profile.name, profile.mood, profile.hunger
    ));
}

fn selected_civet_index(state: &mut GameState) -> Option<usize> {
    state.ensure_civet_profiles();
    let index = state.selected_civet?;
    (index < state.civet_profiles.len()).then_some(index)
}

fn feed_selected_civet(state: &mut GameState) {
    let Some(index) = selected_civet_index(state) else {
        state.log_line("Select a civet first.");
        return;
    };

    if state.coffee_fruit < 2.0 {
        state.civet_happiness -= 2.0;
        state.suspicion += 1.2;
        state.log_line("Not enough coffee fruit for a personal snack tray.");
        state.clamp();
        return;
    }

    state.coffee_fruit -= 2.0;
    state.civet_feed += 1.0;
    let profile = &mut state.civet_profiles[index];
    profile.hunger -= 28.0;
    profile.mood += 8.0;
    let name = profile.name.clone();
    state.civet_happiness += 4.0;
    state.suspicion -= 0.8;
    state.dirty_visuals = true;
    state.log_line(format!(
        "{name} gets a hand-picked fruit tray and approves with grave professionalism."
    ));
    state.clamp();
}

fn pet_selected_civet(state: &mut GameState) {
    let Some(index) = selected_civet_index(state) else {
        state.log_line("Select a civet first.");
        return;
    };

    let profile = &mut state.civet_profiles[index];
    profile.mood += 11.0;
    profile.hunger += 1.0;
    let name = profile.name.clone();
    state.civet_happiness += 2.5;
    state.reputation += 1;
    state.suspicion -= 0.4;
    state.dirty_visuals = true;
    state.log_line(format!(
        "{name} receives sanctuary-grade attention. This is excellent press, if anyone asks."
    ));
    state.clamp();
}

fn inspect_selected_civet(state: &mut GameState) {
    let Some(index) = selected_civet_index(state) else {
        state.log_line("Select a civet first.");
        return;
    };

    let profile = &state.civet_profiles[index];
    state.log_line(format!(
        "{}: favorite {}, mood {:.0}%, hunger {:.0}%. Note: {}.",
        profile.name, profile.favorite_fruit, profile.mood, profile.hunger, profile.note
    ));
}

fn use_inventory_item(state: &mut GameState, item: crate::model::InventoryItem) {
    let Some(index) = selected_civet_index(state) else {
        state.log_line("Select a civet first.");
        return;
    };

    if !state.inventory.contains(&item) {
        state.log_line("That item is not in the sanctuary basket.");
        return;
    }

    let profile = &mut state.civet_profiles[index];
    let name = profile.name.clone();
    match item {
        crate::model::InventoryItem::TinyBrush => {
            profile.mood += 7.0;
            profile.hunger += 0.5;
            state.civet_happiness += 2.0;
            state.reputation += 1;
            state.log_line(format!(
                "{name} gets brushed. The fur situation becomes investor-ready."
            ));
        }
        crate::model::InventoryItem::RibbonCollar => {
            profile.mood += 5.0;
            state.civet_happiness += 1.5;
            state.suspicion -= 0.7;
            state.log_line(format!(
                "{name} tries a ribbon collar and looks extremely non-cartel."
            ));
        }
        crate::model::InventoryItem::FruitPuzzle => {
            profile.mood += 8.0;
            profile.hunger -= 4.0;
            state.civet_happiness += 2.5;
            state.suspicion -= 0.4;
            state.log_line(format!(
                "{name} works on a fruit puzzle with tiny, serious paws."
            ));
        }
    }

    state.dirty_visuals = true;
    state.clamp();
}

fn resolve_pending_order(state: &mut GameState, action: Action) {
    let Some(order) = state.pending_order.take() else {
        state.log_line("No premium order is waiting.");
        return;
    };

    match action {
        Action::AcceptOrder => {
            state.log_line(format!(
                "Accepted order from {}: {:.1} bags due day {}.",
                order.client, order.bags, order.due_day
            ));
            state.active_order = Some(order);
            state.suspicion += 1.5;
        }
        Action::DeclineOrder => {
            state.log_line(format!(
                "Declined {}. The contract used too many quiet adjectives.",
                order.client
            ));
            state.reputation -= 1;
            state.suspicion -= 1.5;
        }
        _ => {}
    }
    state.clamp();
}

fn deliver_order(state: &mut GameState) {
    let Some(order) = state.active_order.clone() else {
        state.log_line("No active premium order to deliver.");
        return;
    };

    if state.roasted_coffee < order.bags {
        state.log_line(format!(
            "Order needs {:.1} roasted bags. Current stock is {:.1}.",
            order.bags, state.roasted_coffee
        ));
        return;
    }

    state.roasted_coffee -= order.bags;
    state.money += order.payout;
    state.daily_sales += order.payout;
    state.reputation += order.reputation_reward + i32::from(state.tasting_room);
    let legal_reduction = if state.legal_office { 1.5 } else { 0.0 };
    state.suspicion += (order.suspicion_risk - legal_reduction).max(0.5);
    state.active_order = None;
    state.log_line(format!(
        "Delivered premium order to {} for ${}.",
        order.client, order.payout
    ));
    state.clamp();
}

fn buy_upgrade(
    state: &mut GameState,
    cost: i32,
    already_bought: impl Fn(&GameState) -> bool,
    apply: impl Fn(&mut GameState),
    message: &'static str,
) {
    if already_bought(state) {
        state.log_line("That upgrade is already in place.");
        return;
    }
    if state.money < cost {
        state.log_line(format!("Upgrade needs ${cost}."));
        return;
    }

    state.money -= cost;
    state.daily_expenses += cost;
    state.suspicion += 2.0;
    state.reputation += 1;
    apply(state);
    state.dirty_visuals = true;
    state.log_line(message);
}

fn resolve_event(state: &mut GameState, action: Action) {
    let Some(event) = state.event.take() else {
        state.log_line("No event is waiting for a decision.");
        return;
    };

    match (event.kind, action) {
        (RandomEventKind::PoliceVisit, Action::EventOptionA) => {
            state.money -= 14;
            state.suspicion -= 13.0 + state.paperwork_level as f32 * 1.5;
            state.reputation += 1;
            state.log_line("Police accept the paperwork and leave with a laminated bean diagram.");
        }
        (RandomEventKind::PoliceVisit, Action::EventOptionB) => {
            state.roasted_coffee = (state.roasted_coffee - 2.0).max(0.0);
            state.suspicion -= 8.0;
            state.reputation += 2;
            state.log_line("Officers attend a tasting and downgrade the threat to 'nutty finish'.");
        }
        (RandomEventKind::PoliceVisit, Action::EventOptionC) => {
            state.suspicion += 9.0;
            state.reputation -= 2;
            state
                .log_line("You answer evasively. The officers write 'too much coffee confidence'.");
        }

        (RandomEventKind::JournalistQuestions, Action::EventOptionA) => {
            state.reputation += 4;
            state.suspicion += 4.0;
            state
                .log_line("The journalist loves the civets. The headline still uses 'mysterious'.");
        }
        (RandomEventKind::JournalistQuestions, Action::EventOptionB) => {
            state.money -= 20;
            state.suspicion -= 9.0;
            state.reputation += 1;
            state.log_line("You give a controlled tour. Every label says 'coffee' twice.");
        }
        (RandomEventKind::JournalistQuestions, Action::EventOptionC) => {
            state.suspicion += 12.0;
            state.reputation -= 3;
            state.log_line("No comment becomes the story. The goat is photographed in profile.");
        }

        (RandomEventKind::WelfareInspection, Action::EventOptionA) => {
            state.money -= 24;
            state.civet_happiness += 12.0;
            state.reputation += 3;
            state.suspicion -= 5.0;
            state.log_line(
                "Emergency enrichment deployed. Civets receive excellent tiny furniture.",
            );
        }
        (RandomEventKind::WelfareInspection, Action::EventOptionB) => {
            if state.civet_happiness >= 60.0 {
                state.reputation += 4;
                state.suspicion -= 7.0;
                state.log_line("Inspection passes. Civets look professionally satisfied.");
            } else {
                state.reputation -= 4;
                state.suspicion += 10.0;
                state.log_line(
                    "Inspection finds disappointed civets and suspiciously tidy excuses.",
                );
            }
        }
        (RandomEventKind::WelfareInspection, Action::EventOptionC) => {
            state.money -= 10;
            state.reputation -= 1;
            state.suspicion += 2.0;
            state.log_line("You reschedule. It works, but the clipboard remembers.");
        }

        (RandomEventKind::HelicopterOverhead, Action::EventOptionA) => {
            state.suspicion -= 7.0;
            state.money -= 12;
            state.log_line(
                "Reflective coffee tarps deployed. Perfectly normal agricultural behavior.",
            );
        }
        (RandomEventKind::HelicopterOverhead, Action::EventOptionB) => {
            state.reputation += 2;
            state.suspicion += 5.0;
            state.log_line("You wave cheerfully. This is either innocence or advanced theater.");
        }
        (RandomEventKind::HelicopterOverhead, Action::EventOptionC) => {
            state.suspicion += 13.0;
            state
                .log_line("Everyone hides. The helicopter learns nothing and suspects everything.");
        }

        (RandomEventKind::BinturongEscape, Action::EventOptionA) => {
            state.money -= 16;
            state.binturong_home = true;
            state.civet_happiness += 3.0;
            state.suspicion -= 4.0;
            state.log_line("A caretaker retrieves the binturong with snacks and quiet bargaining.");
        }
        (RandomEventKind::BinturongEscape, Action::EventOptionB) => {
            state.binturong_home = false;
            state.reputation += 1;
            state.suspicion += 7.0;
            state.log_line("The binturong becomes a local celebrity and a regulatory problem.");
        }
        (RandomEventKind::BinturongEscape, Action::EventOptionC) => {
            state.goat_present = true;
            state.binturong_home = true;
            state.reputation -= 1;
            state.suspicion += 3.0;
            state.log_line("The goat is sent as negotiator. Nobody understands why it works.");
        }

        (RandomEventKind::PickyCivet, Action::EventOptionA) => {
            let spent = state.coffee_fruit.min(8.0);
            state.coffee_fruit -= spent;
            state.civet_happiness += 10.0;
            state.reputation += 1;
            state.log_line("Only the best fruit is served. The civet accepts tribute.");
        }
        (RandomEventKind::PickyCivet, Action::EventOptionB) => {
            state.money -= 18;
            state.civet_happiness += 8.0;
            state.suspicion -= 2.0;
            state.log_line("Imported fruit arrives with more documentation than the staff.");
        }
        (RandomEventKind::PickyCivet, Action::EventOptionC) => {
            state.civet_happiness -= 9.0;
            state.suspicion += 4.0;
            state.reputation -= 1;
            state.log_line("You insist the fruit is fine. The civet disagrees in silence.");
        }

        (RandomEventKind::GoatAppearance, Action::EventOptionA) => {
            state.goat_present = true;
            state.suspicion += 2.0;
            state.reputation += 1;
            state.log_line("The goat is listed as unpaid compliance intern.");
        }
        (RandomEventKind::GoatAppearance, Action::EventOptionB) => {
            state.goat_present = false;
            state.money -= 9;
            state.suspicion -= 4.0;
            state.log_line("The goat is escorted off-site by a very serious courier.");
        }
        (RandomEventKind::GoatAppearance, Action::EventOptionC) => {
            state.goat_present = true;
            state.suspicion -= 2.0;
            state.civet_happiness += 2.0;
            state.log_line("You blame the goat preemptively. Oddly, morale improves.");
        }
        _ => {}
    }

    state.dirty_visuals = true;
    state.clamp();
}
