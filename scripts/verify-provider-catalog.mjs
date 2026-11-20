import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { ProviderModelConfiguration } from '../libs/contract/dist/validation.js'

const root = new URL('../', import.meta.url)
const snapshot = await readFile(new URL('libs/postgres/src/migration/m20261117_000047_provider_catalog/after.rs', root), 'utf8')
const catalog = JSON.parse(snapshot.slice(snapshot.indexOf('r###"') + 5, snapshot.lastIndexOf('"###')))
const manifest = JSON.parse(await readFile(new URL('portal/public/assets/providers/manifest.json', root), 'utf8'))
const providers = new Map(catalog.provider_templates.map(provider => [provider.id, provider]))
let modelInfoRecipes = 0
for (const record of [...catalog.provider_templates, ...catalog.provider_model_templates]) {
  assert(Array.isArray(record.model_info_sources) && record.model_info_sources.length > 0 && record.model_info_sources.length <= 32)
  for (const source of record.model_info_sources) {
    modelInfoRecipes++
    assert(['api', 'documentation', 'repository', 'manual'].includes(source.kind))
    assert(['observed', 'documented', 'reference', 'unavailable'].includes(source.status))
    assert(source.notes.trim().length > 0)
    if (source.kind === 'api') {
      assert(source.request.method === 'GET')
      if (source.access === 'public') assert(source.request.authentication === 'none')
      if (source.status === 'observed') assert(source.response && Object.keys(source.response.fields).length > 0)
      if (source.response) {
        assert.deepEqual([...source.fields].sort(), Object.keys(source.response.fields).sort())
        if (record.upstream_model_id) assert(source.response.identity_value, 'model API recipe needs an explicit provider-scoped identity selector')
      }
    }
    if (source.kind !== 'manual') assert.equal(typeof source.url, 'string')
    if (source.kind === 'api') {
      const url = new URL(source.url)
      assert(['http:', 'https:'].includes(url.protocol) && !url.username && !url.password && !url.hash)
      for (const key of url.searchParams.keys()) assert(!['key', 'api_key', 'apikey', 'token', 'access_token', 'authorization', 'password', 'secret'].includes(key.toLowerCase()))
    }
  }
}
const metadataStatuses = new Set(['unknown', 'account_specific', 'subscription_based', 'variable',
  'unsupported_precision', 'standard_input_rate', 'conditional', 'requires_authentication', 'unconfigured', 'not_applicable',
  'unpublished', 'no_public_endpoint', 'migration_required', 'credit_based', 'deprecated', 'client_restricted'])
