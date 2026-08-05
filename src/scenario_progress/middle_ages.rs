use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::string::ArrayCString;
use asr::watcher::Pair;

pub struct MiddleAges;
impl MiddleAges {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<i8>,
        scenario_progress: &Pair<i32>,
        map_id: &Pair<ArrayCString<64>>,
        transition_state: &Pair<u32>,
        duration_frames_value: &Pair<i32>,
        battle_id: &Pair<i32>,
        battle_result: u8
    ) {
        // Start Split
        if settings.start_middle_ages
            && current_chapter.old == Chapter::Menu as i8
            && current_chapter.current == Chapter::MiddleAges as i8
        {
            split(splits, "start_middle_ages")
        }
        if current_chapter.current == Chapter::MiddleAges as i8 {
            // Put Scenario Splits Here
            if settings.middle_ages_streibough_joins
                && scenario_progress.current == 80
                && scenario_progress.old < 80
            {
                split(splits, "middle_ages_streibough_joins")
            }
            if settings.middle_ages_hasshe_house_1
                && scenario_progress.current == 100
                && scenario_progress.old < 100
            {
                split(splits, "middle_ages_hasshe_house_1")
            }
            if settings.middle_ages_uranus_joins
                && scenario_progress.current == 110
                && scenario_progress.old < 110
            {
                split(splits, "middle_ages_uranus_joins")
            }
            if settings.middle_ages_hasshe_joins
                && scenario_progress.current == 130
                && scenario_progress.old < 130
            {
                split(splits, "middle_ages_hasshe_joins")
            }
            if settings.middle_ages_archons_roost_1
                && scenario_progress.current == 150
                && duration_frames_value.changed_from_to(&0, &122)
            {
                split(splits, "middle_ages_archons_roost_1")
            }
            if settings.middle_ages_defeat_lord_of_dark
                && battle_id.current == 228
                && duration_frames_value.changed_from_to(&0, &347)
            {
                split(splits, "middle_ages_defeat_lord_of_dark")
            }
            if settings.middle_ages_banished
                && scenario_progress.current == 250
                && scenario_progress.old < 250
            {
                split(splits, "middle_ages_banished")
            }
            if settings.middle_ages_arrested
                && scenario_progress.current == 270
                && scenario_progress.old < 270
            {
                split(splits, "middle_ages_arrested")
            }
            if settings.middle_ages_prison_escape
                && scenario_progress.current == 360
                && scenario_progress.old < 360
            {
                split(splits, "middle_ages_prison_escape")
            }
            if settings.middle_ages_defeat_claustrophobia
                && battle_id.current == 231
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "middle_ages_defeat_claustrophobia")
            }
            if settings.middle_ages_defeat_scotophobia
                && battle_id.current == 232
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "middle_ages_defeat_scotophobia")
            }
            if settings.middle_ages_defeat_acrophobia
                && battle_id.current == 233
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "middle_ages_defeat_acrophobia")
            }
            if settings.middle_ages_defeat_hygrophobia
                && battle_id.current == 234
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "middle_ages_defeat_hygrophobia")
            }
            if settings.middle_ages_defeat_streibough
                && battle_id.current == 302
                && duration_frames_value.changed_from_to(&0, &360)
            {
                split(splits, "middle_ages_defeat_streibough")
            }
            if settings.middle_ages_end_split
                && scenario_progress.current == 510
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "middle_ages_end_split")
            }
        }
    }
}
