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
    bosses_defeated: &Pair<u32>,
    frame_pointer_value: &Pair<i32>,
    duration_frames_value: &Pair<i32>,
    battle_id: &Pair<i32>,
    battle_last_flowprocessor: &Pair<ArrayCString<48>>,
    battle_result: u8
) {
    // Start Split
    if current_chapter.old == Chapter::Menu as i8
    {
        split(splits, "start_dominion_of_hate")
    }
        // Put Scenario Splits Here
    if scenario_progress.old == 0
        && scenario_progress.current == 30
    {
        split(splits, "dominion_start_not_oersted")
    }
    if scenario_progress.current == 40
        && scenario_progress.old < 40
    {
        split(splits, "dominion_enter_roost")
    }
    if battle_last_flowprocessor.bytes_changed()
        && duration_frames_value.current != 200
        && battle_last_flowprocessor.current.matches("BP_BtlProcessor_BattleEnd_08Last") 
    {
        split(splits, match battle_id.current {
            884 => "dominion_pure_odio_skip",
            861 => "dominion_jaggedy_jacks",
            _ => ""
        })
    }
    else if battle_id.current == 884
        && duration_frames_value.changed_from(&0) 
    {
        split(splits,match duration_frames_value.current {
            212 => "dominion_enter_odio",
            637 => "dominion_defeat_odio_face",
            368 => "dominion_defeat_pure_odio",
            _ => ""
        })
    }
    if battle_id.current == 885
        && duration_frames_value.changed_from_to(&0, &270)
    {
        split(splits, "dominion_enter_sin_fight")
    }
    if battle_id.current == 885
        && duration_frames_value.changed_from_to(&0, &330)
    {
        split(splits, "dominion_end_sin_phase1")
    }
    if battle_id.current == 885
        && duration_frames_value.current == 705
        && frame_pointer_value.old != 0
        && frame_pointer_value.current < 60
    {
        split(splits, "split_on_sin_odio")
    }

    //optional boss splits
    if duration_frames_value.changed_from_to(&0,&347) {
        split(splits, match battle_id.current {
            858 => "dominion_headhunter",
            //861 => split(splits, "dominion_jaggedy_jacks"),
            862 => "dominion_apophisphio",
            863 => "dominion_lucretius",
            864 => "dominion_death_prophet",
            865 => "dominion_headhunter",
            _ => ""
        })
    }

    //Oersted side splits
    if scenario_progress.changed_from_to(&0,&1000)
    {
        split(splits, "dominion_oersted_start")
    }
    if battle_id.current == 882
        && duration_frames_value.changed_from_to(&0, &347)
    {
        split(splits, "dominion_oersted_defeat_steel_titan")
    }
    if scenario_progress.current == 1010
        && duration_frames_value.changed_from_to(&0, &321)
    {
        split(splits, "dominion_oersted_armageddon")
    }


    //Various endings (excluding Best End because it doesn't have this screen and also the run is considered done earlier)
    if map_id.current.matches("None")
        && transition_state.changed_from_to(&4, &0) 
    {
        split(splits, match scenario_progress.current {
            80 => "dominion_never_end",
            110 => "dominion_incomplete_destiny",
            1130 => "dominion_oersted_sad_ending",
            _ => ""
        })
    }
}