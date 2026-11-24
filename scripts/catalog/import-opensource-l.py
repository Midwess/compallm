import argparse
import ast
import collections
import hashlib
import json
import subprocess
import uuid
from decimal import Decimal
from pathlib import Path

REVISION = '65a926600a92b11f1464433a3a60a2c25cb93574'
ALIASES = {
    'together_ai': 'together', 'nvidia_nim': 'nvidia', 'cohere_chat': 'cohere',
    'fireworks_ai': 'fireworks', 'fireworks_ai-embedding-models': 'fireworks',
    'cloudflare': 'cloudflare-ai', 'featherless_ai': 'featherless',
    'aws_polly': 'aws-polly', 'fal_ai': 'fal-ai', 'black_forest_labs': 'black-forest-labs',
    'stability': 'stability-ai', 'vercel_ai_gateway': 'vercel-ai-gateway',
    'jina_ai': 'jina-ai', 'voyage': 'voyage-ai', 'xiaomi_mimo': 'xiaomi-mimo',
    'you_com': 'youcom', 'exa_ai': 'exa', 'google_pse': 'google-pse',
    'bedrock_converse': 'bedrock', 'bedrock_mantle': 'bedrock-mantle',
    'opencode_go': 'opencode-go', 'github_copilot': 'github', 'github': 'github-models',
    'veniceai': 'venice', 'vertex_ai_beta': 'vertex-ai',
    'text-completion-codestral': 'codestral', 'text-completion-inception': 'inception',
    'text-completion-openai': 'openai', 'anthropic_text': 'anthropic',
    'ollama_chat': 'ollama-local', 'watsonx_text': 'watsonx',
    'ai21_chat': 'ai21', 'sagemaker_chat': 'sagemaker',
}
EXCLUDED = {
    'a2a', 'a2a_agent', 'auto_router', 'aiohttp_openai', 'custom',
    'custom_openai', 'openai_like', 'dotprompt', 'humanloop', 'langfuse',
    'langflow', 'langgraph', 'litellm_agent', 'litellm_proxy', 'milvus',
    'mongodb', 'opensearch', 'pg_vector', 's3_vectors', 'valkey',
}
SERVICES = {'chat': 'llm', 'responses': 'llm', 'completion': 'llm',
            'embedding': 'embedding', 'rerank': 'rerank', 'audio_speech': 'tts',
            'audio_transcription': 'stt', 'image_generation': 'image',
            'image_edit': 'image', 'video_generation': 'video'}


def encode(value):
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'), default=str)


def load(path):
    return json.loads(path.read_text(), parse_float=Decimal)


def provider_id(slug):
    if slug.startswith('vertex_ai-'):
        return 'vertex-ai'
    return ALIASES.get(slug, slug.replace('_', '-'))


def source(fields):
    return [{'kind': 'repository', 'reference': 'opensource(l)', 'url': None,
             'access': 'public', 'status': 'reference', 'fields': fields,
             'request': None, 'response': None, 'pagination': None, 'revision': REVISION,
             'notes': 'Pinned catalog and provider source reference. Imported facts require provider-specific validation before execution.'}]


def adapter_endpoint(root, slugs):
    found = {}
    for slug in slugs:
        directory = root / 'litellm' / 'llms' / slug
        for path in sorted(directory.rglob('*.py')) if directory.is_dir() else []:
            if path.name not in ('transformation.py', 'common_utils.py'):
                continue
            tree = ast.parse(path.read_text())
            for function in ast.walk(tree):
                if not isinstance(function, (ast.FunctionDef, ast.AsyncFunctionDef)) or function.name not in (
                    '_get_openai_compatible_provider_info', 'get_openai_compatible_provider_info', 'get_api_base'
                ):
                    continue
                for node in ast.walk(function):
                    targets = node.targets if isinstance(node, ast.Assign) else [node.target] if isinstance(node, ast.AnnAssign) else []
                    value = node.value if isinstance(node, (ast.Assign, ast.AnnAssign)) else node.value if isinstance(node, ast.Return) and function.name == 'get_api_base' else None
                    if value is None or (targets and not any(isinstance(t, ast.Name) and t.id == 'api_base' for t in targets)):
                        continue
                    dynamic = {id(part) for joined in ast.walk(value) if isinstance(joined, ast.JoinedStr) for part in ast.walk(joined)}
                    for literal in ast.walk(value):
                        if id(literal) not in dynamic and isinstance(literal, ast.Constant) and isinstance(literal.value, str) and literal.value.startswith(('https://', 'http://')):
                            found[literal.value] = str(path.relative_to(root)) + ':' + str(literal.lineno)
    if len(found) == 1:
        url, location = next(iter(found.items()))
        return {'base_url': url, 'source_location': location}
    return {}


