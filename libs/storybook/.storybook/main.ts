import graphql from '@compallm/contract/houdini/vite'
import { fileURLToPath } from 'node:url'
import type { StorybookConfig } from '@storybook/svelte-vite'
import tailwindcss from '@tailwindcss/vite'
const config: StorybookConfig = {
  stories: ['../stories/**/*.stories.ts'],
  framework: '@storybook/svelte-vite',
  staticDirs: [{ from: '../../../portal/static/assets/providers', to: '/assets/providers' }],
  addons: ['@storybook/addon-docs', '@storybook/addon-a11y'],
  core: { disableTelemetry: true },
  async viteFinal(config) {
    const { mergeConfig } = await import('vite')
    return mergeConfig(config, {
      plugins: [graphql(), tailwindcss()],
      resolve: {
        alias: { $houdini: fileURLToPath(new URL('../../contract/houdini/generated', import.meta.url)) },
        dedupe: ['svelte'],
      },
    })
  },
}
export default config
