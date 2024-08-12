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
        martial_artists_defeated: (u8, u8),
        duration_frames_value: &Pair<u32>,
    ) {
        // Start Split
        if settings.start_present_day
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::PresentDay as u8
        {
            split(splits, "start_present_day")
        }
        if current_chapter.current == Chapter::PresentDay as u8 {
            if settings.present_day_defeated_1
                && martial_artists_defeated.1 == 0
                && martial_artists_defeated.0 == 1
            {
                split(splits, "present_day_defeated_1")
            }
            if settings.present_day_defeated_2
                && martial_artists_defeated.1 == 1
                && martial_artists_defeated.0 == 2
            {
                split(splits, "present_day_defeated_2")
            }
            if settings.present_day_defeated_3
                && martial_artists_defeated.1 == 2
                && martial_artists_defeated.0 == 3
            {
                split(splits, "present_day_defeated_3")
            }
            if settings.present_day_defeated_4
                && martial_artists_defeated.1 == 3
                && martial_artists_defeated.0 == 4
            {
                split(splits, "present_day_defeated_4")
            }
            if settings.present_day_defeated_5
                && martial_artists_defeated.1 == 4
                && martial_artists_defeated.0 == 5
            {
                split(splits, "present_day_defeated_5")
            }
            if settings.present_day_defeated_all
                && martial_artists_defeated.1 == 5
                && martial_artists_defeated.0 == 6
            {
                split(splits, "present_day_defeated_all")
            }
            if settings.present_day_defeat_odie
                && duration_frames_value.current == 360
                && duration_frames_value.old == 0
            {
                split(splits, "present_day_defeat_odie")
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
