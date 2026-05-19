// MLB schedule discovery logic.
//
// Responsible for:
// - fetching daily schedules
// - discovering watched-team games
// - extracting gamePk values
//
// This module handles schedule API interaction only.
// It should not contain live game polling logic.

const SCHEDULE_BASE_URL: &str = "https://statsapi.mlb.com/api/v1/schedule";

pub fn is_watched_team(team_id: u64, watched: &[u64]) -> bool {
    watched.contains(&team_id)
}

pub fn build_schedule_url(date: &str) -> String {
    format!("{SCHEDULE_BASE_URL}?sportId=1&date={date}")
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
}
