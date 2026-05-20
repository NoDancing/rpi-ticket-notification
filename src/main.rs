use chrono::Local;
use serde_json::Value;
use std::error::Error;

mod config;
mod models;
mod schedule;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = config::load_config("config.toml")?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let url = schedule::build_schedule_url(&today);
    let json = schedule::fetch_schedule(&url)?;
    let games = schedule::extract_games_from_schedule(&json);

    let watched_ids: Vec<u64> = cfg.teams.iter().map(|t| t.id).collect();

    for game in &games {
        if schedule::is_watched_team(game.away.id, &watched_ids)
            || schedule::is_watched_team(game.home.id, &watched_ids)
        {
            println!(
                "Watched game: {} @ {} (gamePk {})",
                game.away.name, game.home.name, game.game_pk
            );
        }
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
