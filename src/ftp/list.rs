use suppaftp::FtpStream;
use crate::ftp::targets::RemoteDirectory;

pub fn root(ftp: &mut FtpStream) -> Vec<String> {
    ftp.nlst(Some("/")).expect("Couldn't list directory")
}

pub fn libraries(ftp: &mut FtpStream, full: bool) -> Vec<String> {
    let path = RemoteDirectory::path(&RemoteDirectory::Libraries);

    let files = ftp.nlst(Some(path)).expect("Couldn't list directory");

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
pub fn fonts(_ftp: &mut FtpStream) {
    return;
}

// TODO: List trophy
pub fn trophy(_ftp: &mut FtpStream) {
    return;
}

// TODO: Return contents of custom directory
pub fn custom(_ftp: &mut FtpStream, _full: bool) {
    return;
}
