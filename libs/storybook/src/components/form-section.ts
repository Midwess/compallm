import Root from './FormSectionRoot.svelte'
import Heading from './FormSectionHeading.svelte'
import Marker from './FormSectionMarker.svelte'
import Content from './SectionSurface.svelte'
export const FormSection: { Root: typeof Root; Heading: typeof Heading; Marker: typeof Marker; Content: typeof Content } = {
  Root,
  Heading,
  Marker,
  Content,
}
