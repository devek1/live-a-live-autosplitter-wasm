use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::string::ArrayCString;
use asr::watcher::Pair;

pub struct PresentDay;
impl PresentDay {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        map_id: &Pair<ArrayCString<256>>,
        transition_state: &Pair<u32>,
        duration_frames_value: &Pair<u32>,
        battle_id: &Pair<ArrayCString<256>>,
        battle_result: u8
    ) {
        // Start Split
        if settings.start_present_day
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::PresentDay as u8
        {
            split(splits, "start_present_day")
        }
        if current_chapter.current == Chapter::PresentDay as u8 {
            if settings.present_day_defeat_tula_han
                && battle_id.current.matches("id.battleLayout.218")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "present_day_defeat_tula_han")
            }
            if settings.present_day_defeat_aja
                && battle_id.current.matches("id.battleLayout.219")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "present_day_defeat_aja")
            }
            if settings.present_day_defeat_max
                && battle_id.current.matches("id.battleLayout.220")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "present_day_defeat_max")
            }
            if settings.present_day_defeat_jackie
                && battle_id.current.matches("id.battleLayout.221")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "present_day_defeat_jackie")
            }
            if settings.present_day_defeat_seishi_moribe
                && battle_id.current.matches("id.battleLayout.222")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "present_day_defeat_seishi_moribe")
            }
            if settings.present_day_defeat_namkiat
                && battle_id.current.matches("id.battleLayout.223")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "present_day_defeat_namkiat")
            }
            if settings.present_day_defeat_odie
                && battle_id.current.matches("id.battleLayout.246")
                && duration_frames_value.changed_from_to(&0, &360)
            {
                split(splits, "present_day_defeat_odie")
            }
            if settings.present_day_end_split
                && scenario_progress.current == 0
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "present_day_end_split")
            }
        }
    }
}
