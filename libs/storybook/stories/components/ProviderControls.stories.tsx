import { useEffect, useRef, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { KeyValueEditor, type KeyValueEntry } from '../../src/components/key-value-editor'
import { EffortMapping, type EffortValues } from '../../src/components/effort-mapping'
import { ProviderAvatar } from '../../src/components/provider-avatar'
import { Preview, Sample } from './Preview'

const meta = { title: 'Components/Provider controls', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
function HeaderDemo({ initial = [] }: { initial?: KeyValueEntry[] }) {
  const [value, change] = useState(initial)
  return (
    <KeyValueEditor value={value} onValueChange={change}>
      <KeyValueEditor.Disclosure defaultOpen>Headers</KeyValueEditor.Disclosure>
    </KeyValueEditor>
  )
}
function EffortDemo({ initial = {}, focused }: { initial?: EffortValues; focused?: 'low' }) {
  const [value, change] = useState(initial)
  return <EffortMapping value={value} onValueChange={change} defaultFocused={focused} />
}
function AvatarDemo() {
  const [src, setSrc] = useState<string | null>(null)
  const current = useRef<string | null>(null)
  useEffect(
    () => () => {
      if (current.current) URL.revokeObjectURL(current.current)
    },
    [],
  )
  return (
    <ProviderAvatar
      provider="openai"
      src={src}
      onFileChange={(file) => {
        if (current.current) URL.revokeObjectURL(current.current)
        current.current = file ? URL.createObjectURL(file) : null
        setSrc(current.current)
      }}
    />
  )
}
export const Headers: Story = {
  render: () => (
    <Preview>
      <Sample label="Ordered key/value editing" wide>
        <HeaderDemo
          initial={[
            { id: 'client', key: 'X-Client', value: 'Compallm' },
            { id: 'version', key: 'X-Version', value: '1' },
          ]}
        />
      </Sample>
    </Preview>
  ),
}
export const EmptyHeaders: Story = {
  render: () => (
    <Preview>
      <Sample label="Key-only entry" wide>
        <HeaderDemo />
      </Sample>
    </Preview>
  ),
}
export const Efforts: Story = {
  render: () => (
    <Preview>
      <Sample label="Empty effort names" wide>
        <EffortDemo />
      </Sample>
      <Sample label="Saved names with low editing" wide>
        <EffortDemo initial={{ low: 'low', medium: 'medium', high: 'high' }} focused="low" />
      </Sample>
    </Preview>
  ),
}
export const Avatar: Story = {
  render: () => (
    <Preview>
      <Sample label="Provider avatar">
        <AvatarDemo />
      </Sample>
    </Preview>
  ),
}
