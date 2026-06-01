use bevy::prelude::*;

use crate::model::{DayReport, DayTick, EventTick, GameResult, GameState, GameTick};

pub fn tick_game(time: Res<Time>, mut timer: ResMut<GameTick>, mut state: ResMut<GameState>) {
    if !timer.0.tick(time.delta()).just_finished()
        || state.inspection
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
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

pub fn advance_day(time: Res<Time>, mut timer: ResMut<DayTick>, mut state: ResMut<GameState>) {
    if !timer.0.tick(time.delta()).just_finished()
        || state.inspection
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
        return;
    }

    let report = settle_day(&mut state);
    state.day_report = Some(report);
}

fn settle_day(state: &mut GameState) -> DayReport {
    let day = state.day;
    let upkeep = 18
        + state.civets as i32 * 5
        + state.enclosure_level as i32 * 7
        + state.paperwork_level as i32 * 3
        + (state.coffee_plants as i32 / 3);
    state.money -= upkeep;
    state.daily_expenses += upkeep;

    let mut reputation_delta = 0;
    let mut suspicion_delta = 0.0;

    if state.daily_sales >= 120 {
        reputation_delta += 2;
        suspicion_delta += 3.5;
    } else if state.daily_sales <= 0 {
        reputation_delta -= 1;
        suspicion_delta += 1.0;
    }

    if state.civet_happiness >= 75.0 {
        reputation_delta += 2;
        suspicion_delta -= 3.0;
    } else if state.civet_happiness < 35.0 {
        reputation_delta -= 3;
        suspicion_delta += 8.0;
    }

    if state.suspicion >= 80.0 {
        reputation_delta -= 1;
    }
    if state.paperwork_level >= state.day {
        suspicion_delta -= 2.5;
    }

    state.reputation += reputation_delta;
    state.suspicion += suspicion_delta;
    state.clamp();

    let title = if state.suspicion >= 85.0 {
        "Daily Report: Everyone Is Being Very Calm".to_string()
    } else if state.civet_happiness < 40.0 {
        "Daily Report: Civet Morale Committee Convenes".to_string()
    } else if state.daily_sales >= 120 {
        "Daily Report: Premium Beans, Premium Questions".to_string()
    } else {
        "Daily Report: Boring Coffee, Dramatic Shadows".to_string()
    };

    let summary = format!(
        "Sales ${}. Operating costs ${}. Reputation {:+}. Suspicion {:+.1}%.",
        state.daily_sales, state.daily_expenses, reputation_delta, suspicion_delta
    );

    state.log_line(format!(
        "End of day {day}: sales ${}, costs ${}, reputation {:+}, suspicion {:+.1}%.",
        state.daily_sales, state.daily_expenses, reputation_delta, suspicion_delta
    ));

    state.daily_sales = 0;
    state.daily_expenses = 0;

    if state.money < -80 {
        state.game_result = Some(GameResult::Failed(
            "The plantation collapses under debt. The goat denies fiduciary responsibility."
                .to_string(),
        ));
    } else if state.reputation <= -8 {
        state.game_result = Some(GameResult::Failed(
            "Reputation bottoms out. Reviewers describe the coffee as 'procedurally concerning'."
                .to_string(),
        ));
    } else if day >= 7 {
        if state.money >= 320 && state.reputation >= 18 && state.suspicion < 80.0 {
            state.game_result = Some(GameResult::Won(
                "Seven days survived: profitable, reputable, and only moderately surveilled."
                    .to_string(),
            ));
        } else {
            state.game_result = Some(GameResult::Failed(
                "Seven days pass, but the board calls the result 'not yet investable'.".to_string(),
            ));
        }
    } else {
        state.day += 1;
    }

    DayReport {
        day,
        title,
        summary,
        upkeep,
        reputation_delta,
        suspicion_delta,
    }
}

pub fn trigger_random_events(
    time: Res<Time>,
    mut timer: ResMut<EventTick>,
    mut state: ResMut<GameState>,
) {
    if !timer.0.tick(time.delta()).just_finished()
        || state.inspection
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
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
