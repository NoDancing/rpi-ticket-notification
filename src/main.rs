use chrono::Local;
use serde_json::Value;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mets_game_id = get_mets_game()?;

    match mets_game_id {
        Some(id) => {
            println!("Mets game today! gamePk = {}", id);
        }
        None => {
            println!("No Mets game today.");
        }
    }

    Ok(())
}
fn get_mets_game() -> Result<Option<u64>, Box<dyn Error>> {
    let today = Local::now().format("%Y-%m-%d");

    let url = format!(
        "https://statsapi.mlb.com/api/v1/schedule?sportId=1&date={}",
        today
    );

    let resp = reqwest::blocking::get(url)?.text()?; //explain more

    let json: Value = serde_json::from_str(&resp)?; //explain more
    //

    // Mets team ID
    const METS_ID: u64 = 121;

    // Navigate into dates[0].games
    let games = &json["dates"][0]["games"];

    if let Some(games_array) = games.as_array() {
        for game in games_array {
            let away_id = game["teams"]["away"]["team"]["id"].as_u64().unwrap_or(0);

            let home_id = game["teams"]["away"]["team"]["id"].as_u64().unwrap_or(0);

            // Is either team the Mets?
            if away_id == METS_ID || home_id == METS_ID {
                let game_pk = game["gamePk"].as_u64().unwrap_or(0);

                return Ok(Some(game_pk));
            }
        }
    }

    Ok(None)
}
