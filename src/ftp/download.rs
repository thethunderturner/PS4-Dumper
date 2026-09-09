use std::{
    fs::File,
    io,
    path::Path,
};

use suppaftp::{FtpError, FtpResult, FtpStream};

pub fn file(
    ftp: &mut FtpStream,
    remote_path: &str,
    local_directory: &str,
) -> FtpResult<()> {
    let filename = Path::new(remote_path)
        .file_name()
        .ok_or_else(|| {
            FtpError::ConnectionError(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Remote path has no filename",
                )
            )
        })?;

    let local_path =
        Path::new(local_directory).join(filename);

    ftp.retr(remote_path, |stream| {
        let mut file =
            File::create(&local_path)
                .map_err(FtpError::ConnectionError)?;

        io::copy(stream, &mut file)
            .map_err(FtpError::ConnectionError)?;

        Ok(println!("Downloaded file: {:?}", filename))
    })
}
pub fn directory() {
    return;
}
