# Creative Work Preservation Toolkit - Phase One Final Plan

## Executive Summary

**CORE USE CASE CLARIFIED**: System focuses on **archiving completed creative projects or distinct project stages**, not daily iterative work management. Each ingestion represents a complete project snapshot.

**ARCHITECTURE RESOLVED**: Tauri desktop application combining Rust backend capabilities with React frontend, eliminating all PWA/Node dependency contradictions.

**Duration**: 3 weeks
**Goal**: Thin vertical slice validating core archival workflow with robust deletion protection
**Deliverable**: Working desktop app that students can use to archive project stages and create BagIt packages

## Critical Architecture Decisions (Final)

### 1. Runtime Architecture: Tauri Desktop App

**Decision**: Tauri 2.0 desktop application with Rust backend + React frontend
**Rationale**:
- Resolves PWA/Node dependency contradictions identified by Codex
- Provides native filesystem access on all platforms
- Enables SQLite database with future encryption capability
- Maintains familiar web development experience
- Single binary distribution with no installation complexity

```
┌─────────────────────────────────────┐
│         Tauri Desktop App           │
├─────────────────────────────────────┤
│  Frontend (React + TypeScript)     │
│  ├── Project archival interface    │
│  ├── BagIt package management      │
│  ├── Deletion protection UI        │
│  └── Vault integrity dashboard     │
├─────────────────────────────────────┤
│  Backend (Rust)                    │
│  ├── File system operations        │
│  ├── SQLite database (rusqlite)    │
│  ├── BagIt package creation        │
│  ├── Vault integrity scanning      │
│  └── Quarantine management         │
├─────────────────────────────────────┤
│  OS Integration                    │
│  ├── Native drag-and-drop         │
│  ├── Cross-platform file access   │
│  └── System notifications         │
└─────────────────────────────────────┘
```

### 2. Core Use Case: Project Archival (Not Versioning)

**Key Insight from Gemini**: The system archives completed projects or distinct project stages, not daily iterative work.

**Implications**:
- Each "file" in our database represents a complete project archive
- No complex versioning within projects - each ingestion is self-contained
- Simplified database schema focused on project snapshots
- User workflow: "I finished this stage of my thesis project, let me archive it"

**User Scenarios**:
1. **Thesis Stage**: "I completed my literature review draft" → Archive entire folder with docs, images, references
2. **Design Project**: "Final logo design approved" → Archive working files, exports, presentation materials
3. **Art Project**: "Painting series completed" → Archive high-res photos, process documentation, artist statement

### 3. Database Schema (Simplified for Archival)

```sql
-- Core table: Each record represents a complete archived project/stage
CREATE TABLE archived_projects (
    id TEXT PRIMARY KEY,                 -- UUID for the project archive
    name TEXT NOT NULL,                  -- User-provided project name
    description TEXT,                    -- Optional project description
    archived_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    bagit_package_id TEXT,              -- Links to the BagIt package
    file_count INTEGER NOT NULL,        -- Number of files in this archive
    total_size INTEGER NOT NULL,        -- Total size in bytes
    is_quarantined BOOLEAN DEFAULT FALSE, -- Soft-delete flag
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (bagit_package_id) REFERENCES bagit_packages(id)
);

-- BagIt packages (one per archived project)
CREATE TABLE bagit_packages (
    id TEXT PRIMARY KEY,
    archived_project_id TEXT NOT NULL,
    bag_path TEXT NOT NULL,             -- Path to bag directory
    manifest_sha256 TEXT NOT NULL,      -- Checksum of manifest file
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
```

### 4. Three-Layer File System (Archival-Focused)

```
/user-vault/
├── archived-projects/                 # User Layer - browsable archived projects
│   ├── thesis-lit-review-v1/         # Complete project archive (readable copy)
│   │   ├── literature-review.docx
│   │   ├── references/
│   │   └── notes/
│   ├── logo-design-final/            # Another archived project
│   │   ├── working-files/
│   │   ├── final-exports/
│   │   └── client-presentation.pdf
│   └── painting-series-complete/
├── .cwpt-preservation/               # Hidden preservation layer
│   ├── bags/                        # BagIt packages (authoritative copies)
│   │   ├── uuid-a1b2c3.../         # BagIt package for thesis project
│   │   │   ├── data/               # Original files
│   │   │   ├── manifest-sha256.txt
│   │   │   ├── bagit.txt
│   │   │   └── bag-info.txt
│   │   └── uuid-b2c3d4.../
│   └── quarantine/                  # Soft-deleted items
│       └── uuid-quarantined.../
├── .cwpt-system/                    # System files
│   ├── database.sqlite
│   ├── config.json
│   └── logs/
└── .cwpt-previews/                  # Generated previews/thumbnails
    ├── uuid-a1b2c3-thumb.jpg
    └── uuid-b2c3d4-thumb.jpg
```

### 5. Deletion Protection & Vault Integrity

**Quarantine System**:
1. User "deletes" archived project → moves to quarantine (not hard deleted)
2. BagIt package moved to `.cwpt-preservation/quarantine/`
3. User-layer copy removed from `archived-projects/`
4. Entry created in `quarantine_entries` with 30-day deletion schedule
5. User can restore within grace period

**Vault Integrity Scanning**:
1. Background job periodically scans vault structure
2. Generates checksums for user layer and preservation layer
3. Compares against last known good state in `vault_snapshots`
4. Detects external modifications (files deleted/moved outside app)
5. Alerts user and offers restoration options

