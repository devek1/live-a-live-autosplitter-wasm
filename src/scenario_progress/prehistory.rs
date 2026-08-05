use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::string::ArrayCString;
use asr::watcher::Pair;

pub struct Prehistory;
impl Prehistory {
    pub fn maybe_split(
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
        if settings.start_prehistory
            && current_chapter.old == Chapter::Menu as i8
            && current_chapter.current == Chapter::Prehistory as i8
        {
            split(splits, "start_prehistory")
        }
        if current_chapter.current == Chapter::Prehistory as i8 {
            if settings.prehistory_turn_in_meat_to_elder
                && scenario_progress.old >= 70
                && scenario_progress.old < 80
                && scenario_progress.current == 80
            {
                split(splits, "prehistory_turn_in_meat_to_elder")
            }
            if settings.prehistory_defeat_cavemen
                && scenario_progress.old >= 230
                && scenario_progress.old < 240
                && scenario_progress.current == 240
            {
                split(splits, "prehistory_defeat_cavemen")
            }
            if settings.prehistory_defeat_zaki_1
                && scenario_progress.old >= 240
                && scenario_progress.old < 250
                && scenario_progress.current == 250
            {
                split(splits, "prehistory_defeat_zaki_1")
            }
            if settings.prehistory_defeat_zaki_2
                && scenario_progress.old >= 285
                && scenario_progress.old < 291
                && scenario_progress.current == 291
            {
                split(splits, "prehistory_defeat_zaki_2")
            }
            if settings.prehistory_defeat_zaki_3
                && scenario_progress.old >= 383
                && scenario_progress.old < 405
                && scenario_progress.current == 405
            {
                split(splits, "prehistory_defeat_zaki_3")
            }
            if settings.prehistory_defeat_odo
                && battle_id.current == 243
                && duration_frames_value.changed_from_to(&0, &360)
            {
                split(splits, "prehistory_defeat_odo")
            }
            if settings.prehistory_end_split
                && scenario_progress.current == 440
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "prehistory_end_split")
            }
        }
    }
}
