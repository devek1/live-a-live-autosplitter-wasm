use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use crate::ChapterData;
use asr::string::ArrayCString;
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
        map_id: &Pair<ArrayCString<256>>,
        transition_state: &Pair<u32>,
        duration_frames_value: &Pair<u32>,
        battle_id: &Pair<ArrayCString<256>>,
        battle_result: u8
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
            if settings.twilight_attic_ninja_appears
                && scenario_progress.old >= 70
                && scenario_progress.old < 80
                && scenario_progress.current == 80
            {
                split(splits, "twilight_attic_ninja_appears")
            }

            if settings.twilight_level_5_storehouse_leave
                && map_id.old.matches("id.map.07bakuma.050kura.03")
                && map_id.current.matches("id.map.07bakuma.000odecastle.01")
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
                split(splits, "twilight_level_5_storehouse_leave")
            }
            
            if settings.twilight_level_6_storehouse_leave
                && map_id.old.matches("id.map.07bakuma.050kura.03")
                && map_id.current.matches("id.map.07bakuma.000odecastle.01")
                && chapter_data
                    .character_data
                    .clone()
                    .into_iter()
                    .nth(0)
                    .unwrap()
                    .level
                    == 6
            {
                split(splits, "twilight_level_6_storehouse_leave")
            }

            if settings.twilight_defeat_gennai
                && scenario_progress.old >= 80
                && scenario_progress.old < 120
                && scenario_progress.current == 120
            {
                split(splits, "twilight_defeat_gennai")
            }

            if settings.twilight_defeat_monks
                && scenario_progress.old >= 130
                && scenario_progress.old < 160
                && scenario_progress.current == 160
            {
                split(splits, "twilight_defeat_monks")
            }

            if settings.twilight_defeat_musashi
                && battle_id.current.matches("id.battleLayout.140")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "twilight_defeat_musashi")
            }

            if settings.twilight_defeat_yodogimi
                && (battle_id.current.matches("id.battleLayout.164") || battle_id.current.matches("id.battleLayout.137")) //there seem to be two different versions of this battle in the game files
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "twilight_defeat_yodogimi")
            }

            if settings.twilight_defeat_ode_iou
                && battle_id.current.matches("id.battleLayout.171")
                && duration_frames_value.changed_from_to(&0, &180)
            {
                split(splits, "twilight_defeat_ode_iou")
            }

            if settings.twilight_defeat_gamahebi
                && battle_id.current.matches("id.battleLayout.208")
                && duration_frames_value.changed_from_to(&0, &360)
            {
                split(splits, "twilight_defeat_gamahebi")
            }

            if settings.twilight_end_split
                && scenario_progress.current == 280
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "twilight_end_split")
            }
        }
    }
}
