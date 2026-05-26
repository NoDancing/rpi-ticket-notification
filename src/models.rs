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

/// A single play pulled from the live feed
///
/// `id` is the MLB `atBatIndex` - sequential per game
/// and used as the deduplication key by the polling loop
#[derive(Debug, Clone)]
pub struct PlayEvent {
    pub id: u64,
    pub inning: u64,
    pub is_top_inning: bool,
    pub batter: String,
    pub pitcher: String,
    pub description: String,
    pub event_type: String,
    pub is_scoring: bool,
    pub away_score: u64,
    pub home_score: u64,
}
