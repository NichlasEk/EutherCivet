use bevy::prelude::*;

use crate::model::{
    DayReport, DayTick, EventState, EventTick, GameResult, GameScreen, GameState, GameTick,
    OrderOffer, OrderTick, RandomEventKind,
};

pub fn tick_game(time: Res<Time>, mut timer: ResMut<GameTick>, mut state: ResMut<GameState>) {
    let delta = time.delta().mul_f32(state.time_scale.multiplier());
    if !timer.0.tick(delta).just_finished()
        || state.screen != GameScreen::Playing
        || state.inspection
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

    update_animal_care(&mut state, eaten);

    if state.coffee_plants > 24 {
        state.suspicion += 0.25;
    }
    if state.reputation < 0 {
        state.suspicion += 0.2;
    }
    state.clamp();
}

fn update_animal_care(state: &mut GameState, eaten: f32) {
    state.ensure_civet_profiles();
    if state.civet_profiles.is_empty() {
        return;
    }

    let per_civet_food = if state.civets > 0 {
        eaten / state.civets as f32
    } else {
        0.0
    };
    let hunger_drift = if state.caretaker { 1.0 } else { 1.45 };
    let mood_support =
        state.enclosure_level as f32 * 0.18 + if state.caretaker { 0.55 } else { 0.0 };

    let mut hunger_total = 0.0;
    let mut mood_total = 0.0;
    for profile in &mut state.civet_profiles {
        profile.hunger += hunger_drift - per_civet_food * 7.5;
        if profile.hunger > 72.0 {
            profile.mood -= 1.0;
        } else if profile.hunger < 38.0 {
            profile.mood += 0.35;
        }
        profile.mood += mood_support;
        profile.hunger = profile.hunger.clamp(0.0, 100.0);
        profile.mood = profile.mood.clamp(0.0, 100.0);
        hunger_total += profile.hunger;
        mood_total += profile.mood;
    }

    let count = state.civet_profiles.len() as f32;
    let average_hunger = hunger_total / count;
    let average_mood = mood_total / count;
    let care_score = (average_mood * 0.72 + (100.0 - average_hunger) * 0.28).clamp(0.0, 100.0);
    state.civet_happiness = (state.civet_happiness * 0.84 + care_score * 0.16).clamp(0.0, 100.0);
    if average_hunger > 82.0 {
        state.suspicion += 0.45;
        state.reputation -= 1;
    }
}

pub fn advance_day(time: Res<Time>, mut timer: ResMut<DayTick>, mut state: ResMut<GameState>) {
    if state.screen != GameScreen::Playing
        || state.inspection
        || state.day_report.is_some()
        || state.game_result.is_some()
    {
        return;
    }

    let delta = time.delta().mul_f32(state.time_scale.multiplier());
    let finished = timer.0.tick(delta).just_finished();
    let duration = timer.0.duration().as_secs_f32().max(1.0);
    state.day_progress = (timer.0.elapsed().as_secs_f32() / duration).clamp(0.0, 1.0);

    if !finished {
        return;
    }

    let report = settle_day(&mut state);
    state.day_report = Some(report);
    state.day_progress = 0.0;
}

pub fn generate_order_offers(
    time: Res<Time>,
    mut timer: ResMut<OrderTick>,
    mut state: ResMut<GameState>,
) {
    let delta = time.delta().mul_f32(state.time_scale.multiplier());
    if !timer.0.tick(delta).just_finished()
        || state.screen != GameScreen::Playing
        || state.inspection
        || state.pending_order.is_some()
        || state.day_report.is_some()
        || state.game_result.is_some()
        || state.pending_order.is_some()
        || state.active_order.is_some()
    {
        return;
    }

    let client = match state.rand_index(5) {
        0 => "Nordic Embassy Breakfast Desk",
        1 => "Suspiciously Calm Boutique Hotel",
        2 => "Ministry of Agricultural Irony",
        3 => "Very Normal Import Cooperative",
        _ => "Monaco Goat-Free Espresso Bar",
    };
    let bags = 3.0 + state.rand_index(5) as f32;
    let base_price = 22.0 + state.reputation.max(0) as f32 * 0.9;
    let tasting_bonus = if state.tasting_room { 1.15 } else { 1.0 };
    let payout = (bags * base_price * tasting_bonus).round() as i32;
    let reputation_reward = 2 + (bags / 4.0) as i32;
    let suspicion_risk = 2.5 + bags * 0.55;
    let due_day = (state.day + 3).min(7);

    state.pending_order = Some(OrderOffer {
        client: client.to_string(),
        bags,
        payout,
        reputation_reward,
        suspicion_risk,
        due_day,
    });
    state.log_line("New mailbox letter: a premium buyer sends a contract.");
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

    if state
        .active_order
        .as_ref()
        .is_some_and(|order| order.due_day <= day)
    {
        state.active_order = None;
        reputation_delta -= 3;
        suspicion_delta += 6.0;
        state.log_line("Missed a premium order. The buyer files a complaint with adjectives.");
    }

    if state
        .pending_order
        .as_ref()
        .is_some_and(|order| order.due_day <= day)
    {
        state.pending_order = None;
        reputation_delta -= 1;
        suspicion_delta += 2.0;
        state.log_line("An unopened contract expires in the mailbox. Mildly bad optics.");
    }

    if state
        .event
        .as_ref()
        .is_some_and(|event| event.due_day <= day)
    {
        let title = state
            .event
            .as_ref()
            .map(|event| event.title.clone())
            .unwrap_or_else(|| "Mailbox incident".to_string());
        state.event = None;
        reputation_delta -= 1;
        suspicion_delta += 2.5;
        state.civet_happiness -= 1.5;
        state.log_line(format!(
            "{title} times out in the mailbox. The world keeps spinning."
        ));
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
    let delta = time.delta().mul_f32(state.time_scale.multiplier());
    if !timer.0.tick(delta).just_finished()
        || state.screen != GameScreen::Playing
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
        due_day: (state.day + 2).min(7),
    });
    state.log_line("New mailbox letter: an incident needs attention in the office.");
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
