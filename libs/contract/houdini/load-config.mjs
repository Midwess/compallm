import { fileURLToPath } from 'node:url'
import { readFile } from 'node:fs/promises'
import { buildSchema } from 'graphql'
import { get_config } from 'houdini'

export async function loadConfig() {
  const config = await get_config({
    config_path: fileURLToPath(new URL('./houdini.config.js', import.meta.url)),
    skip_schema: true,
  })
  config.root_dir = fileURLToPath(new URL('../../../', import.meta.url)).replace(/\/$/, '')
  config.config_file.runtimeDir = 'libs/contract/houdini/generated'
  config.schema = buildSchema(await readFile(new URL('../schema.graphql', import.meta.url), 'utf8'))
  return config
}
