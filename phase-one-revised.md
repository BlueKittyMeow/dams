# Creative Work Preservation Toolkit - Phase One Revised

## Executive Summary

**MAJOR ARCHITECTURE CHANGE**: Switching from PWA to Tauri desktop application based on critical feedback identifying runtime contradictions.

**Duration**: 3 weeks maximum
**Goal**: Thin vertical slice proving core concept with real user feedback
**Deliverable**: Working desktop app that students can use to ingest files and create basic BagIt packages

## Critical Architecture Decision: Tauri Desktop App

### Why Tauri Over PWA

**The Problem**: Our original plan mixed browser-only PWA vision with Node-native dependencies (better-sqlite3, Prisma, Fastify) that can't run in browsers. This created an impossible runtime architecture.

**The Solution**: Tauri provides the best of both worlds:
- **Rust backend**: Native filesystem access, SQLite, crypto operations
- **Web frontend**: Keep our React + TypeScript UI development experience
- **Single binary**: Cross-platform desktop app with no installation complexity
- **Local-first**: No servers, no cloud dependencies by default

### Tauri Architecture Benefits

```
┌─────────────────────────────────────┐
│           Tauri Desktop App         │
├─────────────────────────────────────┤
│  Frontend (Web Technologies)       │
│  ├── React + TypeScript            │
│  ├── TailwindCSS                   │
│  └── Modern UI/UX                  │
├─────────────────────────────────────┤
│  Backend (Rust)                    │
│  ├── File system operations        │
│  ├── SQLite database              │
│  ├── BagIt package creation       │
│  ├── Checksum calculation         │
│  └── Future: encryption, sync     │
├─────────────────────────────────────┤
│  OS Integration                    │
│  ├── Native drag-and-drop         │
│  ├── File system access           │
│  ├── Cross-platform packaging     │
│  └── System tray (optional)       │
└─────────────────────────────────────┘
```

## Drastically Reduced Phase 1 Scope

### Week 1: Foundation
**Goal**: Basic Tauri app with file ingestion

**Day 1-2**: Tauri Setup
- Initialize Tauri project with React frontend
- Configure cross-platform build targets
- Basic window and drag-and-drop setup

**Day 3-4**: File Ingestion
- Drag file onto app window
- Copy file to vault directory structure
- Simple SQLite database with one table: `files(id, name, path, ingested_at)`

**Day 5**: Basic UI
- File list view showing ingested files
- Simple drag-and-drop target area
- Basic error handling and feedback

### Week 2: BagIt Creation
**Goal**: Create valid BagIt packages manually

**Day 1-2**: BagIt Structure
- Create proper BagIt directory structure
- Generate manifest-sha256.txt with file checksums
- Create bag-info.txt with basic metadata

**Day 3-4**: Validation
- Implement BagIt validation (structure + checksums)
- Simple success/error reporting
- Export completed bags to user-selected directory

**Day 5**: Polish
- Basic progress indicators
- Error messages that make sense
- Prepare for student testing

### Week 3: Student Validation
**Goal**: Real user feedback on core concept

**Day 1-2**: Deploy to Students
- Build and distribute to 3-5 art students
- Simple onboarding: "Drag files here, click create bag"
- Document any installation/usage issues

**Day 3-4**: Feedback Collection
- Structured interviews about workflow
- Pain points in current process
- Feature priorities for next iteration

**Day 5**: Analysis and Next Phase Planning
- Synthesize feedback into concrete requirements
- Identify biggest usability blockers
- Plan Phase 2 based on actual user needs

## Simplified Technical Architecture

### Database Schema (Minimal)

```sql
-- Single table for MVP
CREATE TABLE files (
    id TEXT PRIMARY KEY,
    original_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    sha256_checksum TEXT,
    mime_type TEXT,
    ingested_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    bag_created_at DATETIME NULL,
    bag_path TEXT NULL
);

-- Settings for vault location, user preferences
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

### Rust Backend Commands

```rust
// Tauri commands callable from frontend
#[tauri::command]
async fn ingest_file(file_path: String) -> Result<FileRecord, String>;

#[tauri::command]
async fn create_bagit_package(file_id: String) -> Result<BagitResult, String>;

#[tauri::command]
async fn validate_bagit_package(bag_path: String) -> Result<ValidationResult, String>;

