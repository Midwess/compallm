import { fileURLToPath } from 'node:url'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  base: './',
  build: {
    outDir: 'storybook-static/page-preview',
    rolldownOptions: { input: fileURLToPath(new URL('./route-preview.html', import.meta.url)) },
  },
})
