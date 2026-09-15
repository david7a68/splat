#!/usr/bin/env python3
"""Convert supported SRD clauses, with complete source coverage and explicit gaps.

Run from the repository root. Requires PyYAML. This deliberately rejects any
unconsumed action text. A numerical attack prefix is not a complete conversion
if its source also imposes a condition, movement, resource or other effect.
"""
from pathlib import Path
import argparse, copy, hashlib, json, re, uuid
import yaml

SOURCE=Path('builtin/srd-5.2.1')
MANUAL=Path('builtin/dnd2024-mechanics')
OUTPUT=MANUAL/'generated'
NAMESPACE=uuid.UUID('71871979-b5d2-59a8-81c8-2081b92bb2fe')
ABILITIES={'Str':'strength','Dex':'dexterity','Con':'constitution','Int':'intelligence','Wis':'wisdom','Cha':'charisma'}
DAMAGE={'acid','bludgeoning','cold','fire','force','lightning','necrotic','piercing','poison','psychic','radiant','slashing','thunder'}
CONDITIONS={'blinded','charmed','deafened','exhaustion','frightened','grappled','incapacitated','invisible','paralyzed','petrified','poisoned','prone','restrained','stunned','unconscious'}
LOAD=getattr(yaml,'CSafeLoader',yaml.SafeLoader)
class Unsupported(ValueError): pass

def load_note(path):
    _,front,body=path.read_text().split('---',2)
    return yaml.load(front,Loader=LOAD),body

def slug(name):return re.sub('[^a-z0-9]+','_',name.lower()).strip('_')
def clean(body):
    body=body.split('Original layout:')[0]
    body=re.sub(r'<!--.*?-->','',body,flags=re.S)
    body=body.replace('\\\n',' ').replace('−','-')
    body=re.sub(r'([a-z])-\s+([a-z])',r'\1\2',body)
    return re.sub(r'\s+',' ',body).strip()
def required(pattern,text,label):
    found=re.search(pattern,text)
    if not found:raise Unsupported('Cannot resolve '+label+' from source')
    return found

def attack(name,bonus,normal,long,damage,kind,ranged=False):
    target={'kind':'creature','count':{'min':1,'max':1},'allow_self':False,'range':{'feet':normal}}
    if long:target['range']['long']=long
    return {'name':name,'activation':{'economy':'action','category':'attack','timing':'on_turn','cost_at':'commit'},
            'targets':{'chosen':target},'rolls':{'attack':{'expression':'1d20','modifier':bonus,'scope':'attack'},'damage':{'expression':damage,'scope':'attack'}},
            'steps':[{'key':'attack','operation':'dnd2024:attack','kind':'ranged_weapon' if ranged else 'melee_weapon','target':'chosen','roll':'attack','on_hit':{'damage':[{'type':kind,'amount':{'roll':'damage'}}]}}]}

