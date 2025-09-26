import { invoke } from '@tauri-apps/api/core'
import { useCallback } from 'react'
import {
  ArchivedProject,
  ArchiveResult,
  BagResult,
  IntegrityReport
} from '../types/preservation.types'

interface ArchiveRequest {
  name: string
  description?: string
  files: string[]
}

export const useTauriCommands = () => {
  const archiveProject = async (request: ArchiveRequest): Promise<ArchiveResult> => {
    try {
      console.log('Invoking archive_project with:', request)
      const result = await invoke<ArchiveResult>('archive_project', { request })
      console.log('Archive result:', result)
      return result
    } catch (error) {
      console.error('Failed to archive project:', error)
      throw error
    }
  }

  const createBagitPackage = async (projectId: string): Promise<BagResult> => {
    try {
      console.log('Creating BagIt package for project:', projectId)
      const result = await invoke<BagResult>('create_bagit_package', { projectId })
      console.log('BagIt result:', result)
      return result
    } catch (error) {
      console.error('Failed to create BagIt package:', error)
      throw error
    }
  }

  const getAllProjects = useCallback(async (): Promise<ArchivedProject[]> => {
    try {
      const result = await invoke<ArchivedProject[]>('get_archived_projects')
      console.log('Retrieved projects:', result)
      return result
    } catch (error) {
      console.error('Failed to get projects:', error)
      throw error
    }
  }, [])

  const quarantineProject = async (projectId: string, reason: string): Promise<void> => {
    try {
      await invoke('quarantine_project', { projectId, reason })
      console.log('Project quarantined:', projectId)
    } catch (error) {
      console.error('Failed to quarantine project:', error)
      throw error
    }
  }

  const restoreProject = async (projectId: string): Promise<void> => {
    try {
      await invoke('restore_project', { projectId })
      console.log('Project restored:', projectId)
    } catch (error) {
      console.error('Failed to restore project:', error)
      throw error
    }
  }

  const scanVaultIntegrity = async (): Promise<IntegrityReport> => {
    try {
      const result = await invoke<IntegrityReport>('scan_vault_integrity')
      console.log('Integrity scan result:', result)
      return result
    } catch (error) {
      console.error('Failed to scan vault integrity:', error)
      throw error
    }
  }

  return {
    archiveProject,
    createBagitPackage,
    getAllProjects,
    quarantineProject,
    restoreProject,
    scanVaultIntegrity
  }
}