"""Plan readable columns from record shapes; no D&D-specific parser rules."""
import re
import json
import uuid
from pathlib import PurePosixPath
import yaml

KEY = re.compile(r'[a-z][a-z0-9_-]*\Z')
LABELS = {'id':'Variant', 'name':'Name', 'item':'Item', 'definition':'Item', 'spell':'Spell',
          'mass':'Weight', 'price':'Cost', 'purchase_quantity':'Quantity', 'quantity':'Quantity',
          'heal_hp/roll':'Healing', 'cast_level':'Cast level', 'result/creature':'Creature',
          'result/damage_type':'Damage type', 'storage':'Storage', 'usage/kind':'Frequency',
          'usage/uses':'Uses', 'usage/recovery':'Recovery', 'usage/pool':'Use pool'}


def get(row,path):
    for part in path.split('/'):
        if not isinstance(row,dict) or part not in row: return None,False
        row=row[part]
    return row,True


def leaf_kind(value):
    if value is None:return 'null'
    if isinstance(value,dict):
        if set(value) in ({'key'},{'id'}) and isinstance(next(iter(value.values())),str):
            identity=next(iter(value.values()))
            if 'key' in value and re.fullmatch(r'[a-z0-9_-]+:(?:[a-z0-9_-]+/)*[a-z0-9_-]+',identity):return 'reference'
            if 'id' in value:
                try: uuid.UUID(identity);return 'reference'
                except ValueError:pass
        if set(value)=={'value','unit'} and isinstance(value['value'],(int,float)) and not isinstance(value['value'],bool) and isinstance(value['unit'],str): return 'quantity'
        if set(value)=={'count','sides','add'} and all(type(v)==int for v in value.values()): return 'dice'
    if type(value)==bool: return 'boolean'
    if type(value)==int: return 'integer'
    if type(value)==float: return 'number'
    if isinstance(value,str) and value and value.strip()==value and '\n' not in value and len(value)<140: return 'string'
    if isinstance(value,list) and len(value)<=5 and all(not isinstance(v,(dict,list)) for v in value): return 'yaml'
    return None


def leaves(value,path='',depth=0):
    kind=leaf_kind(value)
    if kind: return {path:kind}
    if isinstance(value,dict) and depth<3 and value and all(KEY.fullmatch(k) for k in value):
        result={}
        for k,v in value.items(): result.update(leaves(v,path+'/'+k if path else k,depth+1))
        return result
    return {}


def plan(path,rows):
    if not isinstance(rows,list) or len(rows)<2 or not all(isinstance(r,dict) and r for r in rows): return None
    # Do not turn command sequences into comparison tables.
    if any(any(k in r for k in ('op','operation','steps')) for r in rows): return None
    candidates={}
    for row in rows:
        for field,kind in leaves(row).items(): candidates.setdefault(field,set()).add(kind)
    nullable={p for p,kinds in candidates.items() if 'null' in kinds}
    candidates={p:next(iter(kinds-{'null'}),'yaml') for p,kinds in candidates.items() if len(kinds-{'null'})<=1}
    candidates={p:k for p,k in candidates.items() if all(not present or leaf_kind(v) is not None for v,present in (get(row,p) for row in rows))}
    # A field cannot be both a compound value and a path prefix in other rows.
    candidates={p:k for p,k in candidates.items() if not any(q!=p and (q.startswith(p+'/') or p.startswith(q+'/')) for q in candidates)}
    if not candidates or len(candidates)>10: return None
    if any(not any(get(row,p)[1] for p in candidates) for row in rows): return None
    columns={}; identity=None; ref_identity=None
    for p in ['id','name','key']:
        values=[get(r,p)[0] for r in rows]
        if p in candidates and all(isinstance(v,str) and KEY.fullmatch(v) for v in values) and len(set(values))==len(values):
            identity=p; keys=values; break
    if identity is None:
        # References offer stable local keys without copying their full URI into
        # a second visible column. Fall back to persistent generated row keys.
        ref=next((p for p,k in candidates.items() if k=='reference' and all(get(r,p)[1] for r in rows)),None)
        keys=[re.sub('[^a-z0-9_-]+','_',next(iter(get(r,ref)[0].values())).rsplit('/',1)[-1]).strip('_') for r in rows] if ref else []
        if not keys or any(not KEY.fullmatch(k) for k in keys) or len(set(keys))!=len(keys): keys=[f'row_{i+1}' for i in range(len(rows))]
        if ref and len(set(keys))==len(keys) and all(KEY.fullmatch(k) for k in keys) and keys != [f'row_{i+1}' for i in range(len(rows))]: ref_identity=ref
        else: columns['Key']={'type':'key'}
    # Identity comes first even if it was late in the source mapping.
    paths=list(candidates)
    first=identity or ref_identity
    if first: paths.remove(first); paths.insert(0,first)
    if 'min' in paths and 'max' in paths and all(type(r.get('min'))==int and type(r.get('max'))==int and 0<=r['min']<=r['max'] for r in rows):
        paths.remove('min');paths.remove('max')
        columns['Roll']={'path':'','type':'range'}
    for p in paths:
        label=LABELS.get(p,p.replace('/',' ').replace('_',' ').capitalize())
        if label in columns: label=p.replace('/',' ').replace('_',' ').capitalize()
        if label in columns: return None
        kind=candidates[p]
        # Human labels round-trip simple schema symbols; unrestricted text stays literal.
        values=[get(r,p)[0] for r in rows if get(r,p)[1]]
        if kind=='string' and all(v is None or KEY.fullmatch(v) for v in values):kind='symbol'
        column={'path':p,'type':kind}
        if p in nullable:column['nullable']=True
        if len(values)!=len(rows):column['optional']=True
        columns[label]=column
    key_label=next((label for label,c in columns.items() if c.get('path')==(identity or ref_identity)),'Key') if identity or ref_identity else 'Key'
    import copy
    has_details=False
    for row in rows:
        remainder=copy.deepcopy(row)
        for column in columns.values():
            if column.get('path')=='':
                for field in ['min','max']:remove_path(remainder,field)
            elif column.get('path'):remove_path(remainder,column['path'])
        has_details |= bool(remainder)
    if has_details:columns['Details']={'type':'section','optional':True}
    return {'path':path,'key':key_label,'order':keys,'columns':columns}


