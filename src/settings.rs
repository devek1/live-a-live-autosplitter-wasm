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
    /// Park
    pub near_future_park: bool,
    /// Enter Steel Titan 1
    pub near_future_enter_titan: bool,
    /// Dock
    pub near_future_dock: bool,
    /// Matsu Joins
    pub near_future_matsu_joins: bool,
    /// Robot
    pub near_future_robot: bool,
    /// Enter Steel Titan 2
    pub near_future_enter_titan_2: bool,
    /// Enter Great Inko Fight
    pub near_future_enter_inko_fight: bool,
    /// Defeat The Great Inko
    pub near_future_defeat_inko: bool,
    /// Chapter Complete
    pub near_future_end_split: bool,
    
    distant_future: Title,
    /// Distant Future - Confront OD-10
    pub distant_future_confront_od10: bool,
    /// Distant Future - Defeat OD-10
    pub distant_future_defeat_od10: bool,
    /// Distant Future - Chapter Complete
    pub distant_future_end_split: bool,

    /// Twilight of Edo Japan
    twilight_of_edo_japan: Title,
    /// After Attic Ninja Appears
    pub twilight_attic_ninja_appears: bool,
    /// Level 5 (>=56xp) Storehouse Leave
    pub twilight_level_5_storehouse_leave: bool,
    /// OR: Level 6 Storehouse Leave
    pub twilight_level_6_storehouse_leave: bool,
    /// Defeat Gennai
    pub twilight_defeat_gennai: bool,
    /// After Dialog in room after Monk Trio
    pub twilight_defeat_monks: bool,
    /// Defeat Musashi
    pub twilight_defeat_musashi: bool,
    /// Defeat Yodogimi
    pub twilight_defeat_yodogimi: bool,
    /// Defeat Ode Iou (Human Form)
    pub twilight_defeat_ode_iou: bool,
    /// Defeat Ode Iou (Gamahebi Transformation)
    pub twilight_defeat_gamahebi: bool,
    // Chapter Complete
    pub twilight_end_split: bool,

    present_day: Title,
    // Defeat Namkiat
    pub present_day_defeat_namkiat: bool,
    // Defeat The Great Aja
    pub present_day_defeat_aja: bool,
    // Defeat Tula Han
    pub present_day_defeat_tula_han: bool,
    // Defeat Seishi Moribe
    pub present_day_defeat_seishi_moribe: bool,
    // Defeat Max Morgan
    pub present_day_defeat_max: bool,
    // Defeat Jackie Iaukea
    pub present_day_defeat_jackie: bool,
    // Defeat Odie
    pub present_day_defeat_odie: bool,
    // Chapter Complete
    pub present_day_end_split: bool,

    wild_west: Title,
    /// Intro - Defeat Mad Dog
    pub wild_west_defeat_mad_dog_intro: bool,
    /// Defeat Pike
    pub wild_west_defeat_defeat_pike: bool,
    /// Begin Ambush Phase
    pub wild_west_begin_ambush_phase: bool,
    /// End Ambush Phase
    pub wild_west_end_ambush_phase: bool,
    /// Defeat Odie O'Bright
    pub wild_west_defeat_o_dio: bool,
    /// Last Mad Dog Fight
    pub wild_west_mad_dog_final: bool,
    /// Chapter Complete
    pub wild_west_end_split: bool,

    prehistory: Title,
    /// Turn in meat to elder
    pub prehistory_turn_in_meat_to_elder: bool,
    /// Defeat Cavemen
    pub prehistory_defeat_cavemen: bool,
    /// Defeat Zaki 1
    pub prehistory_defeat_zaki_1: bool,
    /// Defeat Zaki 2
    pub prehistory_defeat_zaki_2: bool,
    /// Defeat Zaki 3
    pub prehistory_defeat_zaki_3: bool,
    /// Defeat Odo
    pub prehistory_defeat_odo: bool,
    /// Chapter Complete
    pub prehistory_end_split: bool,

    imperial_china: Title,
    /// Recruit All Disciples
    pub imperial_china_recruit_all_disciples: bool,
    /// Training Complete
    pub imperial_china_training_complete: bool,
    /// Defeat Sun Tzu Wang
    pub imperial_china_defeat_sun_tzu_wang: bool,
    /// Defeat Temple Guards
    pub imperial_china_defeat_temple_guards: bool,
    /// Defeat Courtyard Guards
    pub imperial_china_defeat_courtyard_guards: bool,
    /// Defeat Table Guards
    pub imperial_china_defeat_table_guards: bool,
    /// G1 Defeat Su Xi / San Xi
    pub imperial_china_defeat_su_xi_san_xi: bool,
    /// G2 Defeat Yi Xi / Er Xi
    pub imperial_china_defeat_yi_xi_er_xi: bool,
    /// G3 Defeat Tong Cha / Sha Cha
    pub imperial_china_defeat_tong_cha_sha_cha: bool,
    /// G4 Defeat Pei Cha / Nan Cha
    pub imperial_china_defeat_pei_cha_nan_cha: bool,
    /// G5 Defeat Xian / Lin / Chan
    pub imperial_china_defeat_xian_lin_chan: bool,
    /// G6 Defeat Yi Pei Kou
    pub imperial_china_defeat_yi_pei_kou: bool,
    /// Defeat Ou Di Wan Li
    pub imperial_china_defeat_ou_di_wan_li: bool,
    /// Chapter Complete
    pub imperial_china_end_split: bool,
    
    middle_ages: Title,
    /// Exit Castle Town
    pub middle_ages_streibough_joins: bool,
    /// Visit Hasshe
    pub middle_ages_hasshe_house_1: bool,
    /// Uranus Joins
    pub middle_ages_uranus_joins: bool,
    /// Hasshe Joins
    pub middle_ages_hasshe_joins: bool,
    /// Confront The Lord of Dark
    pub middle_ages_archons_roost_1: bool,
    /// Defeat The Lord of Dark
    pub middle_ages_defeat_lord_of_dark: bool,
    /// Banished from Lucrece
    pub middle_ages_banished: bool,
    /// Arrested
    pub middle_ages_arrested: bool,
    /// Prison Escape
    pub middle_ages_prison_escape: bool,
    /// Defeat Claustrophobia
    pub middle_ages_defeat_claustrophobia: bool,
    /// Defeat Scotophobia
    pub middle_ages_defeat_scotophobia: bool,
    /// Defeat Acrophobia
    pub middle_ages_defeat_acrophobia: bool,
    /// Defeat Hygrophobia
    pub middle_ages_defeat_hygrophobia: bool,
    /// Defeat Streibough
    pub middle_ages_defeat_streibough: bool,
    /// Chapter Complete
    pub middle_ages_end_split: bool,

    /// Dominion of Hate
    dominion_of_hate: Title,
    /// Start as non-Oersted protagonist
    pub dominion_start_not_oersted: bool,
    /// Enter Archon's Roost
    pub dominion_enter_roost: bool,
    /// Enter Odio Fight
    pub dominion_enter_odio: bool,
    /// Defeat Odio Face
    pub dominion_defeat_odio_face: bool,
    ///Defeat Pure Odio
    pub dominion_defeat_pure_odio: bool,
    ///Perform Pure Odio Skip (kill all 4 face pieces)
    pub dominion_pure_odio_skip: bool,
    /// Complete Never Ending
    pub dominion_never_end: bool,
    /// Complete Good Ending (not Best Ending)
    pub dominion_incomplete_destiny: bool,
    /// Enter Sin Odio
    pub dominion_enter_sin_fight: bool,
    /// End Sin Odio Phase 1
    pub dominion_end_sin_phase1: bool,
    
    /// Dominion of Hate (Oersted)
    dominion_of_hate_oersted: Title,
    /// Start as Oersted
    pub dominion_oersted_start: bool,
    /// Dominion of Hate (Oersted) - Split on defeating Steel Titan
    pub dominion_oersted_defeat_steel_titan: bool,
    /// Dominion of Hate (Oersted) - Split on Sad Ending
    pub dominion_oersted_sad_ending: bool,
    /// Dominion of Hate (Oersted) - Split on Armageddon Ending
    pub dominion_oersted_armageddon: bool,
}
