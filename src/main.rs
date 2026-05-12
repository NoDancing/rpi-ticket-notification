use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    hello_internet()?;

    Ok(())
}

fn hello_internet() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://statsapi.mlb.com/api/v1/teams")?.text()?;

    println!("{}", resp);

    Ok(())
}
