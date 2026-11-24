<script lang="ts">
  import { Dialog } from '../../../../components/dialog'
  import { Form, FormError } from '../../../../components/form'
  import { Select } from '../../../../components/dropdowns'
  import { useTransition } from '../../../../components/motion'
  import DotsSixVerticalIcon from 'phosphor-svelte/lib/DotsSixVerticalIcon'
  import { preferenceLabel, preferenceOptions } from './values'
  import type {
    PreferenceDestination$options as PreferenceDestination,
    PreferenceSidebarCategory$options as PreferenceSidebarCategory,
  } from '@compallm/contract/houdini'
  import { untrack } from 'svelte'
  import { readStore } from '../../../page-state.svelte'
  import { getSettingsController, getUserPreferenceController } from '../../../../context'
  import type { Layout, Drag } from './sidebar-layout'

  import MotionSurface from '../../../../components/MotionSurface.svelte'

  const __preferences = getUserPreferenceController()
  const settings = untrack(() => getSettingsController())
  const controller = untrack(() => getUserPreferenceController())
  let layout = $derived(readStore(__preferences.store, (state) => state.draft.sidebarLayout, ['draft.sidebarLayout']))
  let timing = $derived(useTransition('reorder'))
  let drag: Drag | null = $state(null)
  let target: string | null = $state(null)
  let announcement = $state('')
  let categories = $derived(layout.map(({ category }) => ({ value: category, label: preferenceLabel(category) })))
  const save = (next: Layout) => controller.store.form.change('draft.sidebarLayout', next)
  const moveCategory = (from: PreferenceSidebarCategory, to: PreferenceSidebarCategory) => {
    if (from === to) return
    const next = [...layout]
    const source = next.findIndex((section) => section.category === from)
    const destination = next.findIndex((section) => section.category === to)
    const [section] = next.splice(source, 1)
    next.splice(destination, 0, section)
    save(next)
    announcement = `${preferenceLabel(from)} category moved to position ${destination + 1}`
  }
  const moveItem = (item: PreferenceDestination, category: PreferenceSidebarCategory, before?: PreferenceDestination) => {
    if (item === before) return
    const next = layout.map((section) => ({ ...section, items: section.items.filter((entry) => entry !== item) }))
    const section = next.find((section) => section.category === category)!
    const index = before ? section.items.indexOf(before) : section.items.length
    section.items.splice(index < 0 ? section.items.length : index, 0, item)
    save(next)
    announcement = `${preferenceLabel(item)} moved to ${preferenceLabel(category)}`
  }
  const drop = (category: PreferenceSidebarCategory, before?: PreferenceDestination) => {
    if (drag?.item) moveItem(drag.item, category, before)
    else if (drag) moveCategory(drag.category, category)
    drag = null
    target = null
  }
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open) settings.pages.dialogs.closeDialog()
  }}
