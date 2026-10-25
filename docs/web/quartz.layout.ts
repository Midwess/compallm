import { PageLayout, SharedLayout } from "./quartz/cfg";
import * as Component from "./quartz/components";
import type { FileTrieNode } from "./quartz/util/fileTrie";
import Availability, { Brand } from "./Availability";

const sortContentTree = (a: FileTrieNode, b: FileTrieNode) => {
  if (a.isFolder !== b.isFolder) return a.isFolder ? -1 : 1;

  const aOrder = a.data?.order;
  const bOrder = b.data?.order;
  if (aOrder !== undefined || bOrder !== undefined) {
    if (aOrder === undefined) return 1;
    if (bOrder === undefined) return -1;
    if (aOrder !== bOrder) return aOrder - bOrder;
  }

  return a.displayName.localeCompare(b.displayName, undefined, {
    numeric: true,
    sensitivity: "base",
  });
};

// components shared across all pages
export const sharedPageComponents: SharedLayout = {
  head: Component.Head(),
  header: [],
  afterBody: [],
  footer: Component.Footer({
    links: {
      GitHub: "https://github.com/Midwess/compallm",
    },
  }),
};

// components for pages that display a single page (e.g. a single note)
export const defaultContentPageLayout: PageLayout = {
  beforeBody: [
    Availability(),
    Component.ConditionalRender({
      component: Component.Breadcrumbs(),
      condition: (page) => page.fileData.slug !== "index",
    }),
    Component.ArticleTitle(),
    Component.ContentMeta(),
    Component.TagList(),
  ],
  left: [
    Brand(),
    Component.MobileOnly(Component.Spacer()),
    Component.Flex({
      components: [
        {
          Component: Component.Search(),
          grow: true,
        },
        { Component: Component.ReaderMode() },
      ],
    }),
    Component.Explorer({
      title: "Content",
      alwaysVisible: true,
      folderDefaultState: "open",
      useSavedState: false,
      sortFn: sortContentTree,
    }),
  ],
  right: [
    Component.Graph(),
    Component.DesktopOnly(Component.TableOfContents()),
    Component.Backlinks(),
  ],
};

// components for pages that display lists of pages  (e.g. tags or folders)
export const defaultListPageLayout: PageLayout = {
  beforeBody: [
    Availability(),
    Component.Breadcrumbs(),
    Component.ArticleTitle(),
    Component.ContentMeta(),
  ],
  left: [
    Brand(),
    Component.MobileOnly(Component.Spacer()),
    Component.Flex({
      components: [
        {
          Component: Component.Search(),
          grow: true,
        },
      ],
    }),
    Component.Explorer({
      title: "Content",
      alwaysVisible: true,
      folderDefaultState: "open",
      useSavedState: false,
      sortFn: sortContentTree,
    }),
  ],
  right: [],
};
