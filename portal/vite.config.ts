import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { fileURLToPath } from 'node:url'
import { defaultClientConditions, defineConfig, searchForWorkspaceRoot } from 'vite'

// https://vite.dev/config/
export default defineConfig(({ command }) => ({
  plugins: [react(), ...(command === 'serve' ? [tailwindcss()] : [])],
  resolve: {
    dedupe: ['react', 'react-dom'],
    conditions: command === 'serve' ? ['compallm-source', ...defaultClientConditions] : [...defaultClientConditions],
  },
  optimizeDeps: {
    exclude: ['@compallm/contract'],
    include: ['@compallm/contract > zod'],
  },
  server: {
    fs: {
      allow: [
        searchForWorkspaceRoot(process.cwd()),
        fileURLToPath(new URL('../libs/storybook', import.meta.url)),
        fileURLToPath(new URL('../libs/contract', import.meta.url)),
      ],
    },
    port: 41868,
    strictPort: true,
  },
}))
