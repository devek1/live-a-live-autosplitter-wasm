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
        split(splits, "start_middle_ages")
    }
    // Put Scenario Splits Here
    if scenario_progress.current == 80
        && scenario_progress.old < 80
    {
        split(splits, "middle_ages_streibough_joins")
    }
    if scenario_progress.current == 100
        && scenario_progress.old < 100
    {
        split(splits, "middle_ages_hasshe_house_1")
    }
    if scenario_progress.current == 110
        && scenario_progress.old < 110
    {
        split(splits, "middle_ages_uranus_joins")
    }
    if scenario_progress.current == 130
        && scenario_progress.old < 130
    {
        split(splits, "middle_ages_hasshe_joins")
    }
    if scenario_progress.current == 150
        && duration_frames_value.changed_from_to(&0, &122)
    {
        split(splits, "middle_ages_archons_roost_1")
    }
    if battle_id.current == 27
        && duration_frames_value.changed_from_to(&0, &347)
    {
        split(splits, "middle_ages_defeat_lord_of_dark")
    }
    if scenario_progress.current == 250
        && scenario_progress.old < 250
    {
        split(splits, "middle_ages_banished")
    }
    if scenario_progress.current == 270
        && scenario_progress.old < 270
    {
        split(splits, "middle_ages_arrested")
    }
    if scenario_progress.current == 360
        && scenario_progress.old < 360
    {
        split(splits, "middle_ages_prison_escape")
    }
    if battle_id.current == 55
        && duration_frames_value.changed_from_to(&0, &180)
    {
        split(splits, "middle_ages_defeat_claustrophobia")
    }
    if battle_id.current == 56
        && duration_frames_value.changed_from_to(&0, &180)
    {
        split(splits, "middle_ages_defeat_scotophobia")
    }
    if battle_id.current == 57
        && duration_frames_value.changed_from_to(&0, &180)
    {
        split(splits, "middle_ages_defeat_acrophobia")
    }
    if battle_id.current == 58
        && duration_frames_value.changed_from_to(&0, &180)
    {
        split(splits, "middle_ages_defeat_hygrophobia")
    }
    if battle_id.current == 59
        && duration_frames_value.changed_from_to(&0, &360)
    {
        split(splits, "middle_ages_defeat_streibough")
    }
    if scenario_progress.current == 510
        && map_id.current.matches("None")
        && transition_state.changed_from_to(&4, &0)
    {
        split(splits, "middle_ages_end_split")
    }
}
