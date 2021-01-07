use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{ensure, Result};

pub enum CompressionType {
    Zip,
    Tarball,
}

#[cfg(target_family = "unix")]
fn unzip_unix(zip_file: &Path, tgt_dir: &Path) -> Result<()> {
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
    ensure!(
        status.success(),
        "Error with unzip - status: {}",
        status.code().unwrap_or(1)
    );
    Ok(())
}

fn unzip(zip_file: &Path, tgt_dir: &Path) -> Result<()> {
    if cfg!(unix) {
        unzip_unix(zip_file, tgt_dir)
    } else {
        // TODO: Implement other platforms.
        unimplemented!();
    }
}

#[cfg(target_family = "unix")]
fn untar_unix(tar_file: &Path, tgt_dir: &Path) -> Result<()> {
    let mut child = Command::new("tar")
        .arg("xvzf")
        .arg(tar_file)
        .arg("-C")
        .arg(tgt_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        let code = status.code().unwrap_or(1);
        ensure!(status.success(), "Error with untar - status: {}", code);
        Ok(())
    }
}

fn untar(tar_file: &Path, tgt_dir: &Path) -> Result<()> {
    if cfg!(unix) {
        untar_unix(tar_file, tgt_dir)
    } else {
        // TODO: Implement other platforms
        unimplemented!()
    }
}

pub fn extract(path: &Path, tgt_dir: &Path, compression: CompressionType) -> Result<()> {
    match compression {
        CompressionType::Zip => unzip(path, tgt_dir),
        CompressionType::Tarball => untar(path, tgt_dir),
    }
}

pub fn get_compression(ext: &str) -> Option<CompressionType> {
    if ext == "zip" {
        Some(CompressionType::Zip)
    } else if ext == "txz" || ext == "tgz" || ext == "gz" || ext == "xz" {
        // Not perfect. Would be better to have a list of supported .tar extensions.
        Some(CompressionType::Tarball)
    } else {
        None
    }
}
