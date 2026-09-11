use suppaftp::FtpStream;

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
