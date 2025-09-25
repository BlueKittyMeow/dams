# Creative Work Preservation Toolkit - Phase One Detailed Proposal

## Executive Summary

Phase 1 establishes the foundational architecture that will determine the success of all future phases. Rather than rushing to user-facing features, this phase focuses on creating robust, scalable foundations that support the complete vision outlined in the main project readme.

**Duration**: 6 weeks
**Goal**: Solid architectural foundation supporting local-first preservation with BagIt compliance
**Deliverables**: Core file ingestion, BagIt packaging, event sourcing, and vault management

## Critical Architectural Decisions

### 1. Database Schema & Event Sourcing Architecture

**Decision**: SQLite with event sourcing pattern and Prisma ORM
**Rationale**: Local-first requires embedded database, event sourcing enables complete audit trails and future cloud sync conflict resolution.

#### Core Database Tables

```sql
-- Files represent logical entities that may have multiple versions
CREATE TABLE files (
    id TEXT PRIMARY KEY,
    original_name TEXT NOT NULL,
    current_version_id TEXT,
    project_id TEXT,
    file_format TEXT,
    preservation_risk_level INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME NULL,
    FOREIGN KEY (current_version_id) REFERENCES file_versions(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- File versions track each iteration with BagIt packages
CREATE TABLE file_versions (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    bagit_package_id TEXT,
    file_size INTEGER NOT NULL,
    mime_type TEXT,
    checksum_sha256 TEXT NOT NULL,
    checksum_md5 TEXT NOT NULL,
    user_layer_path TEXT NOT NULL, -- Path in user-friendly structure
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_id) REFERENCES files(id),
    FOREIGN KEY (bagit_package_id) REFERENCES bagit_packages(id),
    UNIQUE(file_id, version_number)
);

-- Events provide complete audit trail and enable rollback
CREATE TABLE events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL, -- FileIngested, MetadataUpdated, etc.
    aggregate_type TEXT NOT NULL, -- File, Project, User, etc.
    aggregate_id TEXT NOT NULL,
    payload TEXT NOT NULL, -- JSON payload
    metadata TEXT, -- Additional context (user agent, IP, etc.)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Flexible metadata supporting multiple schemas (Dublin Core, PREMIS, VRACore)
CREATE TABLE metadata_entries (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    schema_type TEXT NOT NULL, -- dublin_core, premis, vra_core, custom
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    data_type TEXT DEFAULT 'string', -- string, number, date, json
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_id) REFERENCES files(id),
    UNIQUE(file_id, schema_type, key)
);

-- Project hierarchies and collections
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT,
    project_type TEXT DEFAULT 'project', -- project, collection, series
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME NULL,
    FOREIGN KEY (parent_id) REFERENCES projects(id)
);

-- File relationships for complex project structures
CREATE TABLE file_relationships (
    id TEXT PRIMARY KEY,
    source_file_id TEXT NOT NULL,
    target_file_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL, -- derived_from, version_of, part_of, requires
    metadata TEXT, -- Additional relationship context
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_file_id) REFERENCES files(id),
    FOREIGN KEY (target_file_id) REFERENCES files(id),
    UNIQUE(source_file_id, target_file_id, relationship_type)
);

-- BagIt package tracking and validation
CREATE TABLE bagit_packages (
    id TEXT PRIMARY KEY,
    file_version_id TEXT NOT NULL,
    bag_path TEXT NOT NULL, -- Path to bag directory
    manifest_checksum TEXT NOT NULL, -- Checksum of manifest-sha256.txt
    tagmanifest_checksum TEXT, -- Checksum of tagmanifest-sha256.txt
    bag_size INTEGER NOT NULL,
    payload_file_count INTEGER NOT NULL,
    is_valid BOOLEAN DEFAULT TRUE,
    last_validated_at DATETIME,
    validation_errors TEXT, -- JSON array of validation issues
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_version_id) REFERENCES file_versions(id),
    UNIQUE(bag_path)
);

-- User settings and vault configuration
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    data_type TEXT DEFAULT 'string',
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Future cloud sync state tracking (Phase 4)
CREATE TABLE cloud_sync_state (
    id TEXT PRIMARY KEY,
    file_version_id TEXT NOT NULL,
    provider TEXT NOT NULL, -- google_drive, onedrive, dropbox
    remote_id TEXT NOT NULL,
    remote_path TEXT NOT NULL,
    last_sync_at DATETIME,
    sync_status TEXT DEFAULT 'pending', -- pending, syncing, synced, error
    sync_error TEXT,
    etag TEXT, -- For change detection
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_version_id) REFERENCES file_versions(id),
    UNIQUE(file_version_id, provider)
);
```

