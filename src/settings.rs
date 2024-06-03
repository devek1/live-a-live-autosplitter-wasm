use asr::settings::Gui;
use asr::settings::gui::Title;

#[derive(Gui)]
enum Category {
    /// Any%
    SingleStory,
    /// Glitchless
    #[default]
    TrueEnding
}

#[derive(Gui)]
pub struct Settings {
    // Chapter Splits
    load_settings: Title,
    /// Load Removal
    #[default = true]
    pub load_removal: bool,

    start_settings: Title,
    /// Automatic Start on Character Select
    pub start: bool,
    /// Automatic Start on New Game
    pub new_start: bool,
    /// Start/Split on Prehistory
    pub start_prehistory: bool,
    /// Start/Split on Distant Future
    pub start_distant_future: bool,
    /// Start/Split on Wild West
    pub start_wild_west: bool,
    /// Start/Split on Present Day
    pub start_present_day: bool,
    /// Start/Split on Imperial China
    pub start_imperial_china: bool,
    /// Start/Split on Twilight of Edo Japan
    pub start_twilight_of_edo_japan: bool,
    /// Start/Split on Middle Ages
    pub start_middle_ages: bool,
    /// Start/Split on Dominion of Hate
    pub start_dominion_of_hate: bool,
    /// Start/Split on Near Future
    pub start_near_future: bool,

    // Chapter Splits
    near_future: Title,
    /// Near Future - Park
    pub near_future_park: bool,
    /// Near Future - Enter Steel Titan 1
    pub near_future_enter_titan: bool,
    /// Near Future - Dock
    pub near_future_dock: bool,
    /// Near Future - Matsu Joins
    pub near_future_matsu_joins: bool,
    /// Near Future - Robot
    pub near_future_robot: bool,
    /// Near Future - Enter Steel Titan 2
    pub near_future_enter_titan_2: bool,
    // Near Future - Chapter Complete
    pub near_future_end_split: bool,

    twilight_of_edo_japan: Title,
    /// Edo Japan - After Dodging Attic Ninja
    pub twilight_dodge_attic_ninja: bool,
    /// Edo Japan - Level 5 (>=56xp) Storehouse Leave
    pub twilight_level_5_storehouse_leave: bool,
    /// Edo Japan - OR: Level 6 Storehouse Leave
    pub twilight_level_6_storehouse_leave: bool,
    /// Edo Japan - Defeat Gennai
    pub twilight_defeat_gennai: bool,
    /// Edo Japan - After Dialog in room after Monk Trio
    pub twilight_defeat_monks: bool,
    /// Edo Japan - Defeat Musashi
    pub twilight_defeat_musashi: bool,
    /// Edo Japan - Defeat Yodogimi
    pub twilight_defeat_yodogimi: bool,
    /// Edo Japan - Defeat Ode Iou (Human Form)
    pub twilight_defeat_ode_iou: bool,
    // Edo Japan - Chapter Complete
    pub twilight_end_split: bool,

    present_day: Title,
    #[heading_level = 2]
    last_fight_before_odie_will_not_trigger: Title,
    // Present Day - Moribe Defeated
    pub present_day_moribe_defeated: bool,
    // Present Day - Max Morgan Defeated
    pub present_day_max_morgan_defeated: bool,
    // Present Day - Jackie Defeated
    pub present_day_jackie_defeated: bool,
    // Present Day - Namkiat Defeated
    pub present_day_namkiat_defeated: bool,
    // Present Day - Aja Defeated
    pub present_day_aja_defeated: bool,
    // Present Day - Tula Han Defeated
    pub present_day_tula_han_defeated: bool,
    // Present Day - Start Odie Fight
    pub present_day_start_odie: bool,
    // Present Day - Chapter Complete
    pub present_day_end_split: bool,

    wild_west: Title,
    /// Wild West - Intro - Defeat Mad Dog
    pub wild_west_defeat_mad_dog_intro: bool,
    /// Wild West - Defeat Pike
    pub wild_west_defeat_defeat_pike: bool, 
    /// Wild West - Begin Ambush Phase
    pub wild_west_begin_ambush_phase: bool, 
    /// Wild West - End Ambush Phase
    pub wild_west_end_ambush_phase: bool, 
    /// Wild West - Defeat Odie O'Bright
    pub wild_west_defeat_odie: bool, 
    /// Wild West - Last Mad Dog Fight
    pub wild_west_mad_dog_final: bool, 
    /// Wild West - Chapter Complete
    pub wild_west_end_split: bool, 
}
