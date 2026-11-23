import type { Preview } from '@storybook/svelte-vite'
import './tailwind.css'
const palettes = ['basic-dark', 'basic-white', 'blueberry-dark', 'blueberry-white', 'marshmallow-dark', 'marshmallow-white']
const preview: Preview = {
  globalTypes: { theme: { toolbar: { title: 'Theme', items: palettes, dynamicTitle: true } } },
  initialGlobals: { theme: 'basic-dark' },
  decorators: [
    (story, context) => {
      const theme = String(context.globals.theme ?? 'basic-dark')
      const root = document.documentElement
      root.dataset.theme = theme
      root.dataset.appearance = theme.endsWith('-dark') ? 'dark' : 'light'
      root.classList.toggle('dark', theme.endsWith('-dark'))
      return story()
    },
  ],
}
export default preview
