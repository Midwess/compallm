import adapter from '@sveltejs/adapter-node'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import graphql from '@compallm/contract/houdini/vite'
import { defineConfig, loadEnv } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')
  return {
    plugins: [
      graphql(),
      tailwindcss(),
      sveltekit({
        preprocess: vitePreprocess(),
        compilerOptions: { experimental: { async: true } },
        experimental: { forkPreloads: true },
        serviceWorker: { register: false },
        adapter: adapter({ out: 'build' }),
      }),
    ],
    resolve: { dedupe: ['svelte'] },
    optimizeDeps: { exclude: ['@cedar-policy/cedar-wasm'] },
    server: {
      port: Number(env.FRONTEND_PORT || 4868),
      strictPort: true,
      proxy: { '/api': { target: env.RUST_API_ORIGIN || 'http://127.0.0.1:3000' } },
      fs: { allow: ['..'] },
    },
    cacheDir: 'node_modules/.vite-svelte-portal',
  }
})
