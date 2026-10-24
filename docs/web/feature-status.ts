import type { QuartzFilterPlugin, QuartzTransformerPlugin } from "./quartz/plugins/types"
import { readFileSync, existsSync } from "node:fs"
import path from "node:path"
import matter from "gray-matter"
import { visit } from "unist-util-visit"
import type { Root, Element } from "hast"
import { pathToRoot } from "./quartz/util/path"

export const includeDevelopment = process.env.DOCS_INCLUDE_DEVELOPMENT === "true"
export const isFeature = (slug: string) => slug.startsWith("features/") && !slug.endsWith("/index")
export const isAvailable = (status: unknown) => status === "available"

export const FeatureAvailability: QuartzFilterPlugin = () => ({
  name: "FeatureAvailability",
  shouldPublish(_ctx, [_tree, file]) {
    return includeDevelopment || !isFeature(file.data.slug ?? "") || isAvailable(file.data.frontmatter?.status)
  },
})

export const PreviewReload: QuartzTransformerPlugin = () => ({
  name: "PreviewReload",
  externalResources() {
    if (process.env.DOCS_PREVIEW !== "true") return {}
    return { js: [{ contentType: "inline", loadTime: "afterDOMReady", spaPreserve: true,
      script: `if (!window.__docsReload) { window.__docsReload = new EventSource('/__docs_events'); window.__docsReload.onmessage = () => location.reload(); }`,
    }] }
  },
})

// A general page may discuss unfinished work. Keep those references usable
// without putting development features into the available navigation or index.
export const DevelopmentLinks: QuartzTransformerPlugin = () => ({
  name: "DevelopmentLinks",
  htmlPlugins(ctx) {
    if (includeDevelopment) return []
    return [() => (tree: Root, file) => {
      visit(tree, "element", (node: Element) => {
        const slug = node.properties?.["data-slug"]
        if (node.tagName !== "a" || typeof slug !== "string" || !isFeature(slug)) return
        const source = path.resolve(ctx.argv.directory, `${slug}.md`)
        if (!source.startsWith(path.resolve(ctx.argv.directory) + path.sep) || !existsSync(source)) return
        const status = matter(readFileSync(source, "utf8")).data.status
        if (isAvailable(status)) return
        const anchor = String(node.properties.href ?? "").split("#")[1]
        node.properties.href = `${pathToRoot(file.data.slug!)}/planning/${slug}${anchor ? "#" + anchor : ""}`
        node.properties["data-slug"] = `planning/${slug}`
        node.properties["data-router-ignore"] = true
      })
    }]
  },
})
