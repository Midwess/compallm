export default {
  url: '/api/graphql',
  schemaPath: 'libs/contract/schema.graphql',
  include: ['portal-svelte/src/**/*', 'libs/storybook-svelte/src/**/*'],
  runtimeDir: '.houdini',
  watchSchema: false,
  scalars: { ID: { type: 'string' }, ContextTokens: { type: 'number' } },
  plugins: { 'houdini-svelte': { framework: 'svelte', client: './libs/graphql-svelte/src/client', static: true } },
}
