use asr::{
    future::next_tick,
    timer::{self, TimerState},
    watcher::Watcher,
    Process,
    settings::Gui 
};
mod settings;
use settings::Settings;
use std::collections::HashSet;

asr::async_main!(stable);

async fn main() {
    // TODO: Set up some general state and settings.
    let mut start_watcher = Watcher::<u8>::new(); // CurrentGameChapterID
    let mut new_game_start_watcher = Watcher::<u8>::new(); // Start Trigger for New Game
    let mut loading_watcher = Watcher::<u8>::new(); // CurrentGameChapterID
    let mut near_future_scenario_progress_watcher = Watcher::<u32>::new(); // CurrentGameChapterID
    let mut splits = HashSet::<String>::new();
    let mut settings = Settings::register();
    loop {
        let process = match asr::get_os().ok().unwrap().as_str() {
            "linux" => Process::wait_attach("LIVEALIVE-Win64").await,
            _ => Process::wait_attach("LIVEALIVE-Win64-Shipping.exe").await,
        };
        let (main_module_base, _main_module_size) = process
            .wait_module_range("LIVEALIVE-Win64-Shipping.exe")
            .await;

        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                loop {
                    settings.update();
                    let start = start_watcher
                        .update(
                            process
                                .read_pointer_path64(
                                    main_module_base,
                                    &vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x28],
                                )
                                .ok(),
                        )
                        .unwrap();

                    let new_start_value = match process.read_pointer_path64(
                        main_module_base,
                        &vec![0x508ACE0, 0x10, 0xB0, 0xE0, 0x348],
                    ) {
                        Ok(val) => Some(val),
                        Err(_e) => Some(0),
                    };
                    let new_start = new_game_start_watcher.update(new_start_value).unwrap();

                    // TODO: Move all this to one query to join it all at once instead of recursing
                    let near_future_scenario_progress_value = match process.read_pointer_path64(
                        main_module_base,
                        &vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x30, 0x1080 + 0x70],
                    ) {
                        Ok(val) => Some(val),
                        Err(_e) => Some(0),
                    };
                    let near_future_scenario_progress = near_future_scenario_progress_watcher
                        .update(near_future_scenario_progress_value)
                        .unwrap();
                    // asr::print_message(&near_future_scenario_progress.current.to_string());

                    let loading = loading_watcher
                        .update(
                            process
                                .read_pointer_path64(
                                    main_module_base,
                                    &vec![0x5092A98, 0x8, 0x10, 0x50, 0x30, 0x3FA],
                                )
                                .ok(),
                        )
                        .unwrap();

                    // Scenario Progress

                    match timer::state() {
                        TimerState::NotRunning => {
                            if settings.start && start.old == 9 && start.current != 9 {
                                // asr::print_message("Clearing Splits and Starting");
                                splits = HashSet::<String>::new();
                                timer::start();
                            }

                            if settings.new_start && new_start.old == 0 && new_start.current > 0 {
                                // asr::print_message("Clearing Splits and Starting");
                                splits = HashSet::<String>::new();
                                timer::start();
                            }
                        }
                        TimerState::Running => {
                            // CHAPTER SPLITS

                            if settings.start_prehistory && start.old == 9 && start.current == 1 {
                                split(&mut splits, "start_prehistory")
                            }
                            if settings.start_distant_future && start.old == 9 && start.current == 2
                            {
                                split(&mut splits, "start_distant_future")
                            }
                            if settings.start_imperial_china && start.old == 9 && start.current == 3
                            {
                                split(&mut splits, "start_imperial_china")
                            }
                            if settings.start_wild_west && start.old == 9 && start.current == 4 {
                                split(&mut splits, "start_wild_west")
                            }
                            if settings.start_present_day && start.old == 9 && start.current == 5 {
                                split(&mut splits, "start_present_day")
                            }

                            // Near Future
                            if settings.start_near_future && start.old == 9 && start.current == 6 {
                                split(&mut splits, "start_near_future")
                            }
                            if settings.near_future_park
                                && near_future_scenario_progress.old == 85
                                && near_future_scenario_progress.current == 110
                            {
                                split(&mut splits, "near_future_park")
                            }
                            if settings.near_future_enter_titan
                                && near_future_scenario_progress.old == 270
                                && near_future_scenario_progress.current == 280
                            {
                                split(&mut splits, "near_future_enter_titan")
                            }
                            if settings.near_future_dock
                                && near_future_scenario_progress.old == 380
                                && near_future_scenario_progress.current == 390
                            {
                                split(&mut splits, "near_future_dock")
                            }
                            if settings.near_future_matsu_joins
                                && near_future_scenario_progress.old == 410
                                && near_future_scenario_progress.current == 450
                            {
                                split(&mut splits, "near_future_matsu_joins")
                            }
                            if settings.near_future_robot
                                && near_future_scenario_progress.old == 460
                                && near_future_scenario_progress.current == 490
                            {
                                split(&mut splits, "near_future_robot")
                            }
                            if settings.near_future_enter_titan_2
                                && near_future_scenario_progress.old == 670
                                && near_future_scenario_progress.current == 746
                            {
                                split(&mut splits, "near_future_enter_titan_2")
                            }

                            if settings.start_twilight_of_edo_japan
                                && start.old == 9
                                && start.current == 7
                            {
                                split(&mut splits, "start_twilight_of_edo_japan")
                            }
                            if settings.start_middle_ages && start.old == 9 && start.current == 0 {
                                split(&mut splits, "start_middle_ages")
                            }
                            if settings.start_dominion_of_hate
                                && start.old == 9
                                && start.current == 8
                            {
                                split(&mut splits, "start_dominion_of_hate")
                            }

                            if settings.load_removal {
                                // load/save removal

                                if loading.old == 0 && loading.current == 1 {
                                    // asr::print_message("resuming game time");
                                    timer::resume_game_time()
                                }

                                if loading.old == 1 && loading.current == 0 {
                                    // asr::print_message("pausing game time");
                                    timer::pause_game_time()
                                }
                            }
                        }
                        _ => {}
                    }
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}

pub fn split(splits: &mut HashSet<String>, key: &str) {
    if !splits.contains(key) {
        splits.insert(key.to_string());
        asr::print_message(&key.to_string());
        timer::split()
    }
}
