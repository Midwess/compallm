import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  build: {
    emptyOutDir: false,
    assetsInlineLimit: 0,
    rolldownOptions: {
      input: 'src/styles.ts',
      output: {
        entryFileNames: 'styles.js',
        assetFileNames: (asset) =>
          asset.names.some((name) => name.endsWith('.css')) ? 'styles.css' : 'assets/[name]-[hash][extname]',
      },
    },
  },
})
