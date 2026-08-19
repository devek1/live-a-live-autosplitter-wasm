use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use crate::ChapterData;
use asr::string::ArrayCString;
use asr::watcher::Pair;

// Locations
// 10228295 - outside storehouse
// 10228695 - Inside Storehouse


pub fn check_splits(
    settings: &Settings,
    splits: &mut HashSet<String>,
    current_chapter: &Pair<i8>,
    scenario_progress: &Pair<i32>,
    chapter_data: &ChapterData,
    map_id: &Pair<ArrayCString<64>>,
    transition_state: &Pair<u32>,
    duration_frames_value: &Pair<i32>,
    battle_id: &Pair<i32>,
    battle_result: u8
) {
    // Start Split
    if current_chapter.old == Chapter::Menu as i8
    {
        split(splits, "start_twilight_of_edo_japan")
    }
        // Put Scenario Splits Here
    if scenario_progress.old >= 70
        && scenario_progress.old < 80
        && scenario_progress.current == 80
    {
        split(splits, "twilight_attic_ninja_appears")
    }

    if map_id.old.matches("id.map.07bakuma.050kura.03")
        && map_id.current.matches("id.map.07bakuma.000odecastle.01")
        && chapter_data
            .character_data[0]
            .level
            == 5
        && chapter_data
            .character_data[0]
            .exp
            >= 56
    {
        split(splits, "twilight_level_5_storehouse_leave")
    }
    
    if map_id.old.matches("id.map.07bakuma.050kura.03")
        && map_id.current.matches("id.map.07bakuma.000odecastle.01")
        && chapter_data
            .character_data[0]
            .level
            == 6
    {
        split(splits, "twilight_level_6_storehouse_leave")
    }

    if scenario_progress.old >= 80
        && scenario_progress.old < 120
        && scenario_progress.current == 120
    {
        split(splits, "twilight_defeat_gennai")
    }

    if scenario_progress.old >= 130
        && scenario_progress.old < 160
        && scenario_progress.current == 160
    {
        split(splits, "twilight_defeat_monks")
    }

    if duration_frames_value.changed_from_to(&0, &180) {
        split(splits,match battle_id.current {
            532 => "twilight_defeat_shiro",
            533 => "twilight_defeat_gennai",
            534 => "twilight_defeat_musashi",
            530 | 531 => "twilight_defeat_yodogimi",
            540 | 541 => "twilight_defeat_ode_iu",
            537 => "twilight_defeat_hayate",
            _ => ""
        })
    }
    if battle_id.current == 542
        && duration_frames_value.changed_from_to(&0, &360)
    {
        split(splits, "twilight_defeat_gamahebi")
    }

    if scenario_progress.current == 280
        && map_id.current.matches("None")
        && transition_state.changed_from_to(&4, &0)
    {
        split(splits, "twilight_end_split")
    }
}
