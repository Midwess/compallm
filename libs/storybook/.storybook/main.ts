import { fileURLToPath } from 'node:url'
import { readFile } from 'node:fs/promises'
import type { StorybookConfig } from '@storybook/react-vite'
import type { Plugin } from 'vite'
import tailwindcss from '@tailwindcss/vite'

const workspace = fileURLToPath(new URL('..', import.meta.url))

const config: StorybookConfig = {
  stories: ['../stories/**/*.stories.@(ts|tsx)'],
  framework: '@storybook/react-vite',
  addons: ['@storybook/addon-docs', '@storybook/addon-a11y', '@storybook/addon-mcp'],
  core: { disableTelemetry: true },
  features: { componentsManifest: true },
  async viteFinal(config) {
    // The library preview has no dependency on the portal application.
    const { mergeConfig } = await import('vite')
    return mergeConfig(config, {
      plugins: [
        tailwindcss(),
        {
          name: 'route-page-previews',
          configureServer(server) {
            server.middlewares.use(async (request, response, next) => {
              const pathname = new URL(request.url ?? '/', 'http://localhost').pathname.replace(/\/$/, '')
              if (pathname !== '/signup' && pathname !== '/signin') return next()
              try {
                const source = await readFile(new URL('../route-preview.html', import.meta.url), 'utf8')
                const html = await server.transformIndexHtml(pathname, source)
                response.setHeader('Content-Type', 'text/html')
                response.end(html)
              } catch (error) {
                next(error)
              }
            })
          },
        } satisfies Plugin,
      ],
      resolve: {
        dedupe: ['react', 'react-dom'],
      },
      server: { fs: { allow: [workspace] } },
    })
  },
}

export default config
