use crate::utils::checksums::calculate_sha256;
use crate::utils::file_operations::FileInfo;
use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use chrono::{DateTime, Utc};
use std::fs;
use std::io::Write;

pub struct BagItPackage {
    pub bag_root: Utf8PathBuf,
    pub data_dir: Utf8PathBuf,
    pub manifest_path: Utf8PathBuf,
    pub bag_info_path: Utf8PathBuf,
    pub bagit_txt_path: Utf8PathBuf,
}

pub struct BagInfo {
    pub source_organization: Option<String>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub external_description: String,
    pub internal_sender_identifier: String,
    pub internal_sender_description: Option<String>,
    pub bagging_date: DateTime<Utc>,
    pub bag_size: String,
    pub payload_oxum: String,
}

impl BagItPackage {
    /// Create a new BagIt package structure
    pub fn new(bag_root: Utf8PathBuf) -> Result<Self> {
        let data_dir = bag_root.join("data");
        let manifest_path = bag_root.join("manifest-sha256.txt");
        let bag_info_path = bag_root.join("bag-info.txt");
        let bagit_txt_path = bag_root.join("bagit.txt");

        // Create the bag directory structure
        fs::create_dir_all(&bag_root)?;
        fs::create_dir_all(&data_dir)?;

        Ok(BagItPackage {
            bag_root,
            data_dir,
            manifest_path,
            bag_info_path,
            bagit_txt_path,
        })
    }

    /// Create the bagit.txt declaration file
    pub fn create_bagit_declaration(&self) -> Result<()> {
        let mut file = fs::File::create(&self.bagit_txt_path)?;
        writeln!(file, "BagIt-Version: 1.0")?;
        writeln!(file, "Tag-File-Character-Encoding: UTF-8")?;
        Ok(())
    }

    /// Copy files to the bag's data directory
    pub fn add_files(&self, files: &[FileInfo], source_root: &Utf8Path) -> Result<()> {
        for file_info in files {
            if file_info.is_directory {
                // Create directory structure
                let relative_path = file_info.path.strip_prefix(source_root)?;
                let dest_path = self.data_dir.join(relative_path);
                fs::create_dir_all(&dest_path)?;
            } else {
                // Copy file
                let relative_path = file_info.path.strip_prefix(source_root)?;
                let dest_path = self.data_dir.join(relative_path);

                // Create parent directory if needed
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::copy(&file_info.path, &dest_path)?;
            }
        }
        Ok(())
    }

    /// Generate the manifest file with SHA-256 checksums
    pub async fn create_manifest(&self) -> Result<()> {
        let mut manifest_entries = Vec::new();

        // Walk through all files in the data directory
        for entry in ::walkdir::WalkDir::new(&self.data_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let file_path = Utf8Path::from_path(entry.path())
                    .ok_or_else(|| anyhow::anyhow!("Non-UTF8 path encountered"))?;

                // Calculate SHA-256 checksum
                let checksum = calculate_sha256(file_path).await?;

                // Get relative path from bag root (include "data/" prefix)
                let relative_path = file_path.strip_prefix(&self.bag_root)?;

                manifest_entries.push(format!("{}  {}", checksum, relative_path));
            }
        }

        // Sort entries for consistent output
        manifest_entries.sort();

        // Write manifest file
        let mut file = fs::File::create(&self.manifest_path)?;
        for entry in manifest_entries {
            writeln!(file, "{}", entry)?;
        }

