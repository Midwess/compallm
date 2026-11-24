import { mkdir, rm } from 'node:fs/promises'
import { init_db, codegen_setup } from 'houdini'
import { loadConfig } from './load-config.mjs'

const config = await loadConfig()
await rm(new URL('./generated', import.meta.url), { recursive: true, force: true })
await mkdir(new URL('./generated', import.meta.url), { recursive: true })
const [db, dbFile] = await init_db(config, false)
const compiler = await codegen_setup(config, 'development', db, dbFile)
try {
  await compiler.run_pipeline({ after: 'Schema' })
  console.log('Houdini artifacts generated from Rust schema')
} finally {
  await compiler.close()
}
