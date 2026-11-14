import { createElement, useLayoutEffect, useState } from 'react'
import { PermissionControllerContext } from '../src/context'
import { PermissionAbilityProvider } from '../src/permission'
import { PreviewPermissionController } from '../stories/permission-controller'
import type { Preview } from '@storybook/react-vite'
import { themes } from 'storybook/theming'
import { MINIMAL_VIEWPORTS } from 'storybook/viewport'
import './tailwind.css'
import { PreferenceDarkPalette, PreferenceLightPalette } from '@compallm/contract/validation'
import { preferenceLabel } from '../src/pages/settings/preferences/user/values'
import { ScrollMotion } from '../src/components/scroll'

const preview: Preview = {
  decorators: [
    function ThemePreview(Story, context) {
      const [permissions] = useState(
        () =>
          new PreviewPermissionController((check) => {
            const denied = context.parameters.deniedPermissionActions as string[] | undefined
            return denied?.includes(check.action) ? 'denied' : 'allowed'
          }),
      )
      const theme = String(context.globals.theme ?? 'BASIC_DARK')
      useLayoutEffect(() => {
        const root = document.documentElement
        const previous = root.dataset.theme
        const appearance = root.dataset.appearance
        const dark = root.classList.contains('dark')
        root.dataset.theme = theme.toLowerCase().replaceAll('_', '-')
        root.dataset.appearance = theme.endsWith('_DARK') ? 'dark' : 'light'
        root.classList.toggle('dark', theme.endsWith('_DARK'))
        return () => {
          if (previous === undefined) delete root.dataset.theme
          else root.dataset.theme = previous
          if (appearance === undefined) delete root.dataset.appearance
          else root.dataset.appearance = appearance
          root.classList.toggle('dark', dark)
        }
      }, [theme])
      return createElement(
        'div',
        { className: 'min-h-dvh bg-background font-sans text-foreground' },
        createElement(ScrollMotion),
        createElement(
          PermissionControllerContext,
          { value: permissions },
          createElement(PermissionAbilityProvider, { value: permissions.ability, children: createElement(Story) }),
        ),
      )
    },
  ],
  parameters: {
    layout: 'padded',
    docs: { theme: themes.dark },
    viewport: { options: MINIMAL_VIEWPORTS },
    backgrounds: { options: { canvas: { name: 'Canvas', value: 'var(--background)' } } },
    controls: { matchers: { color: /(background|color)$/i, date: /Date$/i } },
  },
  globalTypes: {
    theme: {
      description: 'Interface palette',
      toolbar: {
        icon: 'paintbrush',
        dynamicTitle: true,
        items: [...PreferenceLightPalette.options, ...PreferenceDarkPalette.options].map((value) => ({
          value,
          title: preferenceLabel(value),
        })),
      },
    },
  },
  initialGlobals: { backgrounds: { value: 'canvas' }, theme: 'BASIC_DARK' },
  tags: ['autodocs'],
}

export default preview
