#!/usr/bin/env python3
"""Propose source-pinned links from cached D&D Beyond classes to the SRD catalogue.

Usage: dndbeyond_class_links.py CACHE_DIRECTORY [--check]

Reads local character cache records and writes builtin/dndbeyond/class-links.json. Only classes
named in CLASSES are linked, because a name alone does not prove the edition: add a class's
D&D Beyond ID after you have compared its source with the SRD.

Within a reviewed class, this proposes a feature's rule from a source-matched conversion, or else
from its name and level. Those proposals need review. Printed similarity scores compare each linked
feature's D&D Beyond text with the SRD text, lowest first. Read the low ones before you commit.
Subclass and feat links are written by hand, and a rerun keeps them.
"""
import difflib
import hashlib
import html
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / 'builtin/dndbeyond/class-links.json'
CONVERSIONS = ROOT / 'builtin/dndbeyond/conversions.json'
DEFINITIONS = ROOT / 'builtin/dnd2024-definitions'

# D&D Beyond class definition IDs reviewed against the 2024 SRD.
CLASSES = {2190875: 'barbarian', 2190886: 'wizard'}
FEATURE_KEYS = ('id', 'entityTypeId', 'definitionKey', 'name', 'description', 'snippet', 'sources',
                'isLegacy', 'level', 'damage', 'damageType', 'properties', 'range', 'longRange',
                'armorClass', 'armorTypeId', 'components', 'duration', 'activation')
CLASS_KEYS = ('id', 'definitionKey', 'name', 'sources')
OPTION_KEYS = ('id', 'label', 'description', 'sourceId')


def pin(source, keys):
    selected = {key: source.get(key) for key in sorted(keys)}
    data = json.dumps(selected, sort_keys=True, separators=(',', ':'), ensure_ascii=False)
    return hashlib.sha256(data.encode()).hexdigest()


def slug(name):
    return re.sub(r'[^a-z0-9]+', '_', name.lower().replace('’', '_').replace("'", '_')).strip('_')


def text(markup):
    return ' '.join(html.unescape(re.sub(r'<[^>]+>', ' ', markup or '')).split())


def catalogue_text(key):
    path = DEFINITIONS / (key.split(':', 1)[1] + '.md')
    body = path.read_text(encoding='utf-8')
    return ' '.join(body.split('## Source text', 1)[-1].split('## Rule', 1)[0].split())


def rule_for(class_name, feature, conversions):
    name = re.sub(r'^\d+:\s*', '', feature['name'])
    if name.lower() == f'core {class_name} traits':
        return f'dnd2024:classes/{class_name}'
    conversion = conversions.get(f"feature:{feature['id']}", {})
    if conversion.get('builtin') and conversion['source_sha256'] == pin(feature, FEATURE_KEYS):
        return conversion['builtin']
    if name == 'Weapon Mastery':  # Later rows raise the same rule's count.
        return f'dnd2024:class_features/{class_name}_level_1_weapon_mastery'
    level = 4 if name == 'Ability Score Improvement' else feature['requiredLevel']
    key = f'dnd2024:class_features/{class_name}_level_{level}_{slug(name)}'
    return key if (DEFINITIONS / (key.split(':', 1)[1] + '.md')).exists() else None


def options_for(record, choice):
    definitions = record['choices'].get('choiceDefinitions', [])
    wanted = f"{choice['componentTypeId']}-{choice['type']}"
    allowed = set(choice.get('optionIds') or [])
    return [option for definition in definitions if definition['id'] == wanted
            for option in definition['options'] if option['id'] in allowed]


def weapon_value(label):
    match = re.fullmatch(r"(.+) \((\w+)\)", label)
    if not match:
        raise SystemExit(f'Unrecognized Weapon Mastery option: {label}')
    path = DEFINITIONS / 'equipment' / f'{slug(match[1])}.md'
    # D&D Beyond also offers weapons outside the SRD, such as firearms. Those stay unlinked.
    if not path.exists():
        return None
    if f'` mastery ` | ` {match[2].lower()} `' not in path.read_text('utf-8'):
        raise SystemExit(f'{label} has a different mastery in the catalogue')
    return f'dnd2024:equipment/{slug(match[1])}'


