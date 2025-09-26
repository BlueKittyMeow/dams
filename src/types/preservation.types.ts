export interface ArchivedProject {
  id: string
  name: string
  description?: string
  archivedAt: Date
  bagitPackageId?: string
  fileCount: number
  totalSize: number
  isQuarantined: boolean
  createdAt: Date
  updatedAt: Date
}

export interface BagitPackage {
  id: string
  archivedProjectId: string
  bagPath: string
  manifestSha256: string
  bagSize: number
  payloadFileCount: number
  isValid: boolean
  validatedAt?: Date
  createdAt: Date
}

export interface QuarantineEntry {
  id: string
  archivedProjectId: string
  quarantinedAt: Date
  originalBagPath: string
  scheduledForDeletionAt?: Date
  reason?: string
}

export interface VaultSnapshot {
  id: string
  snapshotAt: Date
  userLayerChecksum: string
  bagsLayerChecksum: string
  anomaliesDetected?: string[]
  createdAt: Date
}

export interface FileMetadata {
  path: string
  name: string
  size: number
  mimeType?: string
  checksumSha256?: string
  checksumMd5?: string
}

export interface ArchiveResult {
  success: boolean
  projectId?: string
  error?: string
}

export interface BagResult {
  success: boolean
  bagPath?: string
  validationResults?: ValidationResult[]
  error?: string
}

export interface ValidationResult {
  type: 'error' | 'warning' | 'info'
  message: string
  file?: string
}

export interface IntegrityReport {
  isHealthy: boolean
  issues: Array<{
    type: 'missing_file' | 'corrupted_file' | 'external_modification'
    severity: 'critical' | 'warning' | 'info'
    message: string
    affectedFiles: string[]
  }>
  lastScanAt: Date
}