def label_ref(identity,names):
    return names.get(identity, identity.rsplit('/',1)[-1].replace('_',' ').capitalize())


def inline(value):
    # Code spans protect literal strings; unlike field-name table cells these are typed,
    # so strings need no YAML quoting. Newlines are excluded by planning.
    value=str(value).replace('|','\\|')
    fence='`'*(max([len(s) for s in re.findall('`+',value)] or [0])+1)
    return fence+' '+value+' '+fence


def format_cell(value,column,names,current):
    kind=column['type']
    if value is None:return 'null'
    if kind=='string' and (value.startswith(chr(34)) or any(c in value for c in ['|','\\','`']) or value=='null' and column.get('nullable')):return inline(json.dumps(value,ensure_ascii=False).replace('|',r'\u007c'))
    if kind=='reference':
        identity=next(iter(value.values())); target=names.get('__path__'+identity)
        if target:
            import posixpath
            target=posixpath.relpath(target,str(PurePosixPath(current).parent))
            label=label_ref(identity,names).replace('[','\\[').replace(']','\\]').replace('|','\\|')
            return f'[{label}](<{target}> "{identity}")'
        return inline(identity)
    if kind=='quantity':
        amount=value['value']; unit=value['unit']
        amount=str(amount)
        if amount.endswith('.5'):amount=amount[:-2]+'½'
        return inline(amount+' '+(unit.upper() if unit in ['gp','sp','cp','ep','pp'] else unit))
    if kind=='range':return inline(str(value['min']) if value['min']==value['max'] else f"{value['min']}–{value['max']}")
    if kind=='dice':return inline(f"{value['count']}d{value['sides']}"+(f"{value['add']:+}" if value['add'] else ''))
    if kind=='symbol':return inline(value.replace('_',' ').capitalize())
    if kind=='boolean':return str(value).lower()
    if kind=='yaml':return inline(json.dumps(value,ensure_ascii=False).replace('|',r'\u007c'))
    return inline(value)


def remove_path(row,path):
    head,*tail=path.split('/')
    if not isinstance(row,dict) or head not in row:return False
    if tail:
        if not remove_path(row[head],'/'.join(tail)):return False
        if not row[head]:del row[head]
    else:del row[head]
    return True


def section_anchor(path,key):
    import hashlib
    return '^splat-'+hashlib.sha256(path.encode()).hexdigest()[:8]+'-'+key


def render(mapping,rows,names,current,render_child,depth):
    import copy
    result='<!-- splat:table\n'+yaml.dump(mapping,Dumper=yaml.CSafeDumper,sort_keys=False,allow_unicode=True)+'-->\n\n'
    labels=list(mapping['columns'])
    result+='| '+' | '.join(labels)+' |\n| '+' | '.join(['---']*len(labels))+' |\n'
    remainders=[]
    for key,row in zip(mapping['order'],rows):
        cells=[]; remainder=copy.deepcopy(row)
        for column in mapping['columns'].values():
            if column['type']=='section':
                remaining_row=copy.deepcopy(row)
                for c in mapping['columns'].values():
                    if c.get('path')=='':
                        for field in ['min','max']:remove_path(remaining_row,field)
                    elif c.get('path'):remove_path(remaining_row,c['path'])
                anchor=section_anchor(mapping['path'],key)
                cells.append(f'[Rules](#{anchor} "section:{key}")' if remaining_row else '')
                continue
            if 'path' not in column:cells.append(inline(key));continue
            value,present=(row,True) if column['path']=='' else get(row,column['path'])
            cells.append(format_cell(value,column,names,current) if present else '')
            if present:
                if column['path']=='':
                    for field in column.get('fields',['min','max']):remove_path(remainder,field)
                else:remove_path(remainder,column['path'])
        result+='| '+' | '.join(cells)+' |\n'
        remainders.append((key,remainder))
    result+='\n'
    for key,remainder in remainders:
        if remainder:
            result+='#'*min(depth+1,6)+' '+key.replace('_',' ').capitalize()+'\n\n^'+section_anchor(mapping['path'],key)[1:]+'\n\n'
            def remaining(field,value,level):
                if isinstance(value,dict) and any(c.get('path','').startswith(field+'/') for c in mapping['columns'].values()):
                    return ''.join(remaining(field+'/'+child,v,level) for child,v in value.items())
                return render_child(mapping['path']+'/@'+key+'/'+field,value,level)
            for field,value in remainder.items():result+=remaining(field,value,depth+2)
    return result