def link_class(record, entry, conversions, report, existing):
    definition = entry['definition']
    class_name = CLASSES[definition['id']]
    features = {}
    for item in entry.get('classFeatures', []):
        feature = item['definition']
        rule = rule_for(class_name, feature, conversions)
        features[str(feature['id'])] = {
            'name': feature['name'],
            'source_sha256': pin(feature, FEATURE_KEYS),
            'required_level': feature.get('requiredLevel'),
            'rule': rule,
        }
        if rule and rule.startswith('dnd2024:class_features/'):
            score = difflib.SequenceMatcher(
                None, text(feature['description']), catalogue_text(rule), autojunk=False).ratio()
            report.append((round(score, 2), class_name, feature['name'], rule))
    mastery = f'dnd2024:class_features/{class_name}_level_1_weapon_mastery'
    for feat in record.get('feats', []):
        source = feat['definition']
        if conversions.get(f"feature:{source['id']}", {}).get('builtin') == mastery:
            features[str(source['id'])] = {
                'name': source['name'],
                'source_sha256': pin(source, FEATURE_KEYS),
                'required_level': source.get('requiredLevel'),
                'rule': mastery,
            }
    skills, weapons = {}, {}
    for choice in (c for group in ('class', 'feat') for c in record['choices'].get(group, [])):
        linked = features.get(str(choice['componentId']))
        if not linked:
            continue
        for option in options_for(record, choice):
            value = {'label': option['label'], 'source_sha256': pin(option, OPTION_KEYS)}
            if linked['rule'] == f'dnd2024:classes/{class_name}' and choice['type'] == 2:
                skills[str(option['id'])] = value | {'value': slug(option['label'])}
            elif linked['rule'] == mastery and choice['type'] == 3:
                weapon = weapon_value(option['label'])
                if weapon:
                    weapons[str(option['id'])] = value | {'value': weapon}
    return str(definition['id']), {
        'name': definition['name'],
        'class': class_name,
        'source_sha256': pin(definition, CLASS_KEYS),
        'features': features,
        'skills': skills,
        'weapons': weapons,
        'subclasses': existing.get('subclasses', {}),
        'feats': existing.get('feats', {}),
    }


def main():
    arguments = [argument for argument in sys.argv[1:] if argument != '--check']
    if len(arguments) != 1:
        raise SystemExit(__doc__)
    conversions = json.loads(CONVERSIONS.read_text('utf-8'))['sources']
    previous = json.loads(OUTPUT.read_text('utf-8'))['classes'] if OUTPUT.exists() else {}
    classes, report = {}, []
    for path in sorted(Path(arguments[0]).rglob('*.json')):
        record = json.loads(path.read_text('utf-8'))
        if not isinstance(record, dict) or 'source' not in record:
            continue
        raw = record['source']['raw']
        for entry in raw.get('classes', []):
            if entry['definition']['id'] in CLASSES:
                existing = previous.get(str(entry['definition']['id']), {})
                key, value = link_class(raw, entry, conversions, report, existing)
                if classes.get(key, value) != value:
                    raise SystemExit(f"Cached records disagree about class {value['name']}")
                classes[key] = value
    output = json.dumps({'version': 1, 'classes': classes}, indent=2, ensure_ascii=False,
                        sort_keys=True) + '\n'
    for score, class_name, name, rule in sorted(report):
        print(f'{score:5} {class_name:10} {name:32} {rule}')
    unmatched = [(key, feature['name']) for key, value in classes.items()
                 for feature in value['features'].values() if not feature['rule']]
    print('Features with no rule:', [name for _, name in unmatched])
    if '--check' in sys.argv:
        if not OUTPUT.exists() or OUTPUT.read_text('utf-8') != output:
            raise SystemExit('class-links.json is out of date')
        return
    OUTPUT.write_text(output, 'utf-8')
    print(f'Wrote {OUTPUT.relative_to(ROOT)}')


if __name__ == '__main__':
    main()
