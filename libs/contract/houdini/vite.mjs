import transform from 'houdini-svelte/vite/transform'
import { fileURLToPath } from 'node:url'
import { loadConfig } from './load-config.mjs'

/** @returns {Promise<import('vite').Plugin>} */
export default async function graphql() {
  const config = await loadConfig()
  return {
    name: 'compallm-houdini-svelte',
    enforce: 'pre',
    config: () => ({ resolve: { alias: { $houdini: fileURLToPath(new URL('./generated', import.meta.url)) } } }),
    async transform(content, filepath) {
      if (!config.includeFile(filepath)) return
      const result = await transform('svelte', {
        config,
        content,
        filepath,
        watch_file: this.addWatchFile.bind(this),
        map: this.getCombinedSourcemap(),
      })
      return {
        code: result.code,
        map: typeof result.map === 'string' ? result.map : result.map ? JSON.stringify(result.map) : null,
      }
    },
  }
}