>
  <Dialog.Content class="w-120 rounded-xl bg-inset">
    <Form.Root model={controller.store.form}>
      <Dialog.Header class="h-14 px-6">
        <Dialog.Title>Customize sidebar</Dialog.Title><Dialog.Close></Dialog.Close>
      </Dialog.Header><Dialog.Body class="gap-4 px-6 pt-4 pb-6">
        <div class="flex items-center justify-between rounded-lg bg-elevated px-3 py-2 text-xsm">
          <span>Default badge style</span><Select.Control
            aria-label="Default badge style"
            class="w-26"
            name="draft.sidebarBadgeStyle"
            options={preferenceOptions.sidebarBadgeStyle}
          ></Select.Control>
        </div>
        <p class="text-xs text-muted-foreground">
          Drag categories to reorder. Drag items between categories, or choose a category from the menu.
        </p>
        <div class="grid gap-4">
          {#each layout as section, index (section.category)}<MotionSurface
              layout
              transition={timing}
              class={target === section.category && drag && !drag.item
                ? 'grid gap-2 rounded-lg bg-elevated p-2 ring-1 ring-primary'
                : 'grid gap-2 rounded-lg bg-elevated p-2'}
              ondragover={(event) => {
                if (drag) {
                  event.preventDefault()
                  target = section.category
                }
              }}
              ondrop={(event) => {
                event.preventDefault()
                event.stopPropagation()
                drop(section.category)
              }}
            >
              <div
                role="group"
                draggable
                class="flex cursor-grab items-center gap-2 active:cursor-grabbing"
                ondragstart={(event) => {
                  drag = { category: section.category }
                  if (event.dataTransfer) event.dataTransfer.effectAllowed = 'move'
                  event.dataTransfer?.setData('text/plain', section.category)
                }}
                ondragend={() => {
                  drag = null
                  target = null
                }}
                ondragover={(event) => {
                  if (drag) {
                    event.preventDefault()
                    target = section.category
                  }
                }}
                ondrop={(event) => {
                  event.preventDefault()
                  event.stopPropagation()
                  drop(section.category)
                }}
              >
                <button
                  type="button"
                  aria-label={`Reorder ${preferenceLabel(section.category)} category`}
                  class="flex size-6 cursor-grab items-center justify-center
text-muted-foreground focus-visible:outline-ring"
                  onkeydown={(event) => {
                    if (event.key !== 'ArrowUp' && event.key !== 'ArrowDown') return
                    event.preventDefault()
                    const destination = layout[index + (event.key === 'ArrowUp' ? -1 : 1)]
                    if (destination) moveCategory(section.category, destination.category)
                  }}
                >
                  <DotsSixVerticalIcon aria-hidden="true" class="size-3"></DotsSixVerticalIcon>
                </button>
                <h3 class="text-xsm font-medium">{preferenceLabel(section.category)}</h3>
              </div>
              <div
                role="group"
                class={target === section.category ? 'min-h-9 rounded ring-1 ring-primary' : 'min-h-9 rounded'}
                ondragover={(event) => {
                  if (drag) {
                    event.preventDefault()
                    target = section.category
                  }
                }}
                ondrop={(event) => {
                  event.preventDefault()
                  event.stopPropagation()
                  drop(section.category)
                }}
              >
                {#each section.items as item, itemIndex (item)}<MotionSurface
                    layout
                    transition={timing}
                    class={target === item
                      ? 'flex min-h-10 items-center gap-2 border-t border-primary px-1'
                      : 'flex min-h-10 items-center gap-2 px-1'}
                    ondragover={(event) => {
                      if (drag) {
                        event.preventDefault()
                        event.stopPropagation()
                        target = item
                      }
                    }}
                    ondrop={(event) => {
                      event.preventDefault()
                      event.stopPropagation()
                      drop(section.category, item)
                    }}
                  >
                    <button
                      type="button"
                      draggable
                      aria-label={`Reorder ${preferenceLabel(item)}`}
                      class="flex size-6 shrink-0 cursor-grab items-center justify-center
text-muted-foreground focus-visible:outline-ring"
                      ondragstart={(event) => {
                        event.stopPropagation()
                        drag = { category: section.category, item }
                        if (event.dataTransfer) event.dataTransfer.effectAllowed = 'move'
                        event.dataTransfer?.setData('text/plain', item)
                      }}
                      ondragend={() => {
                        drag = null
                        target = null
                      }}
                      onkeydown={(event) => {
                        if (event.key !== 'ArrowUp' && event.key !== 'ArrowDown') return
                        event.preventDefault()
                        const direction = event.key === 'ArrowUp' ? -1 : 1
                        const destination = section.items[itemIndex + direction]
                        if (destination) {
                          const next = layout.map((group) => ({ ...group, items: [...group.items] }))
                          const items = next[index].items
                          ;[items[itemIndex], items[itemIndex + direction]] = [items[itemIndex + direction], items[itemIndex]]
                          save(next)
                        }
                      }}
                    >
                      <DotsSixVerticalIcon aria-hidden="true" class="size-3"></DotsSixVerticalIcon>
                    </button><span class="min-w-0 flex-1 text-xs">{preferenceLabel(item)}</span><Select.Control
                      aria-label={`${preferenceLabel(item)} category`}
                      class="w-29"
                      value={section.category}
                      options={categories}
                      onValueChange={(category) => moveItem(item, category as PreferenceSidebarCategory)}
                    ></Select.Control><Select.Control
                      aria-label={`${preferenceLabel(item)} visibility`}
                      class="w-29"
                      name={`draft.sidebar${item}`}
                      options={preferenceOptions.sidebarVisibility}
                    ></Select.Control><FormError name={`draft.sidebar${item}`}></FormError>
                  </MotionSurface>{/each}{#if !section.items.length}<p class="px-2 py-3 text-xs text-muted-foreground">
                    Drop items here
                  </p>{:else}{/if}
              </div>
            </MotionSurface>{/each}
        </div>
        <FormError name="draft.sidebarLayout"></FormError><FormError name="draft.sidebarBadgeStyle"></FormError><span
          role="status"
          class="sr-only"
        >
          {announcement}
        </span>
      </Dialog.Body>
    </Form.Root>
  </Dialog.Content>
</Dialog.Root>
