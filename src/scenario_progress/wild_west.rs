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
        && current_chapter.current == Chapter::WildWest as i8
    {
        split(splits, "start_wild_west")
    }
    if current_chapter.current == Chapter::WildWest as i8 {
        if scenario_progress.old == 30
            && scenario_progress.current == 40
        {
            split(splits, "wild_west_defeat_mad_dog_intro")
        }
        if scenario_progress.old == 60
            && scenario_progress.current == 70
        {
            split(splits, "wild_west_defeat_pike")
        }
        if scenario_progress.old == 90
            && scenario_progress.current == 100
        {
            split(splits, "wild_west_begin_ambush_phase")
        }
        if scenario_progress.old >= 100
            && scenario_progress.old < 180
            && scenario_progress.current == 180
        {
            split(splits, "wild_west_end_ambush_phase")
        }
        if battle_id.current == 211
            && duration_frames_value.changed_from_to(&0, &360)
        {
            split(splits, "wild_west_defeat_o_dio")
        }
        if scenario_progress.old == 210
            && scenario_progress.current == 220
        {
            split(splits, "wild_west_mad_dog_final")
        }
        if (scenario_progress.current == 250 || scenario_progress.current == 260)
            && map_id.current.matches("None")
            && transition_state.changed_from_to(&4, &0)
        {
            split(splits, "wild_west_end_split")
        }
    }
}