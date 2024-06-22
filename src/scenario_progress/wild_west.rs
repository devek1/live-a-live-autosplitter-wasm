use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::watcher::Pair;

pub struct WildWest;
impl WildWest {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<u8>,
        scenario_progress: &Pair<u16>,
        map_id: &Pair<u32>,
        transition_state: &Pair<u32>,
    ) {
        // Start Split
        if settings.start_wild_west
            && current_chapter.old == Chapter::Menu as u8
            && current_chapter.current == Chapter::WildWest as u8
        {
            split(splits, "start_wild_west")
        }
        if current_chapter.current == Chapter::WildWest as u8 {
            if settings.wild_west_defeat_mad_dog_intro
                && scenario_progress.old == 30
                && scenario_progress.current == 40
            {
                split(splits, "wild_west_defeat_mad_dog_intro_split")
            }
            if settings.wild_west_defeat_defeat_pike
                && scenario_progress.old == 60
                && scenario_progress.current == 70
            {
                split(splits, "wild_west_defeat_pike_split")
            }
            if settings.wild_west_begin_ambush_phase
                && scenario_progress.old == 90
                && scenario_progress.current == 100
            {
                split(splits, "wild_west_begin_ambush_phase_split")
            }
            if settings.wild_west_end_ambush_phase
                && scenario_progress.old >= 100
                && scenario_progress.old < 180
                && scenario_progress.current == 180
            {
                split(splits, "wild_west_end_ambush_phase_split")
            }
            if settings.wild_west_defeat_odie
                && scenario_progress.old == 200
                && scenario_progress.current == 210
            {
                split(splits, "wild_west_defeat_odie")
            }
            if settings.wild_west_mad_dog_final
                && scenario_progress.old == 210
                && scenario_progress.current == 220
            {
                split(splits, "wild_west_mad_dog_final")
            }
            if settings.wild_west_end_split
                && scenario_progress.current == 250
                && map_id.current == 0
                && transition_state.old == 4
                && transition_state.current == 0
            {
                split(splits, "wild_west_end_split")
            }
        }
    }
}
