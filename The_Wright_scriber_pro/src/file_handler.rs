use std::path::Path;
use std::error::Error;
use std::fs;
use sha2::{Sha256, Digest};

/// Compute the SHA-256 checksum of the file at the given path.
pub fn compute_checksum<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Export a transcript's contents to the specified path.
/// This stub simply writes the raw text to the file. Support for multiple formats should
/// be added later (e.g. PDF, HTML, DOCX, etc.).
pub fn export_transcript<P: AsRef<Path>>(content: &str, path: P) -> Result<(), Box<dyn Error>> {
    fs::write(path, content)?;
    Ok(())
}
