"""Read reviewed SRD rule specifications and validate their data-only boundary.

Research Markdown is the authored source for this pass. Generated definition
notes retain the complete original prose separately from these typed fields.
No declaration in this module supplies an engine implementation.
"""
import copy
import hashlib
import json
import re
from pathlib import Path

import yaml

RESEARCH = Path('docs/research/srd-conversion')
INPUT_TYPES = {
    'participant', 'participant_list', 'item_instance', 'item_list', 'object',
    'object_list', 'location', 'location_list', 'integer', 'number', 'boolean',
    'choice', 'effect', 'resolution', 'event', 'campaign', 'duration', 'definition',
    'definition_list', 'text',
}
IDENTIFIER = re.compile(r'[a-z0-9_-]+:(?:[a-z0-9_-]+/)*[a-z0-9_-]+\Z')


class UniqueLoader(yaml.CSafeLoader):
    """PyYAML normally accepts duplicate keys. Content may not."""


def unique_mapping(loader, node, deep=False):
    loader.flatten_mapping(node)
    result = {}
    for key_node, value_node in node.value:
        key = loader.construct_object(key_node, deep=deep)
        if not isinstance(key, str):
            # Rule tables can have integer lookup keys; normalize them as YAML
            # object keys while preserving their value for schema validation.
            if isinstance(key, int) and not isinstance(key, bool):
                key = str(key)
            else:
                raise ValueError(f'Unsupported mapping key {key!r}')
        if key in result:
            raise ValueError(f'Duplicate YAML key {key}')
        result[key] = loader.construct_object(value_node, deep=deep)
    return result


UniqueLoader.add_constructor(yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, unique_mapping)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False, allow_nan=False)


