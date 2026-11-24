import LabeledFieldPreview from './Forms-LabeledField.svelte'
import FormFieldLabelPreview from './Forms-FormFieldLabel.svelte'
import FormStepConnectorPreview from './Forms-FormStepConnector.svelte'
import FormStepHeadingPreview from './Forms-FormStepHeading.svelte'
import FormStepMarkerPreview from './Forms-FormStepMarker.svelte'
import FormIntroductionPreview from './Forms-FormIntroduction.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Form primitives',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const FormIntroduction = { render: () => ({ Component: FormIntroductionPreview, props: {} }) }
export const FormStepMarker = { render: () => ({ Component: FormStepMarkerPreview, props: {} }) }
export const FormStepHeading = { render: () => ({ Component: FormStepHeadingPreview, props: {} }) }
export const FormStepConnector = { render: () => ({ Component: FormStepConnectorPreview, props: {} }) }
export const FormFieldLabel = { render: () => ({ Component: FormFieldLabelPreview, props: {} }) }
export const LabeledField = { render: () => ({ Component: LabeledFieldPreview, props: {} }) }
