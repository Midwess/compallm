import type { Preview } from '@storybook/svelte-vite'
import './tailwind.css'
import PreviewFrame from './PreviewFrame.svelte'
const palettes = ['basic-dark', 'basic-white', 'blueberry-dark', 'blueberry-white', 'marshmallow-dark', 'marshmallow-white']
const preview: Preview = {
  globalTypes: { theme: { toolbar: { title: 'Theme', items: palettes, dynamicTitle: true } } },
  initialGlobals: { theme: 'basic-dark' },
  parameters: { layout: 'padded', backgrounds: { options: { canvas: { name: 'Canvas', value: 'var(--background)' } } } },
  decorators: [
    (story, context) => {
      const theme = String(context.globals.theme ?? 'basic-dark')
      const root = document.documentElement
      root.dataset.theme = theme
      root.dataset.appearance = theme.endsWith('-dark') ? 'dark' : 'light'
      root.classList.toggle('dark', theme.endsWith('-dark'))
      story()
      return { Component: PreviewFrame, props: { deniedActions: context.parameters.deniedPermissionActions ?? [] } }
    },
  ],
}
export default preview
