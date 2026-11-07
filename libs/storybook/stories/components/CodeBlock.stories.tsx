import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { CodePreview, CodeEditor } from '../../src/components/code-block'
import { initialSettings, modelOperatorSource } from '../pages/settings/fixtures'

const meta = { title: 'Components/Code block', parameters: { layout: 'padded' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Reference: Story = {
  render: () => (
    <div className="max-w-156">
      <CodePreview language="typescript" {...initialSettings.codeExamples!.typescript} />
    </div>
  ),
}
function Editable() {
  const [value, setValue] = useState(modelOperatorSource)
  return (
    <div className="flex h-80 max-w-208">
      <CodeEditor label="Editable Cedar" language="cedar" value={value} onValueChange={setValue} />
    </div>
  )
}
export const EditableCedar: Story = { render: () => <Editable /> }
export const Light: Story = {
  render: () => (
    <div className="max-w-156">
      <CodePreview theme="light" language="typescript" {...initialSettings.codeExamples!.typescript} />
    </div>
  ),
}
export const PlainFallback: Story = {
  render: () => <CodePreview code={'<script>never execute displayed code</script>\n<&>'} language="unknown language" />,
}
