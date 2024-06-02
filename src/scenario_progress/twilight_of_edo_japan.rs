use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;


// Locations
// 10228295 - outside storehouse
// 10228695 - Inside Storehouse

pub struct TwilightOfEdoJapan;
impl TwilightOfEdoJapan {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        _scenario_progress: &Pair<u16>,
    ) {
        // Start Split
        if settings.start_twilight_of_edo_japan
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::TwilightOfEdoJapan as u8
        {
            split(splits, "start_twilight_of_edo_japan")
        }
        if current_chapter.current == Chapter::TwilightOfEdoJapan as u8 {
            // Put Scenario Splits Here
        }
    }
}
