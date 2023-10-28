#[derive(asr::settings::Gui)]
pub struct Settings {
    #[default = true]
    /// Load Removal
    pub load_removal: bool,
    /// Automatic Start on character select
    pub start: bool,
    /// Automatic Start on New Game
    pub new_start: bool,
    /// Start Prehistory
    pub start_prehistory: bool,
    /// Start Distant Future
    pub start_distant_future: bool,
    /// Start Near Future
    pub start_near_future: bool,
    /// Start Wild West
    pub start_wild_west: bool,
    /// Start Present Day
    pub start_present_day: bool,
    /// Start Imperial China
    pub start_imperial_china: bool,
    /// Start Twilight of Edo Japan
    pub start_twilight_of_edo_japan: bool,
    /// Start Middle Ages
    pub start_middle_ages: bool,
    /// Start Dominion of Hate
    pub start_dominion_of_hate: bool,

    // Chapter Splits
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
}
