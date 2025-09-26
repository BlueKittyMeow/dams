use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use ::walkdir::WalkDir;

pub struct DirectoryStats {
    pub file_count: usize,
    pub total_size: u64,
    pub files: Vec<FileInfo>,
}

pub struct FileInfo {
    pub path: Utf8PathBuf,
    pub name: String,
    pub size: u64,
    pub is_directory: bool,
}

/// Analyze a directory or file to get comprehensive statistics
pub fn analyze_path(path: &str) -> Result<DirectoryStats> {
    let utf8_path = Utf8Path::new(path);

    if !utf8_path.exists() {
        return Err(anyhow::anyhow!("Path does not exist: {}", path));
    }

    let mut files = Vec::new();
    let mut total_size = 0u64;
    let mut file_count = 0usize;

    if utf8_path.is_file() {
        // Single file
        let metadata = fs::metadata(utf8_path)?;
        let size = metadata.len();

        files.push(FileInfo {
            path: utf8_path.to_path_buf(),
            name: utf8_path.file_name().unwrap_or("Unknown").to_string(),
            size,
            is_directory: false,
        });

        total_size += size;
        file_count = 1;
    } else if utf8_path.is_dir() {
        // Directory - walk recursively
        for entry in WalkDir::new(utf8_path) {
            let entry = entry?;
            let entry_path = Utf8Path::from_path(entry.path())
                .ok_or_else(|| anyhow::anyhow!("Non-UTF8 path encountered"))?;

            if entry.file_type().is_file() {
                let metadata = entry.metadata()?;
                let size = metadata.len();

                files.push(FileInfo {
                    path: entry_path.to_path_buf(),
                    name: entry_path.file_name().unwrap_or("Unknown").to_string(),
                    size,
                    is_directory: false,
                });

                total_size += size;
                file_count += 1;
            } else if entry.file_type().is_dir() && entry.depth() > 0 {
                // Include subdirectories in the list (but not the root)
                files.push(FileInfo {
                    path: entry_path.to_path_buf(),
                    name: entry_path.file_name().unwrap_or("Unknown").to_string(),
                    size: 0,
                    is_directory: true,
                });
            }
        }
    }

    Ok(DirectoryStats {
        file_count,
        total_size,
        files,
    })
}

/// Copy files to a destination directory, maintaining relative structure
pub fn copy_files_to_destination(
    files: &[FileInfo],
    source_root: &Utf8Path,
    destination: &Utf8Path,
) -> Result<()> {
    // Create destination directory if it doesn't exist
    fs::create_dir_all(destination)?;

    for file_info in files {
        if file_info.is_directory {
            // Create directory structure
            let relative_path = file_info.path.strip_prefix(source_root)?;
            let dest_path = destination.join(relative_path);
            fs::create_dir_all(&dest_path)?;
        } else {
            // Copy file
            let relative_path = file_info.path.strip_prefix(source_root)?;
            let dest_path = destination.join(relative_path);

            // Create parent directory if needed
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(&file_info.path, &dest_path)?;
        }
    }

    Ok(())
}

/// Get the common root directory for a list of file paths
pub fn find_common_root(paths: &[String]) -> Result<Utf8PathBuf> {
    if paths.is_empty() {
        return Err(anyhow::anyhow!("No paths provided"));
    }

    if paths.len() == 1 {
        let path = Utf8Path::new(&paths[0]);
        return Ok(if path.is_file() {
            path.parent().unwrap_or(path).to_path_buf()
        } else {
            path.to_path_buf()
        });
    }

    // Find common prefix of all paths
    let first_path = Utf8Path::new(&paths[0]);
    let mut common_root = first_path.parent().unwrap_or(first_path);

    for path_str in &paths[1..] {
        let path = Utf8Path::new(path_str);
        let path_parent = if path.is_file() {
            path.parent().unwrap_or(path)
        } else {
            path
        };

        // Find common ancestor
        while !path_parent.starts_with(common_root) {
            if let Some(parent) = common_root.parent() {
                common_root = parent;
            } else {
                return Err(anyhow::anyhow!("No common root found"));
            }
        }
    }

    Ok(common_root.to_path_buf())
}

/// Validate that all provided paths exist and are accessible
pub fn validate_paths(paths: &[String]) -> Result<Vec<FileInfo>> {
    let mut validated_files = Vec::new();

    for path_str in paths {
        let path = Utf8Path::new(path_str);

        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {}", path_str));
        }

        let metadata = fs::metadata(path)?;

        validated_files.push(FileInfo {
            path: path.to_path_buf(),
            name: path.file_name().unwrap_or("Unknown").to_string(),
            size: metadata.len(),
            is_directory: metadata.is_dir(),
        });
    }

    Ok(validated_files)
}

/// Generate a safe directory name from a project name
pub fn sanitize_directory_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            // Replace problematic characters with safe alternatives
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            // Keep alphanumeric, spaces, hyphens, underscores, periods
            c if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' || c == '.' => c,
            // Replace other characters with underscores
            _ => '_',
        })
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_analyze_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, World!").unwrap();

        let stats = analyze_path(file_path.to_str().unwrap()).unwrap();
        assert_eq!(stats.file_count, 1);
        assert!(stats.total_size > 0);
        assert_eq!(stats.files.len(), 1);
        assert!(!stats.files[0].is_directory);
    }

    #[test]
    fn test_sanitize_directory_name() {
        assert_eq!(sanitize_directory_name("My Project"), "My Project");
        assert_eq!(sanitize_directory_name("Project/With\\Slashes"), "Project-With-Slashes");
        assert_eq!(sanitize_directory_name("Invalid:Name*?"), "Invalid-Name--");
        assert_eq!(sanitize_directory_name("Normal_Project-v1.0"), "Normal_Project-v1.0");
    }

    #[test]
    fn test_find_common_root_single_path() {
        let paths = vec!["/home/user/documents/file.txt".to_string()];
        let root = find_common_root(&paths).unwrap();
        assert_eq!(root.as_str(), "/home/user/documents");
    }
}