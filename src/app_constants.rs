pub struct AppConstants;

impl AppConstants {
    /// Setup Folders to point to assets folder instead of the program root directory
    pub const BASE: &str = "./assets";
    /// Domain part of the app's reverse domain name
    pub const DOMAIN: &str = "tech";
    /// Company part of the app's reverse domain name
    pub const COMPANY: &str = "CyberCitadel";
    /// Name of the app
    pub const APP_NAME: &str = "BasicRoguelike";
}
