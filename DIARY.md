# Mini-Capstone Dev Diary / Notes

## Tuesday May 19

### Done

- Added serde and serde_json to cargo
- wrote get_mets_game function to hit API and parse game number.

### Plan
- [ ] Define test data structure
- [x] Parse JSON

### Next Steps:

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
