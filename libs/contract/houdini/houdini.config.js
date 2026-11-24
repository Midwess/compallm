export default {
  url: '/api/graphql',
  schemaPath: 'libs/contract/schema.graphql',
  include: ['libs/contract/houdini/operations/**/*', 'portal/src/**/*', 'libs/storybook/src/**/*'],
  runtimeDir: 'generated',
  watchSchema: false,
  scalars: { ID: { type: 'string' }, ContextTokens: { type: 'number' } },
  plugins: { 'houdini-svelte': { framework: 'svelte', client: './libs/contract/houdini/client', static: true } },
}
