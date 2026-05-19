# Mini-Capstone Dev Diary / Notes

## Tuesday May 19

### Done

- Added serde and serde_json to cargo
- Wrote get_mets_game function to hit API and parse game number
- Fixed home/away copy-paste bug in get_mets_game
- Created src/models.rs with core domain types: WatchedTeam, Config, Team, GameInfo
- Added toml crate; wrote config.toml with poll interval + two watched teams
- Implemented load_config() in src/config.rs with doc comments and 3 unit tests
  (valid load, missing file, invalid TOML)
- Scaffolded src/schedule.rs (is_watched_team + tests) and src/live.rs
- Completed roadmap Phase 0

### Plan
- [x] Define test data structure
- [x] Parse JSON
- [x] Load config from TOML

### Next Steps:
- [ ] Move build_schedule_url + fetch_schedule out of main.rs into schedule.rs
- [ ] Write extract_games_from_schedule() and discover_watched_games()
- [ ] Capture a real MLB schedule JSON response into tests/fixtures/ for unit tests
- [ ] Delete prototype get_mets_game() once discover_watched_games works
- [ ] Catch up to plan.md (currently ~1 week behind — owe event pipeline + printing by May 25)

## Tuesday May 12

### Done

- Setting up initial tooling and environments

  - Initialized Cargo project

  - Installed Cross for Raspberry Pi cross-compilation
    (note that the target for the Zero 2 W is `aarch64-unknown-linux-gnu`)

  - Installed OrbStack to run a container for testing (instead of Docker
    Desktop)

  - Added dockerignore

  - Basic git setup

    - Created repo
    - Added Bill Pfeil as a contributor (`azrugger`)
    - Added README & DIARY
    - Added gitignore

- Initial Hardware Setup

  - Flashed Raspberry pi OS onto the flash drive.
  - Confirmed Raspberry Pi connects to network, connected via SSH
  - Compiled "Hello World" using cross + orbstack, succesfully ran on Pi

- Initial Rust Development

  - Compiled Hello World
  - Compiled Hello Internet (successfully hits MLB Api)

### Next Steps:

- [x] Learn how to parse JSON in Rust
- [ ] Learn how to send data over TCP
