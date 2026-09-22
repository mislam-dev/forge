use crate::shared::error::AppError;
use bytes::Bytes;
use std::path::Path;
use tar::Builder as TarBuilder;

pub fn build_tar_context(source_path: &Path, ignore: &[&str]) -> Result<Bytes, AppError> {
    let mut archive = TarBuilder::new(Vec::new());

    for entry in std::fs::read_dir(source_path).map_err(|e| {
        AppError::InternalServerError(format!("Failed to read source directory: {}", e))
    })? {
        let entry = entry.map_err(|e| {
            AppError::InternalServerError(format!("Failed to read directory entry: {}", e))
        })?;

        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if ignore.iter().any(|i| name_str.starts_with(i)) {
            continue;
        }

        let path = entry.path();
        if path.is_dir() {
            archive.append_dir_all(&name, &path).map_err(|e| {
                AppError::InternalServerError(format!("Failed to add directory to tar: {}", e))
            })?;
        } else {
            archive.append_path_with_name(&path, &name).map_err(|e| {
                AppError::InternalServerError(format!("Failed to add file to tar: {}", e))
            })?;
        }
    }

    let build_context_tar = archive.into_inner().map_err(|e| {
        AppError::InternalServerError(format!("Failed to finalize tar archive: {}", e))
    })?;

    Ok(Bytes::from(build_context_tar))
}
