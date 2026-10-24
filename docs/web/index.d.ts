declare module "*.scss" {
  const content: string
  export = content
}

interface CustomEventMap {
  prenav: CustomEvent<{}>
  nav: CustomEvent<{ url: import("./quartz/util/path").FullSlug }>
  themechange: CustomEvent<{ theme: "light" | "dark" }>
  readermodechange: CustomEvent<{ mode: "on" | "off" }>
}

type ContentIndex = Record<import("./quartz/util/path").FullSlug, import("./quartz/plugins/emitters/contentIndex").ContentDetails>
declare const fetchData: Promise<ContentIndex>
