#!/usr/bin/env python3
"""Convert the whole official SRD into a local definition catalogue.

Source clauses without typed semantics remain conversion gaps. A missing engine
adapter is a separate status, reserved for explicitly specified contracts.
No third-party defaults, formulas or executable scripts are imported.
"""
import argparse
import collections
import hashlib
import json
import re
import subprocess
import tempfile
import uuid
from pathlib import Path

import yaml
from convert_srd import SOURCE, MANUAL, NAMESPACE, ABILITIES, load_note, clean, slug
from import_srd import positioned_lines, render, PDF_SHA256, ATTRIBUTION, SECTIONS
from srd_contracts import read_research, merge_fields, contract_record, fingerprint
from markdown_content import render_note

OUTPUT = Path('builtin/dnd2024-definitions')
CONTRACTS = Path('scripts/srd_inputs/contracts.yaml')
SCHOOLS = 'Abjuration Conjuration Divination Enchantment Evocation Illusion Necromancy Transmutation'.split()
CLASSES = 'Barbarian Bard Cleric Druid Fighter Monk Paladin Ranger Rogue Sorcerer Warlock Wizard'.split()


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def prose(body):
    return clean(body).removeprefix('# ').strip()


def spell_fields(body):
    """Spell header fields come from 5.2.1, including wrapped reaction triggers."""
    text = clean(body)
    level = re.search(r'\bLevel ([1-9]) ('+'|'.join(SCHOOLS)+r') \(([^)]+)\)', text)
    cantrip = re.search(r'\b('+'|'.join(SCHOOLS)+r') Cantrip \(([^)]+)\)', text)
    if not level and not cantrip:
        raise ValueError('Unrecognized spell level/school')
    header = re.search(r'Casting Time: (.*?) Range: (.*?) Components?: (.*?) Duration: (.*)', text)
    if not header:
        raise ValueError('Unrecognized spell header')
    casting, distance, components, tail = header.groups()
    duration = re.match(r'(Instantaneous|Concentration,? up to \d+ (?:round|minute|hour|day)s?|(?:Up to )?\d+ (?:round|minute|hour|day)s?|Until dispelled(?: or triggered)?|Special)\b', tail)
    if not duration:
        raise ValueError('Unrecognized spell duration: '+tail[:90])
    result = {'level': int(level[1]) if level else 0,
              'school': (level[2] if level else cantrip[1]).lower(),
              'classes': [s.strip().lower() for s in (level[3] if level else cantrip[2]).split(',')],
              'components': [name for short,name in [('V','verbal'),('S','somatic'),('M','material')] if re.search(r'\b'+short+r'\b', components.split('(')[0])],
              'concentration': duration[0].startswith('Concentration'),
              'duration': duration[0], 'casting_time': casting,
              'ritual': 'Ritual' in casting,
              'range': {'feet': int(distance.split()[0])} if re.fullmatch(r'\d+ feet', distance) else {'mode': distance.lower()}}
    if '(' in components:
        material = components[components.index('(')+1:].removesuffix(')')
        result['material'] = {'description': material, 'consumed': 'consumes' in material}
        cost = re.search(r'worth (?:at least )?([\d,]+) (GP|SP|CP)', material)
        if cost: result['material']['minimum_value'] = {'value': int(cost[1].replace(',','')), 'unit': cost[2].lower()}
    return {'splat_spell': result}