#### Event Sourcing Implementation

**Event Types**:
- `FileIngested`: New file added to system
- `FileVersionCreated`: New version of existing file
- `MetadataExtracted`: Automated metadata extraction completed
- `MetadataUpdated`: User-modified metadata
- `ProjectCreated`: New project or collection created
- `FileAddedToProject`: File associated with project
- `BagitPackageCreated`: BagIt package successfully created
- `BagitValidationCompleted`: Package validation results
- `SettingsChanged`: User preference updates
- `VaultInitialized`: New preservation vault created

**Event Payload Structure**:
```typescript
interface BaseEvent {
  id: string;
  eventType: string;
  aggregateType: string;
  aggregateId: string;
  payload: any;
  metadata?: any;
  createdAt: Date;
}

interface FileIngestedEvent extends BaseEvent {
  eventType: 'FileIngested';
  aggregateType: 'File';
  payload: {
    originalName: string;
    filePath: string;
    fileSize: number;
    mimeType: string;
    checksums: {
      sha256: string;
      md5: string;
    };
    projectId?: string;
  };
}
```

### 2. Three-Layer File System Architecture

**Decision**: Vault-based storage with UUID-mapped layers
**Rationale**: Maintains clean user experience while ensuring preservation integrity and enabling future cloud sync.

#### Directory Structure

```
/user-selected-vault/
├── user-files/                    # User Layer - clean, browsable structure
│   ├── projects/
│   │   ├── thesis-work/
│   │   │   ├── sketches/
│   │   │   ├── drafts/
│   │   │   └── final/
│   │   └── coursework/
│   └── collections/
│       ├── photography/
│       └── design-work/
├── .cwpt-preservation/            # Hidden preservation layer
│   ├── bags/                      # BagIt packages (UUID-named)
│   │   ├── a1b2c3d4-e5f6-7890.../
│   │   │   ├── data/              # Actual file content
│   │   │   ├── manifest-sha256.txt
│   │   │   ├── bagit.txt
│   │   │   └── bag-info.txt
│   │   └── b2c3d4e5-f6g7-8901.../
│   ├── manifests/                 # Master manifests and integrity data
│   └── metadata/                  # Cached extracted metadata
├── .cwpt-proxies/                # Web-friendly versions
│   ├── thumbnails/               # 150x150 thumbnails
│   │   ├── a1b2c3d4.jpg
│   │   └── b2c3d4e5.jpg
│   └── previews/                 # Optimized preview versions
│       ├── a1b2c3d4.webp
│       └── b2c3d4e5.webp
└── .cwpt-system/                 # System files
    ├── database.sqlite           # Main SQLite database
    ├── database-wal              # SQLite WAL file
    ├── database-shm              # SQLite shared memory
    ├── keys/                     # Encryption keys
    │   └── master.key
    └── logs/                     # Operation logs
        ├── ingestion.log
        └── validation.log
```

#### File Path Resolution System

```typescript
interface FilePathResolver {
  // Convert user-friendly path to preservation UUID
  resolveToUuid(userPath: string): Promise<string>;

  // Convert UUID to user-friendly path
  resolveToUserPath(uuid: string): Promise<string>;

  // Get all proxy versions for a file
  getProxyPaths(uuid: string): Promise<ProxyPaths>;

  // Ensure path mapping consistency
  validatePathMapping(uuid: string): Promise<ValidationResult>;
}

interface ProxyPaths {
  thumbnail?: string;
  preview?: string;
  webOptimized?: string;
}
```

### 3. BagIt Standards Implementation

**Decision**: Full BagIt v1.0 compliance with comprehensive validation
**Rationale**: Industry standard ensures long-term preservation and migration compatibility with existing digital preservation workflows.

#### BagIt Structure Requirements

