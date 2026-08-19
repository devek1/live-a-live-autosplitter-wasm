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
    battle_last_flowprocessor: &Pair<ArrayCString<48>>,
    battle_result: u8
) {
    // Start Split
    if current_chapter.old == Chapter::Menu as i8
    {
        split(splits, "start_imperial_china")
    }
    // Put Scenario Splits Here
    //
    if scenario_progress.old >= 50
        && scenario_progress.old < 160
        && scenario_progress.current == 160
    {
        split(splits, "imperial_china_recruit_all_diciples")
    }
    if scenario_progress.old >= 300
        && scenario_progress.old < 320
        && scenario_progress.current == 320
    {
        split(splits, "imperial_china_training_complete")
    }
    if scenario_progress.old >= 390
        && scenario_progress.old < 400
        && scenario_progress.current == 400
    {
        split(splits, "imperial_china_defeat_sun_tzu_wang")
    }
    if scenario_progress.old >= 470
        && scenario_progress.old < 490
        && scenario_progress.current == 490
    {
        split(splits, "imperial_china_defeat_temple_guards")
    }
    if scenario_progress.old >= 490
        && scenario_progress.old < 495
        && scenario_progress.current == 495
    {
        split(splits, "imperial_china_defeat_courtyard_guards")
    }
    if scenario_progress.old >= 510
        && scenario_progress.old < 520
        && scenario_progress.current == 520
    {
        split(splits, "imperial_china_defeat_table_guards")
    }
    if battle_id.current == 319
        && battle_last_flowprocessor.bytes_changed()
        && battle_last_flowprocessor.current.matches("BP_BtlProcessor_BattleEnd_03Kunfu")
    {
        split(splits, "imperial_china_defeat_su_xi_san_xi")
    }
    if scenario_progress.current == 522
        && battle_last_flowprocessor.bytes_changed()
        && duration_frames_value.current != 200
        && battle_last_flowprocessor.current.matches("BP_BtlProcessor_BattleEnd_03Kunfu")
    {
        split(splits, "imperial_china_defeat_yi_xi_er_xi")
    }
    if scenario_progress.current == 523
        && battle_last_flowprocessor.bytes_changed()
        && duration_frames_value.current != 200
        && battle_last_flowprocessor.current.matches("BP_BtlProcessor_BattleEnd_03Kunfu")
    {
        split(splits, "imperial_china_defeat_tong_cha_sha_cha")
    }
    if scenario_progress.current == 524
        && battle_last_flowprocessor.bytes_changed()
        && duration_frames_value.current != 200
        && battle_last_flowprocessor.current.matches("BP_BtlProcessor_BattleEnd_03Kunfu")
    {
        split(splits, "imperial_china_defeat_pei_cha_nan_cha")
    }
    if scenario_progress.current == 530
        && battle_last_flowprocessor.bytes_changed()
        && duration_frames_value.current != 200
        && battle_last_flowprocessor.current.matches("BP_BtlProcessor_BattleEnd_03Kunfu")
    {
        split(splits, "imperial_china_defeat_xian_lin_chan")
    }
    if scenario_progress.current == 531
        && duration_frames_value.changed_from_to(&0, &180)
    {
        split(splits, "imperial_china_defeat_yi_pei_kou")
    }
    if scenario_progress.current >= 532
        && duration_frames_value.changed_from_to(&0, &360)
    {
        split(splits, "imperial_china_defeat_ou_di_wan_lee")
    }
    if scenario_progress.current == 650
        && map_id.current.matches("None")
        && transition_state.changed_from_to(&4, &0)
    {
        split(splits, "imperial_china_end_split")
    }
}