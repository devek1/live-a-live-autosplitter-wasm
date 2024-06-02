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

struct ChapterData {
  character_data: Vec<CharacterData>,
  map_id: u32,
}

impl ChapterData {
    pub fn update(&mut self, process: &Process, module_base: Address) {
        self.update_character_data(process, module_base);
        self.update_map_id(process, module_base);
    }

    pub fn update_character_data(&mut self, process: &Process, module_base: Address) {
        let mut character_data: Vec<CharacterData> = vec![];

        let count_addr = vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x158];

        let count: Option<u8> = match process.read_pointer_path(module_base, asr::PointerSize::Bit64, &count_addr) {
            Ok(val) => Some(val),
            Err(_e) => Some(0),
        };

        const SIZE: u64 = 0xB0;

        for x in 0..count.unwrap() {
            let offset: u64 = (x as u64) * SIZE;
            let mut data_addr: Vec<u64> = vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x150];

            data_addr.push(offset);
            
            let character_data_struct: Option<CharacterData> = match process.read_pointer_path(module_base, asr::PointerSize::Bit64,  &data_addr) {
                Ok(val) => Some(val),
                Err(_e) => None,
            };
            if let Some(val) = character_data_struct {
                character_data.push(val);
            }
        }
        self.character_data = character_data;
    }

    pub fn update_map_id(&mut self, process: &Process, module_base: Address) {

        let map_id_addr = vec![ 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x378, 0x418];

        let map_id: Option<u32> = match process.read_pointer_path(module_base, asr::PointerSize::Bit64, &map_id_addr) {
            Ok(val) => Some(val),
            Err(_e) => Some(0),
        };

        if let Some(val) = map_id {
            self.map_id = val;
        } else {
            self.map_id = 0;
        }
        
    }
}

#[derive(bytemuck::CheckedBitPattern, Copy, Clone)]
#[repr(C)]
struct CharacterData {
   _tag_name: u64,
   level: u32,
   max_hp: u32,
   physical_attack: u32,
   physical_defense: u32,
   special_attack: u32,
   special_defense: u32,
   agility: u32,
   accuracy: u32,
   evasion: u32,
   exp: u32
}


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
        let value: Option<T> = match process.read_pointer_path(self.module_base, asr::PointerSize::Bit64, &self.address) {
            Ok(val) => Some(val),
            Err(_e) => Some(T::zeroed()),
        };
        return *self.watcher.update_infallible(value.unwrap());
    }
}

#[repr(u8)]
enum Chapter {
    MiddleAges = 0,         // Oersted
    Prehistory = 1,         // Pogo
    DistantFuture = 2,      // Cube
    ImperialChina = 3,      // Master
    WildWest = 4,           // Sundown
    PresentDay = 5,         // Masaru
    NearFuture = 6,         // Akira
    TwilightOfEdoJapan = 7, // Oborumaru
    DominionOfHate = 8,     // End Chapter3
    Menu = 9,
}

async fn main() {
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
        // Managers
        // 0x4A2DA88, 0x20, 0x20 // Engine off of GameInstance_C (for now).
        // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78 // World
        // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x120 // GameState
        // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118 // AuthorityGameMode
        // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x338 // BattleManager
        // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x368 // EventManager
        // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x378, 0x417 // FieldManager -> CurrentMapTag.TagName
        let mut current_chapter_pointer =
            GamePointer::<u8>::new(main_module_base, vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x28]);
        let mut new_game_start_pointer =
            GamePointer::<u8>::new(main_module_base, vec![0x508ACE0, 0x10, 0xB0, 0xE0, 0x348]);
        let mut scenario_progress_pointer =
            GamePointer::<u16>::new(main_module_base, vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x1C0]);
        let mut loading_pointer =
            GamePointer::<u16>::new(main_module_base, vec![0x5092A98, 0x8, 0x10, 0x50, 0x30, 0x3FA]);
        let mut chapter_data = ChapterData { character_data: vec![], map_id: 0 };
        // asr::print_message("UPDATING");
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                loop {
                    settings.update();

                    let loading = loading_pointer.update_value(&process);
                    let current_chapter = current_chapter_pointer.update_value(&process);
                    let new_game_start = new_game_start_pointer.update_value(&process);
                    let scenario_progress = scenario_progress_pointer.update_value(&process);

                    chapter_data.update(&process, main_module_base);

                    // #[cfg(debug_assertions)]
                    {
                        timer::set_variable_int("Current Chapter", current_chapter.current);
                        timer::set_variable_int("Scenario Progress", scenario_progress.current);
                        timer::set_variable_int("Map ID", chapter_data.map_id);
                        for (i, character) in chapter_data.character_data.clone().iter().enumerate() {
                            timer::set_variable_int(&format!("Character {} Level:", i), character.level);
                            timer::set_variable_int(&format!("Character {} Exp:", i), character.exp);
                        }      
                    }
       
                    match timer::state() {
                        TimerState::NotRunning => {
                            if settings.start
                                && current_chapter.old == Chapter::Menu as u8
                                && current_chapter.current != Chapter::Menu as u8
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

                            scenario_progress::prehistory::Prehistory::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );

                            scenario_progress::distant_future::DistantFuture::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );

                            scenario_progress::imperial_china::ImperialChina::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );
                            
                            scenario_progress::wild_west::WildWest::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );
                            scenario_progress::present_day::PresentDay::maybe_split(
                                &settings,
                                &mut splits,
                                &current_chapter,
                                &scenario_progress,
                            );
    
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
