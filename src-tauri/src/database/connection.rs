use anyhow::Result;
use tauri_plugin_sql::{Migration, MigrationKind};

/// Initialize database migrations for the preservation toolkit
pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_initial_schema",
            sql: include_str!("../../migrations/001_initial_schema.sql"),
            kind: MigrationKind::Up,
        },
        // Future migrations will be added here
    ]
}

/// Database utility functions for working with the preservation database
pub mod queries {
    use crate::models::preservation::ArchivedProject;
    use anyhow::Result;
    use tauri_plugin_sql::DatabaseInstance;

    /// Insert a new archived project into the database
    pub async fn insert_archived_project(
        db: &DatabaseInstance<tauri::Wry>,
        project: &ArchivedProject,
    ) -> Result<()> {
        let query = "
            INSERT INTO archived_projects (
                id, name, description, archived_at, bagit_package_id,
                file_count, total_size, is_quarantined, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";

        db.execute(
            query,
            &[
                &project.id,
                &project.name,
                &project.description,
                &project.archived_at.to_rfc3339(),
                &project.bagit_package_id,
                &project.file_count,
                &project.total_size,
                &project.is_quarantined,
                &project.created_at.to_rfc3339(),
                &project.updated_at.to_rfc3339(),
            ],
        )
        .await?;

        Ok(())
    }

    /// Get all archived projects from the database
    pub async fn get_all_archived_projects(
        db: &DatabaseInstance<tauri::Wry>,
    ) -> Result<Vec<ArchivedProject>> {
        let query = "
            SELECT id, name, description, archived_at, bagit_package_id,
                   file_count, total_size, is_quarantined, created_at, updated_at
            FROM archived_projects
            ORDER BY created_at DESC
        ";

        let rows = db.select(query).await?;

        let mut projects = Vec::new();
        for row in rows {
            let project = ArchivedProject {
                id: row.get("id").unwrap(),
                name: row.get("name").unwrap(),
                description: row.get("description").unwrap(),
                archived_at: chrono::DateTime::parse_from_rfc3339(
                    row.get::<String>("archived_at").unwrap().as_str()
                ).unwrap().with_timezone(&chrono::Utc),
                bagit_package_id: row.get("bagit_package_id").unwrap(),
                file_count: row.get("file_count").unwrap(),
                total_size: row.get("total_size").unwrap(),
                is_quarantined: row.get("is_quarantined").unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(
                    row.get::<String>("created_at").unwrap().as_str()
                ).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(
                    row.get::<String>("updated_at").unwrap().as_str()
                ).unwrap().with_timezone(&chrono::Utc),
            };
            projects.push(project);
        }

        Ok(projects)
    }

    /// Update a project's quarantine status
    pub async fn update_quarantine_status(
        db: &DatabaseInstance<tauri::Wry>,
        project_id: &str,
        is_quarantined: bool,
    ) -> Result<()> {
        let query = "
            UPDATE archived_projects
            SET is_quarantined = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";

        db.execute(query, &[&is_quarantined, &project_id]).await?;

        Ok(())
    }

    /// Insert an event record for audit trail
    pub async fn insert_event(
        db: &DatabaseInstance<tauri::Wry>,
        event_type: &str,
        aggregate_id: &str,
        payload: &str,
    ) -> Result<()> {
        let query = "
            INSERT INTO events (id, event_type, aggregate_id, payload, created_at)
            VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
        ";

        let event_id = uuid::Uuid::new_v4().to_string();
        db.execute(query, &[&event_id, &event_type, &aggregate_id, &payload]).await?;

        Ok(())
    }
}