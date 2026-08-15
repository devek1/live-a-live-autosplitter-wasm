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
        split(splits, "start_present_day")
    }
    if duration_frames_value.changed_from_to(&0, &180) {
        split(splits, match battle_id.current {
            701 => "present_day_defeat_tula_han",
            702 => "present_day_defeat_aja",
            703 => "present_day_defeat_max",
            704 => "present_day_defeat_jackie",
            705 => "present_day_defeat_seishi_moribe",
            706 => "present_day_defeat_namkiat",
            707 => "present_day_defeat_odie",
            _ => ""
        })
    }
    if scenario_progress.current == 0
        && map_id.current.matches("None")
        && transition_state.changed_from_to(&4, &0)
    {
        split(splits, "present_day_end_split")
    }
}