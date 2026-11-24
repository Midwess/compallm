import argparse
import hashlib
import json
import re
from collections import defaultdict
from decimal import Decimal, InvalidOperation
from pathlib import Path


def rate(value):
    try:
        amount = Decimal(str(value)) * 1_000_000
        if not amount.is_finite() or amount < 0 or amount >= Decimal('1e12'):
            return None
        text = format(amount.normalize(), 'f')
        if amount.as_tuple().exponent < -18 or len(amount.as_tuple().digits) > 28:
            return None
        return {'amount': text, 'currency': 'USD', 'quantity': 1_000_000}
    except (InvalidOperation, ValueError):
        return None


def token(raw, direction, suffix=''):
    main = rate(raw.get(direction + '_cost_per_token' + suffix))
    cached = rate(raw.get('cache_read_input_token_cost' + suffix)) if direction == 'input' else None
    if main is None and cached is None:
        return None
    return {'rate': main or {'amount': None, 'currency': 'USD', 'quantity': 1_000_000},
            'cache': {'type': 'separate_rate', 'configuration': {'rate': cached}} if cached else None}


def pricing(raw, direction, suffix):
    base = token(raw, direction, suffix)
    if suffix == '' and isinstance(raw.get('tiered_pricing'), list):
        tiers = []
        previous = 0
        for item in raw['tiered_pricing']:
            bounds = item.get('range', [])
            if len(bounds) != 2:
                return None
            start, end = (Decimal(str(v)) for v in bounds)
            value = token(item, direction)
            if value is None or start != previous or start != int(start) or end != int(end) or end <= start:
                return None
            tiers.append({'fromTokens': int(start), 'toTokens': int(end), 'rate': value})
            previous = end
        if not tiers:
            return None
        tiers[-1]['toTokens'] = None
        return {'type': 'usage', 'configuration': {'tiered_token': {'measuredBy': 'input_tokens', 'tiers': tiers}}}
    thresholds = {}
    for key in raw:
        match = re.fullmatch(r'(?:input_cost_per_token|output_cost_per_token|cache_read_input_token_cost)_above_(\d+)(k?)_tokens' + re.escape(suffix), key)
        if match:
            bound = int(match[1]) * (1000 if match[2] else 1)
            if raw.get('litellm_provider') == 'xai':
                bound -= 1
            thresholds[bound] = '_above_' + match[1] + match[2] + '_tokens' + suffix
    if thresholds:
        if base is None:
            return None
        tiers = []
        start = 0
        current = base
        for bound, threshold_suffix in sorted(thresholds.items()):
            tiers.append({'fromTokens': start, 'toTokens': bound, 'rate': current})
            next_rate = token(raw, direction, threshold_suffix)
            if next_rate is not None:
                if next_rate['rate']['amount'] is None:
                    next_rate['rate'] = current['rate']
                if next_rate['cache'] is None:
                    next_rate['cache'] = current['cache']
                current = next_rate
            start = bound
        tiers.append({'fromTokens': start, 'toTokens': None, 'rate': current})
        return {'type': 'usage', 'configuration': {'tiered_token': {'measuredBy': 'input_tokens', 'tiers': tiers}}}
    return {'type': 'usage', 'configuration': {'fixed': {'token': base}}} if base else None


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    manifest = json.loads(Path('libs/postgres/src/migration/m20261122_000054_catalog_search/import.json').read_text())['manifest']
    path = Path('.dev/litellm/model_prices_and_context_window.json')
    if hashlib.sha256(path.read_bytes()).hexdigest() != manifest['source_hashes']['model_prices_and_context_window.json']:
        raise ValueError('Source does not match the pinned import')
    source = json.loads(path.read_text(), parse_float=Decimal)
    groups = defaultdict(list)
    for binding in manifest['bindings']:
        raw = source[binding['key']]
        if raw.get('mode') not in ('chat', 'completion', 'responses'):
            continue
        for suffix in ['', '_priority', '_flex', '_ultrafast', '_balanced', '_decisions', '_batches']:
            if suffix and not any(k.endswith(suffix) and k.startswith(('input_cost_per_token', 'output_cost_per_token')) for k in raw):
                continue
            prices = {feature: value for direction, feature in [('input', 'text_input'), ('output', 'text_output')]
                      if (value := pricing(raw, direction, suffix)) is not None}
            if not prices:
                continue
            variant = {'provider': binding['provider_template_id'], 'model': binding['model_template_id'],
                       'service_tier': suffix[1:] if suffix and suffix != '_batches' else None,
                       'request_mode': 'batch' if suffix == '_batches' else 'standard', 'prices': prices}
            key = (variant['provider'], variant['model'], variant['service_tier'], variant['request_mode'])
            groups[key].append((binding['key'], variant))
    rows = []
    ambiguous = 0
    for values in groups.values():
        unique = {json.dumps(v['prices'], sort_keys=True) for _, v in values}
        if len(unique) != 1:
            ambiguous += 1
            continue
        source_key, variant = values[0]
        rows.append(dict(variant, source_key=source_key))
    args.output.write_text(json.dumps({'revision': manifest['revision'], 'ambiguous_selectors': ambiguous, 'books': rows}, indent=2) + '\n')
    print(json.dumps({'books': len(rows), 'ambiguous_selectors': ambiguous}))


if __name__ == '__main__':
    main()
