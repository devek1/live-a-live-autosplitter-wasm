use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct ImperialChina;
impl ImperialChina {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        _scenario_progress: &Pair<u16>,
    ) {
        // Start Split
        if settings.start_middle_ages
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::ImperialChina as u8
        {
            split(splits, "start_imperial_china")
        }
        if current_chapter.current == Chapter::ImperialChina as u8 {
            // Put Scenario Splits Here
        }
    }
}