def creature(meta,body):
    text=clean(body)
    if any('### '+heading in text for heading in ['Traits','Reactions','Bonus Actions','Legendary Actions']):
        raise Unsupported('Passive traits or additional action windows need rule adapters')
    before,sep,actions_text=text.partition('### Actions ')
    if not sep:raise Unsupported('No actions section')
    if '###' in actions_text:raise Unsupported('Additional source sections after Actions')
    ac=int(required(r'\bAC (\d+)\b',before,'AC')[1]);hp=int(required(r'\bHP (\d+)\b',before,'HP')[1])
    initiative=int(required(r'Initiative\s*([+-]\d+)',before,'initiative')[1])
    stats={}
    for short,score,modifier,save in re.findall(r'\b(Str|Dex|Con|Int|Wis|Cha)\s*(\d+)\s*([+-]\d+)\s*([+-]\d+)',before):
        if (int(score)-10)//2!=int(modifier):raise Unsupported('Ability score/modifier mismatch')
        stats[ABILITIES[short]]={'value':int(score)};stats['save_'+ABILITIES[short]]={'value':int(save)}
    if len(stats)!=12:raise Unsupported('Not all six scores and saves were recovered')
    stats.update(armor_class={'value':ac},max_hp={'value':hp},initiative={'value':initiative})
    size,ctype=required(r'\b(Tiny|Small|Medium|Large|Huge|Gargantuan)(?: or (?:Tiny|Small|Medium|Large))? (\w+),',before,'creature size/type').groups()
    speed=required(r'Speed (.*?)(?:MOD SAVE|Str)',before,'speed')[1].strip()
    movement=re.fullmatch(r'(\d+) ft\.((?:, (?:Burrow|Climb|Fly|Swim) \d+ ft\.)*)(?: \(hover\))?',speed)
    if not movement:raise Unsupported('Special movement rule')
    stats['speed']={'value':int(movement[1]),'unit':'ft'}
    for mode,value in re.findall(r'(Burrow|Climb|Fly|Swim) (\d+) ft\.',movement[2]):stats[mode.lower()+'_speed']={'value':int(value),'unit':'ft'}
    traits={};affinities={}
    for label,key in [('Resistances','resistances'),('Vulnerabilities','vulnerabilities'),('Immunities','immunities')]:
        found=re.search(r'\b'+label+r' (.*?)(?=\b(?:Resistances|Vulnerabilities|Immunities|Gear|Senses|Languages|CR)\b)',before)
        if found:
            values=[v.strip().lower() for v in re.split('[,;]',found[1])]
            if any(v not in DAMAGE|CONDITIONS for v in values):raise Unsupported('Conditional '+label.lower())
            typed=[v for v in values if v in DAMAGE]
            if typed:affinities[key]=typed
            conditions=[v for v in values if v in CONDITIONS]
            if conditions:
                if label!='Immunities':raise Unsupported('Condition affinity is not an immunity')
                traits['condition_immunities']={'rule':'dnd2024:condition_immunities','parameters':{'conditions':conditions}}
    if affinities:traits['damage_affinities']={'rule':'dnd2024:damage_affinities','parameters':affinities}
    multi=None
    if actions_text.startswith('Multiattack. '):
        sentence,_,actions_text=actions_text.partition('. ')
        multi,sep,actions_text=actions_text.partition('. ')
        if not sep:raise Unsupported('Incomplete Multiattack')
    # Full action grammar: consume every character, including trailing clauses.
    pattern=re.compile(r'(?P<name>[A-Za-z][A-Za-z -]*?)\. (?P<mode>Melee or Ranged|Melee|Ranged) Attack Roll: (?P<bonus>[+-]\d+), (?P<range>.*?) Hit: (?P<fixed>\d+)(?: \((?P<dice>[^)]+)\))? (?P<type>[A-Za-z]+) damage\. ?')
    actions={};position=0
    while position<len(actions_text):
        match=pattern.match(actions_text,position)
        if not match:raise Unsupported('Unconverted action clause: '+actions_text[position:position+180])
        position=match.end();g=match.groupdict();name=g['name'];kind=g['type'].lower()
        if kind not in DAMAGE:raise Unsupported('Unknown damage type')
        expr=g['dice'] or g['fixed']
        if not re.fullmatch(r'\d+(?:d\d+)?(?: ?[+-] ?\d+)?',expr):raise Unsupported('Special damage formula')
        reach=re.fullmatch(r'reach (\d+) ft\.',g['range'])
        ranged=re.fullmatch(r'range (\d+)/(\d+) ft\.',g['range'])
        both=re.fullmatch(r'reach (\d+) ft\. or range (\d+)/(\d+) ft\.',g['range'])
        k=slug(name)
        if k in actions:raise Unsupported('Duplicate action name')
        if g['mode']=='Melee' and reach:actions[k]=attack(name,int(g['bonus']),int(reach[1]),None,expr,kind)
        elif g['mode']=='Ranged' and ranged:actions[k]=attack(name,int(g['bonus']),int(ranged[1]),int(ranged[2]),expr,kind,True)
        elif g['mode']=='Melee or Ranged' and both:
            actions[k]=attack(name+' (melee)',int(g['bonus']),int(both[1]),None,expr,kind)
            actions[k+'_ranged']=attack(name+' (ranged)',int(g['bonus']),int(both[2]),int(both[3]),expr,kind,True)
        else:raise Unsupported('Unsupported attack range')
    if not actions:raise Unsupported('Empty actions')
    if multi:
        match=re.fullmatch(r'The [a-z ]+ makes (two|three|four) ([A-Za-z ]+) attacks',multi)
        if not match or slug(match[2]) not in actions:raise Unsupported('Multiattack choices or other operations')
        action=copy.deepcopy(actions[slug(match[2])]);action['name']='Multiattack';action['steps']=[{'key':'multiattack','operation':'splat:repeat','count':{'two':2,'three':3,'four':4}[match[1]],'steps':action['steps']}];actions['multiattack']=action
    return {'splat_creature':{'size':size.lower(),'type':ctype.lower()},'splat_stats':stats,'splat_health':{'zero_hp':'dnd2024:monster'},**({'splat_traits':traits} if traits else {}),'splat_actions':actions}

