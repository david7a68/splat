"""Write lossless, marked Markdown data. Headings are presentation only."""
import json
import re
import yaml

DUMP = getattr(yaml, 'CSafeDumper', yaml.SafeDumper)
IDENTITY = {'splat_schema', 'splat_id', 'splat_kind', 'splat_key', 'splat_name',
            'splat_origin', 'splat_sources', 'tags'}
REPORT_FIELDS = {'splat_conversion', 'splat_clauses', 'splat_contracts'}


def compact(value):
    return value is None or isinstance(value, (str, bool, int, float)) or (
        isinstance(value, list) and bool(value) and all(v is None or isinstance(v, (str, bool, int, float)) for v in value))


def cell(value):
    # JSON is a YAML subset. Quote strings only when needed to preserve their
    # type or Markdown syntax. Code spans prevent Markdown interpreting values.
    if isinstance(value, str) and value and value.strip() == value and not any(c in value for c in ['\n', '|', '\\', '`']):
        try:
            plain = yaml.safe_load(value)
        except yaml.YAMLError:
            plain = None
        if plain == value and isinstance(plain, str):
            encoded = value
        else:
            encoded = json.dumps(value, ensure_ascii=False)
    else:
        encoded = json.dumps(value, ensure_ascii=False)
    encoded = encoded.replace('|', r'\u007c')
    fence = '`' * (max([len(s) for s in re.findall(r'`+', encoded)] or [0]) + 1)
    return fence + ' ' + encoded + ' ' + fence


def title(path):
    return path.rsplit('/', 1)[-1].removeprefix('splat_').replace('_', ' ').capitalize()


def table(mode, path, headers, rows):
    return (f'<!-- splat:{mode} path={path} -->\n\n'
            + '| ' + ' | '.join(headers) + ' |\n'
            + '| ' + ' | '.join('---' for _ in headers) + ' |\n'
            + ''.join('| ' + ' | '.join(cell(v) if v is not MISSING else '' for v in row) + ' |\n' for row in rows) + '\n')


MISSING = object()


def block(path, value):
    raw = yaml.dump(value, Dumper=DUMP, sort_keys=False, allow_unicode=True, width=100)
    fence = '`' * max(3, 1 + max([len(s) for s in re.findall(r'`+', raw)] or [0]))
    return f'<!-- splat:yaml path={path} -->\n\n{fence}yaml\n{raw}{fence}\n\n'


def render_value(path, value, depth=2):
    heading = '#' * min(depth, 6) + ' ' + title(path) + '\n\n'
    if path == 'splat_stats' and isinstance(value, dict):
        abilities = ['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma']
        selected = [key for key in abilities if set(value.get(key, {})) == {'value'} and
                    ('save_'+key not in value or set(value['save_'+key]) == {'value'})]
        if selected:
            rows = [[key.capitalize(), value[key]['value'], value.get('save_'+key, {}).get('value', MISSING)] for key in selected]
            remainder = {k:v for k,v in value.items() if k not in selected and k not in ['save_'+a for a in selected]}
            result = '#' * min(depth, 6) + ' Abilities\n\n' + table('abilities', path, ['Ability', 'Score', 'Save override'], rows)
            if remainder: result += render_value('splat_stats', remainder, depth)
            return result
    if isinstance(value, dict) and value:
        if any(not re.fullmatch(r'[a-z][a-z0-9_-]*', key) for key in value):
            return heading + block(path, value)
        # Stat/resource maps with uniform simple rows.
        if path in {'splat_stats', 'splat_resources'} and all(isinstance(v, dict) and v for v in value.values()):
            columns = list(dict.fromkeys(k for row in value.values() for k, v in row.items() if compact(v)))
            if columns and all(any(compact(v) for v in row.values()) for row in value.values()):
                rows = [[k] + [v.get(c, MISSING) if compact(v.get(c)) else MISSING for c in columns] for k,v in value.items()]
                result = heading + table('map', path, ['Key'] + columns, rows)
                for key, row in value.items():
                    for k, v in row.items():
                        if not compact(v): result += render_value(path+'/'+key+'/'+k, v, depth+1)
                return result
        simple = {k: v for k,v in value.items() if compact(v)}
        complex_values = {k: v for k,v in value.items() if not compact(v)}
        # Keep deeper mechanics together; split features and top-level data.
        if depth <= 3 or not complex_values or path.endswith(('/features', '/senses', '/defenses', '/communication')):
            result = heading
            if simple: result += table('values', path, ['Field', 'Value'], list(simple.items()))
            for k,v in complex_values.items(): result += render_value(path+'/'+k, v, depth+1)
            return result
    if isinstance(value, list) and value and all(isinstance(row, dict) and row for row in value):
        columns = list(dict.fromkeys(k for row in value for k,v in row.items() if compact(v)))
        if columns and all(any(k in row and compact(row[k]) for k in columns) for row in value):
            result = heading + table('rows', path, ['Row'] + columns,
                [[i] + [row.get(c, MISSING) if compact(row.get(c)) else MISSING for c in columns] for i,row in enumerate(value)])
            for i,row in enumerate(value):
                for k,v in row.items():
                    if not compact(v): result += render_value(path+'/'+str(i)+'/'+k, v, depth+1)
            return result
    return heading + block(path, value)


def render_note(front, body):
    meta = {k:v for k,v in front.items() if k in IDENTITY or not k.startswith('splat_')}
    fields = {k:v for k,v in front.items() if k not in meta and k not in REPORT_FIELDS}
    implementation = fields.get('splat_rule', {}).get('implementation')
    if implementation and fields.get('splat_requires_mechanics') == [implementation]:
        del fields['splat_requires_mechanics']
    parts = ['---\n', yaml.dump(meta,Dumper=DUMP,sort_keys=False,allow_unicode=True,width=100), '---\n\n', body.strip().split('\n',1)[0], '\n\n## Source text\n\n', body.strip().split('\n',1)[1].lstrip() if '\n' in body.strip() else '', '\n\n']
    for key,value in fields.items(): parts.append(render_value(key,value))
    return ''.join(parts).rstrip() + '\n'
