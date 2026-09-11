use std::path::Path;
use suppaftp::{FtpResult, FtpStream};

#[derive(Debug)]
pub struct Title {
    pub title_id: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub category: Option<String>,

    pub app_path: String,
    pub patch_path: Option<String>,
    pub union_path: Option<String>,
}

pub fn root(ftp: &mut FtpStream) -> Vec<String> {
    ftp
        .nlst(Some("/"))
        .expect("Couldn't list directory")
}

pub fn libraries(ftp: &mut FtpStream, full: bool) -> Vec<String> {
    let path = "/system/common/lib/";

    let files = ftp
        .nlst(Some(path))
        .expect("Couldn't list directory");

    if full {
        files
            .into_iter()
            .map(|file| format!("{path}{file}"))
            .collect()
    } else {
        files
    }
}

// TODO: List fonts
pub fn fonts(ftp: &mut FtpStream) {
    return;
}

// TODO: List trophy
pub fn trophy(ftp: &mut FtpStream) {
    return;
}

// TODO: Return contents of custom directory
pub fn custom(ftp: &mut FtpStream, full: bool) {
    return;
}

pub fn title(
    ftp: &mut FtpStream,
) -> FtpResult<Option<Title>> {
    let path = "/mnt/sandbox/pfsmnt/";

    let entries = ftp.nlst(Some(path))?;

    let mut title_id = None;
    let mut app_path = None;
    let mut patch_path = None;
    let mut union_path = None;

    for entry in entries {
        let name = Path::new(&entry)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&entry);

        if let Some(id) = name.strip_suffix("-app0") {
            if is_title_id(id) {
                title_id = Some(id.to_string());
                app_path = Some(format!("{path}{name}"));
            }
        }

        if let Some(id) = name.strip_suffix("-patch0") {
            if is_title_id(id) {
                patch_path = Some(format!("{path}{name}"));
            }
        }

        if name.ends_with("-app0-patch0-union") {
            union_path = Some(format!("{path}{name}"));
        }
    }

    let Some(title_id) = title_id else {
        return Ok(None);
    };

    let Some(app_path) = app_path else {
        return Ok(None);
    };

    Ok(Some(Title {
        title_id,
        name: None,
        version: None,
        category: None,

        app_path,
        patch_path,
        union_path,
    }))
}

fn is_title_id(value: &str) -> bool {
    value.len() == 9
        && value
        .chars()
        .take(4)
        .all(|c| c.is_ascii_uppercase())
        && value
        .chars()
        .skip(4)
        .all(|c| c.is_ascii_digit())
}
