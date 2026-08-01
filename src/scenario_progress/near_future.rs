use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::string::ArrayCString;
use asr::watcher::Pair;

pub struct NearFuture;
impl NearFuture {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        map_id: &Pair<ArrayCString<256>>,
        transition_state: &Pair<u32>,
        duration_frames_value: &Pair<u32>,
        battle_id: &Pair<ArrayCString<256>>,
        battle_result: u8
    ) {
        // Start Split
        if settings.start_near_future
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::NearFuture as u8
        {
            split(splits, "start_near_future")
        }

        if current_chapter.current == Chapter::NearFuture as u8 {
            // Put Scenario Splits Here
            if settings.near_future_park
                && scenario_progress.old >= 85
                && scenario_progress.old < 110
                && scenario_progress.current == 110
            {
                split(splits, "near_future_park")
            }
            if settings.near_future_enter_titan
                && scenario_progress.old >= 270
                && scenario_progress.old < 280
                && scenario_progress.current == 280
            {
                split(splits, "near_future_enter_titan")
            }
            if settings.near_future_dock
                && scenario_progress.old >= 380
                && scenario_progress.old < 390
                && scenario_progress.current == 390
            {
                split(splits, "near_future_dock")
            }
            if settings.near_future_matsu_joins
                && scenario_progress.old >= 410
                && scenario_progress.old < 450
                && scenario_progress.current == 450
            {
                split(splits, "near_future_matsu_joins")
            }
            if settings.near_future_robot
                && scenario_progress.old >= 460
                && scenario_progress.old < 490
                && scenario_progress.current == 490
            {
                split(splits, "near_future_robot")
            }
            if settings.near_future_enter_titan_2
                && scenario_progress.old >= 670
                && scenario_progress.old < 746
                && scenario_progress.current == 746
            {
                split(splits, "near_future_enter_titan_2")
            }
            if settings.near_future_enter_inko_fight
                && battle_id.current.matches("id.battleLayout.247")
                && duration_frames_value.changed_from_to(&0, &122)
            {
                split(splits, "near_future_enter_inko_fight")
            }
            if settings.near_future_defeat_inko
                && battle_id.current.matches("id.battleLayout.247")
                && duration_frames_value.changed_from_to(&0, &360)
            {
                split(splits, "near_future_defeat_inko")
            }
            if settings.near_future_end_split
                && scenario_progress.current == 900
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "near_future_end_split")
            }
        }
    }
}
