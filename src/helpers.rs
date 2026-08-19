use std::{collections::HashSet, ops::Add};

use asr::{Address, PointerSize::Bit64, Process, game_engine::unreal::FNameKey, timer, watcher::{Pair, Watcher}};



pub fn split(splits: &mut HashSet<String>, key: &str) {
    if key == "" {return;}
    let map = asr::settings::Map::load();
    if map.get(key).unwrap_or(asr::settings::Value::from(false)).get_bool().unwrap_or_default() && !splits.contains(key) {
        splits.insert(key.to_string());
        asr::print_message(&key.to_string());
        timer::split();
        if map.get("start_on_any_split").unwrap_or(asr::settings::Value::from(false)).get_bool().unwrap_or_default() {
            timer::start();
        }
    }
}


pub struct ChapterData {
    pub character_data: Vec<CharacterData>,
}

impl ChapterData {
    pub fn update(&mut self, process: &Process, chapter_data_base: Address) {
        self.update_character_data(process, chapter_data_base);
    }

    pub fn update_character_data(&mut self, process: &Process, chapter_data_base: Address) {
        let mut character_data: Vec<CharacterData> = vec![];

        let count : u32 = process.read(chapter_data_base + 0x8).unwrap_or_default();

        if count == 0 { return; }

        let base_addr = process.read_pointer(chapter_data_base,Bit64).unwrap_or(Address::NULL);

        const SIZE: u64 = 0xB0;

        for x in 0..count {
            let offset: u64 = (x as u64) * SIZE;

            if let Ok(character_data_struct) = process.read::<CharacterData>(base_addr + offset) {
                character_data.push(character_data_struct);
            };
        }
        self.character_data = character_data;
    }
}

#[derive(bytemuck::CheckedBitPattern, Copy, Clone)]
#[repr(C)]
pub struct CharacterData {
    pub _tag_name: FNameKey, //FName.ComparisonIndex
    _tag_suffix: u32, //FName.Number - probably useless here but we need to pad to match the memory layout anyway 
    pub level: u32,
    max_hp: u32,
    physical_attack: u32,
    physical_defense: u32,
    special_attack: u32,
    special_defense: u32,
    agility: u32,
    accuracy: u32,
    evasion: u32,
    pub exp: u32,
}

pub struct GamePointer<T: Clone> {
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

#[repr(i8)]
pub enum Chapter {
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
pub enum BattleResult {
    None       = 0,
    Victory    = 1,
    Defeat     = 2, //NOTE - this is actually the default upon loading into a savefile or chapter
    Escape     = 3,
    Teleport   = 4,
    Armageddon = 5,
    Quit       = 6,
    MAX        = 7
}

pub static PARTY_MEMBERS : phf::Map<&'static str,&'static str> = phf::phf_map!(
    "ORST" => "Oersted",
    "STRY" => "Streibough",
    "URNU" => "Uranus",
    "HASH" => "Hasshe",
    "POGO" => "Pogo",
    "GORI" => "Gori",
    "BERU" => "Beru",
    "ZAKI" => "Zaki",
    "ROU" => "Shifu",
    "YUN" => "Yun",
    "REI" => "Lei",
    "SAMO" => "Hung",
    "KID" => "Sundown",
    "MAD" => "Mad Dog",
    "OBOR" => "Oboromaru",
    "RYOM" => "Sakomoto Ryoma",
    "MASA" => "Masaru",
    "AKIR" => "Akira",
    "TARO" => "Taroimo",
    "MUHO" => "Matsu",
    "CUBE" => "Cube",
    "CAPS" => "Captain Square",
    "KARA" => "Karakurimaru",
    "BURI" => "Tin Great King",
    "ORSTB" => "Demon King Odio", //don't remember if this one is used
);