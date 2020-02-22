use std::path::Path;
use std::process::{Command, Stdio};

use crate::error::{BinmanError, BinmanResult, Cause};

#[cfg(target_family = "unix")]
fn unzip_unix(zip_file: &Path, tgt_dir: &Path) -> BinmanResult<()> {
    // TODO: Validate that unzip is installed.
    // This throws a bad error.
    let mut child = Command::new("unzip")
        .arg(zip_file.to_str().unwrap())
        .arg("-d")
        .arg(tgt_dir.to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        let code = status.code().unwrap_or(1);
        Err(BinmanError::new(
            Cause::InvalidState,
            &format!("Error with unzip - status: {}", code),
        ))
    }
}

pub fn unzip(zip_file: &Path, tgt_dir: &Path) -> BinmanResult<()> {
    if cfg!(unix) {
        unzip_unix(zip_file, tgt_dir)
    } else {
        // TODO: Implement other platforms.
        unimplemented!();
    }
}
