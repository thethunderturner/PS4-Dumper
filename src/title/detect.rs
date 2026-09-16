use crate::title::Title;
use crate::title::app::{find_app, find_title_id};
use crate::title::patch::find_patch;
use suppaftp::FtpStream;

pub const PFSMNT_PATH: &str = "/mnt/sandbox/pfsmnt";
pub fn current(ftp: &mut FtpStream) -> Vec<Title> {
    let mut titles: Vec<Title> = Vec::new();

    let mounts = match ftp.nlst(Some(PFSMNT_PATH)) {
        Ok(mounts) => mounts,
        Err(error) => {
            println!("Could not read mounted titles: {error}");
            return titles;
        }
    };

    let title_ids = find_title_id(&mounts);
    for title_id in title_ids {
        let app = find_app(&mounts, &title_id).expect("Could not find app");
        let patch = find_patch(&mounts, &title_id);

        titles.push(Title {
            title_id,
            name: None,
            version: None,
            category: None,
            app,
            patch,
            dlcs: Vec::new(),
        });
    }

    titles
}
