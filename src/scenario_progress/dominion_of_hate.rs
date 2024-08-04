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
        map_id: &Pair<u32>,
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
            if settings.split_on_enter_odio
                && scenario_progress.current == 60
                && duration_frames_value.current == 212
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "split_on_enter_odio")
            }
            if settings.split_on_odio_face
                && scenario_progress.current == 60
                && duration_frames_value.current == 637
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "split_on_odio_face")
            }
            if settings.split_on_pure_odio
                && scenario_progress.old == 60
                && scenario_progress.current == 70
            {
                split(splits, "split_on_pure_odio")
            }
            if settings.split_on_sin_start
                && scenario_progress.current == 110
                && duration_frames_value.current == 270
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "split_on_sin_start")
            }
            if settings.split_on_sin_phase1
                && scenario_progress.current == 110
                && duration_frames_value.current == 330
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "split_on_sin_phase1")
            }
            if settings.split_on_sin_odio
                && scenario_progress.current == 110
                //&& map_id.current == 10237032
                && duration_frames_value.current == 705
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "split_on_sin_odio")
            }
        }
    }
}
