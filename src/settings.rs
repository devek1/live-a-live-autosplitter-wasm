#[derive(asr::Settings, Clone)]
pub struct Settings {
    #[default = true]
    /// Automatic Start on character select
    pub start: bool,
    #[default = true]
    /// Load Removal
    pub load_removal: bool,
}
