use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct ImperialChina;
impl ImperialChina {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        map_id: &Pair<u32>,
        transition_state: &Pair<u32>,
    ) {
        // Start Split
        if settings.start_imperial_china
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::ImperialChina as u8
        {
            split(splits, "start_imperial_china")
        }
        if current_chapter.current == Chapter::ImperialChina as u8 {
            // Put Scenario Splits Here
            //
            if settings.imperial_china_recruit_all_disciples
                && scenario_progress.old >= 50
                && scenario_progress.old < 160
                && scenario_progress.current == 160
            {
                split(splits, "imperial_china_recruit_all_diciples")
            }
            if settings.imperial_china_training_complete
                && scenario_progress.old >= 300
                && scenario_progress.old < 320
                && scenario_progress.current == 320
            {
                split(splits, "imperial_china_training_complete")
            }
            if settings.imperial_china_defeat_sun_tzu_wang
                && scenario_progress.old >= 390
                && scenario_progress.old < 400
                && scenario_progress.current == 400
            {
                split(splits, "imperial_china_defeat_sun_tzu_wang")
            }
            if settings.imperial_china_defeat_temple_guards
                && scenario_progress.old >= 470
                && scenario_progress.old < 490
                && scenario_progress.current == 490
            {
                split(splits, "imperial_china_defeat_temple_guards")
            }
            if settings.imperial_china_defeat_courtyard_guards
                && scenario_progress.old >= 490
                && scenario_progress.old < 495
                && scenario_progress.current == 495
            {
                split(splits, "imperial_china_defeat_courtyard_guards")
            }
            if settings.imperial_china_defeat_table_guards
                && scenario_progress.old >= 510
                && scenario_progress.old < 520
                && scenario_progress.current == 520
            {
                split(splits, "imperial_china_defeat_table_guards")
            }
            if settings.imperial_china_defeat_su_xi_san_xi
                && scenario_progress.old >= 521
                && scenario_progress.old < 522
                && scenario_progress.current == 522
            {
                split(splits, "imperial_china_defeat_su_xi_san_xi")
            }
            if settings.imperial_china_defeat_yi_xi_er_xi
                && scenario_progress.old >= 522
                && scenario_progress.old < 523
                && scenario_progress.current == 523
            {
                split(splits, "imperial_china_defeat_yi_xi_er_xi")
            }
            if settings.imperial_china_defeat_tong_cha_sha_cha
                && scenario_progress.old >= 523
                && scenario_progress.old < 524
                && scenario_progress.current == 524
            {
                split(splits, "imperial_china_defeat_tong_cha_sha_cha")
            }
            if settings.imperial_china_defeat_pei_cha_nan_cha
                && scenario_progress.old >= 524
                && scenario_progress.old < 530
                && scenario_progress.current == 530
            {
                split(splits, "imperial_china_defeat_pei_cha_nan_cha")
            }
            if settings.imperial_china_defeat_xian_lin_chan
                && scenario_progress.old >= 530
                && scenario_progress.old < 531
                && scenario_progress.current == 531
            {
                split(splits, "imperial_china_defeat_xian_lin_chan")
            }
            if settings.imperial_china_defeat_yi_pei_kou
                && scenario_progress.old >= 531
                && scenario_progress.old < 532
                && scenario_progress.current == 532
            {
                split(splits, "imperial_china_defeat_yi_pei_kou")
            }
            if settings.imperial_china_defeat_ou_di_wan_li
                && scenario_progress.old >= 533
                && scenario_progress.old < 550
                && scenario_progress.current == 550
            {
                split(splits, "imperial_china_defeat_yi_pei_kou")
            }
            if settings.imperial_china_end_split
                && scenario_progress.current == 650
                && map_id.current == 0
                && transition_state.old == 4
                && transition_state.current == 0
            {
                split(splits, "imperial_china_end_split")
            }
        }
    }
}