def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--check',action='store_true');args=ap.parse_args()
    manual={}
    for path in MANUAL.rglob('*.md'):
        if OUTPUT in path.parents:continue
        meta,_=load_note(path)
        origin=meta.get('splat_origin',{}).get('key')
        if origin:manual.setdefault(origin,[]).append(meta['splat_key'])
    records=[];outputs={}
    for path in sorted(SOURCE.rglob('*.md')):
        meta,body=load_note(path);category=meta.get('splat_reference',{}).get('category')
        if category=='package':continue
        source=meta['splat_key'];record={'source':source,'name':meta['splat_name'],'category':category,'pages':meta['splat_reference']['pages'],'source_sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
        if source in manual:record.update(status='reviewed_execution_slice',definitions=manual[source])
        elif category=='creature':
            try:
                fields=creature(meta,body);key='dnd2024:creatures/'+path.stem
                front={'splat_schema':1,'splat_id':str(uuid.uuid5(NAMESPACE,'mechanics/creatures/'+path.stem)),'splat_kind':'creature','splat_key':key,'splat_name':meta['splat_name'],'splat_origin':{'key':source},'splat_sources':[{'title':'SRD 5.2.1','locator':'pp. '+', '.join(map(str,record['pages'])),'url':'https://www.dndbeyond.com/srd'}],**fields}
                # Full source prose stays with the typed note for review of the profile.
                data='---\n'+yaml.safe_dump(front,sort_keys=False,allow_unicode=True)+'---\n\n'+body.strip().replace('[[../pages/','[[../../srd-5.2.1/pages/')+'\n'
                outputs[path.stem+'.md']=data;record.update(status='converted_attack_profile',definitions=[key])
            except Unsupported as error:record.update(status='outside_execution_profile',reason=str(error))
        else:record.update(status='outside_execution_profile',reason='Outside the bounded execution profile; see the full definition catalogue for typed rules and engine gaps')
        records.append(record)
    coverage={'format':1,'source_version':'SRD 5.2.1','boundary':'Execution profiles cover fixed stat-block combat configuration, not all creature ecology or gear transitions. Entries outside this profile are not supported by the bounded interpreter; the full definition catalogue tracks their rules and engine gaps separately. Page records retain coverage for classes, feats, equipment and other sections.','counts':{status:sum(r['status']==status for r in records) for status in ['reviewed_execution_slice','converted_attack_profile','outside_execution_profile']},'entries':records}
    report=json.dumps(coverage,indent=2,ensure_ascii=False)+'\n';outputs['coverage.json']=report
    if args.check:
        actual={p.name:p.read_text() for p in OUTPUT.glob('*') if p.is_file()}
        changed=sorted(k for k in actual.keys()|outputs.keys() if actual.get(k)!=outputs.get(k))
        if changed:raise ValueError('Conversion differs: '+', '.join(changed))
    else:
        OUTPUT.mkdir(parents=True,exist_ok=True)
        unknown={p.name for p in OUTPUT.iterdir() if p.is_file()}-outputs.keys()
        if unknown:raise ValueError('Refusing to remove unexpected output: '+', '.join(unknown))
        for name,text in outputs.items():(OUTPUT/name).write_text(text)
    print(json.dumps(coverage['counts']))
if __name__=='__main__':main()
