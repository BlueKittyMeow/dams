# Creative Work Preservation Toolkit (CWPT) - Development Plan

## Architecture Overview

### Tech Stack Decisions
**Frontend:**
- **React 18** with TypeScript for type safety and maintainability
- **Vite** for fast development and build pipeline
- **PWA** configuration with service workers for offline capability
- **TailwindCSS** for responsive, design-focused UI
- **React Query/TanStack Query** for data fetching and caching
- **Zustand** for lightweight state management

**Local Data Layer:**
- **SQLite** via `better-sqlite3` for main database (metadata, relationships)
- **File System Access API** for direct file operations
- **IndexedDB** as fallback for browsers without OPFS support
- **Event sourcing** pattern for audit trail and rollback capability

**Backend Services (Node.js):**
- **Fastify** for lightweight, fast API server
- **Prisma** as ORM for database operations
- **Bull/BullMQ** for background job processing
- **Siegfried** integration for file format identification

**Cloud Integration:**
- **OAuth 2.0** libraries for Google Drive, OneDrive, Dropbox
- **Rate-limited polling** workers for change detection

## Core Components Architecture

### 1. File System Layer
```
/src/filesystem/
├── bagit/           # BagIt package creation and validation
├── storage/         # Local storage management
├── cloud-sync/      # Cloud storage integrations
├── integrity/       # Checksum and validation
└── proxy/           # Web-friendly conversion pipeline
```

### 2. Metadata Management
```
/src/metadata/
├── schemas/         # Dublin Core, PREMIS, VRACore schemas
├── extractors/      # Automated metadata extraction
├── validators/      # Schema validation
└── search/          # Full-text and faceted search
```

### 3. Ingestion Pipeline
```
/src/ingestion/
├── workflows/       # Quick vs Detailed modes
├── processors/      # File type-specific handling
├── jobs/           # Background processing tasks
└── profiles/       # Reusable tool configurations
```

### 4. Project Management
```
/src/projects/
├── versioning/     # File relationship tracking
├── timeline/       # Visual project evolution
├── collections/    # Project hierarchies
└── export/         # Portfolio generation
```

## Data Protection & Privacy

### Privacy by Design
**Local-First Architecture:**
- User data remains on personal devices by default
- No central server required for core functionality
- User maintains full control of their creative work
- Optional cloud sync with user consent

**Data Security:**
- Local database encryption with user-controlled keys
- End-to-end encryption for optional cloud sync
- Secure transmission protocols (TLS 1.3+)
- Zero-knowledge architecture for cloud storage

**User Control:**
- Granular privacy controls for sharing features
- Clear consent mechanisms for cloud services
- Data export and deletion capabilities
- Transparent data processing

## Development Roadmap

### Phase 1: Core Foundation (4-6 weeks)
1. **Project Setup & Tooling**
   - Vite + React + TypeScript setup
   - ESLint, Prettier, testing framework
   - PWA manifest and service worker basics
   - SQLite schema design and Prisma setup

2. **Data Protection Foundation**
   - Privacy-focused design implementation
   - Local encryption implementation
   - Basic audit logging infrastructure

3. **Basic File Ingestion**
   - File System Access API implementation
   - Drag-and-drop interface
   - Basic metadata extraction
   - Local SQLite storage with encryption

4. **BagIt Implementation**
   - BagIt package creation
   - Manifest generation
   - Checksum calculation (background jobs)
   - Package validation

### Phase 2: Metadata & Search (3-4 weeks)
1. **Metadata Management**
   - Dublin Core + VRACore schema implementation
   - Form-based metadata entry
   - Automated extraction pipeline
   - Tag management system

2. **Search & Discovery**
   - Full-text search implementation
   - Faceted search filters
   - Visual browsing interface
   - Export functionality

### Phase 3: Project Management (4-5 weeks)
1. **Version Tracking**
   - File relationship mapping
   - Visual timeline component
   - Version branching support
   - Project hierarchies

2. **Risk Assessment**
   - Format risk evaluation engine
   - Migration timeline alerts
   - Dependency tracking
   - Future-compatibility scoring

### Phase 4: Cloud Integration (5-6 weeks)
1. **OAuth Implementation**
   - Google Drive, OneDrive, Dropbox APIs
   - Token management and refresh
   - Secure credential storage

2. **Sync Engine**
   - End-to-end encrypted sync
   - Change detection algorithms
   - Conflict resolution workflows
   - Integrity verification across locations
   - Deletion protection (quarantine)

### Phase 5: Polish & Testing (3-4 weeks)
1. **UX Refinement**
   - Performance optimization
   - Accessibility compliance (WCAG 2.1 AA)
   - Mobile/tablet responsive design
   - User testing with art students

2. **Production Readiness**
   - Comprehensive test suite
   - Security audit and testing
   - Error handling and recovery
   - Migration tools
   - Documentation

## Key Design Principles

### DRY & Modular Architecture
- **Service Layer Pattern:** Separate business logic from UI components
- **Plugin Architecture:** Extensible metadata schemas and processors
- **Event-Driven Design:** Decoupled components communicating via events
- **Factory Pattern:** File processor selection based on format detection

### Privacy-First Design
- **Local-First:** Core functionality works without cloud services
- **Zero-Knowledge:** Cloud sync providers cannot access file contents
- **Granular Control:** User controls every aspect of data sharing
- **Transparent Operations:** All data flows visible to users

### Performance Considerations
- **Lazy Loading:** Progressive loading for large collections
- **Virtual Scrolling:** Handle thousands of files efficiently
- **Web Workers:** Background processing for checksums/metadata
- **Caching Strategy:** Multi-layer caching (memory, SQLite, cloud)

### Preservation Standards Compliance
- **BagIt v1.0:** Full specification implementation with validation
- **PREMIS v3.0:** Complete provenance and technical metadata
- **OAIS Model:** Reference architecture alignment
- **Format Migration:** Automated pathway suggestions

This plan creates a robust, scalable foundation while maintaining the local-first, zero-friction philosophy. The modular architecture supports future extensibility and the phased approach delivers value incrementally with strong data protection principles.