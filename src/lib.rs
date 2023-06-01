use asr::{
    future::next_tick,
    timer::{self, TimerState},
    watcher::Watcher,
    Process,
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
    let mut splits = HashSet::<String>::new();
    let settings = Settings::register();
    loop {
        let process = Process::wait_attach("LIVEALIVE-Win64-Shipping.exe").await;
        let (main_module_base, _main_module_size) = process
            .wait_module_range("LIVEALIVE-Win64-Shipping.exe")
            .await;

        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                loop {
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
                            if settings.start_near_future && start.old == 9 && start.current == 6 {
                                split(&mut splits, "start_near_future")
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
                                && start.current == 7
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
