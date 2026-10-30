import type { Preview } from '@storybook/react-vite'
import { themes } from 'storybook/theming'
import { MINIMAL_VIEWPORTS } from 'storybook/viewport'
import '../../portal/src/styles/tailwind.css'
import './preview.css'

const preview: Preview = {
  parameters: {
    layout: 'padded',
    docs: { theme: themes.dark },
    viewport: { options: MINIMAL_VIEWPORTS },
    backgrounds: { options: { canvas: { name: 'Canvas', value: 'var(--color-background)' } } },
    controls: { matchers: { color: /(background|color)$/i, date: /Date$/i } },
  },
  initialGlobals: { backgrounds: { value: 'canvas' } },
  tags: ['autodocs'],
}

export default preview