for (const record of [...catalog.provider_templates, ...catalog.provider_model_templates]) {
  for (const metadata of Object.values(record.field_metadata)) {
    assert(metadataStatuses.has(metadata.status), `unknown metadata status: ${metadata.status}`)
    assert(typeof metadata.reason === 'string' && metadata.reason.length > 0)
  }
}
assert.equal(providers.size, catalog.provider_templates.length)
assert.equal(manifest.length, providers.size)
for (const provider of providers.values()) {
  assert.match(provider.logo_path, /^\/assets\/providers\/[a-z0-9-]+\.(svg|png|webp)$/)
  const entry = manifest.find(entry => entry.provider === provider.id)
  assert.equal(entry?.path, provider.logo_path)
  const data = await readFile(new URL(`portal/public${provider.logo_path}`, root))
  assert.equal(createHash('sha256').update(data).digest('hex'), entry.sha256)
  if (provider.logo_path.endsWith('.svg')) {
    const svg = data.toString('utf8')
    assert(!/<(?:script|foreignObject)\b|\bon\w+\s*=|(?:href|url)\s*[=(]\s*["']?\s*(?:https?:|\/\/)/i.test(svg))
  } else assert(data.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])))
  assert(provider.source_refs.length > 0)
  assert(provider.auth_methods.length > 0)
  assert(Object.keys(provider.auth_config).length === provider.auth_methods.length)
  if (provider.base_url == null) assert(provider.field_metadata.base_url?.status)
}
const ids = new Set(), upstream = new Set(), missing = {}, ready = new Set()
let text = 0, complete = 0
for (const model of catalog.provider_model_templates) {
  assert(['metered','subscription','subscription_with_overage','free','unknown'].includes(model.billing_model))
  assert(['fixed','tiered','api','account_specific','not_applicable','unpublished','scheduled','unknown'].includes(model.pricing_mode))
  assert(['separate_rate','standard_input_rate','unsupported','included','not_applicable','per_route','unknown'].includes(model.cache_billing))
  assert(model.capabilities && !Array.isArray(model.capabilities))
  for (const value of Object.values(model.capabilities)) assert(value === null || typeof value === 'boolean')
  for (const estimateKind of ['fallback', 'cost_proxy']) {
    const fallback = model.pricing_config[estimateKind]
    if (!fallback) continue
    assert.equal(fallback.currency, 'USD')
    assert.equal(fallback.unit_quantity, 1000000)
    assert(['same_model_other_provider','related_model','target_provider_estimate'].includes(fallback.basis))
    assert(['high','medium','low'].includes(fallback.confidence))
    assert(fallback.reference_provider && fallback.reference_model && fallback.notes)
    assert(fallback.source_urls.length > 0 && fallback.source_urls.length <= 16)
    if (estimateKind === 'fallback') assert(!['free','subscription'].includes(model.billing_model))
    else assert(['free','subscription','subscription_with_overage'].includes(model.billing_model))
    for (const field of ['input_price','output_price','cached_price']) {
      if (fallback[field] != null) assert(Number(fallback[field]) >= 0)
    }
  }
  if (model.billing_model === 'subscription') {
    assert(model.pricing_config.subscription?.product)
    assert(model.pricing_mode === 'not_applicable' && model.billing_unit === null)
    assert(['input_price','output_price','cached_price'].every(field => model[field] === null))
  }
  if (model.cache_billing !== 'separate_rate' && model.cached_price !== null) {
    assert.equal(model.cache_billing, 'unknown')
    assert.equal(model.field_metadata.cached_price.status, 'conditional')
    assert.equal(Number(model.cached_price), Number(model.pricing_config.fallback.cached_price))
  }
  if (model.pricing_mode === 'tiered') assert(model.pricing_config.tiers?.length)
  for (const field of Object.values(model.unresolved_fields)) assert(field.reason?.trim())
  assert(providers.has(model.provider_template_id))
  const identity = JSON.stringify([model.provider_template_id, model.id])
  assert(!ids.has(identity)); ids.add(identity)
  assert(model.source_refs.length > 0)
  for (const limit of ['max_context_tokens', 'max_input_tokens', 'max_output_tokens']) {
    assert(model[limit] == null || (Number.isInteger(model[limit]) && model[limit] > 0 && model[limit] <= 4294967295))
  }
  for (const reference of model.source_refs) {
    assert(['repository', 'web'].includes(reference.kind))
    assert(['imported', 'estimate', 'verified'].includes(reference.status))
    assert(typeof reference.url === 'string' && reference.url.trim())
  }
  for (const field of ['input_price', 'output_price', 'cached_price']) {
    const evidence = model.pricing_config.zero_price_evidence?.[field]
    if (model[field] !== null && Number(model[field]) === 0) {
      assert(evidence && Number.isInteger(evidence.source_index) && evidence.reason?.trim())
      const source = model.source_refs[evidence.source_index]
      assert(source && source.fields.includes(field))
      assert(source.status === 'verified' || (source.status === 'estimate' &&
        model.field_metadata[field]?.status === 'conditional' &&
        model.pricing_config.fallback?.[field] != null && Number(model.pricing_config.fallback[field]) === 0))
    } else assert(!evidence)
  }
  if (!model.protocols.length) continue
  text++
  const name = JSON.stringify([model.provider_template_id, model.upstream_model_id])
  assert(!upstream.has(name), `duplicate selectable upstream model: ${name}`); upstream.add(name)
  let valid = false
  for (const protocol of model.protocols) {
    assert(model.paths[protocol], `missing path for ${identity}/${protocol}`)
    const result = ProviderModelConfiguration.safeParse({
      name: model.upstream_model_id, protocol: protocol.toUpperCase(), path: model.paths[protocol],
      maxContextTokens: model.max_context_tokens ?? undefined, enabled: true,
      effortMapping: model.effort_mapping, capabilities: Object.entries(model.capabilities).filter(([,supported]) => supported === true).map(([value]) => value.toUpperCase()),
      inputPrice: model.input_price, outputPrice: model.output_price, cachedPrice: model.cached_price,
      catalogRef: { provider: model.provider_template_id, model: model.id },
    })
    if (result.success) valid = true
    else for (const field of new Set(result.error.issues.map(issue => issue.path.join('.')))) {
      assert(['maxContextTokens', 'inputPrice', 'outputPrice', 'cachedPrice'].includes(field), `${identity}: ${field}`)
    }
  }
  for (const field of ['max_context_tokens', 'input_price', 'output_price', 'cached_price']) {
    if (model[field] == null) missing[field] = (missing[field] ?? 0) + 1
  }
  if (valid) { complete++; ready.add(model.provider_template_id) }
}
console.log(JSON.stringify({ providers: providers.size, models: ids.size, sourcedLogos: manifest.length, modelInfoRecipes,
  allTextModelRecords: catalog.provider_model_templates.filter(model => model.service_kind === 'text').length,
  nullableFieldsAcrossAllTextRecords: Object.fromEntries(
    ['input_price','output_price','cached_price','max_context_tokens','max_input_tokens','max_output_tokens'].map(field =>
      [field, catalog.provider_model_templates.filter(model => model.service_kind === 'text' && model[field] === null).length])),
  nonTextModelRecords: catalog.provider_model_templates.filter(model => model.service_kind !== 'text').length,
  textModels: text, modelsWithEstimatedPriceFallback: catalog.provider_model_templates.filter(model => model.pricing_config.fallback).length, modelsWithTokenCostProxy: catalog.provider_model_templates.filter(model => model.pricing_config.cost_proxy).length, completeLegacyFormDefaults: complete, providersWithCompleteModels: ready.size,
  billingModels: Object.fromEntries(['metered','subscription','subscription_with_overage','free','unknown'].map(method => [method,catalog.provider_model_templates.filter(model => model.billing_model === method).length])),
  pricingModes: Object.fromEntries(['fixed','tiered','api','account_specific','not_applicable','unpublished','scheduled','unknown'].map(method => [method,catalog.provider_model_templates.filter(model => model.pricing_mode === method).length])),
  nullableScalarFieldsInTextCatalog: missing,
  unresolvedRequirementCounts: catalog.provider_model_templates.filter(model => model.protocols.length).reduce((counts, model) => {
    for (const field of Object.keys(model.unresolved_fields)) counts[field] = (counts[field] ?? 0) + 1
    return counts
  }, {}), snapshot: fileURLToPath(new URL('libs/postgres/src/migration/m20261117_000047_provider_catalog/after.rs', root)),
}, null, 2))
