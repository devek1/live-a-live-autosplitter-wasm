use std::collections::HashSet;

use crate::settings::Settings;
use crate::split;
use crate::Chapter;
use asr::string::ArrayCString;
use asr::watcher::Pair;

pub struct DominionOfHate;
impl DominionOfHate {
    pub fn maybe_split(
        settings: &Settings,
        splits: &mut HashSet<String>,
        current_chapter: &Pair<i8>,
        scenario_progress: &Pair<i32>,
        map_id: &Pair<ArrayCString<64>>,
        transition_state: &Pair<u32>,
        bosses_defeated: &Pair<u32>,
        frame_pointer_value: &Pair<i32>,
        duration_frames_value: &Pair<i32>,
        battle_id: &Pair<i32>,
        battle_result: u8
    ) {
        // Start Split
        if settings.start_dominion_of_hate
            && current_chapter.old == Chapter::Menu as i8
            && current_chapter.current == Chapter::DominionOfHate as i8
        {
            split(splits, "start_dominion_of_hate")
        }
        if current_chapter.current == Chapter::DominionOfHate as i8 {
            // Put Scenario Splits Here
            if settings.dominion_start_not_oersted
                && scenario_progress.old == 0
                && scenario_progress.current == 30
            {
                split(splits, "dominion_start_not_oersted")
            }
            if settings.dominion_enter_roost
                && scenario_progress.current == 40
                && scenario_progress.old < 40
            {
                split(splits, "dominion_enter_roost")
            }
            if settings.dominion_enter_odio
                && battle_id.current == 256
                && duration_frames_value.changed_from_to(&0, &212)
            {
                split(splits, "dominion_enter_odio")
            }
            if settings.dominion_defeat_odio_face
                && battle_id.current == 256
                && duration_frames_value.changed_from_to(&0, &637)
            {
                split(splits, "dominion_defeat_odio_face")
            }
            if settings.dominion_defeat_pure_odio
                && battle_id.current == 256
                && duration_frames_value.changed_from_to(&0, &368)
            {
                split(splits, "dominion_defeat_pure_odio")
            }
            if bosses_defeated.changed_from_to(&3, &4) {
                match battle_id.current {
                    256 if settings.dominion_pure_odio_skip => split(splits, "dominion_pure_odio_skip"),
                    88 if settings.dominion_jaggedy_jacks => split(splits,"dominion_jaggedy_jacks"),
                    _ => ()
                }
            }
            if settings.dominion_never_end
                && scenario_progress.current == 80
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "dominion_never_end")
            }
            if settings.dominion_incomplete_destiny
                && scenario_progress.current == 110
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "dominion_incomplete_destiny")
            }
            if settings.dominion_enter_sin_fight
                && battle_id.current == 267
                && duration_frames_value.changed_from_to(&0, &270)
            {
                split(splits, "dominion_enter_sin_fight")
            }
            if settings.dominion_end_sin_phase1
                && battle_id.current == 267
                && duration_frames_value.changed_from_to(&0, &330)
            {
                split(splits, "dominion_end_sin_phase1")
            }
            if settings.split_on_sin_odio
                && battle_id.current == 267
                && duration_frames_value.current == 705
                && frame_pointer_value.old != 0
                && frame_pointer_value.current < 60
            {
                split(splits, "split_on_sin_odio")
            }

            //optional boss splits
            if duration_frames_value.changed_from_to(&0,&347) {
                match battle_id.current {
                    189 if settings.dominion_headhunter => split(splits, "dominion_headhunter"),
                    188 if settings.dominion_death_prophet => split(splits, "dominion_death_prophet"),
                    224 if settings.dominion_apophisphio => split(splits, "dominion_apophisphio"),
                    242 if settings.dominion_lucretius => split(splits, "dominion_lucretius"),
                    //88 if settings.dominion_jaggedy_jacks => split(splits, "dominion_jaggedy_jacks"),
                    66 if settings.dominion_erauqs => split(splits, "dominion_headhunter"),
                    _ => ()
                }
            }

            //Oersted side splits
            if settings.dominion_oersted_start
                && scenario_progress.old == 0
                && scenario_progress.current == 1000
            {
                split(splits, "dominion_oersted_start")
            }
            if settings.dominion_oersted_defeat_steel_titan
                && battle_id.current == 334
                && duration_frames_value.changed_from_to(&0, &347)
            {
                split(splits, "dominion_oersted_defeat_steel_titan")
            }
            if settings.dominion_oersted_armageddon
                && scenario_progress.current == 1010
                && duration_frames_value.changed_from_to(&0, &321)
            {
                split(splits, "dominion_oersted_armageddon")
            }
            if settings.dominion_oersted_sad_ending 
                && scenario_progress.current == 1130
                && map_id.current.matches("None")
                && transition_state.changed_from_to(&4, &0)
            {
                split(splits, "dominion_oersted_sad_ending")
            } 
        }
    }
}
