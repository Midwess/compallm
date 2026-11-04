import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { Popup } from '../../../components/popup'
import { capabilityOptions, textModelOptions } from './model-options'
import { useController, useProvider } from './controller'
import type { ModelStatusFilter, TextModelProtocol } from './types'

export function ModelFilters() {
  const controller = useController()
  const status = useProvider((state) => state.modelStatusFilter)
  const protocol = useProvider((state) => state.modelProtocolFilter)
  const capabilities = useProvider((state) => state.modelCapabilityFilter)
  return (
    <Popup>
      <Popup.IconTrigger aria-label="Filter models" title="Filter models">
        <FunnelIcon className="size-3.5" aria-hidden="true" />
      </Popup.IconTrigger>
      <Popup.Content aria-label="Model filters">
        <Popup.Submenu label="Enabled">
          {(['all', 'enabled', 'disabled'] as ModelStatusFilter[]).map((value) => (
            <Popup.Item
              key={value}
              role="menuitemradio"
              aria-checked={status === value}
              className="justify-between"
              disabled={!controller.onModelFiltersChanged}
              onClick={() => controller.onModelFiltersChanged?.({ status: value })}
            >
              {value === 'all' ? 'All' : value === 'enabled' ? 'Enabled' : 'Disabled'}
              {status === value ? <CheckIcon className="size-3" aria-hidden="true" /> : null}
            </Popup.Item>
          ))}
        </Popup.Submenu>
        <Popup.Submenu label="Model type">
          {[{ value: 'all' as const, label: 'All model types' }, ...textModelOptions].map((option) => (
            <Popup.Item
              key={option.value}
              role="menuitemradio"
              aria-checked={protocol === option.value}
              className="justify-between"
              disabled={!controller.onModelFiltersChanged}
              onClick={() => controller.onModelFiltersChanged?.({ protocol: option.value })}
            >
              {option.label}
              {protocol === option.value ? <CheckIcon className="size-3" aria-hidden="true" /> : null}
            </Popup.Item>
          ))}
        </Popup.Submenu>
        <Popup.Submenu label="Capabilities">
          {capabilityOptions.map((option) => (
            <Popup.Item
              key={option.value}
              role="menuitemcheckbox"
              aria-checked={capabilities.includes(option.value)}
              className="justify-between gap-4"
              disabled={!controller.onModelFiltersChanged}
              onClick={(event) => {
                event.preventDefault()
                controller.onModelFiltersChanged?.({
                  capabilities: capabilities.includes(option.value)
                    ? capabilities.filter((value) => value !== option.value)
                    : [...capabilities, option.value],
                })
              }}
            >
              {option.label}
              {capabilities.includes(option.value) ? <CheckIcon className="size-3" aria-hidden="true" /> : null}
            </Popup.Item>
          ))}
        </Popup.Submenu>
        <Popup.Separator />
        <Popup.Item
          className="text-muted"
          onClick={() => controller.onClearModelFilters?.()}
          disabled={!controller.onClearModelFilters}
        >
          Clear all filters
        </Popup.Item>
      </Popup.Content>
    </Popup>
  )
}
export function ProviderFilters() {
  const controller = useController()
  const filter = useProvider((state) => state.providerProtocolFilter)
  return (
    <Popup>
      <Popup.IconTrigger aria-label="Filter providers" title="Filter providers">
        <FunnelIcon className="size-3.5" aria-hidden="true" />
      </Popup.IconTrigger>
      <Popup.Content aria-label="Provider filters">
        <Popup.Submenu label="Model type">
          {[{ value: 'all' as const, label: 'All model types' }, ...textModelOptions].map((option) => (
            <Popup.Item
              key={option.value}
              role="menuitemradio"
              aria-checked={filter === option.value}
              className="justify-between"
              disabled={!controller.onProviderFilterChanged}
              onClick={() => controller.onProviderFilterChanged?.(option.value as TextModelProtocol | 'all')}
            >
              {option.label}
              {filter === option.value ? <CheckIcon className="size-3" aria-hidden="true" /> : null}
            </Popup.Item>
          ))}
        </Popup.Submenu>
        <Popup.Separator />
        <Popup.Item
          className="text-muted"
          disabled={!controller.onProviderFilterChanged}
          onClick={() => controller.onProviderFilterChanged?.('all')}
        >
          Clear all filters
        </Popup.Item>
      </Popup.Content>
    </Popup>
  )
}
