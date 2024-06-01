use asr::{
    future::next_tick,
    settings::Gui,
    timer::{self, TimerState},
    watcher::{Pair, Watcher},
    Address, Process,
};
mod scenario_progress;
mod settings;

use settings::Settings;
use std::collections::HashSet;

asr::async_main!(stable);

struct GamePointer<T: Clone> {
    pub address: Vec<u64>,
    pub watcher: Watcher<T>,
    pub module_base: Address,
}

impl<T: Clone + bytemuck::Pod> GamePointer<T> {
    pub fn new(module_base: Address, address: Vec<u64>) -> GamePointer<T> {
        GamePointer {
            watcher: Watcher::<T>::new(),
            address,
            module_base,
        }
    }
    pub fn update_value(&mut self, process: &Process) -> Pair<T> {
        let value: Option<T> = match process.read_pointer_path64(self.module_base, &self.address) {
            Ok(val) => Some(val),
            Err(_e) => None,
        };

        return *self.watcher.update_infallible(value.unwrap());
    }
}

// fn get_offset(chapter: u8) -> u64 {
//     return match chapter {
//         0 => 0x70,
//         1 => 0x330,
//         2 => 0x5F0,
//         3 => 0x8B0,
//         4 => 0xB70,
//         5 => 0xE30,
//         6 => 0x10F0,
//         7 => 0x13B0,
//         8 => 0x1670,
//         9 => 0x1930,
//         10 => 0x1BF0,
//         _ => 0x0
//     };
// }

#[repr(u8)]
enum Chapter {
    MiddleAges = 0,         // Oersted
    NearFuture = 6,         // Akira
    TwilightOfEdoJapan = 7, // Oborumaru
    DominionOfHate = 8,
    Menu = 9,
}

async fn main() {
    // TODO: Set up some general state and settings.

    //let mut transition_state = Watcher::<u32>::new(); // Transition State for various game states

    let mut loading_watcher = Watcher::<u8>::new(); // CurrentGameChapterID
                                                    // let mut scenario_progress_watcher = Watcher::<u32>::new(); // CurrentGameChapterID
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

        let mut current_chapter_pointer =
            GamePointer::<u8>::new(main_module_base, vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x28]);
        let mut new_game_start_pointer =
            GamePointer::<u8>::new(main_module_base, vec![0x508ACE0, 0x10, 0xB0, 0xE0, 0x348]);
        let mut scenario_progress_pointer =
            GamePointer::<u16>::new(main_module_base, vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x1C0]);

        asr::print_message("UPDATING");
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                loop {
                    settings.update();

                    let current_chapter = current_chapter_pointer.update_value(&process);
                    let new_game_start = new_game_start_pointer.update_value(&process);
                    let scenario_progress = scenario_progress_pointer.update_value(&process);
                    timer::set_variable_int("Current Chapter", current_chapter.current);
                    timer::set_variable_int("Scenario Progress", scenario_progress.current);

                    // if current_chapter.current == Chapter::Oborumaru as u8 {
                    //     asr::print_message("OBORO");
                    // }

                    // asr::print_message(&scenario_progress.current.to_string());
                    let loading_value = match process.read_pointer_path64(
                        main_module_base,
                        &vec![0x5092A98, 0x8, 0x10, 0x50, 0x30, 0x3FA],
                    ) {
                        Ok(val) => Some(val),
                        Err(_e) => Some(0),
                    };

                    let loading = loading_watcher.update(loading_value).unwrap();

                    // Scenario Progress

                    match timer::state() {
                        TimerState::NotRunning => {
                            if settings.start
                                && current_chapter.old == 9
                                && current_chapter.current != 9
                            {
                                // asr::print_message("Clearing Splits and Starting");
                                splits = HashSet::<String>::new();
                                timer::start();
                            }

                            if settings.new_start
                                && new_game_start.old == 0
                                && new_game_start.current > 0
                            {
                                // asr::print_message("Clearing Splits and Starting");
                                splits = HashSet::<String>::new();
                                timer::start();
                            }
                        }
                        TimerState::Running => {
                            // CHAPTER SPLITS

                            if settings.start_prehistory
                                && current_chapter.old == 9
                                && current_chapter.current == 1
                            {
                                split(&mut splits, "start_prehistory")
                            }
                            if settings.start_distant_future
                                && current_chapter.old == 9
                                && current_chapter.current == 2
                            {
                                split(&mut splits, "start_distant_future")
                            }
                            if settings.start_imperial_china
                                && current_chapter.old == 9
                                && current_chapter.current == 3
                            {
                                split(&mut splits, "start_imperial_china")
                            }
                            if settings.start_wild_west
                                && current_chapter.old == 9
                                && current_chapter.current == 4
                            {
                                split(&mut splits, "start_wild_west")
                            }
                            if settings.start_present_day
                                && current_chapter.old == 9
                                && current_chapter.current == 5
                            {
                                split(&mut splits, "start_present_day")
                            }

                            // Near Future
                            scenario_progress::near_future::NearFuture::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );
                            
                            scenario_progress::twilight_of_edo_japan::TwilightOfEdoJapan::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );

                            scenario_progress::middle_ages::MiddleAges::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );

                            scenario_progress::dominion_of_hate::DominionOfHate::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );

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
