use std::path::Path;

use suppaftp::{FtpResult, FtpStream};

use super::{App, DLC, Patch, Title};

const PFSMNT_PATH: &str = "/mnt/sandbox/pfsmnt";
const ADDCONT_PATH: &str = "/user/addcont";

pub fn current(ftp: &mut FtpStream) -> FtpResult<Option<Title>> {
    let mounts = ftp.nlst(Some(PFSMNT_PATH))?;

    // First find the running title from XXXXXYYYY-app0.
    let Some(title_id) = find_title_id(&mounts) else {
        return Ok(None);
    };

    let app_name = format!("{title_id}-app0");
    let patch_name = format!("{title_id}-patch0");
    let union_name = format!("{title_id}-app0-patch0-union");

    let app_path = format!("{PFSMNT_PATH}/{app_name}");

    let patch = if contains_entry(&mounts, &patch_name) {
        Some(Patch {
            path: format!("{PFSMNT_PATH}/{patch_name}"),
        })
    } else {
        None
    };

    let union_path = if contains_entry(&mounts, &union_name) {
        Some(format!("{PFSMNT_PATH}/{union_name}"))
    } else {
        None
    };

    let dlcs = detect_dlcs(
        ftp,
        &title_id,
        &mounts,
    )?;

    Ok(Some(Title {
        title_id,

        // We'll get these from param.sfo later.
        name: None,
        version: None,

        app: App {
            path: app_path,
        },

        patch,
        dlcs,
        union_path,
    }))
}

fn find_title_id(entries: &[String]) -> Option<String> {
    for entry in entries {
        let name = entry_name(entry);

        let Some(title_id) = name.strip_suffix("-app0") else {
            continue;
        };

        if is_title_id(title_id) {
            return Some(title_id.to_string());
        }
    }

    None
}

fn detect_dlcs(
    ftp: &mut FtpStream,
    title_id: &str,
    mounts: &[String],
) -> FtpResult<Vec<DLC>> {
    let dlc_root = format!("{ADDCONT_PATH}/{title_id}");

    // Some titles won't have /user/addcont/<TITLE_ID>.
    //
    // For now, treat a failed listing as "no DLC".
    // Later we can specifically distinguish FTP 550 from real FTP errors.
    let installed = match ftp.nlst(Some(&dlc_root)) {
        Ok(entries) => entries,
        Err(_) => return Ok(Vec::new()),
    };

    let mut dlcs = Vec::new();

    for entry in installed {
        let dlc_id = entry_name(&entry);

        if dlc_id == "." || dlc_id == ".." {
            continue;
        }

        let package_path =
            format!("{dlc_root}/{dlc_id}/ac.pkg");

        let mounted_path =
            find_mounted_dlc(mounts, title_id, dlc_id);

        dlcs.push(DLC {
            id: dlc_id.to_string(),
            package_path,
            mounted_path,
        });
    }

    Ok(dlcs)
}

fn find_mounted_dlc(
    mounts: &[String],
    title_id: &str,
    dlc_id: &str,
) -> Option<String> {
    for entry in mounts {
        let name = entry_name(entry);

        // Mounted additional-content paths normally end in "-ac".
        //
        // We also require both the current title ID and DLC ID
        // so another title's mount can't accidentally match.
        if name.ends_with("-ac")
            && name.contains(title_id)
            && name.contains(dlc_id)
        {
            return Some(format!(
                "{PFSMNT_PATH}/{name}"
            ));
        }
    }

    None
}

fn contains_entry(
    entries: &[String],
    wanted: &str,
) -> bool {
    entries
        .iter()
        .any(|entry| entry_name(entry) == wanted)
}

fn entry_name(entry: &str) -> &str {
    Path::new(entry)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(entry)
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