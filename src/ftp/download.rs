use std::{fs, fs::File, io, path::Path};

use crate::ftp::targets::RemoteDirectory;
use suppaftp::{FtpError, FtpResult, FtpStream};

pub fn file(ftp: &mut FtpStream, remote_path: &str, local_directory: &Path) -> FtpResult<()> {
    let filename = Path::new(remote_path).file_name().ok_or_else(|| {
        FtpError::ConnectionError(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Remote path has no filename",
        ))
    })?;

    let local_path = local_directory.join(filename);

    ftp.retr(remote_path, |stream| {
        let mut file = File::create(&local_path).map_err(FtpError::ConnectionError)?;

        io::copy(stream, &mut file).map_err(FtpError::ConnectionError)?;

        Ok(())
    })?;

    println!("Downloaded: {}", remote_path);

    Ok(())
}

pub fn directory(
    ftp: &mut FtpStream,
    remote_path: &str,
    local_path: &Path,
) -> FtpResult<()> {
    // Create local directory
    fs::create_dir_all(local_path)
        .map_err(FtpError::ConnectionError)?;


    // Ask FTP for everything inside remote_path

    /*
        For every entry:
            directory -> call directory() again
            file      -> call file()
     */
}