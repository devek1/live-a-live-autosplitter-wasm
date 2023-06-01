#[derive(asr::Settings, Clone)]
pub struct Settings {
    #[default = false]
    /// Automatic Start on character select
    pub start: bool,
    #[default = false]
    /// Automatic Start on New Game
    pub new_start: bool,
    #[default = false]
    /// Load Removal
    pub load_removal: bool,
    #[default = false]
    /// Start Prehistory
    pub start_prehistory: bool,
    #[default = false]
    /// Start Distant Future
    pub start_distant_future: bool,
    #[default = false]
    /// Start Near Future
    pub start_near_future: bool,
    #[default = false]
    /// Start Wild West
    pub start_wild_west: bool,
    #[default = false]
    /// Start Present Day
    pub start_present_day: bool,
    #[default = false]
    /// Start Imperial China
    pub start_imperial_china: bool,
    #[default = false]
    /// Start Twilight of Edo Japan
    pub start_twilight_of_edo_japan: bool,
    #[default = false]
    /// Start Middle Ages
    pub start_middle_ages: bool,
    #[default = false]
    /// Start Dominion of Hate
    pub start_dominion_of_hate: bool,
    
}
