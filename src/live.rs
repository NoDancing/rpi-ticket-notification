// Live MLB game feed polling and event extraction.
//
// Responsible for:
// - polling feed/live endpoints
// - extracting play-by-play events
// - tracking live game state
// - deduplicating previously seen events
//
// This module should not contain notification delivery logic.

const LIVE_BASE_URL: &str = "https://statsapi.mlb.com/api/v1.1/game";

pub fn build_live_feed_url(game_pk: u64) -> String {
    format!("{LIVE_BASE_URL}/{game_pk}/feed/live")
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
}
