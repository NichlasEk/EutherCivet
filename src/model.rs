use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const SAVE_PATH: &str = "euther_civet_save.json";

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub day: u32,
    pub coffee_plants: u32,
    pub civets: u32,
    pub coffee_fruit: f32,
    pub civet_feed: f32,
    pub processed_beans: f32,
    pub roasted_coffee: f32,
    pub money: i32,
    pub suspicion: f32,
    pub civet_happiness: f32,
    pub reputation: i32,
    pub enclosure_level: u32,
    pub paperwork_level: u32,
    pub binturong_home: bool,
    pub goat_present: bool,
    pub inspection: bool,
    pub daily_sales: i32,
    pub daily_expenses: i32,
    pub day_report: Option<DayReport>,
    pub game_result: Option<GameResult>,
    pub rng_seed: u64,
    pub log: Vec<String>,
    pub dirty_visuals: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DayReport {
    pub day: u32,
    pub title: String,
    pub summary: String,
    pub upkeep: i32,
    pub reputation_delta: i32,
    pub suspicion_delta: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GameResult {
    Won(String),
    Failed(String),
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            day: 1,
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
            daily_sales: 0,
            daily_expenses: 0,
            day_report: None,
            game_result: None,
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
    pub fn load() -> Option<Self> {
        let text = fs::read_to_string(SAVE_PATH).ok()?;
        let mut state: Self = serde_json::from_str(&text).ok()?;
        state.log_line("Loaded plantation ledger from disk.");
        state.dirty_visuals = true;
        Some(state)
    }

    pub fn save(&mut self) {
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

    pub fn log_line(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
        while self.log.len() > 8 {
            self.log.remove(0);
        }
    }

    pub fn clamp(&mut self) {
        self.suspicion = self.suspicion.clamp(0.0, 100.0);
        self.civet_happiness = self.civet_happiness.clamp(0.0, 100.0);
        if self.suspicion >= 100.0 {
            self.inspection = true;
            self.suspicion = 100.0;
            self.log_line("Operation Bitter Bean begins.");
        }
    }

    pub fn rand_index(&mut self, max: usize) -> usize {
        self.rng_seed = self
            .rng_seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.rng_seed >> 32) as usize) % max
    }
}

#[derive(Resource)]
pub struct GameTick(pub Timer);

#[derive(Resource)]
pub struct EventTick(pub Timer);

#[derive(Resource)]
pub struct DayTick(pub Timer);

#[derive(Component)]
pub struct StatText(pub StatKind);

#[derive(Component)]
pub struct StatusBar(pub StatusKind);

#[derive(Component)]
pub struct LogText;

#[derive(Component)]
pub struct WorldVisual;

#[derive(Component)]
pub struct Helicopter {
    pub offset: Vec3,
}

#[derive(Component)]
pub struct SuspicionGlow;

#[derive(Component)]
pub struct InspectionModal;

#[derive(Component)]
pub struct DayModal;

#[derive(Component, Clone, Copy)]
pub struct ActionButton(pub Action);

#[derive(Clone, Copy)]
pub enum StatKind {
    Day,
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
pub enum StatusKind {
    Suspicion,
    Happiness,
    CoffeePipeline,
}

#[derive(Clone, Copy)]
pub enum Action {
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
    ContinueDay,
    InspectPaperwork,
    InspectTasting,
    InspectGoat,
}
