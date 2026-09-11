use std::{fs::File, io, path::Path};

use suppaftp::{FtpError, FtpResult, FtpStream};
use crate::ftp::targets::RemoteDirectory;

pub fn file(
    ftp: &mut FtpStream,
    remote_path: &str,
    local_directory: &Path,
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

    let local_path = local_directory.join(filename);

    ftp.retr(remote_path, |stream| {
        let mut file =
            File::create(&local_path)
                .map_err(FtpError::ConnectionError)?;

        io::copy(stream, &mut file)
            .map_err(FtpError::ConnectionError)?;

        Ok(())
    })?;

    println!("Downloaded: {}", remote_path);

    Ok(())
}

pub fn directory(
    ftp: &mut FtpStream,
    directory: &RemoteDirectory,
    full: bool,
) -> FtpResult<Vec<String>> {
    let path = directory.path();

    let files = ftp.nlst(Some(path))?;

    if full {
        Ok(
            files
                .into_iter()
                .map(|file| {
                    format!(
                        "{}/{}",
                        path.trim_end_matches('/'),
                        file
                    )
                })
                .collect()
        )
    } else {
        Ok(files)
    }
}