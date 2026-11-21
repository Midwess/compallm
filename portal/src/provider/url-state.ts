import type { CreateProviderState } from '@compallm/storybook/pages/organization/create-provider'
import type { UrlState } from '@compallm/storybook'

export function readProviderUrl(location: UrlState) {
  const query = new URLSearchParams(location.search)
  return {
    step: location.pathname.endsWith('/configure-provider') ? ('configure' as const) : ('choose' as const),
    provider: query.get('provider'),
    model: query.get('model'),
    query: query.get('q') ?? '',
    modelQuery: query.get('model_q') ?? '',
  }
}
export function writeProviderUrl(state: CreateProviderState) {
  return {
    provider: state.selectedProviderId,
    q: state.providerQuery || null,
    model_q: state.modelQuery || null,
    model: state.editingModelId,
    v: null,
    models: null,
    dialog: null,
    name: null,
    base_url: null,
    protocol: null,
    mode: null,
    model_name: null,
    context_tokens: null,
    thinking: null,
    input_price: null,
    output_price: null,
    cached_price: null,
    intent: null,
    model_path: null,
    draft_protocol: null,
    output_tokens: null,
    effort_mapping: null,
    model_status: null,
    model_capabilities: null,
    model_protocol: null,
    provider_protocol: null,
    input_tokens: null,
    context_limit_enabled: null,
    input_limit_enabled: null,
    output_limit_enabled: null,
    provider_context_tokens: null,
    input_price_enabled: null,
    output_price_enabled: null,
    cached_price_enabled: null,
    template_prices: null,
  }
}
