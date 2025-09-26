import { useEffect, useState, type FC, type ReactNode } from 'react'
import { listen } from '@tauri-apps/api/event'

interface DroppedFile {
  path: string
  name: string
}

interface DropZoneProps {
  onFilesDropped: (files: DroppedFile[]) => void
  isProcessing?: boolean
  className?: string
  children?: ReactNode
}
const DropZone: FC<DropZoneProps> = ({
  onFilesDropped,
  isProcessing = false,
  className = '',
  children
}) => {
  const [isDragOver, setIsDragOver] = useState(false)

  useEffect(() => {
    let unlistenDrop: (() => void) | undefined
    let unlistenDragOver: (() => void) | undefined
    let unlistenDragLeave: (() => void) | undefined

    const setupListeners = async () => {
      // Listen for file drop events
      unlistenDrop = await listen<{ paths: string[] }>('tauri://drop', (event) => {
        const files = event.payload.paths.map(path => ({
          path,
          name: path.split('/').pop() || path.split('\\').pop() || 'Unknown'
        }))
        onFilesDropped(files)
        setIsDragOver(false)
      })

      // Listen for drag over events
      unlistenDragOver = await listen('tauri://drag-over', () => {
        setIsDragOver(true)
      })

      // Listen for drag leave events
      unlistenDragLeave = await listen('tauri://drag-leave', () => {
        setIsDragOver(false)
      })
    }

    setupListeners()

    return () => {
      unlistenDrop?.()
      unlistenDragOver?.()
      unlistenDragLeave?.()
    }
  }, [onFilesDropped])

  return (
    <div
      className={`
        relative border-2 border-dashed rounded-lg p-8 text-center
        transition-all duration-200 flex flex-col items-center justify-center
        ${isDragOver && !isProcessing
          ? 'border-blue-500 bg-blue-50 border-solid'
          : 'border-gray-300 bg-white hover:border-gray-400'
        }
        ${isProcessing ? 'opacity-50 pointer-events-none' : 'cursor-pointer'}
        ${className}
      `}
    >
      {isProcessing ? (
        <div className="flex flex-col items-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mb-4"></div>
          <p className="text-lg font-medium text-blue-600">Processing files...</p>
          <p className="text-sm text-gray-500 mt-2">Creating BagIt preservation package</p>
        </div>
      ) : children ? (
        children
      ) : (
        <div>
          <div className="mx-auto h-12 w-12 text-gray-400 mb-4">
            <svg fill="none" stroke="currentColor" viewBox="0 0 48 48">
              <path
                d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02"
                strokeWidth={2}
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          </div>
          <p className="text-lg font-medium text-gray-700">
            Drop your project files here
          </p>
          <p className="text-sm text-gray-500 mt-2">
            Drag folders or files to create a preservation archive
          </p>
          <p className="text-xs text-gray-400 mt-2">
            Supports all file types • Creates BagIt-compliant packages
          </p>
        </div>
      )}
    </div>
  )
}

export default DropZone
