import React from 'react'
import { ArchivedProject } from '../../types/preservation.types'

interface ProjectListProps {
  projects: ArchivedProject[]
}

const ProjectList: React.FC<ProjectListProps> = ({ projects }) => {
  const formatSize = (bytes: number): string => {
    const sizes = ['Bytes', 'KB', 'MB', 'GB']
    if (bytes === 0) return '0 Bytes'
    const i = Math.floor(Math.log(bytes) / Math.log(1024))
    return `${Math.round(bytes / Math.pow(1024, i) * 10) / 10} ${sizes[i]}`
  }

  const formatDate = (date: Date): string => {
    return new Date(date).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  }

  if (projects.length === 0) {
    return (
      <div className="bg-white rounded-lg border-2 border-dashed border-gray-300 p-8 text-center h-64 flex flex-col items-center justify-center">
        <div className="mx-auto h-12 w-12 text-gray-400 mb-4">
          <svg fill="none" stroke="currentColor" viewBox="0 0 48 48">
            <path
              d="M20 6L9 17H4a2 2 0 00-2 2v18a2 2 0 002 2h40a2 2 0 002-2V19a2 2 0 00-2-2h-5l-11-11H20z"
              strokeWidth={2}
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </div>
        <p className="text-lg font-medium text-gray-700">No archived projects yet</p>
        <p className="text-sm text-gray-500 mt-2">
          Drop files to create your first preservation archive
        </p>
      </div>
    )
  }

  return (
    <div className="bg-white rounded-lg border border-gray-200 h-64 overflow-y-auto">
      <div className="divide-y divide-gray-200">
        {projects.map((project) => (
          <div key={project.id} className="p-4 hover:bg-gray-50 transition-colors">
            <div className="flex items-start justify-between">
              <div className="min-w-0 flex-1">
                <div className="flex items-center space-x-2">
                  <h3 className="text-sm font-medium text-gray-900 truncate">
                    {project.name}
                  </h3>
                  {project.isQuarantined && (
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                      Quarantined
                    </span>
                  )}
                </div>
                {project.description && (
                  <p className="text-sm text-gray-500 truncate mt-1">
                    {project.description}
                  </p>
                )}
                <div className="flex items-center space-x-4 mt-2 text-xs text-gray-400">
                  <span>{project.fileCount} files</span>
                  <span>{formatSize(project.totalSize)}</span>
                  <span>Archived {formatDate(project.archivedAt)}</span>
                </div>
              </div>
              <div className="ml-2 flex-shrink-0 flex space-x-1">
                <button className="text-gray-400 hover:text-gray-600">
                  <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                </button>
                <button className="text-gray-400 hover:text-red-600">
                  <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}

export default ProjectList