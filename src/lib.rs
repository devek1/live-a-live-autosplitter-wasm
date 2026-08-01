use asr::{
    Address, Process, future::next_tick, game_engine::unreal::{Version::V4_27, *}, settings::Gui, string::ArrayCString, timer::{self, TimerState}, watcher::{Pair, Watcher},
};
mod scenario_progress;
mod settings;

use bytemuck::Zeroable;
use log::Level;
use settings::Settings;
use std::{collections::HashSet};

asr::async_main!(stable);

struct ChapterData {
    pub character_data: Vec<CharacterData>,
    pub map_key: GamePointer<FNameKey>,
}

impl ChapterData {
    pub fn update(&mut self, process: &Process, module_base: Address) {
        self.update_character_data(process, module_base);
    }

    pub fn update_character_data(&mut self, process: &Process, module_base: Address) {
        let mut character_data: Vec<CharacterData> = vec![];

        let count_addr = vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x158];

        let count: Option<u8> =
            match process.read_pointer_path(module_base, asr::PointerSize::Bit64, &count_addr) {
                Ok(val) => Some(val),
                Err(_e) => Some(0),
            };

        const SIZE: u64 = 0xB0;

        for x in 0..count.unwrap() {
            let offset: u64 = (x as u64) * SIZE;
            let mut data_addr: Vec<u64> = vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x150];

            data_addr.push(offset);

