use crate::models::preservation::*;
use crate::utils::{
    bagit::{BagInfo, BagItPackage},
    file_operations::{analyze_path, find_common_root, sanitize_directory_name, validate_paths},
};
// use crate::database::connection::queries;
use anyhow::Result;
use camino::Utf8Path;
use chrono::Utc;
use std::path::Path;
use tauri::{AppHandle, State};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum PreservationError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid project ID: {0}")]
    InvalidProjectId(String),
    #[error("BagIt creation failed: {0}")]
    BagItCreationFailed(String),
}

impl serde::Serialize for PreservationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// Archive a complete project (folder or multiple files)
#[tauri::command]
pub async fn archive_project(
    app_handle: AppHandle,
    request: ArchiveRequest,
) -> Result<ArchiveResult, PreservationError> {
    println!("Archiving project: {}", request.name);
    println!("Files to archive: {:?}", request.files);

    // 1. Validate files exist
    let validated_files = validate_paths(&request.files)
        .map_err(|e| PreservationError::FileNotFound(e.to_string()))?;

    // 2. Calculate total size and file count
    let mut total_size = 0u64;
    let mut file_count = 0usize;

    for file_info in &validated_files {
        if file_info.is_directory {
            // For directories, analyze recursively
            let stats = analyze_path(file_info.path.as_str())
                .map_err(|e| PreservationError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            total_size += stats.total_size;
            file_count += stats.file_count;
        } else {
            total_size += file_info.size;
            file_count += 1;
        }
    }

    // 3. Create project record
    let project = ArchivedProject::new(
        request.name.clone(),
        request.description,
        file_count as i32,
        total_size as i64,
    );

    // 4. Get database connection and insert project
    let db = app_handle
        .db("preservation.db")
        .map_err(|e| PreservationError::Database(e.to_string()))?;

    queries::insert_archived_project(&db, &project)
        .await
        .map_err(|e| PreservationError::Database(e.to_string()))?;

    // 5. Log the archival event
    let event_payload = serde_json::json!({
        "project_name": request.name,
        "file_count": file_count,
        "total_size": total_size,
        "files": request.files
    });

    queries::insert_event(
        &db,
        "ProjectArchived",
        &project.id,
        &event_payload.to_string(),
    )
    .await
    .map_err(|e| PreservationError::Database(e.to_string()))?;

    println!("Project archived successfully: {}", project.id);

    Ok(ArchiveResult {
        success: true,
        project_id: Some(project.id),
        error: None,
    })
}

/// Create BagIt package from archived project
#[tauri::command]
pub async fn create_bagit_package(
    app_handle: AppHandle,
    project_id: String,
) -> Result<BagResult, PreservationError> {
    println!("Creating BagIt package for project: {}", project_id);

    // 1. Get database connection and validate project exists
    let db = app_handle
        .db("preservation.db")
        .map_err(|e| PreservationError::Database(e.to_string()))?;

    // Get project details from database
    let projects = queries::get_all_archived_projects(&db)
        .await
        .map_err(|e| PreservationError::Database(e.to_string()))?;

    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| PreservationError::InvalidProjectId(project_id.clone()))?;

    // 2. Create BagIt directory structure
    // For now, create in a temporary location (in production, this would be the vault directory)
    let bag_name = format!("{}-{}", sanitize_directory_name(&project.name), &project.id[..8]);
    let bag_root = Utf8Path::new("/tmp/cwpt-bags").join(&bag_name);

    let bag = BagItPackage::new(bag_root.clone())
        .map_err(|e| PreservationError::BagItCreationFailed(e.to_string()))?;

    // 3. Create bagit.txt declaration
    bag.create_bagit_declaration()
        .map_err(|e| PreservationError::BagItCreationFailed(e.to_string()))?;

    // 4. Create manifest (for now, just an empty one - files would be copied first in production)
    bag.create_manifest()
        .await
        .map_err(|e| PreservationError::BagItCreationFailed(e.to_string()))?;

    // 5. Create bag-info.txt with metadata
    let (payload_bytes, payload_files) = bag.calculate_payload_oxum()
        .map_err(|e| PreservationError::BagItCreationFailed(e.to_string()))?;

    let bag_info = BagInfo {
        source_organization: Some("Creative Work Preservation Toolkit".to_string()),
        contact_name: None,
        contact_email: None,
        external_description: project.description.clone().unwrap_or_else(|| format!("Archived project: {}", project.name)),
        internal_sender_identifier: project.id.clone(),
        internal_sender_description: Some(format!("Creative work archived via CWPT on {}", project.archived_at.format("%Y-%m-%d"))),
        bagging_date: Utc::now(),
        bag_size: bag.format_bag_size()
            .map_err(|e| PreservationError::BagItCreationFailed(e.to_string()))?,
        payload_oxum: format!("{}.{}", payload_bytes, payload_files),
    };

    bag.create_bag_info(&bag_info)
        .map_err(|e| PreservationError::BagItCreationFailed(e.to_string()))?;

    // 6. Validate the created bag
    let validation_issues = bag.validate()
        .await
        .map_err(|e| PreservationError::BagItCreationFailed(e.to_string()))?;

    let mut validation_results = vec![];
    for issue in validation_issues {
        validation_results.push(ValidationResult {
            result_type: "error".to_string(),
            message: issue,
            file: None,
        });
    }

    // Add success message if no issues
    if validation_results.is_empty() {
        validation_results.push(ValidationResult {
            result_type: "info".to_string(),
            message: "BagIt package created and validated successfully".to_string(),
            file: None,
        });
    }

    // 7. Log the BagIt creation event
    let event_payload = serde_json::json!({
        "project_id": project_id,
        "bag_path": bag_root.as_str(),
        "validation_issues": validation_results.len() - 1  // Subtract the success message
    });

    queries::insert_event(
        &db,
        "BagitPackageCreated",
        &project_id,
        &event_payload.to_string(),
    )
    .await
    .map_err(|e| PreservationError::Database(e.to_string()))?;

    println!("BagIt package created successfully at: {}", bag_root);

    Ok(BagResult {
        success: validation_results.iter().all(|r| r.result_type != "error"),
        bag_path: Some(bag_root.to_string()),
        validation_results: Some(validation_results),
        error: None,
    })
}

