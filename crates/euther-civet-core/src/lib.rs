use serde::{Deserialize, Serialize};

const MAX_LOG_LINES: usize = 8;
const EVENT_INTERVAL_TICKS: u64 = 9;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
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
    pub rng_seed: u64,
    pub log: Vec<String>,
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
        }
    }
}

impl GameState {
    pub fn log_line(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
        while self.log.len() > MAX_LOG_LINES {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EutherCivetAction {
    PlantCoffee,
    HarvestFruit,
    FeedCivets,
    CollectBeans,
    RoastCoffee,
    SellCoffee,
    ImproveEnclosure,
    ShowPaperwork,
    InspectPaperwork,
    InspectTasting,
    InspectGoat,
}

impl EutherCivetAction {
    pub fn is_inspection_action(self) -> bool {
        matches!(
            self,
            Self::InspectPaperwork | Self::InspectTasting | Self::InspectGoat
        )
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EutherCivetFrame {
    pub frame: u64,
    pub state: GameState,
    pub status: &'static str,
    pub coffee_pipeline: f32,
    pub available_actions: Vec<EutherCivetAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EutherCivetSave {
    pub frame: u64,
    pub state: GameState,
}

#[derive(Debug, Clone)]
pub struct EutherCivetRuntime {
    state: GameState,
    frame: u64,
}

impl Default for EutherCivetRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl EutherCivetRuntime {
    pub fn new() -> Self {
        Self {
            state: GameState::default(),
            frame: 0,
        }
    }

    pub fn from_state(state: GameState) -> Self {
        Self { state, frame: 0 }
    }

    pub fn from_save(save: EutherCivetSave) -> Self {
        Self {
            state: save.state,
            frame: save.frame,
        }
    }

    pub fn save(&self) -> EutherCivetSave {
        EutherCivetSave {
            frame: self.frame,
            state: self.state.clone(),
        }
    }

    pub fn reset(&mut self) -> EutherCivetFrame {
        self.state = GameState::default();
        self.frame = 0;
        self.snapshot()
    }

    pub fn snapshot(&self) -> EutherCivetFrame {
        EutherCivetFrame {
            frame: self.frame,
            state: self.state.clone(),
            status: if self.state.inspection {
                "inspection"
            } else if self.state.civet_happiness < 30.0 {
                "distressed"
            } else if self.state.suspicion >= 70.0 {
                "watched"
            } else {
                "running"
            },
            coffee_pipeline: self.coffee_pipeline(),
            available_actions: available_actions(self.state.inspection),
        }
    }

    pub fn action(&mut self, action: EutherCivetAction) -> EutherCivetFrame {
        run_action(&mut self.state, action);
        self.snapshot()
    }

    pub fn tick(&mut self) -> EutherCivetFrame {
        self.frame += 1;
        tick_game(&mut self.state);
        if self.frame % EVENT_INTERVAL_TICKS == 0 {
            trigger_random_event(&mut self.state);
        }
        self.snapshot()
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    fn coffee_pipeline(&self) -> f32 {
        let stock = self.state.coffee_fruit
            + self.state.civet_feed
            + self.state.processed_beans * 2.0
            + self.state.roasted_coffee * 4.0;
        (stock / 2.4).clamp(0.0, 100.0)
    }
}

fn available_actions(inspection: bool) -> Vec<EutherCivetAction> {
    if inspection {
        return vec![
            EutherCivetAction::InspectPaperwork,
            EutherCivetAction::InspectTasting,
            EutherCivetAction::InspectGoat,
        ];
    }
    vec![
        EutherCivetAction::PlantCoffee,
        EutherCivetAction::HarvestFruit,
        EutherCivetAction::FeedCivets,
        EutherCivetAction::CollectBeans,
        EutherCivetAction::RoastCoffee,
        EutherCivetAction::SellCoffee,
        EutherCivetAction::ImproveEnclosure,
        EutherCivetAction::ShowPaperwork,
    ]
}

fn run_action(state: &mut GameState, action: EutherCivetAction) {
    if state.inspection {
        match action {
            EutherCivetAction::InspectPaperwork => {
                let reduction = 46.0 + state.paperwork_level as f32 * 5.0;
                state.money -= 18;
                state.suspicion -= reduction;
                state.reputation += 3;
                state.inspection = false;
                state.log_line("Authorities read the paperwork and become visibly tired.");
            }
            EutherCivetAction::InspectTasting => {
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
            EutherCivetAction::InspectGoat => {
                state.suspicion -= 24.0;
                state.reputation -= 4;
                state.civet_happiness -= 6.0;
                state.goat_present = false;
                state.inspection = false;
                state.log_line("The goat accepts no blame but leaves under legal advice.");
            }
            _ => state.log_line("Normal work is paused during Operation Bitter Bean."),
        }
        state.clamp();
        return;
    }

    match action {
        EutherCivetAction::PlantCoffee => {
            if state.money >= 14 {
                state.money -= 14;
                state.coffee_plants += 1;
                state.suspicion += if state.coffee_plants > 18 { 3.5 } else { 1.2 };
                state.log_line(
                    "A coffee shrub is planted in a formation lawyers called unfortunate.",
                );
            } else {
                state.log_line("Not enough money for another coffee plant.");
            }
        }
        EutherCivetAction::HarvestFruit => {
            let gained = state.coffee_plants as f32 * 1.6;
            state.coffee_fruit += gained;
            state.civet_happiness -= 0.8;
            state.log_line(format!("Harvested {gained:.0} coffee fruit."));
        }
        EutherCivetAction::FeedCivets => {
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
        EutherCivetAction::CollectBeans => {
            let found = 1.0 + state.civets as f32 * 0.35;
            state.processed_beans += found;
            state.suspicion += 0.7;
            state.log_line(format!(
                "Collected {found:.1} processed beans from the civet area."
            ));
        }
        EutherCivetAction::RoastCoffee => {
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
        EutherCivetAction::SellCoffee => {
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
        EutherCivetAction::ImproveEnclosure => {
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
        EutherCivetAction::ShowPaperwork => {
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
        EutherCivetAction::InspectPaperwork
        | EutherCivetAction::InspectTasting
        | EutherCivetAction::InspectGoat => {}
    }

    if state.civet_happiness < 35.0 {
        state.suspicion += 2.0;
        state.reputation -= 1;
    }
    state.clamp();
}

fn tick_game(state: &mut GameState) {
    if state.inspection {
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

fn trigger_random_event(state: &mut GameState) {
    if state.inspection {
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
            state.log_line("The binturong escapes and naps inside a government vehicle.");
        }
        5 => {
            state.civet_happiness -= 6.0;
            state.log_line("A civet refuses low-quality fruit with devastating eye contact.");
        }
        _ => {
            state.goat_present = true;
            state.suspicion += 3.0;
            state.log_line("A goat appears for no clear reason. Legal recommends silence.");
        }
    }
    state.clamp();
}

#[cfg(test)]
mod tests {
    use super::{EutherCivetAction, EutherCivetRuntime};

    #[test]
    fn action_updates_snapshot() {
        let mut runtime = EutherCivetRuntime::new();
        let before = runtime.snapshot();
        let after = runtime.action(EutherCivetAction::PlantCoffee);

        assert_eq!(after.state.coffee_plants, before.state.coffee_plants + 1);
        assert!(after.state.money < before.state.money);
        assert!(after.available_actions.contains(&EutherCivetAction::SellCoffee));
    }

    #[test]
    fn tick_advances_frame_and_growth() {
        let mut runtime = EutherCivetRuntime::new();
        let before = runtime.snapshot();
        let after = runtime.tick();

        assert_eq!(after.frame, before.frame + 1);
        assert!(after.state.coffee_fruit > before.state.coffee_fruit);
    }
}
