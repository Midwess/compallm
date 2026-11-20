import { useEffect, useRef, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { KeyValueEditor, type KeyValueEntry } from '../../src/components/key-value-editor'
import { EffortMapping, type EffortValues } from '../../src/components/effort-mapping'
import { ProviderAvatar } from '../../src/components/provider-avatar'
import { RangeSlider } from '../../src/components/range-slider'
import { Preview, Sample } from './Preview'

const meta = { title: 'Components/Provider controls', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
function HeaderDemo({ initial = [] }: { initial?: KeyValueEntry[] }) {
  const [value, change] = useState(initial)
  return (
    <KeyValueEditor.Root value={value} onValueChange={change}>
      <KeyValueEditor.Disclosure defaultOpen>Headers</KeyValueEditor.Disclosure>
    </KeyValueEditor.Root>
  )
}
function EffortDemo({
  initial = {},
  focused,
  compact = false,
}: {
  initial?: EffortValues
  focused?: 'low'
  compact?: boolean
}) {
  const [value, change] = useState(initial)
  return (
    <EffortMapping value={value} onValueChange={change} defaultFocused={focused} variant={compact ? 'compact' : 'default'} />
  )
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
export const CompactEfforts: Story = {
  render: () => (
    <Preview>
      <Sample label="Compact efforts with growing glow" wide>
        <EffortDemo compact initial={{ low: 'low', medium: 'medium', high: 'high', xhigh: 'xhigh', max: 'max' }} />
      </Sample>
    </Preview>
  ),
}

function TokenRulerDemo() {
  const [value, change] = useState(262144)
  return (
    <div className="grid gap-2">
      <output className="text-right text-xs text-muted-foreground">{value.toLocaleString()} tokens</output>
      <RangeSlider
        label="Context capacity"
        min={100}
        max={1000000}
        value={value}
        onValueChange={change}
        className="text-muted-foreground"
      />
    </div>
  )
}
export const TokenCapacityRuler: Story = {
  render: () => (
    <Preview>
      <Sample label="Token ruler with tick feedback" wide>
        <TokenRulerDemo />
      </Sample>
    </Preview>
  ),
}
