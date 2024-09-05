use asr::settings::gui::Title;
use asr::settings::Gui;

#[derive(Gui)]
enum Category {
    /// Any%
    SingleStory,
    /// Glitchless
    #[default]
    TrueEnding,
}

#[derive(Gui)]
pub struct Settings {
    // Chapter Splits
    load_settings: Title,
    /// Load Removal
    #[default = true]
    pub load_removal: bool,

    start_settings: Title,
    /// Start on Character Select (Single Story Start)
    pub start: bool,
    /// Automatic Start on New Game
    pub new_start: bool,
    /// Split on Sin Odio Flash
    pub split_on_sin_odio: bool,

    // Full Game Chapter Splits
    full_game_run_chapter_splits: Title,
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
    
    distant_future: Title,
    /// Distant Future - Confront OD-10
    pub distant_future_confront_od10: bool,
    /// Distant Future - Defeat OD-10
    pub distant_future_defeat_od10: bool,
    /// Distant Future - Chapter Complete
    pub distant_future_end_split: bool,

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
    // Present Day - Defeat 1 martial artist
    pub present_day_defeated_1: bool,
    // Present Day - Defeat 2 martial artists
    pub present_day_defeated_2: bool,
    // Present Day - Defeat 3 martial artists
    pub present_day_defeated_3: bool,
    // Present Day - Defeat 4 martial artists
    pub present_day_defeated_4: bool,
    // Present Day - Defeat 5 martial artists
    pub present_day_defeated_5: bool,
    // Present Day - Defeat all martial artists
    pub present_day_defeated_all: bool,
    // Present Day - Defeat Odie
    pub present_day_defeat_odie: bool,
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

    prehistory: Title,
    /// Prehistory - Turn in meat to elder
    pub prehistory_turn_in_meat_to_elder: bool,
    /// Prehistory - Defeat Cavemen
    pub prehistory_defeat_cavemen: bool,
    /// Prehistory - Defeat Zaki 1
    pub prehistory_defeat_zaki_1: bool,
    /// Prehistory - Defeat Zaki 2
    pub prehistory_defeat_zaki_2: bool,
    /// Prehistory - Defeat Zaki 3
    pub prehistory_defeat_zaki_3: bool,
    /// Prehistory - Defeat Odo
    pub prehistory_defeat_odo: bool,
    /// Prehistory - Chapter Complete
    pub prehistory_end_split: bool,

    imperial_china: Title,
    /// Imperial China - Recruit All Disciples
    pub imperial_china_recruit_all_disciples: bool,
    /// Imperial China - Training Complete
    pub imperial_china_training_complete: bool,
    /// Imperial China - Defeat Sun Tzu Wang
    pub imperial_china_defeat_sun_tzu_wang: bool,
    /// Imperial China - Defeat Temple Guards
    pub imperial_china_defeat_temple_guards: bool,
    /// Imperial China - Defeat Courtyard Guards
    pub imperial_china_defeat_courtyard_guards: bool,
    /// Imperial China - Defeat Table Guards
    pub imperial_china_defeat_table_guards: bool,
    /// Imperial China - G1 Defeat Su Xi / San Xi
    pub imperial_china_defeat_su_xi_san_xi: bool,
    /// Imperial China - G2 Defeat Yi Xi / Er Xi
    pub imperial_china_defeat_yi_xi_er_xi: bool,
    /// Imperial China - G3 Defeat Tong Cha / Sha Cha
    pub imperial_china_defeat_tong_cha_sha_cha: bool,
    /// Imperial China - G4 Defeat Pei Cha / Nan Cha
    pub imperial_china_defeat_pei_cha_nan_cha: bool,
    /// Imperial China - G5 Defeat Xian / Lin / Chan
    pub imperial_china_defeat_xian_lin_chan: bool,
    /// Imperial China - G6 Defeat Yi Pei Kou
    pub imperial_china_defeat_yi_pei_kou: bool,
    /// Imperial China - Defeat Ou Di Wan Li
    pub imperial_china_defeat_ou_di_wan_li: bool,
    /// Imperial China - Chapter Complete
    pub imperial_china_end_split: bool,
    
    middle_ages: Title,
    /// Middle Ages - Exit Castle Town
    pub middle_ages_streibough_joins: bool,
    /// Middle Ages - Visit Hasshe
    pub middle_ages_hasshe_house_1: bool,
    /// Middle Ages - Uranus Joins
    pub middle_ages_uranus_joins: bool,
    /// Middle Ages - Hasshe Joins
    pub middle_ages_hasshe_joins: bool,
    /// Middle Ages - Confront The Lord of Dark
    pub middle_ages_archons_roost_1: bool,
    /// Middle Ages - Defeat The Lord of Dark
    pub middle_ages_defeat_lord_of_dark: bool,
    /// Middle Ages - Banished from Lucrece
    pub middle_ages_banished: bool,
    /// Middle Ages - Arrested
    pub middle_ages_arrested: bool,
    /// Middle Ages - Prison Escape
    pub middle_ages_prison_escape: bool,
    /// Middle Ages - Defeat Claustrophobia
    pub middle_ages_defeat_claustrophobia: bool,
    /// Middle Ages - Defeat Scotophobia
    pub middle_ages_defeat_scotophobia: bool,
    /// Middle Ages - Defeat Acrophobia
    pub middle_ages_defeat_acrophobia: bool,
    /// Middle Ages - Defeat Hygrophobia
    pub middle_ages_defeat_hygrophobia: bool,
    /// Middle Ages - Defeat Streibough
    pub middle_ages_defeat_streibough: bool,
    /// Middle Ages - Chapter Complete
    pub middle_ages_end_split: bool,

    dominion_of_hate: Title,
    /// Dominion of Hate - Start as non-Oersted protagonist
    pub dominion_start_not_oersted: bool,
    /// Dominion of Hate - Enter Archon's Roost
    pub dominion_enter_roost: bool,
    /// Dominion of Hate - Enter Odio Fight
    pub dominion_enter_odio: bool,
    /// Dominion of Hate - Defeat Odio Face
    pub dominion_defeat_odio_face: bool,
    ///Dominion of Hate - Defeat Pure Odio
    pub dominion_defeat_pure_odio: bool,
    ///Dominion of Hate - Complete Odio Fight (fade back to cutscene)
    pub dominion_defeat_odio_fade: bool,
    /// Dominion of Hate - Complete Never Ending
    pub dominion_never_end: bool,
    /// Dominion of Hate - Complete Good Ending (not Best Ending)
    pub dominion_incomplete_destiny: bool,
    /// Dominion of Hate - Enter Sin Odio
    pub dominion_enter_sin_fight: bool,
    /// Dominion of Hate - End Sin Odio Phase 1
    pub dominion_end_sin_phase1: bool,
    
    /// Dominion of Hate (Oersted)
    dominion_of_hate_oersted: Title,
    /// Dominion of Hate - Start as Oersted
    pub dominion_oersted_start: bool,
    /// Dominion of Hate (Oersted) - Split on defeating Steel Titan
    pub dominion_oersted_defeat_steel_titan: bool,
    /// Dominion of Hate (Oersted) - Split on Sad Ending
    pub dominion_oersted_sad_ending: bool,
    /// Dominion of Hate (Oersted) - Split on Armageddon Ending
    pub dominion_oersted_armageddon: bool,
}
