// MLB schedule discovery logic.
//
// Responsible for:
// - fetching daily schedules
// - discovering watched-team games
// - extracting gamePk values
//
// This module handles schedule API interaction only.
// It should not contain live game polling logic.

use crate::models::{GameInfo, Team};
use serde_json::Value;
use std::error::Error;

const SCHEDULE_BASE_URL: &str = "https://statsapi.mlb.com/api/v1/schedule";

/// Returns true if `team_id` is in the watched-teams list.
pub fn is_watched_team(team_id: u64, watched: &[u64]) -> bool {
    watched.contains(&team_id)
}

/// Build the MLB schedule API URL for a given date.
///
/// `date` must be in `YYYY-MM-DD` format.
pub fn build_schedule_url(date: &str) -> String {
    format!("{SCHEDULE_BASE_URL}?sportId=1&date={date}")
}

/// Fetch the MLB schedule JSON from the given URL.
///                       
/// # Errors
/// Returns an error if the HTTP request fails or the response body
/// is not valid JSON.
pub fn fetch_schedule(url: &str) -> Result<Value, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    let json: Value = serde_json::from_str(&resp)?;

    Ok(json)
}

pub fn extract_games_from_schedule(json: &Value) -> Vec<GameInfo> {
    let mut games = Vec::new();

    let Some(dates) = json["dates"].as_array() else {
        return games;
    };

    for date_entry in dates {
        let Some(games_array) = date_entry["games"].as_array() else {
            continue;
        };

        for game in games_array {
            let Some(game_pk) = game["gamePk"].as_u64() else {
                continue;
            };
            let Some(away_id) = game["teams"]["away"]["team"]["id"].as_u64() else {
                continue;
            };
            let Some(away_name) = game["teams"]["away"]["team"]["name"].as_str() else {
                continue;
            };
            let Some(home_id) = game["teams"]["home"]["team"]["id"].as_u64() else {
                continue;
            };
            let Some(home_name) = game["teams"]["home"]["team"]["name"].as_str() else {
                continue;
            };

            games.push(GameInfo {
                game_pk,
                away: Team {
                    id: away_id,
                    name: away_name.to_string(),
                },
                home: Team {
                    id: home_id,
                    name: home_name.to_string(),
                },
            });
        }
    }

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_watched_team() {
        let watched = vec![121, 144];

        assert_eq!(is_watched_team(121, &watched), true);
    }

    #[test]
    fn rejects_unwatched_team() {
        let watched = vec![121, 144];

        assert_eq!(is_watched_team(147, &watched), false);
    }

    #[test]
    fn builds_schedule_url_with_date() {
        let url = build_schedule_url("2026-05-19");
        assert_eq!(
            url,
            "https://statsapi.mlb.com/api/v1/schedule?sportId=1&date=2026-05-19"
        );
    }

    #[test]
    fn extracts_single_game() {
        let raw = r#"
        {
          "dates": [
            {
              "games": [
                {
                  "gamePk": 745804,
                  "teams": {
                    "away": { "team": { "id": 121, "name": "New York Mets" } },
                    "home": { "team": { "id": 144, "name": "Atlanta Braves" } }
                  }
                }
              ]
            }
          ]
        }"#;

        let json: Value = serde_json::from_str(raw).unwrap();
        let games = extract_games_from_schedule(&json);

        assert_eq!(games.len(), 1);
        assert_eq!(games[0].game_pk, 745804);
        assert_eq!(games[0].away.id, 121);
        assert_eq!(games[0].away.name, "New York Mets");
        assert_eq!(games[0].home.id, 144);
        assert_eq!(games[0].home.name, "Atlanta Braves");
    }

    #[test]
    fn extracts_multiple_games() {
        let raw = r#"
        {
          "dates": [
            {
              "games": [
                {
                  "gamePk": 745804,
                  "teams": {
                    "away": { "team": { "id": 121, "name": "New York Mets" } },
                    "home": { "team": { "id": 144, "name": "Atlanta Braves" } }
                  }
                },
                {
                  "gamePk": 745805,
                  "teams": {
                    "away": { "team": { "id": 147, "name": "New York Yankees" } },
                    "home": { "team": { "id": 111, "name": "Boston Red Sox" } }
                  }
                }
              ]
            }
          ]
        }"#;

        let json: Value = serde_json::from_str(raw).unwrap();
        let games = extract_games_from_schedule(&json);

        assert_eq!(games.len(), 2);
        assert_eq!(games[0].game_pk, 745804);
        assert_eq!(games[1].game_pk, 745805);
        assert_eq!(games[1].away.name, "New York Yankees");
    }

    #[test]
    fn returns_empty_for_no_dates() {
        let json: Value = serde_json::from_str(r#"{"dates": []}"#).unwrap();
        let games = extract_games_from_schedule(&json);
        assert!(games.is_empty());
    }
}
