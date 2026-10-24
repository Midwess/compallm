import { spawn } from "node:child_process"
import { createServer } from "node:http"
import { rmSync } from "node:fs"
import path from "node:path"
import { fileURLToPath } from "node:url"
import serveHandler from "serve-handler"
import chokidar from "chokidar"

const root = fileURLToPath(new URL(".", import.meta.url))
const serving = process.argv.includes("--serve")
const portIndex = process.argv.indexOf("--port")
const port = portIndex < 0 ? 8080 : Number(process.argv[portIndex + 1])
if (!Number.isInteger(port) || port < 1 || port > 65535) throw new Error("Invalid preview port")
let active
let watcher
let server
const clients = new Set()

async function build(includeDevelopment) {
  await new Promise((resolve, reject) => {
    active = spawn(process.execPath, [
      "quartz/bootstrap-cli.mjs", "build", "--directory", "../content",
      "--output", includeDevelopment ? "public/planning" : "public",
    ], {
      cwd: root,
      env: { ...process.env, DOCS_INCLUDE_DEVELOPMENT: String(includeDevelopment), DOCS_PREVIEW: String(serving) },
      stdio: "inherit",
    })
    active.on("error", reject)
    active.on("exit", (code, signal) => {
      if (signal === "SIGINT" || signal === "SIGTERM") process.exit(0)
      if (code !== 0) reject(new Error(`Quartz failed (${code})`))
      else resolve()
    })
  })
}

let building
async function rebuild() {
  if (building) return building
  building = (async () => {
    await build(false)
    await build(true)
    for (const response of clients) response.write("data: rebuilt\n\n")
  })()
  try { await building } finally { building = undefined }
}

// Only generated output is replaced. Feature sources remain in ../content.
rmSync(path.join(root, "public"), { recursive: true, force: true })
await rebuild()
if (serving) {
  server = createServer((request, response) => {
    if (request.url === "/__docs_events") {
      response.writeHead(200, {
        "Content-Type": "text/event-stream",
        "Cache-Control": "no-cache",
        Connection: "keep-alive",
      })
      response.write(": connected\n\n")
      clients.add(response)
      request.on("close", () => clients.delete(response))
      return
    }
    serveHandler(request, response, { public: path.join(root, "public"), cleanUrls: true })
      .catch((error) => { console.error(error); response.end() })
  })
  await new Promise((resolve, reject) => {
    server.once("error", reject)
    server.listen(port, "127.0.0.1", resolve)
  })
  console.log(`Documentation preview: http://localhost:${port}`)
  let timer
  watcher = chokidar.watch([
    "../content", "quartz.config.ts", "quartz.layout.ts", "Availability.tsx",
    "feature-status.ts", "availability.inline.ts", "availability.scss", "quartz",
  ], { cwd: root, ignoreInitial: true, ignored: /node_modules|\.quartz-cache|tsconfig\.tsbuildinfo/ })
  watcher.on("all", () => {
    clearTimeout(timer)
    timer = setTimeout(() => {
      // Schedule another pass if a save happens during an existing build.
      const pending = building
      Promise.resolve(pending).catch(() => {}).then(rebuild).catch(console.error)
    }, 150)
  })
}

for (const signal of ["SIGINT", "SIGTERM"]) {
  process.on(signal, async () => {
    active?.kill(signal)
    await watcher?.close()
    for (const response of clients) response.end()
    server?.close()
    process.exit(0)
  })
}
