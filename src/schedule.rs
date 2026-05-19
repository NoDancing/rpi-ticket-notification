// MLB schedule discovery logic.
//
// Responsible for:
// - fetching daily schedules
// - discovering watched-team games
// - extracting gamePk values
//
// This module handles schedule API interaction only.
// It should not contain live game polling logic.

pub fn is_watched_team(team_id: u64, watched: &[u64]) -> bool {
    watched.contains(&team_id)
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
}