def creature_fields(body):
    text = clean(body); before = text.split('###')[0]
    stats = {}
    for short, score, modifier, save in re.findall(r'\b(Str|Dex|Con|Int|Wis|Cha)\s*(\d+)\s*([+-]\d+)\s*([+-]?\d+)', before):
        if (int(score)-10)//2 != int(modifier): raise ValueError('Ability modifier mismatch')
        stats[ABILITIES[short]] = {'value': int(score)}
        # SAVE is a supplied total. Unsigned 2 means +2 under the user's ruling.
        # Later bonuses apply to this total; ability/proficiency are not added again.
        stats['save_'+ABILITIES[short]] = {'value': int(save)}
    if len(stats) != 12: raise ValueError('Missing ability scores/saves')
    for name, pattern in [('armor_class',r'\bAC (\d+)'),('max_hp',r'\bHP (\d+)(?!\d)'),('initiative',r'\bInitiative ([+-]\d+)')]:
        m = re.search(pattern,before)
        if m: stats[name] = {'value': int(m[1])}
    # Variable stat blocks must not receive an arbitrary first HP alternative.
    hp = re.search(r'\bHP (.*?)(?= Speed)',before)
    if hp and re.search(r'\), \d+|your|spell|level',hp[1],re.I): stats.pop('max_hp',None)
    shape = re.search(r'\b(Tiny|Small|Medium|Large|Huge|Gargantuan)(.*?) (Aberration|Beast|Celestial|Construct|Dragon|Elemental|Fey|Fiend|Giant|Humanoid|Monstrosity|Ooze|Plant|Undead)(.*?), ([^#]*?) AC',before)
    if not shape: raise ValueError('Missing creature type/size')
    description = {'size': shape[1].lower(), 'type': shape[3].lower(), 'alignment': shape[5]}
    if shape[2]: description['size_options_text'] = shape[1]+shape[2]
    if shape[4].strip(): description['tags_text'] = shape[4].strip()
    speed = re.search(r'Speed (.*?)(?:MOD SAVE|Str)',before)
    if speed:
        for mode, value in re.findall(r'(?:(Burrow|Climb|Fly|Swim) )?(\d+) ft\.',speed[1]):
            stats[(mode.lower()+'_' if mode else '')+'speed'] = {'value':int(value),'unit':'ft'}
        description['movement_text'] = speed[1].strip()
    labels = ['Skills','Resistances','Vulnerabilities','Immunities','Gear','Senses','Languages','CR']
    for label in labels:
        m = re.search(r'\b'+label+r' (.*?)(?=\b(?:'+'|'.join(labels)+r')\b|$)',before)
        if m: description[slug(label)+'_text'] = m[1].strip()
    if hp: description['hit_points_text'] = hp[1]
    return {'splat_creature':description, 'splat_stats':stats, 'splat_health':{'zero_hp':'dnd2024:monster'}}


def item_fields(name,body):
    text = clean(body)
    header = text.removeprefix('# '+name).strip()
    category = re.match(r'(Wondrous Item|Armor|Weapon|Staff|Rod|Wand|Ring|Potion|Scroll)(?:\b)',header)
    data = {'category':slug(category[1]) if category else 'adventuring_gear'}
    rarity = re.match(r'(?:Wondrous Item|Armor|Weapon|Staff|Rod|Wand|Ring|Potion|Scroll)(?: \([^)]*\))?, (Common|Uncommon|Very Rare|Rare|Legendary|Artifact|Rarity Varies|Varies)\b',header)
    if rarity: data['rarity'] = rarity[1].lower().replace(' ','_')
    attune = re.search(r'Requires Attunement(?: by ([^)]+))?',header[rarity.end():].split(')',1)[0] if rarity and header[rarity.end():].lstrip().startswith('(Requires Attunement') else '')
    data['equipment'] = {'attunement':'required' if attune else 'none'}
    if attune and attune[1]: data['equipment']['attunement_requirement_text'] = attune[1]
    # Do not supply absent mass/price defaults from another database.
    price = re.search(r'\(([\d,]+) (GP|SP|CP)\)',name)
    if price: data['price'] = {'value':int(price[1].replace(',','')),'unit':price[2].lower()}
    return {'splat_item':data}


def named_sources():
    for category in ['creature','spell','magic_item','glossary']:
        for path in sorted((SOURCE/category).glob('*.md')):
            meta,body=load_note(path)
            yield {'category':category,'name':meta['splat_name'],'source':meta['splat_key'],
                   'pages':meta['splat_reference']['pages'],'body':body,'source_path':str(path),
                   'source_sha256':digest(path.read_bytes()),'slug':path.stem}


