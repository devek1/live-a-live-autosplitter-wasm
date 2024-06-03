use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct PresentDay;
impl PresentDay {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        map_id: &Pair<u32>,
        transition_state: &Pair<u32>,
        namkiat_defeated: &Pair<u8>,
        aja_defeated: &Pair<u8>,
        tula_han_defeated: &Pair<u8>,
        moribe_defeated: &Pair<u8>,
        max_morgan_defeated: &Pair<u8>,
        jackie_defeated: &Pair<u8>,
    ) {
        // Start Split
        if settings.start_present_day
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::PresentDay as u8
        {
            split(splits, "start_present_day")
        }
        if current_chapter.current == Chapter::PresentDay as u8 {
            if settings.present_day_moribe_defeated
                && moribe_defeated.old == 0
                && moribe_defeated.current == 1
            {
                split(splits, "present_day_moribe_defeated_split")
            }
            if settings.present_day_max_morgan_defeated
                && max_morgan_defeated.old == 0
                && max_morgan_defeated.current == 1
            {
                split(splits, "present_day_max_morgan_defeated_split")
            }
            if settings.present_day_jackie_defeated
                && jackie_defeated.old == 0
                && jackie_defeated.current == 1
            {
                split(splits, "present_day_jackie_defeated_split")
            }
            if settings.present_day_namkiat_defeated
                && namkiat_defeated.old == 0
                && namkiat_defeated.current == 1
            {
                split(splits, "present_day_namkiat_defeated_split")
            }
            if settings.present_day_aja_defeated
                && aja_defeated.old == 0
                && aja_defeated.current == 1
            {
                split(splits, "present_day_aja_defeated_split")
            }
            if settings.present_day_tula_han_defeated
                && tula_han_defeated.old == 0
                && tula_han_defeated.current == 1
            {
                split(splits, "present_day_tula_han_defeated_split")
            }
            if settings.present_day_end_split
                && scenario_progress.current == 0
                && map_id.current == 0
                && transition_state.old == 4
                && transition_state.current == 0
            {
                split(splits, "present_day_end_split")
            }
        }
    }
}