```
bag-root-directory/
├── bagit.txt                    # Version and encoding declaration
├── manifest-<algorithm>.txt     # File checksums (required)
├── tagmanifest-<algorithm>.txt  # Tag file checksums (optional)
├── bag-info.txt                 # Metadata about the bag
├── data/                        # Payload files
│   └── [original files]
└── [optional tag files]         # Additional metadata files
```

#### Implementation Specifications

**bagit.txt Format**:
```
BagIt-Version: 1.0
Tag-File-Character-Encoding: UTF-8
```

**manifest-sha256.txt Format**:
```
7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730 data/image.jpg
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 data/readme.txt
```

**bag-info.txt Format**:
```
Bag-Software-Agent: CWPT v1.0
Bagging-Date: 2024-03-15
Payload-Oxum: 279164.2
Bag-Size: 279 KB
Source-Organization: Creative Work Preservation Toolkit
External-Identifier: file-uuid-a1b2c3d4-e5f6-7890
Internal-Sender-Identifier: user-original-filename.jpg
Internal-Sender-Description: Creative work ingested via CWPT
```

#### Validation Pipeline

```typescript
interface BagitValidator {
  validateStructure(bagPath: string): Promise<StructureValidation>;
  validateManifests(bagPath: string): Promise<ManifestValidation>;
  validateChecksums(bagPath: string): Promise<ChecksumValidation>;
  fullValidation(bagPath: string): Promise<BagValidationResult>;
}

interface BagValidationResult {
  isValid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
  validationTime: number;
  checksumAlgorithms: string[];
  payloadFileCount: number;
  totalSize: number;
}
```

### 4. Plugin Architecture Foundation

**Decision**: Interface-based plugin system with dependency injection
**Rationale**: Enables extensibility without core system modifications, supporting future metadata extractors, format processors, and cloud providers.

#### Core Plugin Interfaces

```typescript
// Metadata extraction plugins
interface MetadataExtractor {
  readonly name: string;
  readonly version: string;
  readonly supportedFormats: string[];

  supports(mimeType: string, fileExtension: string): boolean;
  extract(filePath: string): Promise<MetadataEntry[]>;
  extractBatch(filePaths: string[]): Promise<BatchExtractionResult>;
}

// File format processing plugins
interface FormatProcessor {
  readonly name: string;
  readonly version: string;
  readonly inputFormats: string[];
  readonly outputFormats: string[];

  canProcess(inputFormat: string, outputFormat: string): boolean;
  generateThumbnail(inputPath: string, outputPath: string, size: number): Promise<ProcessResult>;
  generatePreview(inputPath: string, outputPath: string): Promise<ProcessResult>;
  convertFormat(inputPath: string, outputPath: string, targetFormat: string): Promise<ProcessResult>;
}

// Cloud storage provider plugins (Phase 4)
interface CloudProvider {
  readonly name: string;
  readonly version: string;

  authenticate(credentials: CloudCredentials): Promise<AuthResult>;
  uploadFile(localPath: string, remotePath: string): Promise<UploadResult>;
  downloadFile(remotePath: string, localPath: string): Promise<DownloadResult>;
  syncDirectory(localDir: string, remoteDir: string): Promise<SyncResult>;
  listFiles(remotePath: string): Promise<CloudFile[]>;
}

// Plugin registration system
interface PluginRegistry {
  registerMetadataExtractor(plugin: MetadataExtractor): void;
  registerFormatProcessor(plugin: FormatProcessor): void;
  registerCloudProvider(plugin: CloudProvider): void;

  getMetadataExtractors(mimeType: string): MetadataExtractor[];
  getFormatProcessor(inputFormat: string, outputFormat: string): FormatProcessor | null;
  getCloudProvider(name: string): CloudProvider | null;
}
```

### 5. Security & Encryption Foundation

**Decision**: User-controlled encryption with Web Crypto API
**Rationale**: Privacy-first design with industry-standard cryptography, preparing for future E2E cloud encryption.

#### Encryption Architecture

