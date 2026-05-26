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
    for game in &games {
        if watched_ids.contains(game.away.id) || watched_ids.contains(game.home.id) {
            println!(
                "Watched game: {} @ {} (gamePk {})",
                game.away.name, game.home.name, game.game_pk
            );
        }
    }

    Ok(())
}

fn poll_game(game_pk: u64) {
    let live_url = live::build_live_feed_url(game_pk);
    let live_feed_json = live::fetch_live_feed(&live_url);

    if live_feed_json[
    let play_events = live::extract_play_events(
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
