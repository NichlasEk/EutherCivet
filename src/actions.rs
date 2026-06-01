use crate::model::{Action, GameState};

pub fn run_action(state: &mut GameState, action: Action) {
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
                state.daily_sales += earned;
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
            let cost = 16 + state.paperwork_level as i32 * 3;
            if state.money >= cost {
                state.money -= cost;
                state.daily_expenses += cost;
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
        Action::ContinueDay => {}
    }

    if state.civet_happiness < 35.0 {
        state.suspicion += 2.0;
        state.reputation -= 1;
    }
    state.clamp();
}
