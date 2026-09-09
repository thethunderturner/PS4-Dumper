use suppaftp::FtpStream;

pub fn all(ftp: &mut FtpStream) -> Vec<String> {
    ftp.list(Some("/"))
        .expect("Couldn't list directory")
}

// TODO: List libraries
pub fn libraries(ftp: &mut FtpStream) -> Vec<String> {
    ftp.list(Some("/system/common/lib/"))
        .expect("Couldn't list directory")
}

// TODO: List fonts
pub fn fonts(ftp: &mut FtpStream) {
    return;
}

// TODO: List trophy
pub fn trophy(ftp: &mut FtpStream) {
    return;
}
