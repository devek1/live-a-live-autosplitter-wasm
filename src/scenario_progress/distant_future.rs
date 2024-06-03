use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct DistantFuture;
impl DistantFuture {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        _scenario_progress: &Pair<u16>,
    ) {
        // Start Split
        if settings.start_distant_future
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::DistantFuture as u8
        {
            split(splits, "start_distant_future")
        }
        if current_chapter.current == Chapter::DistantFuture as u8 {
            // Put Scenario Splits Here
        }
    }
}
