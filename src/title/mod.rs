pub mod app;
pub mod detect;
pub mod dlc;
pub mod patch;
pub mod sfo;

pub use app::App;
pub use dlc::DLC;
pub use patch::Patch;

#[derive(Debug)]
pub struct Title {
    pub title_id: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub category: Option<String>,
    pub app: App,
    pub patch: Option<Patch>,
    pub dlcs: Vec<DLC>,
}
