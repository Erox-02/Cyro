use std::fs;
use std::io;
use std::path::Path;

pub fn create_backup(path: &Path) -> io::Result<()> {
    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Backup source is not a file",
        ));
    }

    let filename = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid filename",
        )
    })?;

    let backup_name = format!(
        ".{}.bak",
        filename.to_string_lossy()
    );

    let backup_path = path.with_file_name(backup_name);

    if backup_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Backup already taken, no overwrite ",
        ));
    }

    fs::copy(path, backup_path)?;

    Ok(())
}
