use bevy::prelude::*;

use crate::model::{
    DayReport, DayTick, EventState, EventTick, GameResult, GameState, GameTick, RandomEventKind,
};

pub fn tick_game(time: Res<Time>, mut timer: ResMut<GameTick>, mut state: ResMut<GameState>) {
    if !timer.0.tick(time.delta()).just_finished()
        || state.inspection
        || state.event.is_some()
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
        return;
    }

    let fruit_growth = state.coffee_plants as f32 * 0.42;
    state.coffee_fruit += fruit_growth * if state.fruit_sorter { 1.08 } else { 1.0 };

    let appetite = state.civets as f32 * 0.65;
    let eaten = state.civet_feed.min(appetite);
    if eaten > 0.0 {
        state.civet_feed -= eaten;
        let happiness_bonus = (state.civet_happiness / 100.0).max(0.2);
        let enclosure_bonus = 1.0 + state.enclosure_level as f32 * 0.08;
        let caretaker_bonus = if state.caretaker { 1.12 } else { 1.0 };
        state.processed_beans += eaten * 0.32 * happiness_bonus * enclosure_bonus * caretaker_bonus;
        state.civet_happiness += if state.caretaker { 0.55 } else { 0.25 };
    } else {
        state.civet_happiness -= if state.caretaker { 0.6 } else { 1.4 };
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
        || state.event.is_some()
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
        + (state.coffee_plants as i32 / 3)
        + if state.legal_office { 12 } else { 0 }
        + if state.caretaker { 14 } else { 0 }
        + if state.fruit_sorter { 8 } else { 0 }
        + if state.roasting_shed { 10 } else { 0 }
        + if state.tasting_room { 12 } else { 0 };
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
    if state.legal_office {
        suspicion_delta -= 2.0;
    }
    if state.tasting_room && state.daily_sales > 0 {
        reputation_delta += 1;
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
        || state.event.is_some()
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
        return;
    }

    let kind = match state.rand_index(7) {
        0 => RandomEventKind::PoliceVisit,
        1 => RandomEventKind::JournalistQuestions,
        2 => RandomEventKind::WelfareInspection,
        3 => RandomEventKind::HelicopterOverhead,
        4 => RandomEventKind::BinturongEscape,
        5 => RandomEventKind::PickyCivet,
        _ => RandomEventKind::GoatAppearance,
    };

    state.event = Some(EventState {
        kind,
        title: event_title(kind).to_string(),
        body: event_body(kind).to_string(),
    });
}

fn event_title(kind: RandomEventKind) -> &'static str {
    match kind {
        RandomEventKind::PoliceVisit => "Local Police Visit",
        RandomEventKind::JournalistQuestions => "Journalist Asks Questions",
        RandomEventKind::WelfareInspection => "Animal Welfare Inspection",
        RandomEventKind::HelicopterOverhead => "Helicopter Overhead",
        RandomEventKind::BinturongEscape => "Binturong Escape",
        RandomEventKind::PickyCivet => "Civet Refuses Fruit",
        RandomEventKind::GoatAppearance => "Unscheduled Goat",
    }
}

fn event_body(kind: RandomEventKind) -> &'static str {
    match kind {
        RandomEventKind::PoliceVisit => {
            "Two officers arrive to ask why the legal coffee estate has a perimeter plan and mirrored sunglasses."
        }
        RandomEventKind::JournalistQuestions => {
            "A reporter wants a tour, a quote, and a plausible explanation for the phrase 'bean chain of custody'."
        }
        RandomEventKind::WelfareInspection => {
            "An animal welfare inspector has a clipboard, good shoes, and very specific civet enrichment expectations."
        }
        RandomEventKind::HelicopterOverhead => {
            "A helicopter circles low enough to read the coffee bags and mispronounce 'civet' on the radio."
        }
        RandomEventKind::BinturongEscape => {
            "The binturong exits its enclosure with the quiet confidence of a shareholder."
        }
        RandomEventKind::PickyCivet => {
            "A civet rejects today's fruit selection and makes eye contact with everyone responsible."
        }
        RandomEventKind::GoatAppearance => {
            "A goat appears inside the paperwork room. No one hired it. No one can prove that."
        }
    }
}