def unknown():
    return {'status': 'unknown'}


def capability(flag, config, price=None):
    if flag is True:
        return {'status': 'supported', 'configuration': config, 'pricing': price}
    return {'status': 'unsupported'} if flag is False else unknown()


def token_limit(value):
    return value if type(value) is int and 0 < value <= 4294967295 else None


def money(value, quantity=1000000):
    if not isinstance(value, (int, Decimal)) or isinstance(value, bool):
        return None
    number = Decimal(value) * quantity
    if not number.is_finite() or number < 0 or number >= Decimal(10) ** 12:
        return None
    text = format(number.normalize(), 'f')
    if len(text.split('.')[-1]) > 18 and '.' in text:
        return None
    return {'amount': text, 'currency': 'USD', 'quantity': quantity}


def token_price(raw, direction):
    if 'tiered_pricing' in raw or any('_above_' in key or '_below_' in key for key in raw):
        return None
    rate = money(raw.get(direction + '_cost_per_token'))
    cached = money(raw.get('cache_read_input_token_cost')) if direction == 'input' else None
    if rate is None and cached is None:
        return None
    cache = {'type': 'separate_rate', 'configuration': {'rate': cached}} if cached else None
    return {'type': 'usage', 'configuration': {'fixed': {'token': {
        'rate': rate or {'amount': None, 'currency': 'USD', 'quantity': 1000000}, 'cache': cache}}}}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', type=Path, default=Path('.dev/litellm'))
    parser.add_argument('--baseline', required=True, type=Path)
    parser.add_argument('--output', type=Path, default=Path('libs/postgres/src/migration/m20261122_000054_catalog_search/import.json'))
    args = parser.parse_args()
    revision = subprocess.check_output(['git', '-C', str(args.source), 'rev-parse', 'HEAD'], text=True).strip()
    if revision != REVISION:
        raise ValueError('Source checkout does not match the reviewed revision')
    paths = ['model_prices_and_context_window.json', 'litellm/llms/openai_like/providers.json',
             'litellm/types/utils.py', 'litellm/proxy/public_endpoints/provider_create_fields.json']
    for path in paths:
        tracked = subprocess.check_output(['git', '-C', str(args.source), 'show', revision + ':' + path])
        if tracked != (args.source / path).read_bytes():
            raise ValueError('Source has unreviewed local changes: ' + path)
    prices = load(args.source / paths[0])
    configs = load(args.source / paths[1])
    field_variants = collections.defaultdict(list)
    for definition in load(args.source / paths[3]):
        field_variants[definition['litellm_provider']].append(definition)
    fields = {slug: variants[-1] for slug, variants in field_variants.items()}
    tree = ast.parse((args.source / paths[2]).read_text())
    enum = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == 'LlmProviders')
    enum_values = {n.value.value for n in enum.body if isinstance(n, ast.Assign) and isinstance(n.value, ast.Constant)}
    baseline = load(args.baseline)
    providers = {r['id']: r for r in baseline['provider_templates']}
    models = {(r['provider_template_id'], r['id']): r for r in baseline['provider_model_templates']}
    changes = []
    bindings = []
    grouped = collections.defaultdict(list)
    skipped = []
    for key, raw in prices.items():
        if key in ('sample_spec', 'fallback_generalizations'):
            skipped.append({'key': key, 'reason': 'Schema example or lookup rules, not a model'})
            continue
        target = raw
        seen = {key}
        while not target.get('litellm_provider') and target.get('alias_of'):
            ref = target['alias_of']
            if ref in seen or ref not in prices:
                raise ValueError('Unresolved alias: ' + key)
            seen.add(ref)
            target = prices[ref]
        slug = target.get('litellm_provider')
        if not slug:
            raise ValueError('Unclassified source entry: ' + key)
        if slug in EXCLUDED:
            skipped.append({'key': key, 'provider': slug, 'value': raw, 'reason': 'Agent, protocol, routing, or storage integration rather than a model provider'})
            continue
        p = provider_id(slug)
        native = key.removeprefix(slug + '/')
        grouped[(p, native)].append({'key': key, 'provider': slug, 'value': raw, 'resolved': target if raw is not target else None})
    slugs = enum_values | set(configs) | {v.get('litellm_provider') for v in prices.values() if isinstance(v, dict) and v.get('litellm_provider') and v.get('litellm_provider') != 'one of https://docs.litellm.ai/docs/providers'}
    candidates = collections.defaultdict(list)
    for slug in sorted(slugs - EXCLUDED):
        candidates[provider_id(slug)].append(slug)
    registered_slugs = slugs
    for p, slugs in sorted(candidates.items()):
        endpoint = adapter_endpoint(args.source, slugs)
        if p in providers:
            changes.append({'table': 'provider_template_research', 'key': providers[p]['research_id'],
                            'supplement': {'revision': REVISION, 'slugs': slugs,
                                           'configurations': {slug: configs[slug] for slug in slugs if slug in configs},
                                           'adapter_endpoint': endpoint,
                                           'create_fields': {slug: fields[slug] for slug in slugs if slug in fields}, 'create_field_variants': {slug: field_variants[slug] for slug in slugs if slug in field_variants}}})
            continue
        config = next((configs[s] for s in slugs if s in configs), {})
        info = next((fields[s] for s in slugs if s in fields), {})
        base = config.get('base_url') or endpoint.get('base_url')
        auth = {'type': 'api_key', 'configuration': {'placement': {'apiKeyHeader': {'name': 'Authorization', 'prefix': 'Bearer '}} if config else None, 'requiredInputs': []}}
        connection = {'baseUrl': base, 'path': None, 'timeoutMs': None, 'query': {},
                      'authentication': auth, 'endpoint': {'mode': 'fixed' if base else 'unknown', 'urlTemplate': None, 'requiredInputs': []}}
        rid = encode(['opensource(l)', p])
        row = {'id': p, 'name': info.get('provider_display_name') or p, 'description': None,
               'logo_url': None, 'headers': {}, 'connection': connection, 'authentication_options': [auth], 'research_id': rid}
        snapshot = {'id': p, 'connection': {'base_url': base, 'endpoint_mode': 'fixed' if base else 'unknown', 'authentication': {}, 'endpoint': {}}}
        research = {'id': rid, 'provider_template_id': p, 'metadata': {'catalog_import': {'revision': REVISION, 'slugs': slugs, 'configuration': config, 'configurations': {slug: configs[slug] for slug in slugs if slug in configs}, 'adapter_endpoint': endpoint, 'create_fields': {slug: fields[slug] for slug in slugs if slug in fields}, 'create_field_variants': {slug: field_variants[slug] for slug in slugs if slug in field_variants}}},
                    'authentication_order': ['api_key'], 'source_refs': [{'kind': 'repository', 'reference': 'opensource(l)', 'revision': REVISION, 'fields': ['identity', 'endpoint', 'authentication'], 'status': 'imported'}],
                    'field_metadata': {'integration': {'status': 'unknown', 'reason': 'Imported provider identity; authentication and supported operation mapping require review.'}},
                    'model_info_sources': source(['base_url']),
                    'endpoint_config': {}, 'authentication': {'api_key': {'status': 'unknown', 'reason': 'Credential requirements retained from the pinned source; request placement requires adapter review.'}},
                    'configuration_snapshot': snapshot, 'state': 'in_progress'}
        changes.extend([{'table': 'provider_template_research', 'row': research}, {'table': 'provider_templates', 'row': row}])
        providers[p] = row
    supplements = collections.defaultdict(dict)
    for (p, native), observations in sorted(grouped.items()):
        matched = models.get((p, native))
        if matched:
            rid = matched['research_id']
            supplements[rid].update({v['key']: {'provider': v['provider'], 'value': v['value']} for v in observations})
        else:
            raw = next((v['value'] for v in observations if v['value'].get('litellm_provider')), observations[0]['resolved'])
            service = SERVICES.get(raw.get('mode'))
            ri = encode(['opensource(l)', p, native])
            priced = service in ('llm', 'embedding', 'rerank')
            thinking = capability(raw.get('supports_reasoning'), {'effortMapping': {}, 'defaultEffort': None, 'budget': None})
            text = {'input': capability(True if priced or service == 'tts' else None,
                                       {'maxTokens': token_limit(raw.get('max_input_tokens'))}, token_price(raw, 'input') if priced else None),
                    'output': capability(True if service in ('llm', 'stt') else None,
                                         {'maxTokens': token_limit(raw.get('max_output_tokens')), 'thinking': thinking}, token_price(raw, 'output') if service == 'llm' else None)}
            row = {'row_id': str(uuid.uuid5(uuid.NAMESPACE_URL, 'compallm:catalog:' + encode([p, native]))),
                   'id': native, 'name': native, 'provider_template_id': p, 'display_name': None, 'description': None,
                   'research_id': ri, 'user_id': None, 'organization_id': None, 'base_row_id': None, 'custom_fields': [],
                   'service_kind': service, 'connections': {}, 'headers': {}, 'body_defaults': {}, 'text': text,
                   'limits': {'contextTokens': token_limit(raw.get('context_window'))}, 'billing': None, 'route_selection': None,
                   'vision': capability(raw.get('supports_vision'), {'formats': None, 'maxImages': None}),
                   'pdf': capability(raw.get('supports_pdf_input'), {'maxFiles': None, 'maxPages': None}),
                   'video': capability(True if service == 'video' else raw.get('supports_video_input'), {'formats': None, 'maxDurationMs': None, 'maxFrames': None}),
                   'image_generation': capability(True if service == 'image' else None, {'formats': None, 'maxImages': None}),
                   'audio': {'input': capability(True if service == 'stt' else raw.get('supports_audio_input'), {'formats': None, 'maxDurationMs': None, 'maxBytes': None}),
                             'output': capability(True if service == 'tts' else raw.get('supports_audio_output'), {'formats': None, 'maxDurationMs': None, 'maxBytes': None})},
                   'tools': {'calling': capability(raw.get('supports_function_calling'), {'maxTools': None}),
                             'forcedChoice': capability(raw.get('supports_tool_choice'), {}), 'parallelCalls': capability(raw.get('supports_parallel_function_calling'), {})},
                   'structured_output': capability(raw.get('supports_response_schema'), {'jsonMode': unknown(), 'jsonSchema': capability(raw.get('supports_response_schema'), {})}),
                   'search': capability(raw.get('supports_web_search'), {'maxResults': None})}
            research = {'id': ri, 'provider_template_id': p, 'model_template_id': native,
                        'metadata': {'catalog_import': {'revision': REVISION, 'entries': {v['key']: {'provider': v['provider'], 'value': v['value']} for v in observations}}},
                        'operation_order': [], 'source_refs': [{'kind': 'repository', 'reference': 'opensource(l)', 'revision': REVISION, 'fields': list(raw), 'status': 'imported'}],
                        'field_metadata': {}, 'model_info_sources': source(['upstream_model_id', 'specification', 'pricing', 'capabilities']),
                        'pricing_config': {}, 'specification_mode': 'unknown', 'resolution_plan': {},
                        'unresolved_fields': {'integration': {'status': 'unknown', 'reason': 'Imported catalog identity and metadata; request identity, pricing conditions, and operation mapping require review.'}},
                        'configuration_snapshot': {'pricing': {'mode': 'unknown'}, 'source_key': observations[0]['key']}, 'state': 'in_progress'}
            changes.extend([{'table': 'provider_model_template_research', 'row': research}, {'table': 'provider_model_templates', 'row': row}])
        bindings.extend({'key': v['key'], 'provider': v['provider'], 'provider_template_id': p, 'model_template_id': native} for v in observations)
    for rid, entries in sorted(supplements.items()):
        changes.append({'table': 'provider_model_template_research', 'key': rid,
                        'supplement': {'revision': REVISION, 'entries': entries}})
    manifest = {'reference': 'opensource(l)', 'revision': REVISION,
                'source_hashes': {p: hashlib.sha256((args.source / p).read_bytes()).hexdigest() for p in paths},
                'registered_identifiers': sorted(registered_slugs), 'excluded_identifiers': sorted(EXCLUDED & registered_slugs),
                'provider_bindings': {slug: provider_id(slug) for slug in sorted(registered_slugs - EXCLUDED)},
                'ignored_entries': skipped, 'bindings': bindings,
                'counts': {'providers_inserted': sum(v['table'] == 'provider_templates' for v in changes),
                           'models_inserted': sum(v['table'] == 'provider_model_templates' for v in changes),
                           'existing_provider_research_supplemented': sum(v['table'] == 'provider_template_research' and 'key' in v for v in changes),
                           'existing_model_research_supplemented': len(supplements), 'source_entries_bound': len(bindings)}}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text('{\n"manifest":' + json.dumps(manifest, ensure_ascii=False, indent=2, default=str) + ',\n"changes":[\n' + ',\n'.join(encode(change) for change in changes) + '\n]\n}\n')
    print(json.dumps(manifest['counts'], indent=2))


if __name__ == '__main__':
    main()