**Cascade Protection**:
- External deletion of files in `archived-projects/` doesn't auto-delete bags
- Preservation layer remains intact even if user layer is corrupted
- System can rebuild user layer from preservation bags

## Drastically Reduced Phase 1 Scope

### Week 1: Tauri Foundation + Basic Archival

**Days 1-2**: Tauri Setup
- Initialize Tauri project with React frontend
- Basic window, drag-and-drop setup
- SQLite database connection

**Days 3-5**: Single Project Archive
- Drag folder/files onto app
- Create one BagIt package
- Store in database as archived project
- Display in simple project list

**Success Criteria**: Students can drag a project folder and see it archived

### Week 2: BagIt Compliance + Deletion Protection

**Days 1-2**: BagIt Standards
- Proper manifest generation (SHA256)
- bag-info.txt with metadata
- Validation against external tools

**Days 3-5**: Quarantine System
- Implement soft-delete workflow
- Move bags to quarantine directory
- Basic restore functionality

**Success Criteria**: Created bags validate with bagit-python, deleted items can be restored

### Week 3: Student Testing + Vault Integrity

**Days 1-2**: Vault Scanning
- Basic integrity checking
- Detect external modifications
- Simple alerting system

**Days 3-5**: Student Deployment
- Build cross-platform binaries
- Deploy to 3-5 art students
- Collect feedback on workflow

**Success Criteria**: Students successfully archive real projects and understand the value proposition

## Technology Stack (Final)

### Frontend
- **React 18** + TypeScript
- **TailwindCSS** for styling
- **Vite** for development

### Backend (Rust)
- **Tauri 2.0** for app framework
- **rusqlite** for database operations
- **tokio** for async operations
- **sha2** for checksums
- **walkdir** for filesystem scanning

### Core Rust Commands

```rust
// Archive a complete project (folder or multiple files)
#[tauri::command]
async fn archive_project(
    name: String,
    description: Option<String>,
    files: Vec<String>
) -> Result<ArchiveResult, String>;

// Create BagIt package from archived project
#[tauri::command]
async fn create_bagit_package(project_id: String) -> Result<BagResult, String>;

// Soft delete (quarantine) an archived project
#[tauri::command]
async fn quarantine_project(project_id: String, reason: String) -> Result<(), String>;

// Restore from quarantine
#[tauri::command]
async fn restore_project(project_id: String) -> Result<(), String>;

// Scan vault integrity
#[tauri::command]
async fn scan_vault_integrity() -> Result<IntegrityReport, String>;

// Get all archived projects
#[tauri::command]
async fn get_archived_projects() -> Result<Vec<ArchivedProject>, String>;
```

## User Workflow (Final)

### Primary Workflow: Archive Completed Project
1. **Student completes a project stage** (thesis chapter, design iteration, art series)
2. **Opens CWPT app**
3. **Drags project folder** (or multiple files) onto app window
4. **Fills in project name and description** ("Thesis Chapter 2 - Literature Review")
5. **Clicks "Archive Project"**
6. **App creates BagIt package** with progress indicator
7. **Project appears in archived list** with thumbnail/preview
8. **Student can export bag** for submission or backup

### Deletion Protection Workflow
1. **Student "deletes" archived project** from list
2. **App moves to quarantine** (30-day grace period)
3. **Student gets confirmation** with restore option
4. **If not restored in 30 days** → permanent deletion
5. **Student can view quarantined items** and restore at any time

### Integrity Protection Workflow
1. **Background scan detects issue** (files moved/deleted externally)
2. **App shows notification** "Vault integrity issue detected"
3. **User views integrity report** showing what changed
4. **Options**: Restore from bags, ignore, or re-archive

## Success Criteria (Final)

### Week 1 Success
- ✅ Students can drag project folders and create archives
- ✅ Basic SQLite database stores project metadata
- ✅ Simple UI shows archived projects list

### Week 2 Success
- ✅ Generated BagIt packages validate with external tools
- ✅ Quarantine system protects against accidental deletion
- ✅ Students can restore quarantined projects

### Week 3 Success
- ✅ 3+ students successfully archive real projects
- ✅ Basic vault integrity scanning detects external changes
- ✅ Clear user feedback on workflow and priorities
- ✅ Proven foundation for Phase 2 development

## Deferred to Later Phases

### Phase 2 (4-6 weeks)
- Enhanced metadata collection
- Project relationships and collections
- Advanced BagIt features
- Improved UI/UX based on student feedback

### Phase 3 (Future)
- Plugin architecture for extensibility
- Advanced integrity checking
- Cloud storage integration preparation

### Phase 4+ (Much Later)
- Encryption and security features
- Cloud synchronization
- Collaboration features
- Mobile/tablet support

## Risk Mitigation (Final)

### Resolved Risks
- ✅ **Runtime Architecture**: Tauri eliminates PWA/Node contradictions
- ✅ **Cross-platform**: Native Rust APIs work consistently everywhere
- ✅ **File Access**: No dependency on File System Access API
- ✅ **Scope Creep**: Focused 3-week archival workflow only
- ✅ **User Validation**: Week 3 entirely dedicated to student feedback

### Remaining Risks
1. **Rust Learning Curve**: Mitigate with simple, well-documented patterns
2. **Student Adoption**: Address with intuitive UI and clear value proposition
3. **Data Loss**: Prevent with robust quarantine system and integrity checking

This final plan synthesizes all critical feedback while maintaining focus on the core value: helping creative students preserve their completed work with professional-grade standards and bulletproof protection against accidental loss.