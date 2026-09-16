use std::{fs, fs::File as LocalFile, io, path::Path};

use suppaftp::{FtpError, FtpResult, FtpStream, list::File as FtpFile};
pub fn file(ftp: &mut FtpStream, remote_path: &str, local_directory: &Path) -> FtpResult<()> {
    let filename = Path::new(remote_path).file_name().ok_or_else(|| {
        FtpError::ConnectionError(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Remote path has no filename",
        ))
    })?;

    let local_path = local_directory.join(filename);

    ftp.retr(remote_path, |stream| {
        let mut file = LocalFile::create(&local_path).map_err(FtpError::ConnectionError)?;
        io::copy(stream, &mut file).map_err(FtpError::ConnectionError)?;
        Ok(())
    })?;
    println!("Downloaded: {}", remote_path);

    Ok(())
}

pub fn directory(ftp: &mut FtpStream, remote_path: &str, local_path: &Path) -> FtpResult<()> {
    let directory_name = Path::new(remote_path).file_name().ok_or_else(|| {
        FtpError::ConnectionError(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Remote path has no directory name",
        ))
    })?;

    // local_path/CUSAXXXXX-app0 or similar depending on what you're dumping
    let local_root = local_path.join(directory_name);

    // Create local directory, and any missing parent directories
    fs::create_dir_all(&local_root).map_err(FtpError::ConnectionError)?;

    directory_recursive(ftp, remote_path, &local_root)?;

    println!("Download successful");
    Ok(())
}

fn directory_recursive(ftp: &mut FtpStream, remote_path: &str, local_path: &Path) -> FtpResult<()> {
    // Ask FTP for everything inside remote_path
    let entries = ftp.list(Some(remote_path))?;

    // Parse each FTP LIST entry
    for entry in entries {
        let ftp_file = FtpFile::try_from(entry.as_str()).map_err(|error| {
            FtpError::ConnectionError(io::Error::new(
                io::ErrorKind::InvalidData,
                error.to_string(),
            ))
        })?;

        // Skip "." and ".." or it will loop forever!!
        if ftp_file.name() == "." || ftp_file.name() == ".." {
            continue;
        }

        // Build the full remote path
        let remote_entry = format!("{}/{}", remote_path.trim_end_matches('/'), ftp_file.name());
        if ftp_file.is_directory() {
            let local_entry = local_path.join(ftp_file.name());
            fs::create_dir_all(&local_entry).map_err(FtpError::ConnectionError)?;
            directory_recursive(ftp, &remote_entry, &local_entry)?;
        } else if ftp_file.is_file() {
            file(ftp, &remote_entry, local_path)?;
        }
    }

    Ok(())
}