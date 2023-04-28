// #![no_std]
use spinning_top::{const_spinlock, Spinlock};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
mod settings;
use settings::Settings;

use bytemuck::Pod;

use asr::{
    timer::{self, TimerState},
    watcher::Pair,
    Address, Process,
};

static STATE: Spinlock<State> = const_spinlock(State {
    game: None,
    settings: None,
});

pub struct State {
    game: Option<Game>,
    settings: Option<Settings>,
}

struct Watcher<T> {
    watcher: asr::watcher::Watcher<T>,
    address: Vec<u64>,
}

impl<T: Pod> Watcher<T> {
    fn new(address: Vec<u64>) -> Self {
        Self {
            watcher: asr::watcher::Watcher::new(),
            address,
        }
    }

    fn update(&mut self, process: &Process, module: u64) -> Option<&Pair<T>> {
        let value = process.read_pointer_path64::<T>(module, &self.address);
        self.watcher.update(value.ok())
    }
}

struct Game {
    process: Process,
    module: u64,
    start: Watcher<u8>,
    loading: Watcher<u8>,
    splits: HashSet<String>,
}

impl Game {
    fn new(process: Process, module: u64) -> Option<Self> {
        let game = Self {
            process,
            module,
            start: Watcher::new(vec![0x4A2DA88, 0x20, 0x1B8, 0x110, 0x28]), // CurrentGameChapterID
            loading: Watcher::new(vec![0x5092A98, 0x8, 0x10, 0x1f8, 0x50, 0x30, 0x3FA]), // CurrentGameChapterID
            splits: HashSet::new(),
        };
        Some(game)
    }

    fn reset_splits(&mut self) {
        self.splits = HashSet::new();
    }

    fn update_vars(&mut self) -> Option<Vars<'_>> {
        Some(Vars {
            start: self.start.update(&self.process, self.module)?,
            loading: self.loading.update(&self.process, self.module)?,
            splits: &mut self.splits,
        })
    }
}

#[allow(unused)]
pub struct Vars<'a> {
    start: &'a Pair<u8>,
    loading: &'a Pair<u8>,
    splits: &'a mut HashSet<String>,
}

impl Vars<'_> {
    fn split(&mut self, key: &str, settings_field: bool) -> Option<String> {
        if self.splits.contains(key) {
            return None;
        }

        self.splits.insert(key.to_string());

        // only split if in settings
        if settings_field {
            return Some(key.to_string());
        }

        None
    }
}

pub struct Splits(HashSet<String>);

#[no_mangle]
pub extern "C" fn update() {
    let mut state = STATE.lock();
    if state.settings.is_none() {
        state.settings = Some(Settings::register());
        return;
    }

    let settings = state.settings.clone().unwrap();

    if state.game.is_none() {
        match Process::attach("LIVEALIVE") {
            Some(process) => {
                match process.get_module_address("LIVEALIVE-Win64-Shipping.exe") {
                    Ok(Address(module)) => {
                        asr::print_message("attached to process");

                        state.game = Game::new(process, module)
                    }
                    _ => (),
                };
            }
            None => (),
        }
    }

    // linux 
    if state.game.is_none() {
        match Process::attach("LIVEALIVE") {
            Some(process) => {
                match process.get_module_address("LIVEALIVE-Win64-Shipping.exe") {
                    Ok(Address(module)) => {
                        asr::print_message("attached to process");

                        state.game = Game::new(process, module)
                    }
                    _ => (),
                };
            }
            None => (),
        }
    }

    if let Some(game) = &mut state.game {
        if !game.process.is_open() {
            state.game = None;
            return;
        }

        if let Some(mut vars) = game.update_vars() {
            match timer::state() {
                TimerState::NotRunning => {
                    if settings.start && vars.start.old == 9 && vars.start.current != 9 {
                        game.reset_splits();
                        timer::start();
                    }
                }
                TimerState::Running => {
                    if let Some(reason) = should_split(&mut vars, &settings) {
                        asr::print_message(&reason);
                        timer::split();
                    }

                    if settings.load_removal {
                        // load/save removal
                        if vars.loading.old == 0 && vars.loading.current == 1 {
                            timer::resume_game_time()
                        }

                        if vars.loading.old == 1 && vars.loading.current == 0 {
                            timer::pause_game_time()
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn should_split(vars: &mut Vars, settings: &Settings) -> Option<String> {
    // if let Some(split) = splits::hikari::HikariSplits::split(vars, &settings) {
    //     return Some(split);
    // }

    None
}
