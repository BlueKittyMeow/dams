import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

const host = process.env.TAURI_DEV_HOST
const port = Number(process.env.VITE_PORT || process.env.TAURI_DEV_PORT || 1420)
const hmrPort = Number(process.env.VITE_HMR_PORT || port + 1)

export default defineConfig({
  plugins: [react(), tailwindcss()],

  clearScreen: false,
  server: {
    port,
    strictPort: true,
    host: host || false,
    hmr: {
      protocol: host ? 'ws' : undefined,
      host: host || undefined,
      port: hmrPort,
    },
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
})
