use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{ensure, Result};

#[derive(Clone, Copy, Debug)]
pub enum CompressionType {
    Zip,
    Tarball,
    Zstd,
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

#[cfg(target_family = "unix")]
fn unzstd_unix(zstd_file: &Path, tgt_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(tgt_dir)?;

    let no_ext = zstd_file.with_extension("");
    let path = no_ext.file_name().unwrap();

    tracing::debug!("unzstd: path: {:?}", path);

    let mut child = Command::new("unzstd")
        .arg("-d")
        .arg(zstd_file)
        .arg("-o")
        .arg(path)
        .current_dir(tgt_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status = child.wait()?;
    if status.success() {
        // zstd is single file, we must make it executable.
        let tgt_path = tgt_dir.join(path);
        rood::sys::file::make_executable(&tgt_path)?;
        tracing::debug!("made executable: {:?}", tgt_path);
        Ok(())
    } else {
        let code = status.code().unwrap_or(1);
        // Print the stdout

        ensure!(status.success(), "Error with zstd - status: {}", code);
        Ok(())
    }
}

fn unzstd(zstd_file: &Path, tgt_dir: &Path) -> Result<()> {
    if cfg!(unix) {
        unzstd_unix(zstd_file, tgt_dir)
    } else {
        unimplemented!()
    }
}

pub fn extract(path: &Path, tgt_dir: &Path, compression: CompressionType) -> Result<()> {
    match compression {
        CompressionType::Zip => unzip(path, tgt_dir),
        CompressionType::Tarball => untar(path, tgt_dir),
        CompressionType::Zstd => unzstd(path, tgt_dir),
    }
}

pub fn get_compression(ext: &str) -> Option<CompressionType> {
    match ext {
        "zip" => Some(CompressionType::Zip),
        "txz" | "tgz" | "gz" | "xz" => Some(CompressionType::Tarball),
        "zst" => Some(CompressionType::Zstd),
        _ => None,
    }
}
