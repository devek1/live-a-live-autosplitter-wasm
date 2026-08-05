use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::string::ArrayCString;
use asr::watcher::Pair;

pub struct DistantFuture;
impl DistantFuture {
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
        if settings.start_distant_future
            && current_chapter.old == Chapter::Menu as i8
            && current_chapter.current == Chapter::DistantFuture as i8
        {
            split(splits, "start_distant_future")
        }
        if current_chapter.current == Chapter::DistantFuture as i8 {
            // Put Scenario Splits Here
            if settings.distant_future_confront_od10
                && battle_id.current == 244
                && duration_frames_value.current == 122
                && duration_frames_value.old == 0
            {
                split(splits, "distant_future_confront_od10")
            }
            if settings.distant_future_defeat_od10
                && battle_id.current == 244
                && duration_frames_value.current == 360
                && duration_frames_value.old == 0
            {
                split(splits, "distant_future_defeat_od10")
            }
            if settings.distant_future_end_split
                && scenario_progress.current == 650
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "distant_future_end_split")
            }
        }
    }
}
