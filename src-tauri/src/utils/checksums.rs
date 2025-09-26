use anyhow::Result;
use blake3::Hasher as Blake3Hasher;
use md5::{Digest as Md5Digest, Md5};
use sha2::{Digest as Sha2Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub struct FileChecksums {
    pub blake3: String,
    pub sha256: String,
    pub md5: String,
}

/// Calculate multiple checksums for a file efficiently
pub async fn calculate_file_checksums<P: AsRef<Path>>(file_path: P) -> Result<FileChecksums> {
    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);

    let mut blake3_hasher = Blake3Hasher::new();
    let mut sha256_hasher = Sha256::new();
    let mut md5_hasher = Md5::new();

    let mut buffer = [0; 8192]; // 8KB buffer for efficient reading

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let chunk = &buffer[..bytes_read];
        blake3_hasher.update(chunk);
        sha256_hasher.update(chunk);
        md5_hasher.update(chunk);
    }

    Ok(FileChecksums {
        blake3: blake3_hasher.finalize().to_hex().to_string(),
        sha256: format!("{:x}", sha256_hasher.finalize()),
        md5: format!("{:x}", md5_hasher.finalize()),
    })
}

/// Calculate SHA-256 checksum only (for BagIt compatibility)
pub async fn calculate_sha256<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    let mut buffer = [0; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Calculate MD5 checksum (for compatibility with older systems)
pub async fn calculate_md5<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Md5::new();

    let mut buffer = [0; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Calculate BLAKE3 checksum (fastest option)
pub async fn calculate_blake3<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Blake3Hasher::new();

    let mut buffer = [0; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_calculate_checksums() {
        // Create a temporary file with known content
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "Hello, World!").unwrap();
        temp_file.flush().unwrap();

        let checksums = calculate_file_checksums(temp_file.path()).await.unwrap();

        // Verify we get some checksum values (actual values depend on content)
        assert!(!checksums.blake3.is_empty());
        assert!(!checksums.sha256.is_empty());
        assert!(!checksums.md5.is_empty());

        // Checksums should be hexadecimal
        assert!(checksums.blake3.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(checksums.sha256.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(checksums.md5.chars().all(|c| c.is_ascii_hexdigit()));
    }
}