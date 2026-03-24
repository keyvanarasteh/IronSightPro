//! Memory dumping utility.

use std::path::{Path, PathBuf};

/// Dump readable memory regions for a process into a secure file.
#[cfg(target_os = "linux")]
pub fn dump_memory(pid: u32, output_dir: &Path) -> anyhow::Result<(PathBuf, u32, u64)> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let dump_path = output_dir.join(format!("memdump_{pid}_{}.mdmp", now.replace(':', "-")));

    ensure_dump_dir(output_dir)?;

    let mut output = std::fs::File::create(&dump_path)
        .map_err(|e| anyhow::anyhow!("Cannot create dump file: {e}"))?;

    minidump_writer::minidump_writer::MinidumpWriterConfig::new(pid as i32, pid as i32)
        .write(&mut output)
        .map_err(|e| anyhow::anyhow!("Failed to write minidump: {:?}", e))?;

    let meta = output.metadata()
        .map_err(|e| anyhow::anyhow!("Failed to read dump metadata: {e}"))?;

    Ok((dump_path, 1, meta.len()))
}

#[cfg(not(target_os = "linux"))]
pub fn dump_memory(_pid: u32, _output_dir: &Path) -> anyhow::Result<(PathBuf, u32, u64)> {
    Err(anyhow::anyhow!("Memory dump is only supported on Linux"))
}

#[cfg(target_os = "linux")]
fn ensure_dump_dir(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::create_dir_all(path)?;
    let perms = std::fs::Permissions::from_mode(0o700);
    std::fs::set_permissions(path, perms)?;
    let meta = std::fs::metadata(path)?;
    if std::os::unix::fs::MetadataExt::uid(&meta) != 0 {
        tracing::warn!("Dump directory not owned by root: {}", path.display());
    }
    Ok(())
}
