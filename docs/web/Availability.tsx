/* eslint-disable react/only-export-components -- Quartz component factories. */
import { includeDevelopment, isAvailable, isFeature } from "./feature-status";
import { pathToRoot } from "./quartz/util/path";
import type { FullSlug } from "./quartz/util/path";
import type { QuartzComponent } from "./quartz/components/types";
// @ts-ignore Quartz bundles browser scripts.
import script from "./availability.inline";
import style from "./availability.scss";

export function viewLink(
  slug: FullSlug,
  target: string,
  planning: boolean,
): string {
  return `${pathToRoot(slug)}/${includeDevelopment ? "../" : ""}${planning ? "planning/" : ""}${target === "index" ? "" : target}`;
}

export function Brand() {
  const BrandHeader: QuartzComponent = ({ fileData, cfg }) => {
    const slug = fileData.slug!;
    const target = isAvailable(fileData.frontmatter?.available)
      ? slug
      : "index";
    return (
      <div class="docs-brand">
        <a class="docs-brand-title" href={pathToRoot(slug)}>
          {cfg.pageTitle}
        </a>
        <select aria-label="Page filter" class="feature-filter">
          <option
            value={viewLink(slug, target, false)}
            selected={!includeDevelopment}
          >
            Available
          </option>
          <option
            value={viewLink(slug, slug, true)}
            selected={includeDevelopment}
          >
            All
          </option>
        </select>
      </div>
    );
  };
  BrandHeader.afterDOMLoaded = script;
  BrandHeader.css = style;
  return BrandHeader;
}

const Availability: QuartzComponent = ({ fileData, allFiles }) => {
  const slug = fileData.slug!;
  const available = isAvailable(fileData.frontmatter?.available);
  const catalog = ["index", "product", "features/index"].includes(slug);
  const features = allFiles
    .filter((file) => isFeature(file.slug ?? ""))
    .sort((a, b) =>
      String(a.frontmatter?.title).localeCompare(String(b.frontmatter?.title)),
    );
  return (
    <section class="availability" aria-label="Page availability">
      <p class={`feature-status ${available ? "available" : "development"}`}>
        {available ? "Available" : "Under development"}
      </p>
      {catalog && (
        <div class="feature-catalog">
          <h2>Features</h2>
          {features.length === 0 ? (
            <p class="empty-features">
              No features have been marked available yet. Choose “All” beside
              Compallm to explore the product plan.
            </p>
          ) : (
            <ul>
              {features.map((file) => (
                <li key={file.slug}>
                  <a href={`${pathToRoot(slug)}/${file.slug}`}>
                    {file.frontmatter?.title}
                  </a>
                  <span
                    class={`feature-status ${isAvailable(file.frontmatter?.available) ? "available" : "development"}`}
                  >
                    {isAvailable(file.frontmatter?.available)
                      ? "Available"
                      : "Under development"}
                  </span>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </section>
  );
};

Availability.css = style;
export default function createAvailability() {
  return Availability;
}
