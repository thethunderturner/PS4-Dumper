use suppaftp::FtpStream;

pub fn list_all(ftp: &mut FtpStream) -> Vec<String> {
    ftp.list(Some("/"))
        .expect("Couldn't list directory")
}

// TODO: List libraries
pub fn list_libraries(ftp: &mut FtpStream) -> Vec<String> {
    ftp.list(Some("/system/common/lib/"))
        .expect("Couldn't list directory")
}

// TODO: List fonts
pub fn list_fonts(ftp: &mut FtpStream) {
    return;
}

// TODO: List trophy
pub fn list_trophy(ftp: &mut FtpStream) {
    return;
}
