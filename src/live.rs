// Live MLB game feed polling and event extraction.
//
// Responsible for:
// - polling feed/live endpoints
// - extracting play-by-play events
// - tracking live game state
// - deduplicating previously seen events
//
// This module should not contain notification delivery logic.
