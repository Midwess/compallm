import argparse
import ast
import json
from decimal import Decimal
from pathlib import Path


def canonical(value):
    if isinstance(value, dict):
        return {k: canonical(v) for k, v in value.items()}
    if isinstance(value, list):
        return [canonical(v) for v in value]
    return str(value) if isinstance(value, Decimal) else value


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--source', type=Path, default=Path('.dev/litellm'))
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    source = json.loads((args.source / 'model_prices_and_context_window.json').read_text(), parse_float=Decimal)
    snapshot = json.loads(args.snapshot.read_text())
    if 'catalog_source' in snapshot:
        records = [r['source_records'] for r in snapshot['catalog_source'] if isinstance(r['source_records'], dict)]
        snapshot = {
            'provider_templates': snapshot['catalog_provider_book'],
            'provider_model_templates': [dict(r, provider_template_id=r['provider_catalog_id'], research_id=r['source_id'], limits={'contextTokens': r.get('context_tokens')}) for r in snapshot['catalog_model_book'] if r['user_id'] is None],
            'provider_template_research': [r for r in records if 'authentication_order' in r],
            'provider_model_template_research': [r for r in records if 'operation_order' in r],
        }
    expected = {k: canonical(v) for k, v in source.items() if k not in ('sample_spec', 'fallback_generalizations')}
    models = {(r['provider_template_id'], r['id']): r for r in snapshot['provider_model_templates']}
    reviews = {r['id']: r for r in snapshot['provider_model_template_research']}
    observed = {}
    errors = []
    imported = []
    for identity, row in models.items():
        review = reviews.get(row['research_id'])
        if review is None:
            errors.append({'kind': 'missing_research', 'identity': identity})
            continue
        for key, observation in review['metadata'].get('catalog_import', {}).get('entries', {}).items():
            if key in observed:
                errors.append({'kind': 'duplicate_source_binding', 'key': key})
            observed[key] = observation['value']
        if (row['research_id'] or '').startswith('["opensource(l)"'):
            imported.append(row)
    for key in sorted(expected.keys() - observed.keys()):
        errors.append({'kind': 'missing_source_entry', 'key': key})
    for key in sorted(observed.keys() - expected.keys()):
        errors.append({'kind': 'unexpected_source_entry', 'key': key})
    for key in expected.keys() & observed.keys():
        if expected[key] != observed[key]:
            errors.append({'kind': 'source_value_mismatch', 'key': key})
    tree = ast.parse((args.source / 'litellm/types/utils.py').read_text())
    enum = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == 'LlmProviders')
    enum_slugs = {n.value.value for n in enum.body if isinstance(n, ast.Assign) and isinstance(n.value, ast.Constant)}
    json_slugs = set(json.loads((args.source / 'litellm/llms/openai_like/providers.json').read_text()))
    provider_slugs = enum_slugs | json_slugs | {v['litellm_provider'] for v in expected.values() if v.get('litellm_provider')}
    represented_slugs = {slug for row in snapshot['provider_template_research'] for slug in row['metadata'].get('catalog_import', {}).get('slugs', [])}
    provider_observations = {slug: row['metadata']['catalog_import'] for row in snapshot['provider_template_research'] for slug in row['metadata'].get('catalog_import', {}).get('slugs', [])}
    configurations = canonical(json.loads((args.source / 'litellm/llms/openai_like/providers.json').read_text(), parse_float=Decimal))
    for slug, value in configurations.items():
        if provider_observations.get(slug, {}).get('configurations', {}).get(slug) != value:
            errors.append({'kind': 'provider_configuration_mismatch', 'identifier': slug})
    field_definitions = canonical(json.loads((args.source / 'litellm/proxy/public_endpoints/provider_create_fields.json').read_text(), parse_float=Decimal))
    checked_fields = 0
    for value in field_definitions:
        slug = value['litellm_provider']
        if slug in represented_slugs:
            checked_fields += 1
            if value not in provider_observations[slug].get('create_field_variants', {}).get(slug, []):
                errors.append({'kind': 'provider_credential_fields_mismatch', 'identifier': slug})
    import_data = json.loads(Path('libs/postgres/src/migration/m20261122_000054_catalog_search/import.json').read_text())
    exclusions = set(import_data['manifest']['excluded_identifiers'])
    for slug in sorted(provider_slugs - represented_slugs - exclusions):
        errors.append({'kind': 'missing_provider_identifier', 'identifier': slug})
    report = {
        'registered_identifiers': len(provider_slugs),
        'provider_configurations_checked': len(configurations),
        'provider_credential_definitions_checked': checked_fields,
        'represented_provider_identifiers': len(provider_slugs & represented_slugs),
        'excluded_nonprovider_identifiers': sorted(exclusions),
        'source_entries': len(expected), 'stored_source_entries': len(observed),
        'providers': len(snapshot['provider_templates']), 'global_models': len(models),
        'aliases': sum('alias_of' in v for v in expected.values()),
        'missing_or_mismatched': errors,
        'runtime_mapping': {
            'imported_models': len(imported),
            'without_connections': sum(not r['connections'] for r in imported),
            'without_context': sum(r['limits']['contextTokens'] is None for r in imported),
            'providers_without_base_url': sum(not p['connection']['baseUrl'] for p in snapshot['provider_templates']),
        },
    }
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))
    if errors:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
