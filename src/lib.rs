use asr::{
    Address, Process, future::next_tick, game_engine::unreal::{Version::V4_27, *}, settings::Gui, string::{ArrayCString, ArrayWString}, timer::{self, TimerState}, watcher::{Watcher}, PointerSize::Bit64
};
mod scenario_progress;
mod settings;
mod helpers;

use bytemuck::Zeroable;
use settings::Settings;
use std::{collections::HashSet};

use crate::helpers::{*};

asr::async_main!(stable);





async fn main() {
    let mut splits = HashSet::<String>::new();
    let mut settings = Settings::register();

    loop {
        let process = match asr::get_os().ok().unwrap().as_str() {
            "linux" => Process::wait_attach("LIVEALIVE-Win64").await,
            _ => Process::wait_attach("LIVEALIVE-Win64-Shipping.exe").await,
        };
        

        // asr::print_message("UPDATING");
        process
            .until_closes(async {
                let (main_module_base, _) = process //no point in grabbing the module size from here, it'd be inaccurate on Linux anyway. If we want the module size we need to use a PE object of the module
                    .wait_module_range("LIVEALIVE-Win64-Shipping.exe")
                    .await;
                let module = Module::wait_attach(&process, V4_27, main_module_base).await;
                let g_world = loop {
                    match module.g_world() {
                        Address::NULL => (),
                        x => break x
                    }
                };
                let g_engine = loop {
                    match module.g_engine() {
                        Address::NULL => (),
                        x => break x
                    }
                };
                //let last_battle_ptr = UnrealPointer::<8>::new(GWorld,&["AuthorityGameMode","BattleManager","LastBattleLayoutTag"]);
                let last_battle_result_ptr = UnrealPointer::<3>::new(g_world,&["AuthorityGameMode","BattleManager","LastBattleResult"]); //read to - u8/BattleResult
                //let current_battle_result_ptr = UnrealPointer::<8>::new(g_world,&["AuthorityGameMode","BattleManager","CurrentBattleWorld","GameResult"]);
                let battle_layout_row_ptr = UnrealPointer::<4>::new(g_world,&["AuthorityGameMode","BattleManager","CurrentBattleWorld","BattleLayoutMaster"]); //read to - pointer (deref_offsets)
                let level_sequence_duration_ptr = UnrealPointer::<6>::new(g_world,&["AuthorityGameMode","BattleManager","BattleObjectCollection","CurrentBattleEventSequenceActor","SequencePlayer","DurationFrames"]); //read to - i32
                let level_sequence_position_ptr = UnrealPointer::<6>::new(g_world,&["AuthorityGameMode","BattleManager","BattleObjectCollection","CurrentBattleEventSequenceActor","SequencePlayer","NetSyncProps"]); //read to - i32
                //let titles_skippable = UnrealPointer::<3>::new(g_world,&["AuthorityGameMode","WB_StartUp","SkipFlag"]);
                //let battle_end_event_field_ptr = UnrealPointer::<4>::new(g_world,&["AuthorityGameMode","FieldManager","bFireEventFromBattleEnd"]);
                let map_key_ptr = UnrealPointer::<3>::new(g_world,&["AuthorityGameMode","FieldManager","CurrentMapTag"]); //read to - FNameKey
                let loading_alt_ptr = UnrealPointer::<3>::new(g_world,&["AuthorityGameMode","FieldManager","bIsWaitingLoadingScreen"]); //read to - bool
                //let game_mode_state = UnrealPointer::<3>::new(g_world,&["AuthorityGameMode","RICStateManager","<idk where the current state is, maybe in UnknownData_LFXP>"]);

                //note that ChapterData is a struct, not a class, so we have to deref_offset to it and then add a manual offset over that to get anything from the class
                //let game_instance = process.read_pointer(main_module_base + 0x4A2DA88, asr::PointerSize::Bit64).unwrap_or(Address::NULL) + 0x20;
                let chapter_data_ptr = UnrealPointer::<4>::new(g_engine,&["GameInstance","SaveGameManager","RICSaveGamePlay","TemporaryPlayingChapterData"]);  //read to - pointer (deref_offsets)
                let chapter_ptr = UnrealPointer::<4>::new(g_engine,&["GameInstance","SaveGameManager","RICSaveGamePlay","CurrentGameChapterID"]);  //read to - u8/Chapter
                let scenario_progress_ptr = UnrealPointer::<4>::new(g_engine,&["GameInstance","SaveGameManager","RICSaveGamePlay","ScenarioProgress"]); //read to - i32

                //let transition_state_ptr = UnrealPointer::<3>::new(game_instance,&["LocalPlayers","_data","CurrentGameChapterID","ViewportClient","<IDK this shows up as UnknownData_F28S[0xC]>"]);  //read to - u8/Chapter


                //let test_ptr = UnrealPointer::<8>::new(g_world,&["AuthorityGameMode","BattleManager","CurrentBattleWorld","GameResult"]);
                // Managers
                // 0x4A2DA88, 0x20, 0x20 // Engine off of GameInstance_C (for now).
                // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78 // World
                // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x120 // GameState
                // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118 // AuthorityGameMode
                // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x338 // BattleManager
                // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x368 // EventManager
                // 0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x378, 0x417 // FieldManager -> CurrentMapTag.TagName
                /*let mut chapter_pointer =
                    GamePointer::<u8>::new(main_module_base, vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x28]);*/
                let mut new_game_start_pointer =
                    GamePointer::<u8>::new(main_module_base, vec![0x508ACE0, 0x10, 0xB0, 0xE0, 0x348]);
                /*let mut scenario_progress_pointer =
                    GamePointer::<u16>::new(main_module_base, vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x1C0]);*/
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
                /*let mut frame_number_pointer = GamePointer::<u32>::new(
                    main_module_base,
                    vec![
                        0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x338, 0x1B0, 0xF0, 0x250, 0x438,
                    ],
                );
                let mut duration_frames_pointer = GamePointer::<u32>::new(
                    main_module_base,
                    vec![
                        0x4A2DA88, 0x20, 0x20, 0x780, 0x78, 0x118, 0x338, 0x1B0, 0xF0, 0x250, 0x2C4,
                    ],
                );*/

                let mut chapter_watcher = Watcher::<i8>::new();
                let mut frame_number_watcher = Watcher::<i32>::new();
                let mut frame_duration_watcher = Watcher::<i32>::new();
                let mut scenario_progress_watcher = Watcher::<i32>::new();
                let mut loading_watcher = Watcher::<bool>::new();
                //let mut map_key_watcher = Watcher::<FNameKey>::new();
                let mut map_name_watcher = Watcher::<ArrayCString<64>>::new();
                let mut encounter_watcher = Watcher::<i32>::new();
                let mut bosses_defeated_watcher = Watcher::<u32>::new();
                bosses_defeated_watcher.update_infallible(0); //initializing the internal pair

                let mut escape_watcher = Watcher::<i32>::new();
                let mut recruit_watcher = Watcher::<i32>::new();
                escape_watcher.update_infallible(0); //initializing the internal pair
                recruit_watcher.update_infallible(0); //initializing the internal pair


                

                //let mut in_odio_fight = false;
                loop {
                    settings.update();


                    let chapter_data_addr = chapter_data_ptr.deref_offsets(&process, &module).unwrap_or(Address::NULL);

                    let loading = loading_pointer.update_value(&process);
                    let chapter = chapter_watcher.update_infallible(chapter_ptr.deref(&process, &module).unwrap_or(-1)); //chapter_pointer.update_value(&process);
                    let new_game_start = new_game_start_pointer.update_value(&process);
                    let scenario_progress = scenario_progress_watcher.update_infallible(process.read(chapter_data_addr+0x70).unwrap_or_default()); //scenario_progress_pointer.update_value(&process);
                    //let map_key = chapter_data.map_key.update_value(&process);
                    let map_name = map_name_watcher.update_infallible(module.get_fname(&process, map_key_ptr.deref(&process, &module).unwrap_or(FNameKey::zeroed())).unwrap_or_default());

                    let loading_alt = loading_watcher.update_infallible(loading_alt_ptr.deref(&process,&module).unwrap_or_default());


                    //let last_battle_name = module.get_fname::<256>(&process, last_battle_ptr.deref::<FNameKey>(&process, &module).unwrap_or(FNameKey::zeroed())).unwrap_or_default();
                    /*let battle_id = encounter_watcher.update_infallible(
                        module.get_fname::<32>(&process, process.read(battle_layout_row_ptr.deref_offsets(&process,&module).unwrap_or(Address::NULL) + 0x8).unwrap_or(FNameKey::zeroed()))
                        .unwrap_or_default().validate_utf8().unwrap_or_default().get(16..19).unwrap_or_default().parse().unwrap_or_default());*/
                    let battle_id = encounter_watcher.update_infallible(process.read(battle_layout_row_ptr.deref_offsets(&process,&module).unwrap_or(Address::NULL) + 0x10).unwrap_or_default());
                    let battle_result = last_battle_result_ptr.deref::<u8>(&process,&module).unwrap_or_default();

                    let transition_state = transition_state_pointer.update_value(&process);

                    let frame_index = frame_number_watcher.update_infallible(level_sequence_position_ptr.deref(&process, &module).unwrap_or_default()); //frame_number_pointer.update_value(&process);
                    let duration_frames = frame_duration_watcher.update_infallible(level_sequence_duration_ptr.deref(&process, &module).unwrap_or_default()); //duration_frames_pointer.update_value(&process);

                    //Testing ScriptVariables
                    //NOTE FOR TOMORROW: The keys are FStrings (=TArray of UTF16 chars) not FGameplayTags/FNames!
                    'blk: {
                        if chapter.current != Chapter::DominionOfHate as i8 {break 'blk;} //for some reason labeled blocks can't have an if
                        //let start = Instant::now();
                        //let mut IntVariables: std::collections::HashMap<String, i32> = std::collections::HashMap::<String,i32>::new();
                        static INTVAR_SIZE : i32 = 0x20; //8 bytes (pointer), 2*4=8 bytes (count+max), 4 bytes (int), 2*4=8 bytes (hash stuff), IDK where last 4 bytes from
                        let mapPtr = chapter_data_ptr.deref_offsets(&process, &module).unwrap_or(Address::NULL) + 0x78;
                        //asr::print_message(&format!("Map Pointer: {}",mapPtr));
                        let Ok(mapData) = process.read_pointer(mapPtr,Bit64) else {break 'blk;};
                        //asr::print_message("Trying to read ScriptVariables");
                        let count = process.read::<i32>(mapPtr + 0x8).unwrap_or_default();
                        //asr::print_message(&format!("IntVars count: {}", count));
                        for i in 0..count {
                            let addr = mapData + i * INTVAR_SIZE;
                            //let strLen = process.read::<usize>(addr + 0x8).unwrap_or_default();
                            //let str = String::from_utf16_lossy(process.read_pointer_path::<ArrayWString<128>>(addr, Bit64, &[
                            //let _str = process.read_vec(process.read_pointer(addr,Bit64).unwrap_or(Address::NULL),strLen).unwrap_or_default();
                            //let str = String::from_utf16_lossy(&_str);
                            /*if str == "08Last_escape_count" {
                                asr::print_message(&format!("Escape count at: {}",addr + 0x10))
                            }
                            IntVariables.insert(
                                str,
                                process.read(addr + 0x10).unwrap_or_default()
                            );*/
                            match process.read_pointer_path::<ArrayWString<128>>(addr, Bit64, &[0x0,0x0]).unwrap_or_default().as_slice() {
                                //comparisons set up using `<string>.split("").map(a=>"0x" + a.charCodeAt(0).toString(16).toUpperCase()).toString()` in JS
                                &[0x30,0x38,0x4C,0x61,0x73,0x74,0x5F,0x65,0x73,0x63,0x61,0x70,0x65,0x5F,0x63,0x6F,0x75,0x6E,0x74]
                                    => { escape_watcher.update_infallible(process.read(addr + 0x10).unwrap_or_default()); }
                                &[0x30,0x38,0x6C,0x61,0x73,0x74,0x5F,0x74,0x6F,0x74,0x61,0x6C,0x5F,0x73,0x63,0x6F,0x75,0x74]
                                    => { recruit_watcher.update_infallible(process.read(addr + 0x10).unwrap_or_default()); }
                                _ => ()
                            }
                        }
                        //asr::print_message(&format!("TMap at: {}", mapData));
                        //asr::print_message(&format!("IntVars found: {}", IntVariables.len()));
                        /*let mut _str = String::default();
                        for (tag,val) in IntVariables {
                            _str += &format!("{} : {}, ",tag,val);
                        }*/
                        //asr::print_message(&_str);
                        //asr::print_message(&format!("intvars read in {} seconds",start.elapsed().as_secs_f64()));
                    }
                    let escape_count = escape_watcher.pair.unwrap();
                    let recruit_count = recruit_watcher.pair.unwrap();
                        

                    chapter_data.update(&process, main_module_base);

                    if chapter.current == Chapter::ImperialChina as i8 {
                        if scenario_progress.current >= 521
                            && scenario_progress.current < 531 //only run the following checks if you are in the gauntlet (and before Yi Pei Kou)
                        {
                            if scenario_progress.old <= 520 //set counter to 0 at the start of the gauntlet
                            {
                                bosses_defeated_watcher.update_infallible(0);
                            }
                            if duration_frames.current == 180
                                && (duration_frames.old == 0 || frame_index.current < frame_index.old)
                            {
                                //bosses_defeated_watcher.update_infallible(bosses_defeated_watcher.pair.unwrap_.current + 1);
                                match bosses_defeated_watcher.pair {
                                    Some(vals) => bosses_defeated_watcher.update_infallible(vals.current + 1),
                                    None => bosses_defeated_watcher.update_infallible(1)
                                };
                            } 
                            if (duration_frames.current == 200
                                && duration_frames.old == 0) //resets count on game over
                                || scenario_progress.current > scenario_progress.old //resets count between fights
                            {
                                bosses_defeated_watcher.update_infallible(0);
                            }
                        }
                        
                    }

                    if chapter.current == Chapter::DominionOfHate as i8
                    {
                        if battle_id.current == 884 && settings.dominion_pure_odio_skip //Odio fight (i.e. the face and Pure Odio. This logic is intended for the glitch where you skip the latter)
                        {
                            if battle_id.old != 884 //count starts and resets on entering Odio fight
                            {
                                bosses_defeated_watcher.update_infallible(0);
                            }
                            if duration_frames.current == 180
                                && (duration_frames.old == 0 || frame_index.current < frame_index.old)
                            {
                                match bosses_defeated_watcher.pair {
                                    Some(vals) => bosses_defeated_watcher.update_infallible(vals.current + 1),
                                    None => bosses_defeated_watcher.update_infallible(1)
                                };
                            }
                        }
                        else if battle_id.current == 861 && settings.dominion_jaggedy_jacks  //Jaggedy Jacks
                        {
                            if battle_id.old != 861 //count starts and resets on entering Jaggedy Jacks fight
                            {
                                bosses_defeated_watcher.update_infallible(0);
                            }
                            if duration_frames.current == 180
                                && (duration_frames.old == 0 || frame_index.current < frame_index.old)
                            {
                                match bosses_defeated_watcher.pair {
                                    Some(vals) => bosses_defeated_watcher.update_infallible(vals.current + 1),
                                    None => bosses_defeated_watcher.update_infallible(1)
                                };
                            }
                        }
                    }
                    let bosses_defeated = bosses_defeated_watcher.pair.unwrap();


                    //info that is valid to see normally
                    timer::set_variable_int("Current Chapter", chapter.current);
                    timer::set_variable("Current Map", map_name.current.validate_utf8().unwrap_or("[error]"));
                    timer::set_variable("Loading Done", &loading.current.to_string());
                    timer::set_variable_int("Last Battle Result",battle_result);
                    timer::set_variable_int("Dominion Flees", escape_count.current);
                    
                    {
                        #![cfg(debug_assertions)]
                        //info that should be kept internal
                        timer::set_variable_int("Scenario Progress", scenario_progress.current);
                        timer::set_variable_int("Transition State", transition_state.current);
                        //timer::set_variable("Loading (Alt)", &loading_alt.current.to_string());
                        timer::set_variable_int("FPV", frame_index.current);
                        timer::set_variable_int("DF", duration_frames.current);
                        timer::set_variable_int("Current Battle", battle_id.current);
                        timer::set_variable_int("New Game Check", new_game_start.current);
                        timer::set_variable_int("Counted boss vanishes", bosses_defeated.current);
                        timer::set_variable_int("Dominion Recruits", recruit_count.current);
                        /*timer::set_variable("Skippable Splash Logos", match titles_skippable.deref(&process, &module){
                            Ok(true) => "yes",
                            Ok(false) => "no",
                            _ => "N/A"
                        });*/
                        for (i, character) in chapter_data.character_data.clone().iter().enumerate() {
                            let id = module.get_fname::<64>(&process, character._tag_name).unwrap_or_default().validate_utf8().unwrap_or_default().rsplit(".").next().unwrap_or_default().to_owned();
                            timer::set_variable(&format!("Character {}:", i), match PARTY_MEMBERS.get(&id) {
                                Some(name) => name,
                                None => &id
                            });
                            timer::set_variable_int(&format!("Character {} Level:", i), character.level);
                            timer::set_variable_int(&format!("Character {} Exp:", i), character.exp);
                        }      
                    }

                    if settings.start
                        && chapter.changed_from(&(Chapter::Menu as i8))
                    {
                        // asr::print_message("Clearing Splits and Starting");
                        bosses_defeated_watcher.update_infallible(0);
                        splits = HashSet::<String>::new();
                        timer::start();
                    }

                    if settings.new_start
                        && new_game_start.old == 0
                        && new_game_start.current > 0
                    {
                        // asr::print_message("Clearing Splits and Starting");
                        bosses_defeated_watcher.update_infallible(0);
                        splits = HashSet::<String>::new();
                        timer::start();
                    }
                    // CHAPTER SPLITS

                    match chapter.current {
                        0 => scenario_progress::middle_ages::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames,
                                battle_id,
                                battle_result,
                             ),
                        1 => scenario_progress::prehistory::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        2 => scenario_progress::distant_future::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        3 => scenario_progress::imperial_china::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &bosses_defeated,
                                &duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        4 => scenario_progress::wild_west::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        5 => scenario_progress::present_day::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        6 => scenario_progress::near_future::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        7 => scenario_progress::twilight_of_edo_japan::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                &chapter_data,
                                map_name,
                                &transition_state,
                                &duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        8 => scenario_progress::dominion_of_hate::check_splits(
                                &settings,
                                &mut splits,
                                chapter,
                                &scenario_progress,
                                map_name,
                                &transition_state,
                                &bosses_defeated,
                                frame_index,
                                duration_frames,
                                battle_id,
                                battle_result,
                            ),
                        _ => ()
                    }
                    
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
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
