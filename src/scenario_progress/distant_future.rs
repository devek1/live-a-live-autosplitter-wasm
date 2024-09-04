use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct DistantFuture;
impl DistantFuture {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        map_id: &Pair<u32>,
        transition_state: &Pair<u32>,
        duration_frames_value: &Pair<u32>,
    ) {
        // Start Split
        if settings.start_distant_future
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::DistantFuture as u8
        {
            split(splits, "start_distant_future")
        }
        if current_chapter.current == Chapter::DistantFuture as u8 {
            // Put Scenario Splits Here
            if settings.distant_future_confront_od10
                && scenario_progress.current == 570
                && duration_frames_value.current == 122
                && duration_frames_value.old == 0
            {
                split(splits, "distant_future_confront_od10")
            }
            if settings.distant_future_defeat_od10
                && scenario_progress.current == 570
                && duration_frames_value.current == 360
                && duration_frames_value.old == 0
            {
                split(splits, "distant_future_defeat_od10")
            }
            if settings.distant_future_end_split
                && scenario_progress.current == 650
                && map_id.current == 0
                && transition_state.old == 4
                && transition_state.current == 0
            {
                split(splits, "distant_future_end_split")
            }
        }
    }
}
