import { useState } from 'react'
import { DotsSixVerticalIcon } from '@phosphor-icons/react/dist/csr/DotsSixVertical'
import { TrashIcon } from '@phosphor-icons/react/dist/csr/Trash'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { useFieldArray } from 'react-hook-form'
import { Popup } from '../../../components/popup'
import { TextField } from '../../../components/fields'
import { useFormField, FormError } from '../../../components/form'
import { useReorder } from '../../../components/use-reorder'
import { animated } from '../../../components/motion'
import { useController } from './controller'
import { ModelSection } from './model-sections'
import type { HeaderEntry } from './types'

const headerOptions = [
  { key: 'OpenAI-Project', description: 'Project identifier' },
  { key: 'OpenAI-Organization', description: 'Organization identifier' },
  { key: 'X-Client-Request-Id', description: 'Request correlation identifier' },
  { key: 'anthropic-beta', description: 'Model beta features' },
]
const bodyOptions = [
  { key: 'temperature', description: 'Number · Response randomness' },
  { key: 'top_p', description: 'Number · Nucleus sampling' },
  { key: 'max_completion_tokens', description: 'Integer · Output-token budget' },
  { key: 'stop', description: 'String list · Stop sequences' },
  { key: 'seed', description: 'Integer · Reproducible sampling' },
  { key: 'presence_penalty', description: 'Number · Encourage new topics' },
  { key: 'frequency_penalty', description: 'Number · Reduce repetition' },
  { key: 'response_format', description: 'Object · Response structure' },
  { key: 'parallel_tool_calls', description: 'Boolean · Parallel tool calls' },
]