```typescript
interface EncryptionService {
  // Key management
  generateMasterKey(): Promise<CryptoKey>;
  deriveDatabaseKey(masterKey: CryptoKey, salt: Uint8Array): Promise<CryptoKey>;
  deriveFileKey(masterKey: CryptoKey, fileId: string): Promise<CryptoKey>;

  // Database encryption (SQLCipher integration)
  encryptDatabase(dbPath: string, key: CryptoKey): Promise<void>;
  decryptDatabase(dbPath: string, key: CryptoKey): Promise<void>;

  // Optional file encryption
  encryptFile(inputPath: string, outputPath: string, key: CryptoKey): Promise<void>;
  decryptFile(inputPath: string, outputPath: string, key: CryptoKey): Promise<void>;

  // Future cloud encryption
  encryptForCloud(data: ArrayBuffer, cloudKey: CryptoKey): Promise<ArrayBuffer>;
  decryptFromCloud(encryptedData: ArrayBuffer, cloudKey: CryptoKey): Promise<ArrayBuffer>;
}

interface KeyStore {
  storeMasterKey(key: CryptoKey, password?: string): Promise<void>;
  retrieveMasterKey(password?: string): Promise<CryptoKey>;
  keyExists(): Promise<boolean>;
  rotateKeys(): Promise<void>;
}
```

#### Security Implementation Details

- **Database Encryption**: SQLCipher with AES-256 encryption
- **File Encryption**: Optional AES-GCM for sensitive files
- **Key Derivation**: PBKDF2 with 100,000 iterations minimum
- **Key Storage**: Secure OS keyring integration (Windows Credential Manager, macOS Keychain, Linux Secret Service)
- **Random Generation**: Cryptographically secure random number generation via Web Crypto API

### 6. Background Processing System

**Decision**: Multi-threaded Web Workers with job queuing
**Rationale**: CPU-intensive operations (checksums, format detection, metadata extraction) must not block the UI thread.

#### Job Processing Architecture

```typescript
interface BackgroundJob {
  id: string;
  type: JobType;
  priority: JobPriority;
  payload: any;
  status: JobStatus;
  progress: number;
  error?: string;
  createdAt: Date;
  startedAt?: Date;
  completedAt?: Date;
}

enum JobType {
  CalculateChecksum = 'calculate_checksum',
  ExtractMetadata = 'extract_metadata',
  CreateBagitPackage = 'create_bagit_package',
  ValidateBagitPackage = 'validate_bagit_package',
  GenerateProxy = 'generate_proxy',
  DetectFileFormat = 'detect_file_format'
}

enum JobPriority {
  Critical = 1,  // User-initiated operations
  High = 2,      // Recently ingested files
  Normal = 3,    // Background processing
  Low = 4        // Optimization tasks
}

interface JobQueue {
  enqueue(job: BackgroundJob): Promise<void>;
  dequeue(workerType: string): Promise<BackgroundJob | null>;
  updateProgress(jobId: string, progress: number): Promise<void>;
  markCompleted(jobId: string, result: any): Promise<void>;
  markFailed(jobId: string, error: string): Promise<void>;

  // Job monitoring
  getQueueStatus(): Promise<QueueStatus>;
  getPendingJobs(): Promise<BackgroundJob[]>;
  getJobHistory(limit: number): Promise<BackgroundJob[]>;
}
```

#### Worker Implementation

```typescript
// Web Worker for checksum calculation
class ChecksumWorker {
  async calculateChecksums(filePath: string): Promise<ChecksumResult> {
    const buffer = await this.readFileBuffer(filePath);

    const [sha256, md5, sha512] = await Promise.all([
      this.calculateSHA256(buffer),
      this.calculateMD5(buffer),
      this.calculateSHA512(buffer)
    ]);

    return { sha256, md5, sha512, fileSize: buffer.byteLength };
  }

  private async calculateSHA256(buffer: ArrayBuffer): Promise<string> {
    const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);
    return this.bufferToHex(hashBuffer);
  }
}
```

## Development Timeline

### Week 1-2: Foundation Setup

#### Week 1: Project Scaffolding
- **Day 1-2**: Vite + React + TypeScript + TailwindCSS setup
- **Day 3**: ESLint, Prettier, Vitest configuration
- **Day 4-5**: GitHub Actions CI/CD pipeline setup

