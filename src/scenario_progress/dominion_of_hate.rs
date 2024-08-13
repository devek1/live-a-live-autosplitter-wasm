use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct DominionOfHate;
impl DominionOfHate {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        _map_id: &Pair<u32>,
        frame_pointer_value: &Pair<u32>,
        duration_frames_value: &Pair<u32>,
    ) {
        // Start Split
        if settings.start_dominion_of_hate
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::DominionOfHate as u8
        {
            split(splits, "start_dominion_of_hate")
        }
        if current_chapter.current == Chapter::DominionOfHate as u8 {
            // Put Scenario Splits Here
            if settings.dominion_start_not_oersted
                && scenario_progress.old == 0
                && scenario_progress.current == 30
            {
                split(splits, "dominion_start_not_oersted")
            }
            if settings.dominion_enter_roost
                && scenario_progress.current == 40
                && scenario_progress.old < 40
            {
                split(splits, "dominion_enter_roost")
            }
            if settings.dominion_enter_odio
                && scenario_progress.current == 60
                && duration_frames_value.current == 212
                && duration_frames_value.old == 0
            {
                split(splits, "dominion_enter_odio")
            }
            if settings.dominion_defeat_odio_face
                && scenario_progress.current == 60
                && duration_frames_value.current == 637
                && duration_frames_value.old == 0
            {
                split(splits, "dominion_defeat_odio_face")
            }
            if settings.dominion_defeat_pure_odio
                && scenario_progress.current == 60
                && duration_frames_value.current == 368
                && duration_frames_value.old == 0
            {
                split(splits, "dominion_defeat_pure_odio")
            }
            if settings.dominion_defeat_odio_fade
                && scenario_progress.old < 70
                && scenario_progress.current == 70
            {
                split(splits, "dominion_defeat_odio_fade")
            }
            if settings.dominion_enter_sin_fight
                && scenario_progress.current == 110
                && duration_frames_value.current == 270
                && duration_frames_value.old == 0
            {
                split(splits, "dominion_enter_sin_fight")
            }
            if settings.dominion_end_sin_phase1
                && scenario_progress.current == 110
                && duration_frames_value.current == 330
                && duration_frames_value.old == 0
            {
                split(splits, "dominion_end_sin_phase1")
            }
            if settings.split_on_sin_odio
                && scenario_progress.current == 110
                && duration_frames_value.current == 705
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "split_on_sin_odio")
            }
            if settings.dominion_oersted_start
                && scenario_progress.old == 0
                && scenario_progress.current == 1000
            {
                split(splits, "dominion_oersted_start")
            }
            if settings.dominion_oersted_defeat_steel_titan
                && scenario_progress.current == 1010
                && duration_frames_value.current == 347
                && duration_frames_value.old == 0
            {
                split(splits, "dominion_oersted_defeat_steel_titan")
            }
            if settings.dominion_oersted_armageddon
                && scenario_progress.current == 1010
                && duration_frames_value.current == 321
                && duration_frames_value.old == 0
            {
                split(splits, "dominion_oersted_armageddon")
            }
            if settings.dominion_oersted_get_revenge 
                && scenario_progress.current == 1020
                && scenario_progress.old < 1020
            {
                split(splits, "dominion_oersted_get_revenge")
            } 
        }
    }
}
