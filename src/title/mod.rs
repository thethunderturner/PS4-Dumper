pub mod app;
pub mod detect;
pub mod dlc;
pub mod patch;

use app::App;
use dlc::DLC;
use patch::Patch;

#[derive(Debug)]
pub struct Title {
    pub title_id: String,
    pub name: Option<String>,
    pub version: Option<String>,

    pub app: App,
    pub patch: Option<Patch>,
    pub dlcs: Vec<DLC>,

    pub union_path: Option<String>,
}