            let character_data_struct: Option<CharacterData> =
                match process.read_pointer_path(module_base, asr::PointerSize::Bit64, &data_addr) {
                    Ok(val) => Some(val),
                    Err(_e) => None,
                };
            if let Some(val) = character_data_struct {
                character_data.push(val);
            }
        }
        self.character_data = character_data;
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
    exp: u32,
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
        let value: Option<T> = match process.read_pointer_path(
            self.module_base,
            asr::PointerSize::Bit64,
            &self.address,
        ) {
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

#[repr(u8)]
enum BattleResult {
    None       = 0,
    Victory    = 1,
    Defeat     = 2, //NOTE - this is actually the default upon loading into a savefile or chapter
    Escape     = 3,
    Teleport   = 4,
    Armageddon = 5,
    Quit       = 6,
    MAX        = 7
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
        let module = Module::wait_attach(&process, V4_27, main_module_base).await;
        let g_world = module.g_world();
        //let last_battle_ptr = UnrealPointer::<8>::new(GWorld,&["AuthorityGameMode","BattleManager","LastBattleLayoutTag"]);
        let last_battle_result_ptr = UnrealPointer::<3>::new(g_world,&["AuthorityGameMode","BattleManager","LastBattleResult"]);
        //let current_battle_result_ptr = UnrealPointer::<8>::new(g_world,&["AuthorityGameMode","BattleManager","CurrentBattleWorld","GameResult"]);
        let battle_layout_row_ptr = UnrealPointer::<4>::new(g_world,&["AuthorityGameMode","BattleManager","CurrentBattleWorld","BattleLayoutMaster"]);
        //let level_sequence_duration_ptr = UnrealPointer::<6>::new(g_world,&["AuthorityGameMode","BattleManager","BattleObjectCollection","CurrentBattleEventSequenceActor","SequencePlayer","DurationFrames"]);
        //let level_sequence_position_ptr = UnrealPointer::<6>::new(g_world,&["AuthorityGameMode","BattleManager","BattleObjectCollection","CurrentBattleEventSequenceActor","SequencePlayer","NetSyncProps"]);
        //let titles_skippable = UnrealPointer::<3>::new(g_world,&["AuthorityGameMode","WB_StartUp","SkipFlag"]);
        //let battle_end_event_field_ptr = UnrealPointer::<4>::new(g_world,&["AuthorityGameMode","FieldManager","bFireEventFromBattleEnd"]);

        //note that ChapterData is a struct, not a class, so we have to deref_offset to it and then add a manual offset over that to get anything from the class
        let gameinstance = Address::from(process.read_pointer_path::<u64>(main_module_base, asr::PointerSize::Bit64, &[0x4A2DA88, 0x20]).unwrap_or_default());
        let chapter_data_ptr = UnrealPointer::<3>::new(gameinstance,&["SaveGameManager","RICSaveGamePlay","TemporaryPlayingChapterData"]); 


        //let test_ptr = UnrealPointer::<8>::new(g_world,&["AuthorityGameMode","BattleManager","CurrentBattleWorld","GameResult"]);
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
        let mut loading_pointer = GamePointer::<u8>::new(
            main_module_base,
            vec![0x5092A98, 0x8, 0x10, 0x50, 0x30, 0x3FA],
        );
        let mut transition_state_pointer = GamePointer::<u32>::new(
            main_module_base,
            vec![0x4A2DA88, 0x20, 0x0 + 0x38, 0x0, 0x70, 0x6C],
        );

        let mut chapter_data = ChapterData {
            character_data: vec![],
            map_key: GamePointer::<FNameKey>::new(
                main_module_base,
                vec![0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x378, 0x418],
            ),
        };

        // Frame number value for Sin Odio fight.
        let mut last_known_position_frame_number_pointer = GamePointer::<u32>::new(
            main_module_base,
            vec![
                0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x338, 0x1B0, 0xF0, 0x250, 0x438,
            ],
        );
        let mut last_known_position_duration_frames_pointer = GamePointer::<u32>::new(
            main_module_base,
            vec![
                0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x338, 0x1B0, 0xF0, 0x250, 0x2C4,
            ],
        );

        let mut map_name_watcher = Watcher::<ArrayCString<256>>::new();
        let mut encounter_watcher = Watcher::<ArrayCString<256>>::new();

        // asr::print_message("UPDATING");
        process
            .until_closes(async {
                let mut bosses_defeated = (0u8, 0u8);
                let mut in_odio_fight = false;
                loop {
                    settings.update();

                    let loading = loading_pointer.update_value(&process);
                    let chapter = current_chapter_pointer.update_value(&process);
                    let new_game_start = new_game_start_pointer.update_value(&process);
                    let scenario_progress = scenario_progress_pointer.update_value(&process);
                    let map_key = chapter_data.map_key.update_value(&process);
                    let map_name = map_name_watcher.update_infallible(module.get_fname(&process, map_key.current).unwrap_or_default());


                    //let last_battle_name = module.get_fname::<256>(&process, last_battle_ptr.deref::<FNameKey>(&process, &module).unwrap_or(FNameKey::zeroed())).unwrap_or_default();
                    let battle_id = encounter_watcher.update_infallible(module.get_fname::<256>(&process, process.read::<FNameKey>(battle_layout_row_ptr.deref_offsets(&process,&module).unwrap_or(Address::NULL).add(0x8)).unwrap_or(FNameKey::zeroed())).unwrap_or_default());
                    let battle_result = last_battle_result_ptr.deref::<u8>(&process,&module).unwrap_or_default();

                    let transition_state = transition_state_pointer.update_value(&process);

                    let frame_pointer_value = last_known_position_frame_number_pointer.update_value(&process);
                    let duration_frames_value = last_known_position_duration_frames_pointer.update_value(&process);
                        

                    chapter_data.update(&process, main_module_base);

                    if chapter.current == Chapter::ImperialChina as u8 {
                        if scenario_progress.current >= 521
                            && scenario_progress.current < 531 //only run the following checks if you are in the gauntlet (and before Yi Pei Kou)
                        {
                            if scenario_progress.old <= 520 //set counter to 0 at the start of the gauntlet
                            {
                                bosses_defeated.1 = bosses_defeated.0;
                                bosses_defeated.0 = 0u8;
                            }
                            if duration_frames_value.current == 180
                                && (duration_frames_value.old == 0 || frame_pointer_value.current < frame_pointer_value.old)
                            {
                                bosses_defeated.1 = bosses_defeated.0;
                                bosses_defeated.0 += 1u8;
                            } 
                            if (duration_frames_value.current == 200
                                && duration_frames_value.old == 0) //resets count on game over
                                || scenario_progress.current > scenario_progress.old //resets count between fights
                            {
                                bosses_defeated.1 = bosses_defeated.0;
                                bosses_defeated.0 = 0u8;
                            }
                        }
                        timer::set_variable_int("Boss defeat animations", bosses_defeated.0)
                    }

                    if chapter.current == Chapter::PresentDay as u8 {
                        if chapter.old != Chapter::PresentDay as u8 //reset count on entering chapter
                        {
                            bosses_defeated.1 = bosses_defeated.0;
                            bosses_defeated.0 = 0u8;
                        }
                        if duration_frames_value.current == 180
                            && duration_frames_value.old == 0 
                        {
                            bosses_defeated.1 = bosses_defeated.0;
                            bosses_defeated.0 += 1u8;
                        }
                        timer::set_variable_int("Martial artists defeated", bosses_defeated.0)
                    }

                    if chapter.current == Chapter::DominionOfHate as u8
                        && settings.dominion_pure_odio_skip
                    {
                        if scenario_progress.current == 60 //only do the following checks if Odio has appeared and hasn't been defeated yet
                        {
                            if duration_frames_value.current == 212 //count starts and resets on entering Odio fight
                                && duration_frames_value.old == 0 
                            {
                                bosses_defeated.1 = bosses_defeated.0;
                                bosses_defeated.0 = 0u8;
                                in_odio_fight = true;
                            }
                            if in_odio_fight
                                && duration_frames_value.current == 180
                                && (duration_frames_value.old == 0 || frame_pointer_value.current < frame_pointer_value.old)
                            {
                                bosses_defeated.1 = bosses_defeated.0;
                                bosses_defeated.0 += 1u8;
                            }
                        }
                        if scenario_progress.old == 60 
                            && scenario_progress.current > 60 //stop count when you either win the fight, or die and see Armageddon
                        {
                            in_odio_fight = false;
                        }
                        timer::set_variable_int("Boss defeat animations", bosses_defeated.0);
                        timer::set_variable("In Odio Fight", &bool::to_string(&in_odio_fight))
                    }

                    // #[cfg(debug_assertions)]
                    {
                        timer::set_variable_int("Current Chapter", chapter.current);
                        timer::set_variable_int("Scenario Progress", scenario_progress.current);
                        timer::set_variable("Current Map", map_name.current.validate_utf8().unwrap_or("[error]"));
                        timer::set_variable_int("Transition State", transition_state.current);
                        timer::set_variable("Loading", &loading.current.to_string());
                        timer::set_variable_int("FPV", frame_pointer_value.current);
                        timer::set_variable_int("DF", duration_frames_value.current);
                        timer::set_variable_int("Last Battle Result",battle_result);
                        timer::set_variable("Current Battle", battle_id.validate_utf8().unwrap_or_default());
                        /*timer::set_variable("Skippable Splash Logos", match titles_skippable.deref(&process, &module){
                            Ok(true) => "yes",
                            Ok(false) => "no",
                            _ => "N/A"
                        });*/
                        for (i, character) in chapter_data.character_data.clone().iter().enumerate() {
                            timer::set_variable_int(&format!("Character {} Level:", i), character.level);
                            timer::set_variable_int(&format!("Character {} Exp:", i), character.exp);
                        }      
                    }
       
                    match timer::state() {
                        TimerState::NotRunning => {
                            if settings.start
                                && chapter.old == Chapter::Menu as u8
                                && chapter.current != Chapter::Menu as u8
                            {
                                // asr::print_message("Clearing Splits and Starting");
                                bosses_defeated = (0u8, 0u8);
                                splits = HashSet::<String>::new();
                                timer::start();
                            }

                            if settings.new_start
                                && new_game_start.old == 0
                                && new_game_start.current > 0
                            {
                                // asr::print_message("Clearing Splits and Starting");
                                bosses_defeated = (0u8, 0u8);
                                splits = HashSet::<String>::new();
                                timer::start();
                            }
                        }
                        TimerState::Running => {
                            // CHAPTER SPLITS

                            scenario_progress::prehistory::Prehistory::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );

                            scenario_progress::distant_future::DistantFuture::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );

                            scenario_progress::imperial_china::ImperialChina::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                bosses_defeated,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );
                            
                            scenario_progress::wild_west::WildWest::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );
                            scenario_progress::present_day::PresentDay::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );
    
                            scenario_progress::near_future::NearFuture::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );

                            scenario_progress::twilight_of_edo_japan::TwilightOfEdoJapan::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                &chapter_data,
                                map_name,
                                &transition_state,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );

                            scenario_progress::middle_ages::MiddleAges::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );

                            scenario_progress::dominion_of_hate::DominionOfHate::maybe_split(
                                &settings,
                                &mut splits,
                                &chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                bosses_defeated,
                                &frame_pointer_value,
                                &duration_frames_value,
                                battle_id,
                                battle_result,
                            );
                            
                            if settings.load_removal {
                                // load/save removal
                                timer::set_variable_int("LOADING", loading.current);
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
