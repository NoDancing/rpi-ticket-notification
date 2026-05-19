// Core application domain models.
//
// This module contains the application's internal data structures.
//
// These structs represent concepts OUR application cares about,
// independent of the MLB API response structure.

use serde::Deserialize;

/// A team the user wants to monitor. Loaded from `config.toml`.
/// `id` is MLB's numeric team ID — the key used to match games in
/// schedule responses (e.g. Mets = 121, Braves = 144).
#[derive(Debug, Deserialize, Clone)]
pub struct WatchedTeam {
    pub name: String,
    pub id: u64,
}

/// Top-level shape of `config.toml`. Field names must match the TOML
/// keys exactly, since serde uses them for deserialization.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub poll_interval_seconds: u64,
    pub teams: Vec<WatchedTeam>,
}

/// A team participating in a discovered game. Kept distinct from
/// `WatchedTeam` even though the shape is the same — these come from
/// the MLB schedule API, not user config, and the two concepts may
/// diverge later (e.g. adding record, abbreviation, etc.).
#[derive(Debug, Clone)]
pub struct Team {
    pub id: u64,
    pub name: String,
}

/// The internal slice of an MLB schedule entry that the app actually
/// uses. `game_pk` is MLB's per-game ID, needed to build the live feed
/// URL later.
#[derive(Debug, Clone)]
pub struct GameInfo {
    pub game_pk: u64,
    pub home: Team,
    pub away: Team,
}
