# Creative Work Preservation Toolkit (CWPT)

## Overview

The Creative Work Preservation Toolkit is a desktop application designed to help art students and creative professionals archive and preserve their digital work using professional-grade preservation standards. The application creates BagIt-compliant preservation packages that ensure long-term accessibility and integrity of creative projects.

## Features

### Core Functionality
- **Drag-and-Drop Interface**: Simply drag project files or folders into the application to begin archiving
- **BagIt v1.0 Compliance**: Creates industry-standard preservation packages
- **Multiple Checksum Validation**: Uses SHA-256, MD5, and BLAKE3 for comprehensive file integrity
- **Cross-Platform Support**: Works on Windows, macOS, and Linux
- **Local-First Philosophy**: All data stays on your machine - no cloud dependencies required

### Preservation Features
- **Event Sourcing**: Complete audit trail of all operations
- **Quarantine System**: Soft deletion with restoration capabilities
- **Integrity Scanning**: Regular validation of archived content
- **Metadata Preservation**: Dublin Core compliant metadata storage
- **File Format Detection**: Automatic identification of file types

### User Interface
- **Modern React Frontend**: Clean, intuitive interface built with TailwindCSS
- **Real-Time Feedback**: Visual progress indicators during archival operations
- **Project Management**: View and manage all archived projects in one place
- **Responsive Design**: Optimized for various screen sizes

## Getting Started

### Installation

1. Download the latest release from the releases page
2. Install the application following your operating system's standard procedure
3. Launch CWPT from your applications menu or desktop shortcut

### First Use

1. **Archive a Project**: Drag files or folders from your file manager into the drop zone
2. **Review Results**: View your newly created archive in the project list
3. **Verify Integrity**: The system automatically validates file checksums and package structure

### Basic Workflow

1. **Select Project Files**: Choose the files or folders you want to preserve
2. **Drag to CWPT**: Drop them into the application's drop zone
3. **Automatic Processing**: The app creates a BagIt package with checksums and metadata
4. **Verification**: System validates the archive integrity
5. **Storage**: Access your preserved projects through the interface

## Technical Standards

### BagIt Specification
CWPT implements the full BagIt v1.0 specification including:
- `bagit.txt` declaration file
- `bag-info.txt` metadata file
- `manifest-sha256.txt` checksum manifest
- `data/` directory for payload files
- Optional tag file manifests

### Preservation Metadata
- **Dublin Core**: Standard bibliographic metadata
- **PREMIS**: Preservation metadata for digital objects
- **Technical Metadata**: File format, size, creation dates
- **Provenance Information**: Complete chain of custody

### File Integrity
- **SHA-256**: Primary checksum algorithm for security
- **MD5**: Legacy compatibility checksums
- **BLAKE3**: High-performance verification
- **Regular Validation**: Periodic integrity checks

## File Organization

When you archive a project, CWPT creates the following structure:

```
project-archive-[UUID]/
├── bagit.txt                 # BagIt declaration
├── bag-info.txt             # Metadata and bag information
├── manifest-sha256.txt      # SHA-256 checksums
├── manifest-md5.txt         # MD5 checksums (optional)
├── tagmanifest-sha256.txt   # Tag file checksums
└── data/                    # Your original files
    └── [your project files]
```

## Project Management

### Viewing Archives
- All archived projects appear in the main interface
- Click on any project to view details and metadata
- See file counts, sizes, and creation dates at a glance

### Integrity Monitoring
- CWPT regularly validates archive integrity
- Corrupted files are flagged for attention
- Restoration options available for damaged archives

### Export and Migration
- Export projects as standard BagIt packages
- Compatible with other preservation systems
- Full metadata preservation during transfers

## Troubleshooting

### Common Issues

**Files not archiving**: Ensure you have read permissions for all selected files
**Large file warnings**: Very large files may take time to process - this is normal
**Checksum failures**: May indicate file corruption or permission issues

### Getting Help

- Check the console output for detailed error messages
- Verify file permissions and disk space
- Review the development log for technical details

## Development Status

This is Phase 1 of the CWPT project, focusing on core archival functionality. Current features include:

✅ **Complete**: Basic archival workflow, BagIt package creation, drag-and-drop interface
🔄 **In Progress**: Database integration, advanced metadata editing
📋 **Planned**: Cloud sync options, collaborative features, advanced search

## Technical Architecture

- **Frontend**: React with TypeScript and TailwindCSS
- **Backend**: Rust with Tauri framework
- **Database**: SQLite with event sourcing
- **Standards**: BagIt v1.0, Dublin Core, PREMIS
- **Checksums**: SHA-256, MD5, BLAKE3

## License and Credits

Creative Work Preservation Toolkit
Developed for art students and creative professionals
Built with modern web technologies and preservation standards

---

*This documentation reflects the current state of Phase 1 development. Features and capabilities will expand in future releases.*