export function ModelRequestFields({ kind }: { kind: 'headers' | 'bodyDefaults' }) {
  const controller = useController()
  const name = `modelDraft.${kind}`
  const field = useFormField<HeaderEntry[]>(name)!
  const { fields } = useFieldArray({ control: controller.store.form.api.control, name, keyName: 'formKey' })
  const entries = fields.map((_, index) => controller.store.form.get(`${name}.${index}`) as HeaderEntry)
  const isHeader = kind === 'headers'
  const options = isHeader ? headerOptions : bodyOptions
  const label = isHeader ? 'header' : 'body field'
  const reorder = useReorder(
    entries.map((entry) => entry.id),
    (order) => field.change(order.flatMap((id) => entries.filter((entry) => entry.id === id))),
    false,
    label,
  )
  const [custom, setCustom] = useState(false)
  const [customKey, setCustomKey] = useState('')
  const [customIndex, setCustomIndex] = useState<number | undefined>()
  const add = (key: string) => {
    if (entries.some((entry) => (isHeader ? entry.key.toLowerCase() === key.toLowerCase() : entry.key === key))) return
    field.change([...entries, { id: crypto.randomUUID(), key, value: '' }])
  }
  const saveCustom = () => {
    if (!customKey) return
    if (customIndex === undefined) add(customKey)
    else controller.store.form.change(`${name}.${customIndex}.key`, customKey)
    setCustom(false)
  }
  const selector = (entry?: HeaderEntry, index?: number) => (
    <Popup.Root>
      <Popup.Trigger
        aria-label={entry ? `Change ${label} ${entry.key}` : `Add ${label}`}
        className={`
flex h-8 w-full min-w-0 justify-between gap-2 rounded-md bg-foreground/10 px-2 text-left text-xsm
hover:bg-foreground/15
`}
      >
        <span className={`truncate ${entry ? '' : 'text-muted-foreground'}`}>{entry?.key ?? `Select ${label}…`}</span>
        <CaretDownIcon className="size-2.5 shrink-0" />
      </Popup.Trigger>
      <Popup.Content align="start" className="w-80" aria-label={`Available ${label}s`}>
        {options.map((option) => (
          <Popup.Item
            key={option.key}
            className="h-auto min-h-10 flex-col items-start py-2"
            disabled={entries.some(
              (item) =>
                item.id !== entry?.id &&
                (isHeader ? item.key.toLowerCase() === option.key.toLowerCase() : item.key === option.key),
            )}
            onSelect={() => (entry ? controller.store.form.change(`${name}.${index}.key`, option.key) : add(option.key))}
          >
            {option.key}
            <span className="text-ui-11 text-muted-foreground">{option.description}</span>
          </Popup.Item>
        ))}
        <Popup.Separator />
        <Popup.Item
          onSelect={() => {
            setCustom(true)
            setCustomKey(entry?.key ?? '')
            setCustomIndex(index)
          }}
        >
          Custom {label}
        </Popup.Item>
      </Popup.Content>
    </Popup.Root>
  )
  return (
    <ModelSection
      title={isHeader ? 'Headers' : 'Body defaults'}
      help={
        isHeader
          ? `
Optional HTTP headers for this model’s requests, such as a project identifier or provider beta flag. The
provider connection supplies the base URL and credential. Choose a known header or add a custom name. Names are
unique ignoring case. Remove a row to remove its override. Header values may contain secrets, so do not share
them.
`
          : `
Optional JSON body fields used when the caller leaves them unset. Explicit request values take precedence.
Choose a known field or add a custom one. Numbers and booleans use JSON syntax; strings need quotes, and arrays
or objects must be valid JSON. Model, messages, input, and stream are managed separately. Remove a row to stop
supplying its default.
`
      }
      description={isHeader ? undefined : 'Used when the request leaves these fields unset.'}
    >
      <div className="grid min-w-0 gap-2 rounded-control bg-inset px-6 py-4" data-form-field={name}>
        <div className="flex min-w-0 items-center gap-4 text-xs text-muted-foreground">
          <span className="w-6 shrink-0">Order</span>
          <div className="grid min-w-0 flex-1 grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)_1.75rem] gap-1 px-1">
            <span className="px-2">Key</span>
            <span className="px-1.5">Value</span>
            <span />
          </div>
        </div>
        {entries.map((entry, index) => (
          <div
            key={entry.id}
            className="relative flex min-w-0 items-center gap-4"
            style={{ transform: `translateY(${reorder.preview?.offsets[entry.id] ?? 0}px)` }}
            ref={(node) => {
              if (node) reorder.rowNodes.current.set(entry.id, node)
              else reorder.rowNodes.current.delete(entry.id)
            }}
          >
            <animated.button
              type="button"
              aria-label={`Reorder ${label} ${index + 1}`}
              aria-describedby={`${reorder.id}-hint`}
              className={`
flex size-6 shrink-0 touch-none cursor-grab items-center justify-center text-muted-foreground
focus-visible:outline-ring
`}
              onPointerDown={(event) => reorder.startDrag(entry.id, event)}
              onPointerMove={reorder.pointerMove}
              onPointerUp={() => reorder.endDrag(true)}
              onPointerCancel={() => reorder.endDrag(false)}
              onKeyDown={(event) => {
                if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
                  event.preventDefault()
                  reorder.keyboardMove(index, index + (event.key === 'ArrowUp' ? -1 : 1))
                }
              }}
            >
              <DotsSixVerticalIcon className="size-3" />
            </animated.button>
            <div
              className={`grid min-w-0 flex-1 grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)_1.75rem]
              items-start gap-1 rounded-lg bg-elevated p-1`}
            >
              <div className="relative min-w-0">
                {selector(entry, index)}
                <FormError name={`${name}.${index}.key`} />
              </div>
              <div className="relative min-w-0">
                {isHeader ? (
                  <TextField
                    name={`${name}.${index}.value`}
                    aria-label={`${entry.key} value`}
                    placeholder="Enter value"
                    className="h-8 rounded-md border-transparent text-xsm"
                    autoComplete="off"
                  />
                ) : (
                  <TextField
                    name={`${name}.${index}.value`}
                    aria-label={`${entry.key} value`}
                    placeholder="JSON value"
                    className="h-8 rounded-md border-transparent text-xsm"
                  />
                )}
                <FormError name={`${name}.${index}.value`} />
              </div>
              <animated.button
                type="button"
                aria-label={`Remove ${entry.key}`}
                className={`
mt-1 flex size-6 items-center justify-center rounded text-muted-foreground hover:bg-hover
focus-visible:outline-ring
`}
                onClick={() => field.change(entries.filter((item) => item.id !== entry.id))}
              >
                <TrashIcon className="size-3" />
              </animated.button>
            </div>
          </div>
        ))}
        <div className="ml-10 grid min-w-0 grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)_1.75rem] gap-1 rounded-lg bg-elevated p-1">
          <div className="min-w-0">{selector()}</div>
        </div>
        {custom ? (
          <div className="ml-10 flex min-w-0 items-center gap-2">
            <TextField
              aria-label={`Custom ${label} name`}
              placeholder={`Enter ${label} name`}
              value={customKey}
              onChange={(event) => setCustomKey(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === 'Enter') {
                  event.preventDefault()
                  if (customKey) {
                    saveCustom()
                  }
                }
                if (event.key === 'Escape') setCustom(false)
              }}
            />
            <animated.button
              type="button"
              className="h-8 rounded-md bg-elevated px-2 text-xs focus-visible:outline-ring"
              onClick={() => {
                if (customKey) {
                  saveCustom()
                }
              }}
            >
              {customIndex === undefined ? 'Add' : 'Apply'}
            </animated.button>
            <animated.button
              type="button"
              aria-label="Cancel custom field"
              className="h-8 px-1 text-xs text-muted-foreground"
              onClick={() => setCustom(false)}
            >
              Cancel
            </animated.button>
          </div>
        ) : null}
        <p id={`${reorder.id}-hint`} className="sr-only">
          Drag to reorder or use Up and Down on a grip.
        </p>
        <span role="status" className="sr-only">
          {reorder.announcement}
        </span>
        <FormError name={name} />
      </div>
    </ModelSection>
  )
}
