import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'
import type { StorybookConfig } from '@storybook/react-vite'

const workspace = fileURLToPath(new URL('..', import.meta.url))
const reactDirectory = dirname(fileURLToPath(import.meta.resolve('react/package.json')))
const reactDomDirectory = dirname(fileURLToPath(import.meta.resolve('react-dom/package.json')))

const config: StorybookConfig = {
  stories: ['../stories/**/*.stories.@(ts|tsx)'],
  framework: '@storybook/react-vite',
  addons: ['@storybook/addon-docs', '@storybook/addon-a11y', '@storybook/addon-mcp'],
  core: { disableTelemetry: true },
  features: { componentsManifest: true },
  async viteFinal(config) {
    // Do not inherit portal/vite.config.ts: it belongs to the Rust-backed app.
    const { mergeConfig } = await import('vite')
    return mergeConfig(config, {
      resolve: {
        dedupe: ['react', 'react-dom'],
        alias: {
          'react': reactDirectory,
          'react-dom': reactDomDirectory,
        },
      },
      server: { fs: { allow: [resolve(workspace, '..')] } },
    })
  },
}

export default config
