import { fileURLToPath } from 'node:url'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'

const source = (path: string) => fileURLToPath(new URL(`./src/${path}`, import.meta.url))
const families = [
  'buttons',
  'permission',
  'form',
  'choice-picker',
  'provider-icons',
  'provider-avatar',
  'key-value-editor',
  'effort-mapping',
  'dialog',
  'popup',
  'rich-message',
  'table',
  'form-field',
  'form-section',
  'content',
  'code-block',
  'data',
  'dropdowns',
  'feedback',
  'fields',
  'forms',
  'navigation',
  'page-header',
  'page-progress',
  'app-loading-overlay',
  'sidebar',
  'scroll',
  'selection',
  'surfaces',
  'settings',
]

export default defineConfig({
  plugins: [react(), tailwindcss()],
  build: {
    minify: true,
    cssMinify: true,
    sourcemap: false,
    target: 'es2022',
    lib: {
      entry: {
        index: source('index.ts'),
        context: source('context.ts'),
        styles: source('styles.ts'),
        'pages/signup': source('pages/signup/index.tsx'),
        'pages/signin': source('pages/signin/index.tsx'),
        'pages/main': source('pages/main/index.tsx'),
        'pages/settings': source('pages/settings/index.tsx'),
        'pages/organization': source('pages/organization/index.tsx'),
        'pages/organization/chat': source('pages/organization/chat/index.tsx'),
        'pages/organization/create-provider': source('pages/organization/create-provider/index.tsx'),
        ...Object.fromEntries(families.map((name) => [`components/${name}`, source(`components/${name}.tsx`)])),
      },
      formats: ['es'],
      fileName: (_format, name) => `${name}.js`,
      cssFileName: 'styles',
    },
    rolldownOptions: {
      treeshake: true,
      external: (id) =>
        [
          '@compallm/contract',
          '@bprogress/core',
          'react',
          'react-dom',
          '@base-ui/react',
          '@phosphor-icons/react',
          'tailwind-merge',
          'zustand',
          'react-hook-form',
          'use-sync-external-store',
          '@hookform/resolvers',
          'lodash-es',
          'motion',
        ].some((name) => id === name || id.startsWith(`${name}/`)),
      output: { minify: true },
    },
  },
})