def section_sources(lines):
    """Separate every heading/feature outside the already indexed named blocks.

Each line in these sections is assigned exactly once, including tables. This is
an extraction inventory, not a claim that every table is semantically decoded.
"""
    normalized=[]
    for line in lines:
        if normalized and line['text'] in ['Open Hand','Sorcery'] and normalized[-1]['text'] in ['Monk Subclass: Warrior of the','Sorcerer Subclass: Draconic']:
            normalized[-1]={**normalized[-1], 'text':normalized[-1]['text']+' '+line['text']}
        else: normalized.append(line)
    selected=[l for l in normalized if 5<=l['page']<=106 or 192<=l['page']<=208 or 254<=l['page']<=257]
    groups=[];current=[];class_name=None;subclass=None;heading=None;subheading=None
    seen=collections.Counter()
    for line in selected:
        if line['role'] in ['heading','subheading','entry']:
            if current: groups.append((context,current));current=[]
            name=line['text'];page=line['page']
            if line['role']=='heading':
                heading=name;subheading=None
                if name in CLASSES and 28<=page<=82: class_name=name;subclass=None
            if line['role']=='subheading':
                subheading=name
                if class_name and 28<=page<=82:
                    if 'Subclass: ' in name: subclass=name.split('Subclass: ',1)[1]
                    elif any(name.startswith(s) for s in ['Path of','College of','Circle of']): subclass=name
            category='rule';parent=heading
            if 28<=page<=82:
                if name in CLASSES: category='class';parent=None
                elif re.match(r'Level \d+:',name):category='class_feature';parent=subclass or class_name
                elif subclass==name or 'Subclass: ' in name:category='subclass';parent=class_name
            if page==83 and name in ['Acolyte','Criminal','Sage','Soldier']: category='background';parent=None
            if 84<=page<=86 and name in ['Dragonborn','Dwarf','Elf','Gnome','Goliath','Halfling','Human','Orc','Tiefling']: category='species';parent=None
            if 87<=page<=88 and line['role']=='entry':category='feat';parent=subheading
            if 93<=page<=100 and line['role']=='entry' and '(' in name:category='equipment';parent=subheading
            base=slug((parent+' ' if parent and parent!=name else '')+name)
            seen[(category,base)]+=1
            key=base+(('_'+str(seen[(category,base)])) if seen[(category,base)]>1 else '')
            context={'category':category,'name':name,'slug':key,'parent_name':parent}
        current.append(line)
    if current:groups.append((context,current))
    for context,span in groups:
        pages=sorted({l['page'] for l in span})
        yield {**context,'source':'dnd2024:reference/pages/'+f'{pages[0]:03}', 'pages':pages,
               'body':'# '+context['name']+'\n\n'+render(span[1:])+'\n',
               'source_path':str(SOURCE/'source.pdf'),'source_sha256':PDF_SHA256,
               'line_span':{'first':selected.index(span[0]),'count':len(span)},
               'positioned_lines':span}


def equipment_tables():
    """Decode the complete weapon and armor tables from the pinned page layout."""
    from fractions import Fraction
    pages=(SOURCE/'source.txt').read_text().split('\f')
    for page in [91,92]:
        rows=[];group=None
        for raw in pages[page-1].splitlines():
            line=raw.strip()
            if page==91:
                if re.fullmatch(r'(Simple|Martial) (Melee|Ranged) Weapons',line):group=line;continue
                m=re.fullmatch(r'(.+?)\s+(\d+(?:d\d+)?) (Bludgeoning|Piercing|Slashing)\s+(.+?)\s+(Cleave|Graze|Nick|Push|Sap|Slow|Topple|Vex)\s+([\d/]+ lb\.|—)\s+([\d,]+) (CP|SP|GP)',line)
                if m:
                    name,dice,damage,props,mastery,mass,price,coin=m.groups()
                    rows.append({'name':name,'damage':{'expression':dice,'type':damage.lower()},'properties_text':props,'mastery':mastery.lower(),'weapon_category':group.lower().replace(' weapons',''),'mass_text':mass,'price':{'value':int(price.replace(',','')),'unit':coin.lower()}})
                elif rows and line=='Two-Handed':rows[-1]['properties_text']+=' Two-Handed'
            else:
                if re.match(r'(Light Armor|Medium Armor|Heavy Armor|Shield) \(',line):group=line.split(' (')[0];continue
                m=re.fullmatch(r'(.+?)\s{2,}(\+?\d+(?: \+ Dex modifier(?: \(max 2\))?)?)\s{2,}(—|Str \d+)\s{2,}(—|Disadvantage)\s+(\d+) lb\.\s+([\d,]+) GP',line)
                if m:
                    name,ac,strength,stealth,mass,price=m.groups()
                    armor={'category':group.lower(),'stealth_disadvantage':stealth=='Disadvantage'}
                    if ac.startswith('+'):armor['ac_bonus']=int(ac)
                    else:
                        armor['base_ac']=int(ac.split()[0]);armor['add_dexterity_modifier']='Dex' in ac
                        if 'max 2' in ac:armor['dexterity_modifier_max']=2
                    if strength!='—':armor['strength_minimum']=int(strength.split()[1])
                    rows.append({'name':name,'armor':armor,'mass_text':mass+' lb.','price':{'value':int(price.replace(',','')),'unit':'gp'}})
        expected=38 if page==91 else 13
        if len(rows)!=expected:raise ValueError(f'Equipment page {page}: expected {expected} rows, found {len(rows)}')
        for row in rows:
            name=row.pop('name');mass=row.pop('mass_text')
            if mass!='—':row['mass']={'value':float(Fraction(mass.split()[0])),'unit':'lb'}
            row['category']='weapon' if page==91 else 'armor'
            yield {'category':'equipment','name':name,'slug':slug(name),'source':f'dnd2024:reference/pages/{page:03}',
                   'source_path':str(SOURCE/'source.txt'),'source_sha256':digest((SOURCE/'source.txt').read_bytes()),'pages':[page],
                   'body':'# '+name+'\n\nSource table, SRD 5.2.1 page '+str(page)+'.\n\n'+json.dumps(row,ensure_ascii=False,indent=2)+'\n',
                   'fields':{'splat_item':row}}


