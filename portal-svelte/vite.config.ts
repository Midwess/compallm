import graphql from '@compallm/graphql-svelte/vite'
import { defineConfig, loadEnv } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')
  return {
    plugins: [graphql(), tailwindcss(), sveltekit()],
    resolve: { dedupe: ['svelte'] },
    server: {
      port: 41869,
      strictPort: true,
      proxy: { '/api': { target: env.RUST_API_ORIGIN || 'http://127.0.0.1:3000' } },
      fs: { allow: ['..'] },
    },
  }
})