        Ok(())
    }

    /// Create the bag-info.txt metadata file
    pub fn create_bag_info(&self, bag_info: &BagInfo) -> Result<()> {
        let mut file = fs::File::create(&self.bag_info_path)?;

        // Write metadata fields
        writeln!(file, "Bag-Software-Agent: Creative Work Preservation Toolkit v0.1.0")?;
        writeln!(file, "Bagging-Date: {}", bag_info.bagging_date.format("%Y-%m-%d"))?;
        writeln!(file, "Payload-Oxum: {}", bag_info.payload_oxum)?;
        writeln!(file, "Bag-Size: {}", bag_info.bag_size)?;

        if let Some(source_org) = &bag_info.source_organization {
            writeln!(file, "Source-Organization: {}", source_org)?;
        }

        if let Some(contact_name) = &bag_info.contact_name {
            writeln!(file, "Contact-Name: {}", contact_name)?;
        }

        if let Some(contact_email) = &bag_info.contact_email {
            writeln!(file, "Contact-Email: {}", contact_email)?;
        }

        writeln!(file, "External-Description: {}", bag_info.external_description)?;
        writeln!(file, "Internal-Sender-Identifier: {}", bag_info.internal_sender_identifier)?;

        if let Some(description) = &bag_info.internal_sender_description {
            writeln!(file, "Internal-Sender-Description: {}", description)?;
        }

        Ok(())
    }

    /// Calculate payload oxum (byte count and file count)
    pub fn calculate_payload_oxum(&self) -> Result<(u64, usize)> {
        let mut total_bytes = 0u64;
        let mut file_count = 0usize;

        for entry in ::walkdir::WalkDir::new(&self.data_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let metadata = entry.metadata()?;
                total_bytes += metadata.len();
                file_count += 1;
            }
        }

        Ok((total_bytes, file_count))
    }

    /// Format bag size in human-readable format
    pub fn format_bag_size(&self) -> Result<String> {
        let total_size = self.calculate_bag_directory_size()?;
        format_bytes(total_size)
    }

    /// Calculate total size of the entire bag directory
    pub fn calculate_bag_directory_size(&self) -> Result<u64> {
        let mut total_size = 0u64;

        for entry in ::walkdir::WalkDir::new(&self.bag_root) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let metadata = entry.metadata()?;
                total_size += metadata.len();
            }
        }

        Ok(total_size)
    }

    /// Validate the bag structure and checksums
    pub async fn validate(&self) -> Result<Vec<String>> {
        let mut issues = Vec::new();

        // Check required files exist
        if !self.bagit_txt_path.exists() {
            issues.push("Missing bagit.txt file".to_string());
        }

        if !self.manifest_path.exists() {
            issues.push("Missing manifest-sha256.txt file".to_string());
        }

        if !self.data_dir.exists() {
            issues.push("Missing data directory".to_string());
        }

        // Validate bagit.txt content
        if self.bagit_txt_path.exists() {
            let content = fs::read_to_string(&self.bagit_txt_path)?;
            if !content.contains("BagIt-Version: 1.0") {
                issues.push("Invalid BagIt version in bagit.txt".to_string());
            }
            if !content.contains("Tag-File-Character-Encoding: UTF-8") {
                issues.push("Invalid character encoding declaration in bagit.txt".to_string());
            }
        }

        // Validate manifest checksums
        if self.manifest_path.exists() {
            let manifest_content = fs::read_to_string(&self.manifest_path)?;
            for line in manifest_content.lines() {
                if line.trim().is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.splitn(2, "  ").collect();
                if parts.len() != 2 {
                    issues.push(format!("Invalid manifest line format: {}", line));
                    continue;
                }

                let expected_checksum = parts[0];
                let file_path = self.bag_root.join(parts[1]);

                if !file_path.exists() {
                    issues.push(format!("File missing: {}", parts[1]));
                    continue;
                }

                let actual_checksum = calculate_sha256(&file_path).await?;
                if actual_checksum != expected_checksum {
                    issues.push(format!("Checksum mismatch for file: {}", parts[1]));
                }
            }
        }

        Ok(issues)
    }
}

/// Format bytes in human-readable format
fn format_bytes(bytes: u64) -> Result<String> {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return Ok("0 B".to_string());
    }

    let bytes_f = bytes as f64;
    let unit_index = (bytes_f.log(THRESHOLD).floor() as usize).min(UNITS.len() - 1);
    let value = bytes_f / THRESHOLD.powi(unit_index as i32);

    if unit_index == 0 {
        Ok(format!("{} {}", bytes, UNITS[unit_index]))
    } else {
        Ok(format!("{:.1} {}", value, UNITS[unit_index]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0).unwrap(), "0 B");
        assert_eq!(format_bytes(512).unwrap(), "512 B");
        assert_eq!(format_bytes(1024).unwrap(), "1.0 KB");
        assert_eq!(format_bytes(1536).unwrap(), "1.5 KB");
        assert_eq!(format_bytes(1048576).unwrap(), "1.0 MB");
        assert_eq!(format_bytes(1073741824).unwrap(), "1.0 GB");
    }

    #[tokio::test]
    async fn test_bagit_creation() {
        let temp_dir = TempDir::new().unwrap();
        let bag_path = Utf8Path::from_path(temp_dir.path().join("test-bag")).unwrap();

        let bag = BagItPackage::new(bag_path.to_path_buf()).unwrap();

        // Test bagit.txt creation
        bag.create_bagit_declaration().unwrap();
        assert!(bag.bagit_txt_path.exists());

        let content = fs::read_to_string(&bag.bagit_txt_path).unwrap();
        assert!(content.contains("BagIt-Version: 1.0"));
        assert!(content.contains("Tag-File-Character-Encoding: UTF-8"));
    }
}