def progression_tables():
    """All 240 class progression rows, independent of feature prose conversion."""
    numeric = {
        'Barbarian':['rages','rage_damage','weapon_mastery'],
        'Bard':['bardic_die','cantrips','prepared_spells']+[f'spell_slots_{i}' for i in range(1,10)],
        'Cleric':['channel_divinity','cantrips','prepared_spells']+[f'spell_slots_{i}' for i in range(1,10)],
        'Druid':['wild_shape','cantrips','prepared_spells']+[f'spell_slots_{i}' for i in range(1,10)],
        'Fighter':['second_wind','weapon_mastery'],
        'Monk':['martial_arts','focus_points','unarmored_movement'],
        'Paladin':['channel_divinity','prepared_spells']+[f'spell_slots_{i}' for i in range(1,6)],
        'Ranger':['favored_enemy','prepared_spells']+[f'spell_slots_{i}' for i in range(1,6)],
        'Rogue':['sneak_attack'],
        'Sorcerer':['sorcery_points','cantrips','prepared_spells']+[f'spell_slots_{i}' for i in range(1,10)],
        'Warlock':['eldritch_invocations','cantrips','prepared_spells','pact_slots','pact_slot_level'],
        'Wizard':['cantrips','prepared_spells']+[f'spell_slots_{i}' for i in range(1,10)],
    }
    for page,body in enumerate((SOURCE/'source.txt').read_text().split('\f'),1):
        lines=body.splitlines()
        for start,line in enumerate(lines):
            match=re.fullmatch(r'\s*('+'|'.join(CLASSES)+r') Features\s*',line)
            if not match:continue
            name=match[1];rows=[]
            for raw in lines[start+1:]:
                m=re.match(r'\s*(\d+)\s+\+(\d)\s+(.*)',raw)
                if m:
                    level,pb,tail=m.groups();cells=re.split(r'\s{2,}',tail.strip())
                    # Spell slot columns sometimes have a single space between cells.
                    features=cells[0];values=' '.join(cells[1:]).replace(' ft.','').split()
                    if len(values)!=len(numeric[name]):raise ValueError(f'{name} level {level}: invalid table columns {values}')
                    row={'level':int(level),'proficiency_bonus':int(pb),'features_text':features}
                    for key,value in zip(numeric[name],values):
                        row[key]=0 if value=='—' else int(value) if re.fullmatch(r'[+]?\d+',value) else value.lower()
                    rows.append(row)
                    if row['level']==20: break
                elif rows and raw.strip() and 'System Reference Document' not in raw:
                    # Wrapped feature names contain no further numerical table cells.
                    continuation=raw.strip()
                    if re.search(r'\s{2,}',continuation):raise ValueError('Unexpected class table continuation '+continuation)
                    rows[-1]['features_text']+=' '+continuation
            if [r['level'] for r in rows]!=list(range(1,21)):raise ValueError('Incomplete progression '+name)
            for row in rows:
                if row['proficiency_bonus']!=2+(row['level']-1)//4:raise ValueError('Proficiency column mismatch')
            yield {'category':'table','name':name+' Progression','slug':slug(name)+'_progression',
                   'source':f'dnd2024:reference/pages/{page:03}','source_path':str(SOURCE/'source.txt'),
                   'source_sha256':digest((SOURCE/'source.txt').read_bytes()),'pages':[page],
                   'body':'# '+name+' Progression\n\nFrom SRD 5.2.1 page '+str(page)+'.\n\n'+ '\n'.join(lines[start:])+'\n',
                   'fields':{'splat_rule':{'mechanic':'dnd2024:class_progression','parameters':{'class':name.lower(),'rows':rows}}}}


