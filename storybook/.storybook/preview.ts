import type { Preview } from '@storybook/react-vite'
import { themes } from 'storybook/theming'
import { MINIMAL_VIEWPORTS } from 'storybook/viewport'
import '../../portal/src/components/ui/design-system.css'
import './preview.css'

const preview: Preview = {
  parameters: {
    layout: 'padded',
    docs: { theme: themes.dark },
    viewport: { options: MINIMAL_VIEWPORTS },
    backgrounds: { options: { charcoal: { name: 'Charcoal (provisional)', value: '#191a18' } } },
    controls: { matchers: { color: /(background|color)$/i, date: /Date$/i } },
    options: { storySort: { order: ['Start here', 'Foundations', 'Components', 'Wireframes'] } },
  },
  initialGlobals: { backgrounds: { value: 'charcoal' } },
  tags: ['autodocs'],
}

export default preview
