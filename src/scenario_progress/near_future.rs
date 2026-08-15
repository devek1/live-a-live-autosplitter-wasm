use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::string::ArrayCString;
use asr::watcher::Pair;


pub fn check_splits(
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
    if current_chapter.old == Chapter::Menu as i8
    {
        split(splits, "start_near_future")
    }

    // Put Scenario Splits Here
    if scenario_progress.old >= 85
        && scenario_progress.old < 110
        && scenario_progress.current == 110
    {
        split(splits, "near_future_park")
    }
    if scenario_progress.old >= 270
        && scenario_progress.old < 280
        && scenario_progress.current == 280
    {
        split(splits, "near_future_enter_titan")
    }
    if scenario_progress.old >= 380
        && scenario_progress.old < 390
        && scenario_progress.current == 390
    {
        split(splits, "near_future_dock")
    }
    if scenario_progress.old >= 410
        && scenario_progress.old < 450
        && scenario_progress.current == 450
    {
        split(splits, "near_future_matsu_joins")
    }
    if scenario_progress.old >= 460
        && scenario_progress.old < 490
        && scenario_progress.current == 490
    {
        split(splits, "near_future_robot")
    }
    if scenario_progress.old >= 670
        && scenario_progress.old < 746
        && scenario_progress.current == 746
    {
        split(splits, "near_future_enter_titan_2")
    }
    if battle_id.current == 240
        && duration_frames_value.changed_from_to(&0, &122)
    {
        split(splits, "near_future_enter_inko_fight")
    }
    if battle_id.current == 240
        && duration_frames_value.changed_from_to(&0, &360)
    {
        split(splits, "near_future_defeat_inko")
    }
    if scenario_progress.current == 900
        && map_id.current.matches("None")
        && transition_state.changed_from_to(&4, &0)
    {
        split(splits, "near_future_end_split")
    }
}