def generate(lines, require_complete=True):
    researched = read_research()
    registry = []
    missing = []
    contracts=yaml.load(CONTRACTS.read_text(),Loader=yaml.SafeLoader)
    proof={}
    for path in sorted(MANUAL.rglob('*.md')):
        meta,_=load_note(path)
        if 'splat_origin' in meta:proof.setdefault(meta['splat_origin']['key'],[]).append(meta)
    outputs={};records=[];keys=set();ids=set()
    sources=list(named_sources())+list(section_sources(lines))+list(equipment_tables())+list(progression_tables())
    for source in sources:
        category=source['category'];name=source['name'];s=source['slug'];origin=source['source']
        folder={'creature':'creatures','spell':'spells','magic_item':'items','glossary':'glossary','equipment':'equipment','class':'classes','subclass':'subclasses','species':'species'}.get(category,category+'s')
        key='dnd2024:'+folder+'/'+s
        ident=str(uuid.uuid5(NAMESPACE,'mechanics/'+folder+'/'+s))
        for profile in proof.get(origin,[]):
            if profile['splat_key']==key:ident=profile['splat_id']
        if key in keys or ident in ids:raise ValueError('Duplicate definition '+key)
        keys.add(key);ids.add(ident)
        kind={'creature':'creature','spell':'spell','magic_item':'item','equipment':'item','class':'rule_group','subclass':'rule_group','background':'rule_group','species':'rule_group'}.get(category,'rule')
        errors=[]
        try:
            if kind=='creature':fields=creature_fields(source['body'])
            elif kind=='spell':fields=spell_fields(source['body'])
            elif kind=='item':fields=item_fields(name,source['body'])
            elif kind=='rule_group':fields={'splat_rule_group':{'category':category}}
            else:fields={'splat_rule':{'mechanic':key,'parameters':{}}}
        except ValueError as error:
            fields={'splat_'+kind:{}};errors.append(str(error))
        fields.update(source.get('fields',{}))
        status='partial';required=[]
        if key in contracts:
            spec=contracts[key]
            if spec['source_sha256'] != source['source_sha256']: raise ValueError('Reviewed contract source changed: '+key)
            fields.update(spec['fields'])
            required=spec['requires'];status='specified'
        if key in researched:
            spec = researched[key]
            if spec['source_sha256'] != source['source_sha256']:
                raise ValueError('Researched source changed: '+key)
            # Research covers the complete note. Do not retain an older action
            # interpretation alongside its replacement source-specific contract.
            base_fields = {k:v for k,v in fields.items() if k not in {
                'splat_actions', 'splat_traits', 'splat_resources', 'splat_parameters',
            }} if key in contracts else fields
            fields = merge_fields(base_fields, spec.get('fields', {}))
            status = spec['disposition']
            required = spec.get('requires', [])
            if status == 'specified':
                record = contract_record(key, spec, fields)
                registry.append(record)
            if errors:
                raise ValueError('Source extraction remains incomplete: '+key+' '+str(errors))
        else:
            missing.append(key)
        conversion={'status':status,'category':category,'source_sha256':source['source_sha256'],
                    'source_path':source['source_path'],'pages':source['pages'],
                    'engine_gaps':required,'conversion_gaps':errors+(['source_rules'] if status=='partial' else []),
                    'execution_profiles':[p['splat_key'] for p in proof.get(origin,[])] if category in ['creature','spell','magic_item'] else []}
        if key in researched:
            conversion['research_path'] = researched[key]['research_path']
            conversion['specification_sha256'] = fingerprint({k:v for k,v in fields.items() if k != 'splat_contracts'})
        if source.get('parent_name'):conversion['parent_name']=source['parent_name']
        if 'line_span' in source:conversion['line_span']=source['line_span']
        front={'splat_schema':1,'splat_id':ident,'splat_kind':kind,'splat_key':key,'splat_name':name,
               'splat_origin':{'key':origin},'splat_sources':[{'title':'SRD 5.2.1','locator':'pp. '+', '.join(map(str,source['pages'])),'url':'https://www.dndbeyond.com/srd'}],
               **fields}
        if required:front['splat_requires_mechanics']=required
        path=folder+'/'+s+'.md'
        body=source['body'].strip().replace('[[../pages/','[[../../srd-5.2.1/pages/')
        outputs[path]=render_note(front, body)
        records.append({'key':key,'id':ident,'path':path,'source':origin,**conversion,'sha256':digest(outputs[path].encode())})
    extra = researched.keys() - keys
    if extra: raise ValueError('Research names unknown definitions: '+', '.join(sorted(extra)))
    if require_complete and missing:
        raise ValueError(f'Missing complete specifications for {len(missing)} definitions: '+', '.join(missing[:12]))
    page_inventory=[]
    for page in range(1,365):
        page_inventory.append({'page':page,'section':next(title for start,title in reversed(SECTIONS) if page>=start),
                               'definitions':[r['key'] for r in records if page in r['pages']],
                               'status':'reference_only' if page<5 else 'indexed'})
    if any(not p['definitions'] for p in page_inventory if p['page']>=5):raise ValueError('Unindexed source page')
    adjudications=yaml.load(Path('scripts/srd_inputs/adjudications.yaml').read_text(),Loader=yaml.SafeLoader)
    if any(case['key'] not in keys for case in adjudications['cases']): raise ValueError('Adjudication references unknown definition')
    report={'format':1,'source_sha256':PDF_SHA256,'counts':{'definitions':len(records),'categories':dict(sorted(collections.Counter(r['category'] for r in records).items())),
            'specified':sum(r['status']=='specified' for r in records),
            'informational':sum(r['status']=='informational' for r in records),'partial':sum(r['status']=='partial' for r in records),
            'executable_full_definitions':0,'executable_profiles':sum(len(v) for v in proof.values())},
            'adjudications':adjudications['cases'],
            'engine_contracts':sorted({g for r in records for g in r['engine_gaps']}),'entries':records,'pages':page_inventory}
    outputs['contracts.json']=json.dumps({'format':2,'contracts':registry},ensure_ascii=False,indent=2)+'\n'
    outputs['coverage.json']=json.dumps(report,ensure_ascii=False,indent=2)+'\n'
    outputs['LICENSE.txt']=ATTRIBUTION+'\n\nExtracted and converted into typed rule specifications by Splat. Unimplemented engine contracts do not execute.\n'
    return outputs,report


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--check',action='store_true');ap.add_argument('--allow-partial',action='store_true',help='Development only: permit missing research specifications');args=ap.parse_args()
    if digest((SOURCE/'source.pdf').read_bytes())!=PDF_SHA256:raise ValueError('SRD PDF hash mismatch')
    with tempfile.TemporaryDirectory(prefix='splat-definitions-') as scratch:
        xml=Path(scratch)/'source.xml'
        subprocess.run(['pdftohtml','-xml','-hidden','-i',str(SOURCE/'source.pdf'),str(xml)],check=True,stdout=subprocess.DEVNULL)
        outputs,report=generate(positioned_lines(xml.read_text()),require_complete=not args.allow_partial)
    actual={str(p.relative_to(OUTPUT)):p.read_text() for p in OUTPUT.rglob('*') if p.is_file()}
    if args.check:
        changed=sorted(k for k in actual.keys()|outputs.keys() if actual.get(k)!=outputs.get(k))
        if changed:raise ValueError('Definition conversion differs: '+', '.join(changed[:20]))
    else:
        unexpected=actual.keys()-outputs.keys()
        old=json.loads(actual.get('coverage.json','{}'))
        owned={r['path']:r['sha256'] for r in old.get('entries',[])}
        unsafe=[p for p in actual if p.endswith('.md') and owned.get(p)!=digest(actual[p].encode())]
        unsafe += [p for p in unexpected if not p.endswith('.md') and owned.get(p)!=digest(actual[p].encode())]
        if unsafe:raise ValueError('Refusing to remove unknown or edited files: '+', '.join(sorted(unsafe)))
        for path in unexpected:(OUTPUT/path).unlink()
        for path,text in outputs.items():
            dest=OUTPUT/path;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_text(text)
    print(json.dumps(report['counts'],indent=2))

if __name__=='__main__':main()