#### Week 2: Database Foundation
- **Day 1-2**: Complete database schema implementation
- **Day 3-4**: Prisma setup with SQLite and migrations
- **Day 5**: SQLCipher integration for database encryption

### Week 3-4: File System Architecture

#### Week 3: Vault Management
- **Day 1-2**: Vault initialization and directory structure creation
- **Day 3-4**: File path resolution system implementation
- **Day 5**: Cross-platform path handling and validation

#### Week 4: BagIt Implementation
- **Day 1-2**: BagIt package creation with proper structure
- **Day 3-4**: Manifest generation and checksum calculation
- **Day 5**: BagIt validation pipeline with error reporting

### Week 5-6: Core Processing Pipeline

#### Week 5: Background Job System
- **Day 1-2**: Web Worker setup and job queue implementation
- **Day 3-4**: Progress reporting and error handling
- **Day 5**: Job priority and retry logic implementation

#### Week 6: File Processing
- **Day 1-2**: Siegfried WASM integration for format detection
- **Day 3-4**: Basic metadata extraction pipeline
- **Day 5**: Integration testing and validation

## Risk Mitigation Strategy

### Critical Path Items

1. **Database Schema Stability**
   - **Risk**: Schema changes break existing data
   - **Mitigation**: Comprehensive migration testing, backwards compatibility validation
   - **Validation**: Schema supports all planned features from phases 2-5

2. **BagIt Standards Compliance**
   - **Risk**: Non-compliant packages cannot be validated by external tools
   - **Mitigation**: Test against bagit-python and bagit-java validators
   - **Validation**: All created packages validate with industry-standard tools

3. **File System Integrity**
   - **Risk**: File corruption or path resolution failures
   - **Mitigation**: Atomic operations, transaction logging, integrity checks
   - **Validation**: Stress testing with large file collections

4. **Cross-Platform Compatibility**
   - **Risk**: Path handling differences cause failures on different OS
   - **Mitigation**: Abstracted path utilities, extensive cross-platform testing
   - **Validation**: Automated testing on Windows, macOS, and Linux

### Performance Benchmarks

**Target Performance Metrics**:
- File ingestion: < 2 seconds for files up to 100MB
- Checksum calculation: < 5 seconds for 1GB files
- Database queries: < 100ms for typical operations
- BagIt validation: < 10 seconds for packages with 1000+ files

**Scalability Targets**:
- Support collections up to 100,000 files
- Database size up to 10GB
- Individual file size up to 5GB
- Concurrent background jobs: 4-8 depending on system resources

### Testing Strategy

#### Unit Testing (90% coverage target)
- Database operations and event sourcing logic
- BagIt creation and validation functions
- File path resolution and mapping
- Cryptographic operations
- Job queue and worker coordination

#### Integration Testing
- Complete file ingestion workflow
- Database and file system consistency
- Background job processing pipeline
- Plugin system interfaces

#### End-to-End Testing (Playwright)
- Vault creation and initialization
- File drag-and-drop ingestion
- BagIt package validation workflows
- Cross-platform compatibility scenarios

#### Performance Testing
- Large file ingestion benchmarks
- Concurrent operation stress testing
- Memory usage profiling
- Database query performance analysis

## Success Criteria

### Functional Requirements
- ✅ Complete file ingestion from drag-and-drop to BagIt package
- ✅ Full BagIt v1.0 compliance with external tool validation
- ✅ Three-layer file system with consistent path mapping
- ✅ Background processing without UI blocking
- ✅ Database encryption and basic security measures
- ✅ Event sourcing foundation for complete audit trails

### Technical Requirements
- ✅ Modular architecture supporting future plugins
- ✅ Cross-platform compatibility (Windows, macOS, Linux)
- ✅ Performance targets met for all core operations
- ✅ Comprehensive test coverage (>90% unit, full integration)
- ✅ Documentation for all public APIs and interfaces

### Quality Gates
- ✅ All BagIt packages validate with external tools (bagit-python)
- ✅ No data corruption under stress testing
- ✅ Memory usage remains stable during extended operations
- ✅ Clean code architecture with clear separation of concerns
- ✅ Security audit passes with no high-severity findings

This Phase 1 foundation will enable rapid development of user-facing features in subsequent phases while maintaining the architectural integrity required for a professional-grade preservation system.