use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct DominionOfHate;
impl DominionOfHate {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        _scenario_progress: &Pair<u16>,
    ) {
        // Start Split
        if settings.start_dominion_of_hate
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::DominionOfHate as u8
        {
            split(splits, "start_dominion_of_hate")
        }
        if current_chapter.current == Chapter::DominionOfHate as u8 {
            // Put Scenario Splits Here
        }
    }
}
