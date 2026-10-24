import { QuartzConfig } from "./quartz/cfg"
import * as Plugin from "./quartz/plugins"
import { DevelopmentLinks, FeatureAvailability, PreviewReload } from "./feature-status"

/**
 * Quartz 4 Configuration
 *
 * See https://quartz.jzhao.xyz/configuration for more information.
 */
const config: QuartzConfig = {
  configuration: {
    pageTitle: "Compallm",
    pageTitleSuffix: "",
    enableSPA: true,
    enablePopovers: true,
    analytics: null,
    locale: "en-US",
    ignorePatterns: ["private", "templates", ".obsidian", "releases"],
    defaultDateType: "modified",
    theme: {
      fontOrigin: "local",
      cdnCaching: false,
      typography: {
        header: "Arial",
        body: "Arial",
        code: "Courier New",
      },
      colors: {
        lightMode: {
          light: "#111318",
          lightgray: "#2c323d",
          gray: "#919baa",
          darkgray: "#cbd2dc",
          dark: "#f1f4f8",
          secondary: "#99baff",
          tertiary: "#bad0ff",
          highlight: "rgba(153, 186, 255, 0.10)",
          textHighlight: "#475538",
        },
        darkMode: {
          light: "#111318",
          lightgray: "#2c323d",
          gray: "#919baa",
          darkgray: "#cbd2dc",
          dark: "#f1f4f8",
          secondary: "#99baff",
          tertiary: "#bad0ff",
          highlight: "rgba(153, 186, 255, 0.10)",
          textHighlight: "#475538",
        },
      },
    },
  },
  plugins: {
    transformers: [
      Plugin.FrontMatter(),
      Plugin.CreatedModifiedDate({
        priority: ["frontmatter", "filesystem"],
      }),
      Plugin.SyntaxHighlighting({
        theme: {
          light: "github-light",
          dark: "github-dark",
        },
        keepBackground: false,
      }),
      Plugin.ObsidianFlavoredMarkdown({ enableInHtmlEmbed: false }),
      Plugin.GitHubFlavoredMarkdown(),
      Plugin.TableOfContents(),
      Plugin.CrawlLinks({ markdownLinkResolution: "relative" }),
      DevelopmentLinks(),
      Plugin.Description(),
      PreviewReload(),
    ],
    filters: [Plugin.RemoveDrafts(), FeatureAvailability()],
    emitters: [
      Plugin.AliasRedirects(),
      Plugin.ComponentResources(),
      Plugin.ContentPage(),
      Plugin.FolderPage(),
      Plugin.TagPage(),
      Plugin.ContentIndex({
        enableSiteMap: true,
        enableRSS: true,
      }),
      Plugin.Assets(),
      Plugin.Static(),
      Plugin.Favicon(),
      Plugin.NotFoundPage(),
      // Comment out CustomOgImages to speed up build time
    ],
  },
}

export default config
