use std::path::Path;
use suppaftp::{FtpResult, FtpStream};

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

pub fn game(
    ftp: &mut FtpStream,
    full: bool,
) -> FtpResult<Option<String>> {
    let path = "/mnt/sandbox/pfsmnt/";

    let entries = ftp.nlst(Some(path))?;

    for entry in entries {
        let name = Path::new(&entry)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&entry);

        if let Some(title_id) = name.strip_suffix("-app0") {
            if is_title_id(title_id) {
                return if full {
                    Ok(Some(format!("{path}{name}")))
                } else {
                    Ok(Some(title_id.to_string()))
                }
            }
        }
    }

    Ok(None)
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
