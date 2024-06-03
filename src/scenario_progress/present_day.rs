use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct PresentDay;
impl PresentDay {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        map_id: &Pair<u32>,
        transition_state: &Pair<u32>,
    ) {
        // Start Split
        if settings.start_middle_ages
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::PresentDay as u8
        {
            split(splits, "start_present_day")
        }
        if current_chapter.current == Chapter::PresentDay as u8 {
            // Put Scenario Splits Here
            if settings.present_day_end_split
                && scenario_progress.current == 0
                && map_id.current == 0
                && transition_state.old == 4
                && transition_state.current == 0
            {
                split(splits, "present_day_end_split")
            }
        }
    }
}
