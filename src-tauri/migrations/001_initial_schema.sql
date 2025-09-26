-- Core table: Each record represents a complete archived project/stage
CREATE TABLE archived_projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    archived_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    bagit_package_id TEXT,
    file_count INTEGER NOT NULL,
    total_size INTEGER NOT NULL,
    is_quarantined BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (bagit_package_id) REFERENCES bagit_packages(id)
);

-- BagIt packages (one per archived project)
CREATE TABLE bagit_packages (
    id TEXT PRIMARY KEY,
    archived_project_id TEXT NOT NULL,
    bag_path TEXT NOT NULL,
    manifest_sha256 TEXT NOT NULL,
    bag_size INTEGER NOT NULL,
    payload_file_count INTEGER NOT NULL,
    is_valid BOOLEAN DEFAULT TRUE,
    validated_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (archived_project_id) REFERENCES archived_projects(id),
    UNIQUE(bag_path)
);

-- Event sourcing for complete audit trail
CREATE TABLE events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,           -- ProjectArchived, BagCreated, ItemQuarantined, etc.
    aggregate_id TEXT NOT NULL,         -- ID of the archived project
    payload TEXT NOT NULL,              -- JSON event data
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Quarantine system for deletion protection
CREATE TABLE quarantine_entries (
    id TEXT PRIMARY KEY,
    archived_project_id TEXT NOT NULL,
    quarantined_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    original_bag_path TEXT NOT NULL,
    scheduled_for_deletion_at DATETIME, -- 30 days from quarantine
    reason TEXT,                        -- User or system reason
    FOREIGN KEY (archived_project_id) REFERENCES archived_projects(id)
);

-- Vault integrity tracking
CREATE TABLE vault_snapshots (
    id TEXT PRIMARY KEY,
    snapshot_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    user_layer_checksum TEXT NOT NULL, -- Hash of user-visible files
    bags_layer_checksum TEXT NOT NULL, -- Hash of preservation bags
    anomalies_detected TEXT,           -- JSON array of issues found
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Basic settings
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for performance
CREATE INDEX idx_archived_projects_created_at ON archived_projects(created_at);
CREATE INDEX idx_archived_projects_quarantined ON archived_projects(is_quarantined);
CREATE INDEX idx_bagit_packages_project_id ON bagit_packages(archived_project_id);
CREATE INDEX idx_events_aggregate_id ON events(aggregate_id);
CREATE INDEX idx_events_created_at ON events(created_at);
CREATE INDEX idx_quarantine_entries_project_id ON quarantine_entries(archived_project_id);
CREATE INDEX idx_vault_snapshots_created_at ON vault_snapshots(created_at);