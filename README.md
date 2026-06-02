# EutherCivet

Minimal Rust + Bevy prototype for a satirical civet coffee tycoon sim.

## Run

```bash
cargo run
```

The game saves to `euther_civet_save.json` in the project root. Use the in-game Save and Load buttons.

## Structure

```text
Cargo.toml
README.md
src/
  actions.rs
  main.rs
  model.rs
  simulation.rs
  ui.rs
  visuals.rs
```

## Implemented

- Core plantation model in one `GameState`.
- Coffee plants generate fruit over time.
- Civets eat fed fruit and produce processed beans.
- Happiness changes production, suspicion, and reputation pressure.
- Buttons for planting, harvesting, feeding, collecting, roasting, selling, enclosure upgrades, paperwork, save, and load.
- Random absurd events, including police, journalists, inspections, helicopter, binturong escape, picky civet, and goat.
- Random events pause play and present three response choices with different resource, reputation, suspicion, and welfare effects.
- Inspection event: Operation Bitter Bean, with three responses.
- Seven-day run structure with daily reports, upkeep, reputation/suspicion adjustments, and weekly win/fail verdicts.
- Buyable upgrades: Legal Office, Caretaker, Fruit Sorter, Roasting Shed, and Visitor Tasting Room.
- Premium order contracts with accept/decline choices, delivery deadlines, payouts, reputation rewards, and missed-order penalties.
- First scene flow: main menu, company/mission presentation, and animal book before entering the plantation.
- Named civets with in-world name labels.
- Dynamic action buttons that show costs/requirements and dim when unavailable.
- Warm plantation visuals with red suspicion UI and placeholder shapes for plants, civets, binturong, goat, helicopter, and coffee bags.
- Polished first-pass UI with status bars, two-column controls, animated helicopter, enclosure staging, and suspicion glow.
- Local save/load via JSON.

## Expansion Points

- Add new animals by extending `GameState` and `refresh_world_visuals`.
- Add buildings/upgrades by adding new `Action` variants and action handlers.
- Add new events in `random_event`.
- Replace placeholder shapes with original sprites later without changing the game model.
