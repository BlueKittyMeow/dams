import { useState, useEffect } from 'react'
import DropZone from './components/file-operations/DropZone'
import ProjectList from './components/preservation/ProjectList'
import { ArchivedProject } from './types/preservation.types'
import { useTauriCommands } from './hooks/useTauriCommands'

function App() {
  const [projects, setProjects] = useState<ArchivedProject[]>([])
  const [isProcessing, setIsProcessing] = useState(false)
  const { archiveProject, createBagitPackage, getAllProjects } = useTauriCommands()

  // Load projects on app start
  useEffect(() => {
    const loadProjects = async () => {
      try {
        const loadedProjects = await getAllProjects()
        setProjects(loadedProjects)
      } catch (error) {
        console.error('Failed to load projects:', error)
      }
    }

    loadProjects()
  }, [getAllProjects])

  const handleFilesDropped = async (files: Array<{ path: string; name: string }>) => {
    if (files.length === 0) return

    setIsProcessing(true)

    try {
      console.log('Files dropped:', files)

      // Generate a project name from the first file/folder
      const firstFile = files[0]
      const baseName = firstFile.name.replace(/\.[^/.]+$/, "") // Remove extension
      const projectName = `${baseName} Archive`

      // Create archive request
      const request = {
        name: projectName,
        description: `Archived ${files.length} item(s) on ${new Date().toLocaleDateString()}`,
        files: files.map(f => f.path)
      }

      // Archive the project
      const archiveResult = await archiveProject(request)

      if (archiveResult.success && archiveResult.projectId) {
        console.log('Project archived successfully:', archiveResult.projectId)

        // Create BagIt package
        const bagResult = await createBagitPackage(archiveResult.projectId)

        if (bagResult.success) {
          console.log('BagIt package created successfully')

          // Refresh project list
          const updatedProjects = await getAllProjects()
          setProjects(updatedProjects)
        } else {
          console.error('BagIt creation failed:', bagResult.error)
        }
      } else {
        console.error('Archive failed:', archiveResult.error)
      }
    } catch (error) {
      console.error('Error processing files:', error)
    } finally {
      setIsProcessing(false)
    }
  }

  return (
    <div className="h-full bg-gray-50 text-gray-900">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
          <h1 className="text-3xl font-bold">
            Creative Work Preservation Toolkit
          </h1>
          <p className="mt-2 text-gray-600">
            Archive your completed creative projects with professional-grade preservation standards
          </p>
        </div>
      </header>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {/* Drop Zone */}
            <div>
              <h2 className="text-xl font-semibold mb-4">Archive New Project</h2>
              <DropZone
                onFilesDropped={handleFilesDropped}
                isProcessing={isProcessing}
                className="h-64"
              />
            </div>

            {/* Project List */}
            <div>
              <h2 className="text-xl font-semibold mb-4">Archived Projects</h2>
              <ProjectList projects={projects} />
            </div>
          </div>
        </div>
      </main>
    </div>
  )
}

export default App
