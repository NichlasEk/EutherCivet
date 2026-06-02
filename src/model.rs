use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const SAVE_PATH: &str = "euther_civet_save.json";

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameState {
    #[serde(default)]
    pub screen: GameScreen,
    pub day: u32,
    pub coffee_plants: u32,
    pub civets: u32,
    #[serde(default = "default_civet_names")]
    pub civet_names: Vec<String>,
    #[serde(default = "default_civet_profiles")]
    pub civet_profiles: Vec<CivetProfile>,
    #[serde(default)]
    pub selected_civet: Option<usize>,
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
    #[serde(default)]
    pub legal_office: bool,
    #[serde(default)]
    pub caretaker: bool,
    #[serde(default)]
    pub fruit_sorter: bool,
    #[serde(default)]
    pub roasting_shed: bool,
    #[serde(default)]
    pub tasting_room: bool,
    pub binturong_home: bool,
    pub goat_present: bool,
    pub inspection: bool,
    #[serde(default)]
    pub event: Option<EventState>,
    #[serde(default)]
    pub pending_order: Option<OrderOffer>,
    #[serde(default)]
    pub active_order: Option<OrderOffer>,
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

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum RandomEventKind {
    PoliceVisit,
    JournalistQuestions,
    WelfareInspection,
    HelicopterOverhead,
    BinturongEscape,
    PickyCivet,
    GoatAppearance,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EventState {
    pub kind: RandomEventKind,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderOffer {
    pub client: String,
    pub bags: f32,
    pub payout: i32,
    pub reputation_reward: i32,
    pub suspicion_risk: f32,
    pub due_day: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CivetProfile {
    pub name: String,
    pub hunger: f32,
    pub mood: f32,
    pub favorite_fruit: String,
    pub note: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameScreen {
    #[default]
    MainMenu,
    Intro,
    AnimalBook,
    Playing,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            screen: GameScreen::MainMenu,
            day: 1,
            coffee_plants: 6,
            civets: 3,
            civet_names: default_civet_names(),
            civet_profiles: default_civet_profiles(),
            selected_civet: None,
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
            legal_office: false,
            caretaker: false,
            fruit_sorter: false,
            roasting_shed: false,
            tasting_room: false,
            binturong_home: true,
            goat_present: true,
            inspection: false,
            event: None,
            pending_order: None,
            active_order: None,
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

pub fn default_civet_names() -> Vec<String> {
    ["Miso", "Kanel", "Beanie"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

pub fn default_civet_profiles() -> Vec<CivetProfile> {
    [
        (
            "Miso",
            24.0,
            78.0,
            "ruby coffee cherries",
            "chief fruit critic",
        ),
        (
            "Kanel",
            32.0,
            72.0,
            "soft yellow fruit",
            "night-shift bean philosopher",
        ),
        (
            "Beanie",
            27.0,
            80.0,
            "tiny overripe fruit",
            "small paws, large opinions",
        ),
    ]
    .into_iter()
    .map(|(name, hunger, mood, favorite_fruit, note)| CivetProfile {
        name: name.to_string(),
        hunger,
        mood,
        favorite_fruit: favorite_fruit.to_string(),
        note: note.to_string(),
    })
    .collect()
}

impl GameState {
    pub fn load() -> Option<Self> {
        let text = fs::read_to_string(SAVE_PATH).ok()?;
        let mut state: Self = serde_json::from_str(&text).ok()?;
        state.ensure_civet_profiles();
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

    pub fn ensure_civet_profiles(&mut self) {
        if self.civet_profiles.is_empty() {
            self.civet_profiles = default_civet_profiles();
        }
        while self.civet_profiles.len() < self.civets as usize {
            let next = self.civet_profiles.len() + 1;
            self.civet_profiles.push(CivetProfile {
                name: format!("Civet {next}"),
                hunger: 35.0,
                mood: 68.0,
                favorite_fruit: "carefully documented coffee fruit".to_string(),
                note: "new sanctuary resident".to_string(),
            });
        }
        self.civet_names = self
            .civet_profiles
            .iter()
            .map(|profile| profile.name.clone())
            .collect();
        if self
            .selected_civet
            .is_some_and(|idx| idx >= self.civet_profiles.len())
        {
            self.selected_civet = None;
        }
    }

    pub fn clamp(&mut self) {
        self.ensure_civet_profiles();
        for profile in &mut self.civet_profiles {
            profile.hunger = profile.hunger.clamp(0.0, 100.0);
            profile.mood = profile.mood.clamp(0.0, 100.0);
        }
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

#[derive(Resource)]
pub struct OrderTick(pub Timer);

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

#[derive(Component)]
pub struct EventModal;

#[derive(Component)]
pub struct OrderModal;

#[derive(Component)]
pub struct ScreenModal;

#[derive(Component)]
pub struct AnimalPanel;

#[derive(Component)]
pub struct CivetClickTarget {
    pub index: usize,
}

#[derive(Component, Clone, Copy)]
pub struct ActionButton(pub Action);

#[derive(Component, Clone, Copy)]
pub struct DynamicButtonText(pub Action);

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
    Upgrades,
    Order,
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
    BuildLegalOffice,
    HireCaretaker,
    BuildFruitSorter,
    BuildRoastingShed,
    BuildTastingRoom,
    DeliverOrder,
    Save,
    Load,
    FeedSelectedCivet,
    PetSelectedCivet,
    InspectSelectedCivet,
    CloseAnimalPanel,
    StartGame,
    ShowIntro,
    ShowAnimalBook,
    BackToMenu,
    ContinueDay,
    EventOptionA,
    EventOptionB,
    EventOptionC,
    AcceptOrder,
    DeclineOrder,
    InspectPaperwork,
    InspectTasting,
    InspectGoat,
}
