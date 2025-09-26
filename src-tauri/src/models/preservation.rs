use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchivedProject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub archived_at: DateTime<Utc>,
    pub bagit_package_id: Option<String>,
    pub file_count: i32,
    pub total_size: i64,
    pub is_quarantined: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BagitPackage {
    pub id: String,
    pub archived_project_id: String,
    pub bag_path: String,
    pub manifest_sha256: String,
    pub bag_size: i64,
    pub payload_file_count: i32,
    pub is_valid: bool,
    pub validated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuarantineEntry {
    pub id: String,
    pub archived_project_id: String,
    pub quarantined_at: DateTime<Utc>,
    pub original_bag_path: String,
    pub scheduled_for_deletion_at: Option<DateTime<Utc>>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultSnapshot {
    pub id: String,
    pub snapshot_at: DateTime<Utc>,
    pub user_layer_checksum: String,
    pub bags_layer_checksum: String,
    pub anomalies_detected: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub mime_type: Option<String>,
    pub checksum_sha256: Option<String>,
    pub checksum_md5: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveRequest {
    pub name: String,
    pub description: Option<String>,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveResult {
    pub success: bool,
    pub project_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BagResult {
    pub success: bool,
    pub bag_path: Option<String>,
    pub validation_results: Option<Vec<ValidationResult>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub result_type: String, // 'error', 'warning', 'info'
    pub message: String,
    pub file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrityReport {
    pub is_healthy: bool,
    pub issues: Vec<IntegrityIssue>,
    pub last_scan_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrityIssue {
    pub issue_type: String, // 'missing_file', 'corrupted_file', 'external_modification'
    pub severity: String,   // 'critical', 'warning', 'info'
    pub message: String,
    pub affected_files: Vec<String>,
}

impl ArchivedProject {
    pub fn new(name: String, description: Option<String>, file_count: i32, total_size: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            archived_at: now,
            bagit_package_id: None,
            file_count,
            total_size,
            is_quarantined: false,
            created_at: now,
            updated_at: now,
        }
    }
}

impl BagitPackage {
    pub fn new(archived_project_id: String, bag_path: String, manifest_sha256: String, bag_size: i64, payload_file_count: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            archived_project_id,
            bag_path,
            manifest_sha256,
            bag_size,
            payload_file_count,
            is_valid: true,
            validated_at: Some(Utc::now()),
            created_at: Utc::now(),
        }
    }
}