def fingerprint(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def merge_fields(base, additions):
    """Merge mappings; lists and scalar values are replaced deliberately."""
    result = copy.deepcopy(base)
    for key, value in additions.items():
        if isinstance(result.get(key), dict) and isinstance(value, dict):
            result[key] = merge_fields(result[key], value)
        else:
            result[key] = copy.deepcopy(value)
    return result


def json_shape(value):
    """A closed structural type for supplied parameters, not inferred semantics."""
    if value is None:
        return {'type': 'null'}
    if isinstance(value, bool):
        return {'type': 'boolean'}
    if isinstance(value, int):
        return {'type': 'integer'}
    if isinstance(value, float):
        canonical(value)  # rejects non-finite numbers
        return {'type': 'number'}
    if isinstance(value, str):
        return {'type': 'string'}
    if isinstance(value, list):
        variants = {canonical(json_shape(item)): json_shape(item) for item in value}
        return {'type': 'array', 'items': list(variants.values())}
    if isinstance(value, dict) and all(isinstance(key, str) for key in value):
        return {'type': 'object', 'properties': {key: json_shape(item) for key, item in value.items()}}
    raise ValueError(f'Unsupported contract data type: {type(value).__name__}')


def check_shape(value, shape, location='parameters'):
    kind = shape['type']
    if kind == 'object':
        if not isinstance(value, dict) or value.keys() != shape['properties'].keys():
            raise ValueError(f'{location}: object fields differ from its contract')
        for key, child in shape['properties'].items():
            check_shape(value[key], child, location+'/'+key)
    elif kind == 'array':
        if not isinstance(value, list):
            raise ValueError(f'{location}: expected array')
        for item in value:
            for variant in shape['items']:
                try:
                    check_shape(item, variant, location+'[]')
                    break
                except ValueError:
                    pass
            else:
                raise ValueError(f'{location}: array item does not match its contract')
    elif json_shape(value)['type'] != kind:
        raise ValueError(f'{location}: expected {kind}')


def specification_fields(fields):
    """Only mechanical fields enter the contract fingerprint."""
    return {key: value for key, value in fields.items()
            if key.startswith('splat_') and key not in {
                'splat_schema', 'splat_id', 'splat_kind', 'splat_key', 'splat_name',
                'splat_origin', 'splat_sources', 'splat_conversion', 'splat_clauses',
                'splat_requires_mechanics', 'splat_contracts',
            }}


def normalize_input(value):
    if isinstance(value, str):
        value = {'type': value, 'required': True}
    elif isinstance(value, dict):
        value = {'required': True, **value}
    if not isinstance(value, dict) or set(value) != {'type', 'required'}:
        raise ValueError('Contract input must specify type and required')
    if value['type'] not in INPUT_TYPES or not isinstance(value['required'], bool):
        raise ValueError('Unknown contract input type: '+str(value))
    return value



def expression_bindings(value):
    if isinstance(value, dict):
        if set(value) == {'binding'}:
            yield value['binding']
        for child in value.values():
            yield from expression_bindings(child)
    elif isinstance(value, list):
        for child in value:
            yield from expression_bindings(child)


def read_research():
    definitions = {}
    for path in sorted(RESEARCH.glob('*.md')):
        text = path.read_text()
        if not text.startswith('---\n') or '\n---\n' not in text[4:]:
            raise ValueError('Research file requires YAML frontmatter: '+str(path))
        data = yaml.load(text.split('---', 2)[1], Loader=UniqueLoader)
        if data.get('format') != 1 or data.get('source') != 'SRD 5.2.1':
            raise ValueError('Unsupported research format: '+str(path))
        for key, record in data['definitions'].items():
            if key in definitions:
                raise ValueError('Duplicate researched definition: '+key)
            if not IDENTIFIER.fullmatch(key):
                raise ValueError('Invalid definition key: '+key)
            if record.get('disposition') not in ['specified', 'informational']:
                raise ValueError('Incomplete research disposition: '+key)
            if not re.fullmatch('[0-9a-f]{64}', record.get('source_sha256', '')):
                raise ValueError('Research requires exact source hash: '+key)
            if not record.get('citations') or not all(isinstance(c, str) for c in record['citations']):
                raise ValueError('Research requires source citations: '+key)
            fields = record.get('fields', {})
            if not isinstance(fields, dict) or not all(k.startswith('splat_') for k in fields):
                raise ValueError('Research fields must be schema fields: '+key)
            if any(k in fields for k in ['splat_id', 'splat_key', 'splat_schema', 'splat_origin', 'splat_conversion']):
                raise ValueError('Research may not replace definition identity/status: '+key)
            json_shape(fields)
            if record['disposition'] == 'specified':
                contract = record.get('contract')
                if not isinstance(contract, dict):
                    raise ValueError('Specified definition requires an interface: '+key)
                required_fields = {'id', 'inputs', 'outputs', 'events', 'capabilities'}
                if set(contract) != required_fields:
                    raise ValueError('Contract interface fields differ: '+key+' '+str(set(contract)))
                if not IDENTIFIER.fullmatch(contract['id']):
                    raise ValueError('Invalid implementation ID: '+key)
                contract['inputs'] = {k: normalize_input(v) for k, v in contract['inputs'].items()}
                for name in ['outputs', 'events', 'capabilities']:
                    if not isinstance(contract[name], list) or not all(isinstance(s, str) and s for s in contract[name]):
                        raise ValueError('Invalid interface '+name+': '+key)
                if not contract['capabilities']:
                    raise ValueError('Contract must identify missing engine capabilities: '+key)
                undeclared = set(expression_bindings(fields)) - contract['inputs'].keys()
                if undeclared:
                    raise ValueError('Undeclared expression inputs: '+key+' '+str(sorted(undeclared)))
                if not fields:
                    raise ValueError('Specified definition cannot be only a contract name: '+key)
                if not record.get('requires') or contract['id'] not in record['requires']:
                    raise ValueError('Implementation must be a required mechanic: '+key)
            elif record.get('requires') or record.get('contract'):
                raise ValueError('Informational note cannot claim a required engine implementation: '+key)
            definitions[key] = {**record, 'research_path': str(path)}
    return definitions


def contract_record(key, spec, fields):
    """A complete source-specific contract with a closed parameter schema."""
    data = specification_fields(fields)
    return {
        **spec['contract'],
        'definition': key,
        'source_sha256': spec['source_sha256'],
        'research_path': spec['research_path'],
        'citations': spec['citations'],
        'implemented': False,
        'specification_sha256': fingerprint(data),
        'schema': json_shape(data),
    }