/// Get all archived projects
#[tauri::command]
pub async fn get_archived_projects(
    app_handle: AppHandle,
) -> Result<Vec<ArchivedProject>, PreservationError> {
    println!("Getting all archived projects");

    let db = app_handle
        .db("preservation.db")
        .map_err(|e| PreservationError::Database(e.to_string()))?;

    let projects = queries::get_all_archived_projects(&db)
        .await
        .map_err(|e| PreservationError::Database(e.to_string()))?;

    println!("Retrieved {} archived projects", projects.len());
    Ok(projects)
}

/// Soft delete (quarantine) an archived project
#[tauri::command]
pub async fn quarantine_project(
    project_id: String,
    reason: String,
) -> Result<(), PreservationError> {
    println!("Quarantining project: {} (reason: {})", project_id, reason);

    // TODO: Implement quarantine logic
    // 1. Validate project exists
    // 2. Move BagIt package to quarantine directory
    // 3. Update project record (is_quarantined = true)
    // 4. Create quarantine entry with scheduled deletion date
    // 5. Log event

    Ok(())
}

/// Restore from quarantine
#[tauri::command]
pub async fn restore_project(project_id: String) -> Result<(), PreservationError> {
    println!("Restoring project from quarantine: {}", project_id);

    // TODO: Implement restore logic
    // 1. Validate project is quarantined
    // 2. Move BagIt package back from quarantine
    // 3. Update project record (is_quarantined = false)
    // 4. Remove quarantine entry
    // 5. Log event

    Ok(())
}

/// Scan vault integrity
#[tauri::command]
pub async fn scan_vault_integrity() -> Result<IntegrityReport, PreservationError> {
    println!("Scanning vault integrity");

    // TODO: Implement integrity scanning
    // 1. Generate checksum for user layer directory structure
    // 2. Generate checksum for bags layer directory structure
    // 3. Compare against last known good state
    // 4. Identify any discrepancies or missing files
    // 5. Create integrity report with findings
    // 6. Store snapshot in database

    // For now, return healthy status
    Ok(IntegrityReport {
        is_healthy: true,
        issues: vec![],
        last_scan_at: Utc::now(),
    })
}

// Helper functions (to be implemented)
async fn validate_files_exist(files: &[String]) -> Result<()> {
    for file_path in files {
        if !Path::new(file_path).exists() {
            return Err(anyhow::anyhow!("File not found: {}", file_path));
        }
    }
    Ok(())
}

async fn calculate_project_stats(files: &[String]) -> Result<(i32, i64)> {
    let mut file_count = 0;
    let mut total_size = 0;

    for file_path in files {
        let path = Path::new(file_path);
        if path.is_file() {
            file_count += 1;
            total_size += std::fs::metadata(path)?.len() as i64;
        } else if path.is_dir() {
            // TODO: Recursively count files and calculate total size for directories
            file_count += 1; // Placeholder
        }
    }

    Ok((file_count, total_size))
}