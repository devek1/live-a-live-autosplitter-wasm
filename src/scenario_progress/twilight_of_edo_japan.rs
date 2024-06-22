use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use crate::ChapterData;
use asr::watcher::Pair;

// Locations
// 10228295 - outside storehouse
// 10228695 - Inside Storehouse

pub struct TwilightOfEdoJapan;
impl TwilightOfEdoJapan {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        chapter_data: &ChapterData,
        map_id: &Pair<u32>,
        transition_state: &Pair<u32>,
    ) {
        // Start Split
        if settings.start_twilight_of_edo_japan
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::TwilightOfEdoJapan as u8
        {
            split(splits, "start_twilight_of_edo_japan")
        }
        if current_chapter.current == Chapter::TwilightOfEdoJapan as u8 {
            // Put Scenario Splits Here
            if settings.twilight_dodge_attic_ninja
                && scenario_progress.old >= 70
                && scenario_progress.old < 80
                && scenario_progress.current == 80
            {
                split(splits, "twilight_dodge_attic_ninja")
            }

            if settings.twilight_defeat_gennai
                && scenario_progress.old >= 80
                && scenario_progress.old < 120
                && scenario_progress.current == 120
            {
                split(splits, "twilight_defeat_gennai")
            }

            if settings.twilight_level_5_storehouse_leave
                && map_id.old == 10228695
                && map_id.current == 10228295
                && chapter_data
                    .character_data
                    .clone()
                    .into_iter()
                    .nth(0)
                    .unwrap()
                    .level
                    == 5
                && chapter_data
                    .character_data
                    .clone()
                    .into_iter()
                    .nth(0)
                    .unwrap()
                    .exp
                    >= 56
            {
                split(splits, "twilight_level_5_storehouse")
            }
            if settings.twilight_level_6_storehouse_leave
                && map_id.old == 10228695
                && map_id.current == 10228295
                && chapter_data
                    .character_data
                    .clone()
                    .into_iter()
                    .nth(0)
                    .unwrap()
                    .level
                    == 6
                && scenario_progress.current == 120
            {
                split(splits, "twilight_level_6_storehouse")
            }

            if settings.twilight_defeat_monks
                && scenario_progress.old >= 130
                && scenario_progress.old < 160
                && scenario_progress.current == 160
            {
                split(splits, "twilight_defeat_monks")
            }

            if settings.twilight_defeat_musashi
                && scenario_progress.old >= 160
                && scenario_progress.old < 170
                && scenario_progress.current == 170
            {
                split(splits, "twilight_defeat_musashi")
            }

            if settings.twilight_defeat_yodogimi
                && scenario_progress.old >= 180
                && scenario_progress.old < 190
                && scenario_progress.current == 190
            {
                split(splits, "twilight_defeat_yodogimi")
            }

            if settings.twilight_defeat_ode_iou
                && scenario_progress.old >= 190
                && scenario_progress.old < 200
                && scenario_progress.current == 200
            {
                split(splits, "twilight_defeat_ode_iou")
            }

            if settings.twilight_end_split
                && scenario_progress.current == 280
                && map_id.current == 0
                && transition_state.old == 4
                && transition_state.current == 0
            {
                split(splits, "twilight_end_split")
            }
        }
    }
}
