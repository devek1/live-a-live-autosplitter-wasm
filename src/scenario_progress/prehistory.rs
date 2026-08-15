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
        split(splits, "start_prehistory")
    }
    if scenario_progress.old >= 70
        && scenario_progress.old < 80
        && scenario_progress.current == 80
    {
        split(splits, "prehistory_turn_in_meat_to_elder")
    }
    if scenario_progress.old >= 230
        && scenario_progress.old < 240
        && scenario_progress.current == 240
    {
        split(splits, "prehistory_defeat_cavemen")
    }
    if scenario_progress.old >= 240
        && scenario_progress.old < 250
        && scenario_progress.current == 250
    {
        split(splits, "prehistory_defeat_zaki_1")
    }
    if scenario_progress.old >= 285
        && scenario_progress.old < 291
        && scenario_progress.current == 291
    {
        split(splits, "prehistory_defeat_zaki_2")
    }
    if scenario_progress.old >= 383
        && scenario_progress.old < 405
        && scenario_progress.current == 405
    {
        split(splits, "prehistory_defeat_zaki_3")
    }
    if battle_id.current == 150
        && duration_frames_value.changed_from_to(&0, &360)
    {
        split(splits, "prehistory_defeat_oodiioo")
    }
    if scenario_progress.current == 440
        && map_id.current.matches("None")
        && transition_state.changed_from_to(&4, &0)
    {
        split(splits, "prehistory_end_split")
    }
}