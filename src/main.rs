use crate::models::GameInfo;
use chrono::Local;
use serde_json::Value;
use std::error::Error;

mod config;
mod live;
mod models;
mod schedule;

fn main() -> Result<(), Box<dyn Error>> {
    // Load Config
    let cfg = config::load_config("config.toml")?;

    // Fetch today's schedule
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let url = schedule::build_schedule_url(&today);
    let json = schedule::fetch_schedule(&url)?;
    let games = schedule::extract_games_from_schedule(&json);
    let watched_ids: Vec<u64> = cfg.teams.iter().map(|t| t.id).collect();

    // Identify watched games
    let watched_games: Vec<&GameInfo> = games
        .iter()
        .filter(|game| {
            watched_ids.contains(&game.away.id)
                || watched_ids.contains(&game.home.id)
        })
        .collect();

    // Report if no games
    if watched_games.is_empty() {
        println!("No games found!");
        return Ok(());
    }

    // Report if multiple games
    if watched_games.len() > 1 {
        println!("Multiple games found! Tracking the first.");
    }

    let game = watched_games[0].clone();

    println!("Polling {} @ {}", game.away.name, game.home.name);

    Ok(())
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
