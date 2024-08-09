use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct MiddleAges;
impl MiddleAges {
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
            && current_chapter.current == Chapter::MiddleAges as u8
        {
            split(splits, "start_middle_ages")
        }
        if current_chapter.current == Chapter::MiddleAges as u8 {
            // Put Scenario Splits Here
            if settings.middle_ages_streibough_joins
                && scenario_progress.current == 80
                && scenario_progress.old < 80
            {
                split(splits, "middle_ages_streibough_joins")
            }
            if settings.middle_ages_brion
                && scenario_progress.current == 140
                && scenario_progress.old < 140
            {
                split(splits, "middle_ages_brion")
            }
            if settings.middle_ages_defeat_lord_of_dark
                && scenario_progress.current == 160
                && scenario_progress.old < 160
            {
                split(splits, "middle_ages_defeat_lord_of_dark")
            }
            if settings.middle_ages_banished
                && scenario_progress.current == 250
                && scenario_progress.old < 250
            {
                split(splits, "middle_ages_banished")
            }
            if settings.middle_ages_prison_escape
                && scenario_progress.current == 360
                && scenario_progress.old < 360
            {
                split(splits, "middle_ages_prison_escape")
            }
            if settings.middle_ages_defeat_claustrophobia
                && scenario_progress.current == 370
                && scenario_progress.old < 370
            {
                split(splits, "middle_ages_defeat_claustrophobia")
            }
            if settings.middle_ages_defeat_hygrophobia
                && scenario_progress.current == 395
                && scenario_progress.old < 395
            {
                split(splits, "middle_ages_defeat_hygrophobia")
            }
            if settings.middle_ages_defeat_streibough
                && scenario_progress.current == 430
                && scenario_progress.old < 430
            {
                split(splits, "middle_ages_defeat_streibough")
            }
            if settings.middle_ages_end_split
                && scenario_progress.current == 510
                && map_id.current == 0
                && transition_state.old == 4
                && transition_state.current == 0
            {
                split(splits, "middle_ages_end_split")
            }
        }
    }
}
