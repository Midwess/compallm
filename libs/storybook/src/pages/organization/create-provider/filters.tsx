import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
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
          <Popup.RadioGroup
            value={status}
            onValueChange={(value: ModelStatusFilter) => controller.onModelFiltersChanged?.({ status: value })}
          >
            {(['all', 'enabled', 'disabled'] as ModelStatusFilter[]).map((value) => (
              <Popup.RadioItem key={value} value={value} disabled={!controller.onModelFiltersChanged}>
                {value === 'all' ? 'All' : value === 'enabled' ? 'Enabled' : 'Disabled'}
              </Popup.RadioItem>
            ))}
          </Popup.RadioGroup>
        </Popup.Submenu>
        <Popup.Submenu label="Model type">
          <Popup.RadioGroup
            value={protocol}
            onValueChange={(value: TextModelProtocol | 'all') => controller.onModelFiltersChanged?.({ protocol: value })}
          >
            {[{ value: 'all' as const, label: 'All model types' }, ...textModelOptions].map((option) => (
              <Popup.RadioItem key={option.value} value={option.value} disabled={!controller.onModelFiltersChanged}>
                {option.label}
              </Popup.RadioItem>
            ))}
          </Popup.RadioGroup>
        </Popup.Submenu>
        <Popup.Submenu label="Capabilities">
          {capabilityOptions.map((option) => (
            <Popup.CheckboxItem
              key={option.value}
              checked={capabilities.includes(option.value)}
              disabled={!controller.onModelFiltersChanged}
              onCheckedChange={(checked) =>
                controller.onModelFiltersChanged?.({
                  capabilities: checked
                    ? [...capabilities, option.value]
                    : capabilities.filter((value) => value !== option.value),
                })
              }
            >
              {option.label}
            </Popup.CheckboxItem>
          ))}
        </Popup.Submenu>
        <Popup.Separator />
        <Popup.Item
          className="text-muted-foreground"
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
          <Popup.RadioGroup
            value={filter}
            onValueChange={(value: TextModelProtocol | 'all') => controller.onProviderFilterChanged?.(value)}
          >
            {[{ value: 'all' as const, label: 'All model types' }, ...textModelOptions].map((option) => (
              <Popup.RadioItem key={option.value} value={option.value} disabled={!controller.onProviderFilterChanged}>
                {option.label}
              </Popup.RadioItem>
            ))}
          </Popup.RadioGroup>
        </Popup.Submenu>
        <Popup.Separator />
        <Popup.Item
          className="text-muted-foreground"
          disabled={!controller.onProviderFilterChanged}
          onClick={() => controller.onProviderFilterChanged?.('all')}
        >
          Clear all filters
        </Popup.Item>
      </Popup.Content>
    </Popup>
  )
}
