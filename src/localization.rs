use crate::model::{GameState, Language, PlantationRoom};

pub fn state_text(state: &GameState, en: &'static str, sv: &'static str) -> &'static str {
    language_text(state.language, en, sv)
}

pub fn language_text(language: Language, en: &'static str, sv: &'static str) -> &'static str {
    if language == Language::Swedish {
        sv
    } else {
        en
    }
}

pub fn room_name(room: PlantationRoom, language: Language) -> &'static str {
    if language == Language::Swedish {
        match room {
            PlantationRoom::Sanctuary => "Fristad",
            PlantationRoom::CoffeeField => "Kaffefält",
            PlantationRoom::Roastery => "Rosteri",
            PlantationRoom::PaperworkOffice => "Papperskontor",
        }
    } else {
        match room {
            PlantationRoom::Sanctuary => "Sanctuary",
            PlantationRoom::CoffeeField => "Coffee Field",
            PlantationRoom::Roastery => "Roastery",
            PlantationRoom::PaperworkOffice => "Paperwork Office",
        }
    }
}

pub fn room_name_definite(room: PlantationRoom, language: Language) -> &'static str {
    if language == Language::Swedish {
        match room {
            PlantationRoom::Sanctuary => "Fristaden",
            PlantationRoom::CoffeeField => "Kaffefältet",
            PlantationRoom::Roastery => "Rosteriet",
            PlantationRoom::PaperworkOffice => "Papperskontoret",
        }
    } else {
        room_name(room, language)
    }
}

pub fn world_label(state: &GameState, key: &'static str) -> &'static str {
    if state.language == Language::Swedish {
        match key {
            "owner" => "plantageägare",
            "fruit" => "frukt",
            "seedlings" => "plantor",
            "fruit_on_hand" => "Kaffefrukt i säcken",
            "civet_garden" => "palmmårdsträdgård",
            "binturong" => "binturong",
            "snack_trays" => "snackbrickor",
            "roaster" => "ROSTARE",
            "coffee" => "kaffe",
            "bean_crate" => "bönlåda",
            "police_helicopter" => "polishelikopter",
            "suspicion" => "Misstanke",
            "field_goat" => "fältget?",
            "goat" => "get?",
            "witness" => "vittne",
            "field_hint" => "Bästa knapparna här: Plantera kaffe, Skörda frukt, Mata palmmårdar.",
            "sanctuary_hint" => {
                "Klicka på en palmmård för att mata, klappa, granska anteckningar och bygga tillgivenhet."
            }
            "roastery_hint" => "Bästa knapparna här: Samla bönor, Rosta kaffe, Sälj kaffe.",
            "office_hint" => {
                "Bästa knapparna här: Visa papper, bygg kontorsuppgraderingar, håll dig lugn."
            }
            "processed_beans" => "Processade bönor",
            "roasted_bags" => "Rostade säckar",
            "paperwork_level" => "Pappersnivå",
            "sleepy" => "sömnig",
            "hungry" => "hungrig",
            "curious" => "nyfiken",
            "content" => "nöjd",
            _ => key,
        }
    } else {
        match key {
            "owner" => "plantation owner",
            "fruit" => "fruit",
            "seedlings" => "seedlings",
            "fruit_on_hand" => "Coffee fruit on hand",
            "civet_garden" => "civet garden",
            "binturong" => "binturong",
            "snack_trays" => "snack trays",
            "roaster" => "ROASTER",
            "coffee" => "coffee",
            "bean_crate" => "bean crate",
            "police_helicopter" => "police helicopter",
            "suspicion" => "Suspicion",
            "field_goat" => "field goat?",
            "goat" => "goat?",
            "witness" => "witness",
            "field_hint" => "Best buttons here: Plant coffee, Harvest fruit, Feed civets.",
            "sanctuary_hint" => "Click a civet to feed, pet, inspect notes, and build affection.",
            "roastery_hint" => "Best buttons here: Collect beans, Roast coffee, Sell coffee.",
            "office_hint" => "Best buttons here: Show paperwork, build office upgrades, stay calm.",
            "processed_beans" => "Processed beans",
            "roasted_bags" => "Roasted bags",
            "paperwork_level" => "Paperwork level",
            "sleepy" => "sleepy",
            "hungry" => "hungry",
            "curious" => "curious",
            "content" => "content",
            _ => key,
        }
    }
}
