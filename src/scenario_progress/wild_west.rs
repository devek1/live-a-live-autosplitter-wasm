use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct WildWest;
impl WildWest {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        _scenario_progress: &Pair<u16>,
    ) {
        // Start Split
        if settings.start_wild_west
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::WildWest as u8
        {
            split(splits, "start_wild_west")
        }
        if current_chapter.current == Chapter::WildWest as u8 {
            // Put Scenario Splits Here
        }
    }
}
