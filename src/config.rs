// Configuration loading and parsing.
//
// Responsible for:
// - reading config.toml
// - TOML deserialization
// - config validation
//
// This module should not contain application runtime logic.
//

use crate::models::Config;
use std::error::Error;
use std::fs;

/// Load and parse the application config from a TOML file.
///
/// Reads the file at `path` and deserializes it into a [`Config`].
/// The file must contain `poll_interval_seconds` and a `[[teams]]`
/// array, matching the fields on [`Config`] and [`WatchedTeam`].
///
/// # Errors
/// Returns an error if the file cannot be read (e.g. missing path,
/// permission denied) or if the contents are not valid TOML matching
/// the expected schema.
pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_config(name: &str, contents: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rpi_ticket_test_{}_{}.toml",
            std::process::id(),
            name
        ));
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        path
    }

    #[test]
    fn loads_valid_config() {
        let path = write_temp_config(
            "valid",
            r#"
poll_interval_seconds = 10

[[teams]]
name = "New York Mets"
id = 121

[[teams]]
name = "Atlanta Braves"
id = 144
"#,
        );

        let cfg = load_config(path.to_str().unwrap()).unwrap();

        assert_eq!(cfg.poll_interval_seconds, 10);
        assert_eq!(cfg.teams.len(), 2);
        assert_eq!(cfg.teams[0].name, "New York Mets");
        assert_eq!(cfg.teams[0].id, 121);
        assert_eq!(cfg.teams[1].id, 144);
    }

    #[test]
    fn errors_on_missing_file() {
        let result = load_config("/nonexistent/path/does_not_exist.toml");
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_invalid_toml() {
        let path = write_temp_config("invalid", "this is not valid toml = = =");
        let result = load_config(path.to_str().unwrap());
        assert!(result.is_err());
    }
}
