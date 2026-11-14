import { useState } from 'react'
import { motion } from 'motion/react'
import { Dialog } from '../../../../components/dialog'
import { Form, FormError } from '../../../../components/form'
import { Select } from '../../../../components/dropdowns'
import { useTransition } from '../../../../components/motion'
import { DotsSixVerticalIcon } from '@phosphor-icons/react/dist/csr/DotsSixVertical'
import { useSettingsController } from '../../controller'
import { useUserPreference, useUserPreferenceController } from './controller'
import { preferenceLabel, preferenceOptions, type PreferenceDraft } from './values'
import type { PreferenceDestination, PreferenceSidebarCategory } from '@compallm/contract/graphql'

type Layout = PreferenceDraft['sidebarLayout']
type Drag = { category: PreferenceSidebarCategory; item?: PreferenceDestination }

export function CustomizeSidebar() {
  const settings = useSettingsController()
  const controller = useUserPreferenceController()
  const layout = useUserPreference((state) => state.draft.sidebarLayout, ['draft.sidebarLayout'])
  const timing = useTransition('reorder')
  const [drag, setDrag] = useState<Drag | null>(null)
  const [target, setTarget] = useState<string | null>(null)
  const [announcement, announce] = useState('')
  const categories = layout.map(({ category }) => ({ value: category, label: preferenceLabel(category) }))
  const save = (next: Layout) => controller.store.form.change('draft.sidebarLayout', next)
  const moveCategory = (from: PreferenceSidebarCategory, to: PreferenceSidebarCategory) => {
    if (from === to) return
    const next = [...layout]
    const source = next.findIndex((section) => section.category === from)
    const destination = next.findIndex((section) => section.category === to)
    const [section] = next.splice(source, 1)
    next.splice(destination, 0, section)
    save(next)
    announce(`${preferenceLabel(from)} category moved to position ${destination + 1}`)
  }
  const moveItem = (item: PreferenceDestination, category: PreferenceSidebarCategory, before?: PreferenceDestination) => {
    if (item === before) return
    const next = layout.map((section) => ({ ...section, items: section.items.filter((entry) => entry !== item) }))
    const section = next.find((section) => section.category === category)!
    const index = before ? section.items.indexOf(before) : section.items.length
    section.items.splice(index < 0 ? section.items.length : index, 0, item)
    save(next)
    announce(`${preferenceLabel(item)} moved to ${preferenceLabel(category)}`)
  }
  const drop = (category: PreferenceSidebarCategory, before?: PreferenceDestination) => {
    if (drag?.item) moveItem(drag.item, category, before)
    else if (drag) moveCategory(drag.category, category)
    setDrag(null)
    setTarget(null)
  }
  return (
    <Dialog.Root
      open
      onOpenChange={(open) => {
        if (!open) settings.pages.dialogs.closeDialog()
      }}
    >
      <Dialog.Content className="w-120 rounded-xl bg-inset">
        <Form.Root model={controller.store.form}>
          <Dialog.Header className="h-14 px-6">
            <Dialog.Title>Customize sidebar</Dialog.Title>
            <Dialog.Close />
          </Dialog.Header>
          <Dialog.Body className="gap-4 px-6 pt-4 pb-6">
            <div className="flex items-center justify-between rounded-lg bg-elevated px-3 py-2 text-xsm">
              <span>Default badge style</span>
              <Select.Control
                aria-label="Default badge style"
                className="w-26"
                name="draft.sidebarBadgeStyle"
                options={preferenceOptions.sidebarBadgeStyle}
              />
            </div>
            <p className="text-xs text-muted-foreground">
              Drag categories to reorder. Drag items between categories, or choose a category from the menu.
            </p>
            <div className="grid gap-4">
              {layout.map((section, index) => (
                <motion.div
                  key={section.category}
                  layout
                  transition={timing}
                  className={
                    target === section.category && drag && !drag.item
                      ? 'grid gap-2 rounded-lg bg-elevated p-2 ring-1 ring-primary'
                      : 'grid gap-2 rounded-lg bg-elevated p-2'
                  }
                  onDragOver={(event) => {
                    if (drag) {
                      event.preventDefault()
                      setTarget(section.category)
                    }
                  }}
                  onDrop={(event) => {
                    event.preventDefault()
                    event.stopPropagation()
                    drop(section.category)
                  }}
                >
                  <div
                    draggable
                    className="flex cursor-grab items-center gap-2 active:cursor-grabbing"
                    onDragStart={(event) => {
                      setDrag({ category: section.category })
                      event.dataTransfer.effectAllowed = 'move'
                      event.dataTransfer.setData('text/plain', section.category)
                    }}
                    onDragEnd={() => {
                      setDrag(null)
                      setTarget(null)
                    }}
                    onDragOver={(event) => {
                      if (drag) {
                        event.preventDefault()
                        setTarget(section.category)
                      }
                    }}
                    onDrop={(event) => {
                      event.preventDefault()
                      event.stopPropagation()
                      drop(section.category)
                    }}
                  >
                    <button
                      type="button"
                      aria-label={`Reorder ${preferenceLabel(section.category)} category`}
                      className="flex size-6 cursor-grab items-center justify-center
text-muted-foreground focus-visible:outline-ring"
                      onKeyDown={(event) => {
                        if (event.key !== 'ArrowUp' && event.key !== 'ArrowDown') return
                        event.preventDefault()
                        const destination = layout[index + (event.key === 'ArrowUp' ? -1 : 1)]
                        if (destination) moveCategory(section.category, destination.category)
                      }}
                    >
                      <DotsSixVerticalIcon aria-hidden="true" className="size-3" />
                    </button>
                    <h3 className="text-xsm font-medium">{preferenceLabel(section.category)}</h3>
                  </div>
                  <div
                    className={target === section.category ? 'min-h-9 rounded ring-1 ring-primary' : 'min-h-9 rounded'}
                    onDragOver={(event) => {
                      if (drag) {
                        event.preventDefault()
                        setTarget(section.category)
                      }
                    }}
                    onDrop={(event) => {
                      event.preventDefault()
                      event.stopPropagation()
                      drop(section.category)
                    }}
                  >
                    {section.items.map((item, itemIndex) => (
                      <motion.div
                        key={item}
                        layout
                        transition={timing}
                        className={
                          target === item
                            ? 'flex min-h-10 items-center gap-2 border-t border-primary px-1'
                            : 'flex min-h-10 items-center gap-2 px-1'
                        }
                        onDragOver={(event) => {
                          if (drag) {
                            event.preventDefault()
                            event.stopPropagation()
                            setTarget(item)
                          }
                        }}
                        onDrop={(event) => {
                          event.preventDefault()
                          event.stopPropagation()
                          drop(section.category, item)
                        }}
                      >
                        <button
                          type="button"
                          draggable
                          aria-label={`Reorder ${preferenceLabel(item)}`}
                          className="flex size-6 shrink-0 cursor-grab items-center justify-center
text-muted-foreground focus-visible:outline-ring"
                          onDragStart={(event) => {
                            event.stopPropagation()
                            setDrag({ category: section.category, item })
                            event.dataTransfer.effectAllowed = 'move'
                            event.dataTransfer.setData('text/plain', item)
                          }}
                          onDragEnd={() => {
                            setDrag(null)
                            setTarget(null)
                          }}
                          onKeyDown={(event) => {
                            if (event.key !== 'ArrowUp' && event.key !== 'ArrowDown') return
                            event.preventDefault()
                            const direction = event.key === 'ArrowUp' ? -1 : 1
                            const destination = section.items[itemIndex + direction]
                            if (destination) {
                              const next = layout.map((group) => ({ ...group, items: [...group.items] }))
                              const items = next[index].items
                              ;[items[itemIndex], items[itemIndex + direction]] = [
                                items[itemIndex + direction],
                                items[itemIndex],
                              ]
                              save(next)
                            }
                          }}
                        >
                          <DotsSixVerticalIcon aria-hidden="true" className="size-3" />
                        </button>
                        <span className="min-w-0 flex-1 text-xs">{preferenceLabel(item)}</span>
                        <Select.Control
                          aria-label={`${preferenceLabel(item)} category`}
                          className="w-29"
                          value={section.category}
                          options={categories}
                          onValueChange={(category) => moveItem(item, category as PreferenceSidebarCategory)}
                        />
                        <Select.Control
                          aria-label={`${preferenceLabel(item)} visibility`}
                          className="w-29"
                          name={`draft.sidebar${item}`}
                          options={preferenceOptions.sidebarVisibility}
                        />
                        <FormError name={`draft.sidebar${item}`} />
                      </motion.div>
                    ))}
                    {!section.items.length ? <p className="px-2 py-3 text-xs text-muted-foreground">Drop items here</p> : null}
                  </div>
                </motion.div>
              ))}
            </div>
            <FormError name="draft.sidebarLayout" />
            <FormError name="draft.sidebarBadgeStyle" />
            <span role="status" className="sr-only">
              {announcement}
            </span>
          </Dialog.Body>
        </Form.Root>
      </Dialog.Content>
    </Dialog.Root>
  )
}
