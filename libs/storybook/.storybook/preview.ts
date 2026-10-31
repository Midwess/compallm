import { createElement } from 'react'
import type { Preview } from '@storybook/react-vite'
import { themes } from 'storybook/theming'
import { MINIMAL_VIEWPORTS } from 'storybook/viewport'
import './tailwind.css'

const preview: Preview = {
  decorators: [
    (Story) => createElement('div', { className: 'bg-background font-sans text-text scheme-dark' }, createElement(Story)),
  ],
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