#[tauri::command]
async fn get_all_files() -> Result<Vec<FileRecord>, String>;

#[tauri::command]
async fn set_vault_directory(path: String) -> Result<(), String>;
```

### Directory Structure (Simplified)

```
/user-vault/
├── ingested-files/          # Original files (user can browse)
│   ├── image001.jpg
│   ├── design-sketch.psd
│   └── video-draft.mp4
├── bags/                    # Created BagIt packages
│   ├── bag-image001/
│   │   ├── data/
│   │   │   └── image001.jpg
│   │   ├── manifest-sha256.txt
│   │   ├── bagit.txt
│   │   └── bag-info.txt
│   └── bag-design-sketch/
└── .cwpt/                   # Hidden system files
    └── database.sqlite
```

## Technology Stack (Final)

### Frontend
- **React 18** with TypeScript
- **TailwindCSS** for styling
- **Vite** for development (Tauri handles bundling)
- **React Query** for state management (if needed)

### Backend (Rust)
- **Tauri 2.0** for desktop app framework
- **rusqlite** for SQLite database operations
- **tokio** for async operations
- **sha2** crate for checksum calculation
- **serde** for JSON serialization

### Build & Distribution
- **Tauri bundler** for cross-platform packaging
- **GitHub Actions** for automated builds
- **Direct binary distribution** (no app stores initially)

## User Workflow (MVP)

### Student Experience
1. **Download and run CWPT app** (single executable)
2. **Choose vault directory** (one-time setup)
3. **Drag files onto app** (they appear in file list)
4. **Select file(s) and click "Create Bag"**
5. **App shows progress, then "Bag created successfully"**
6. **Student can export bag or continue with more files**

### What Gets Validated
- Does the basic workflow make sense?
- Is drag-and-drop intuitive for creative students?
- Do they understand the concept of "preservation bags"?
- What metadata do they want to add during ingestion?
- How do they want to organize projects vs individual files?

## Implementation Priorities

### Must Have (Week 1-2)
- File drag-and-drop ingestion
- BagIt package creation with valid structure
- Basic validation (checksums match manifests)
- Simple file listing UI
- Error handling for common cases

### Should Have (If time allows)
- Basic metadata entry form
- Progress indicators for long operations
- Batch operations (select multiple files)
- Export completed bags to different locations

### Won't Have (Deferred)
- Event sourcing and complete audit trails
- Plugin architecture
- Background job queues
- Encryption of any kind
- Cloud synchronization
- Advanced metadata schemas
- Project organization features

## Risk Mitigation

### Critical Risks Addressed
1. **Runtime Architecture**: ✅ Tauri solves PWA/Node contradiction
2. **Cross-platform**: ✅ Tauri handles Windows/macOS/Linux consistently
3. **File System Access**: ✅ Native Rust APIs work everywhere
4. **Scope Creep**: ✅ Drastically reduced to essentials only

### New Risks
1. **Rust Learning Curve**: Mitigate with simple, well-documented Rust patterns
2. **Tauri Stability**: Use stable Tauri 2.0 release, avoid experimental features
3. **User Acceptance**: Address with early feedback loop in Week 3

## Success Criteria

### Technical Success
- ✅ Students can drag files and create valid BagIt packages
- ✅ Generated bags validate with external tools (bagit-python)
- ✅ App runs on Windows, macOS, and Linux without issues
- ✅ No data corruption or file loss during normal operations

### User Success
- ✅ At least 3 students complete the full workflow without help
- ✅ Students understand the value proposition
- ✅ Clear feedback on what features matter most for Phase 2
- ✅ No major usability blockers identified

### Project Success
- ✅ Proven architecture foundation for future phases
- ✅ Real user validation of core concepts
- ✅ Clear roadmap for Phase 2 based on feedback
- ✅ Team confidence in technical approach

## Next Phases Preview

### Phase 2 (Based on User Feedback)
- Enhanced metadata collection
- Project organization features
- Background processing for large files
- Improved UI/UX based on student feedback

### Phase 3 (Future)
- Event sourcing for complete audit trails
- Plugin architecture for extensibility
- Advanced BagIt features (versioning, relationships)

### Phase 4 (Much Later)
- Encryption and security features
- Cloud synchronization options
- Collaboration features

This revised approach addresses all critical feedback while maintaining the core vision. The thin vertical slice approach ensures we validate assumptions early and build only what users actually need.