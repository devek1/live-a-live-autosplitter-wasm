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
        frame_pointer_value: &Pair<u32>,
        duration_frames_value: &Pair<u32>,
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
                && duration_frames_value.current == 122
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "middle_ages_archons_roost_1")
            }
            if settings.middle_ages_defeat_lord_of_dark
                && scenario_progress.current == 150
                && duration_frames_value.current == 347
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
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
                && scenario_progress.current == 360
                && duration_frames_value.current == 180
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "middle_ages_defeat_claustrophobia")
            }
            if settings.middle_ages_defeat_scotophobia
                && scenario_progress.current == 370
                && duration_frames_value.current == 180
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "middle_ages_defeat_scotophobia")
            }
            if settings.middle_ages_defeat_acrophobia
                && scenario_progress.current == 380
                && duration_frames_value.current == 180
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "middle_ages_defeat_acrophobia")
            }
            if settings.middle_ages_defeat_hygrophobia
                && scenario_progress.current == 390
                && duration_frames_value.current == 180
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "middle_ages_defeat_hygrophobia")
            }
            if settings.middle_ages_defeat_streibough
                && scenario_progress.current == 420
                && duration_frames_value.current == 360
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
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
