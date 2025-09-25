# Creative Work Preservation Toolkit (CWPT)

## Project Overview

A web-based digital preservation system designed specifically for art, design, and creative students to manage, preserve, and share their digital creative work throughout their academic careers and beyond. The system emphasizes user-friendly workflows while implementing professional-grade preservation standards behind the scenes.

## Core Philosophy

- **Local-first:** Students maintain control and easy access to their files
- **Zero friction:** Minimal barriers to adoption and daily use
- **Future-proof:** Industry-standard preservation practices (BagIt, LOCKSS)
- **Creative-focused:** Built for iterative creative processes, not just document management

## Architecture

### Dual-Layer File System
- **User Layer:** Clean, browsable folder structure with original filenames
- **Preservation Layer:** Hidden BagIt-compliant packages with manifests, checksums, and versioning
- **Proxy Layer:** Auto-generated web-friendly versions for sharing and preview

### Storage Strategy
- **Local primary storage:** Tool manages dedicated preservation vault directory
- **Cloud sync:** Student-configurable backends (OneDrive, Google Drive, Dropbox)
- **LOCKSS implementation:** Multiple redundant copies with integrity checking
- **Deletion protection:** Quarantine workflow prevents cascade deletions across storage locations

## Key Features

### Ingestion Workflows

**Quick Capture Mode:**
- Drag-and-drop file ingestion
- Automatic format identification and metadata extraction
- Pre-configured tool profiles (reusable software/environment settings)
- Background checksum generation

**Detailed Preservation Mode:**
- Comprehensive metadata forms
- Dependency tracking (software versions, plugins, libraries, fonts)
- Process documentation capture
- Environment snapshots
- Custom taxonomy and tagging

### Project Management

**Version Tracking:**
- Link related files across time (sketch → draft → final)
- Visual timeline of project evolution
- Support for multiple completion states/variations
- Branching for alternative versions

**Relationship Mapping:**
- File dependencies and associations
- Project hierarchies and collections
- Cross-project references

### Preservation Features

**BagIt Compliance:**
- Industry-standard packaging for long-term preservation
- Automated manifest generation and validation
- Metadata standardization (Dublin Core + custom creative fields)
- Migration pathway compatibility

**Format Risk Assessment:**
- Real-time warnings for preservation-risk file formats
- Migration suggestions and timeline alerts
- Dependency obsolescence tracking
- Future-compatibility scoring

### Access and Sharing

**Proxy Generation:**
- Automatic creation of web-friendly versions
- Multiple resolution/format variants
- Social media optimized crops
- Presentation-ready exports

**Search and Discovery:**
- Full-text and metadata search
- Visual browsing interfaces
- Filter by format, date, software, project status
- Export functionality for portfolios

### Data Protection

**Database Versioning:**
- Event-sourcing architecture for complete audit trail
- Automated backup scheduling
- Rollback functionality with user confirmation
- Conflict resolution for multi-device usage

**Cloud Integration:**
- OAuth-based authentication for cloud storage
- Periodic polling for change detection
- Intelligent sync conflict resolution
- Cross-platform availability monitoring

## Technical Stack Considerations

### Frontend
- Modern web app with File System Access API
- Progressive Web App (PWA) for offline capability
- Local SQLite database with cloud backup
- Responsive interface for mobile/tablet access

### Backend Services
- Cloud storage API integrations
- Background job processing for checksums/metadata
- File format identification services (DROID/Siegfried integration)
- Automated integrity checking workflows

### Preservation Standards
- BagIt specification compliance
- PREMIS metadata schema
- Dublin Core + VRACore for visual materials
- OAIS reference model alignment

## Use Cases

1. **Daily Creative Work:** Students easily capture and organize work-in-progress
2. **Project Documentation:** Comprehensive records of creative processes and dependencies
3. **Portfolio Preparation:** Curated collections with proper metadata for presentation
4. **Long-term Preservation:** Professional-grade archival practices for lifetime access
5. **Collaboration:** Secure sharing with proper attribution and version control
6. **Academic Requirements:** Integration with thesis/portfolio submission workflows

## Success Metrics

- Student adoption rates and sustained usage
- File format diversity and preservation coverage
- Successful recovery from accidental deletions
- Integration with institutional repository systems
- Community contributions and feature requests

## Future Extensibility

- Plugin architecture for custom metadata schemas
- Integration APIs for institutional systems
- Open source release for broader adoption
- Advanced AI-powered metadata extraction
- Collaborative workspace features

MVP Development Path:

Core local PWA - file ingestion, BagIt creation, metadata management
Cloud sync integration - OneDrive/Google Drive APIs for backup
Polish UX - get real user feedback from art students
Hosted tier development - once proven the concept works