import type {
  QuartzFilterPlugin,
  QuartzTransformerPlugin,
} from "./quartz/plugins/types";
import { readFileSync, existsSync } from "node:fs";
import path from "node:path";
import matter from "gray-matter";
import { visit } from "unist-util-visit";
import type { Root, Element } from "hast";
import { pathToRoot } from "./quartz/util/path";

export const includeDevelopment =
  process.env.DOCS_INCLUDE_DEVELOPMENT === "true";
export const isFeature = (slug: string) =>
  /^[^/]+\/adr$/.test(slug) && slug !== "development/adr";
export const isAvailable = (available: unknown) => available === true;

export const PageAvailability: QuartzFilterPlugin = () => ({
  name: "PageAvailability",
  shouldPublish(_ctx, [_tree, file]) {
    return includeDevelopment || isAvailable(file.data.frontmatter?.available);
  },
});

export const PreviewReload: QuartzTransformerPlugin = () => ({
  name: "PreviewReload",
  externalResources() {
    if (process.env.DOCS_PREVIEW !== "true") return {};
    return {
      js: [
        {
          contentType: "inline",
          loadTime: "afterDOMReady",
          spaPreserve: true,
          script: `if (!window.__docsReload) { window.__docsReload = new EventSource('/__docs_events'); window.__docsReload.onmessage = () => location.reload(); }`,
        },
      ],
    };
  },
});

export const DevelopmentLinks: QuartzTransformerPlugin = () => ({
  name: "DevelopmentLinks",
  htmlPlugins(ctx) {
    if (includeDevelopment) return [];
    return [
      () => (tree: Root, file) => {
        visit(tree, "element", (node: Element) => {
          const slug = node.properties?.["data-slug"];
          if (node.tagName !== "a" || typeof slug !== "string") return;
          const source = path.resolve(ctx.argv.directory, `${slug}.md`);
          if (
            !source.startsWith(path.resolve(ctx.argv.directory) + path.sep) ||
            !existsSync(source)
          )
            return;
          const available = matter(readFileSync(source, "utf8")).data.available;
          if (isAvailable(available)) return;
          const anchor = String(node.properties.href ?? "").split("#")[1];
          node.properties.href = `${pathToRoot(file.data.slug!)}/planning/${slug}${anchor ? "#" + anchor : ""}`;
          node.properties["data-slug"] = `planning/${slug}`;
          node.properties["data-router-ignore"] = true;
        });
      },
    ];
  },
});
