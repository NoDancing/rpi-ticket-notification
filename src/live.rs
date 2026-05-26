// Live MLB game feed polling and event extraction.
//
// Responsible for:
// - polling feed/live endpoints
// - extracting play-by-play events
// - tracking live game state
// - deduplicating previously seen events
//
// This module should not contain notification delivery logic.

use crate::models::PlayEvent;
use serde_json::Value;
use std::error::Error;

const LIVE_BASE_URL: &str = "https://statsapi.mlb.com/api/v1.1/game";

pub fn build_live_feed_url(game_pk: u64) -> String {
    format!("{LIVE_BASE_URL}/{game_pk}/feed/live")
}

pub fn fetch_live_feed(url: &str) -> Result<Value, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    let json: Value = serde_json::from_str(&resp)?;

    Ok(json)
}

/// Extract the full list of play events from
pub fn extract_play_events(json: &Value) -> Vec<PlayEvent> {
    let mut plays = Vec::new();

    let Some(all_plays) = json["liveData"]["plays"]["allPlays"].as_array() else {
        return plays;
    };

    for play in all_plays {
        if play["about"]["isComplete"].as_bool() != Some(true) {
            continue;
        };
        let Some(play_id) = play["about"]["atBatIndex"].as_u64() else {
            continue;
        };
        let Some(inning) = play["about"]["inning"].as_u64() else {
            continue;
        };
        let Some(is_top_inning) = play["about"]["isTopInning"].as_bool() else {
            continue;
        };
        let Some(batter) = play["matchup"]["batter"]["fullName"].as_str() else {
            continue;
        };
        let Some(pitcher) = play["matchup"]["pitcher"]["fullName"].as_str() else {
            continue;
        };
        let Some(description) = play["result"]["description"].as_str() else {
            continue;
        };
        let Some(event_type) = play["result"]["eventType"].as_str() else {
            continue;
        };
        let Some(is_scoring) = play["about"]["isScoringPlay"].as_bool() else {
            continue;
        };
        let Some(away_score) = play["result"]["awayScore"].as_u64() else {
            continue;
        };
        let Some(home_score) = play["result"]["homeScore"].as_u64() else {
            continue;
        };

        plays.push(PlayEvent {
            id: play_id,
            inning,
            is_top_inning,
            batter: batter.to_string(),
            pitcher: pitcher.to_string(),
            description: description.to_string(),
            event_type: event_type.to_string(),
            is_scoring,
            away_score,
            home_score,
        });
    }
    plays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_live_feed_url() {
        let url = build_live_feed_url(745804);
        assert_eq!(
            url,
            "https://statsapi.mlb.com/api/v1.1/game/745804/feed/live"
        );
    }

    #[test]
    fn extracts_plays_from_real_feed() {
        let raw = include_str!("../tests/fixtures/live_feed.json");
        let json: Value = serde_json::from_str(raw).unwrap();
        let plays = extract_play_events(&json);

        assert_eq!(plays.len(), 77);

        let first = &plays[0];
        assert_eq!(first.id, 0);
        assert_eq!(first.inning, 1);
        assert!(first.is_top_inning);
        assert_eq!(first.batter, "Blake Dunn");
        assert_eq!(first.pitcher, "Nolan McLean");
        assert_eq!(first.event_type, "strikeout");
        assert!(!first.is_scoring);
    }
}
