import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
export default {
  preprocess: vitePreprocess(),
  kit: {
    alias: { $houdini: '../libs/graphql-svelte/.houdini', '$houdini/*': '../libs/graphql-svelte/.houdini/*' },
    adapter: adapter({ pages: 'dist', assets: 'dist', fallback: 'index.html' }),
  },
}
