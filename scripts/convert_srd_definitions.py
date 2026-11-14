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
from mapped_tables import plan as table_plan

OUTPUT = Path('builtin/dnd2024-definitions')
CONTRACTS = Path('scripts/srd_inputs/contracts.yaml')
HABITATS = Path('scripts/srd_inputs/habitats.yaml')
MONSTER_MANUAL = 'Monster Manual (2024)'
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
        # SAVE is a supplied total.
        # Later bonuses apply to this total; ability/proficiency are not added again.
        stats['save_'+ABILITIES[short]] = {'value': int(save)}
    if len(stats) != 12: raise ValueError('Missing ability scores/saves')
    # A summoned creature's AC depends on the spell's level, so there is no one number.
    for name, pattern in [('armor_class',r'\bAC (\d+)\b(?! \+)'),('max_hp',r'\bHP (\d+)(?!\d)'),('initiative',r'\bInitiative ([+-]\d+)')]:
        m = re.search(pattern,before)
        if m: stats[name] = {'value': int(m[1])}
    # Variable stat blocks must not receive an arbitrary first HP alternative.
    hp = re.search(r'\bHP (.*?)(?= Speed)',before)
    if hp and re.search(r'\), \d+|your|spell|level',hp[1],re.I): stats.pop('max_hp',None)
    shape = re.search(r'\b(Tiny|Small|Medium|Large|Huge|Gargantuan)(.*?) (Aberration|Beast|Celestial|Construct|Dragon|Elemental|Fey|Fiend|Giant|Humanoid|Monstrosity|Ooze|Plant|Undead)(.*?), ([^#]*?) AC',before)
    if not shape: raise ValueError('Missing creature type/size')
    description = {'size': shape[1].lower(), 'type': shape[3].lower(), 'alignment': shape[5]}
    # `Celestial, Fey, or Fiend (Your Choice), Neutral`: the type is a choice, then the alignment.
    if ', ' in shape[5]:
        choices, description['alignment'] = shape[5].rsplit(', ',1)
        description['type_options_text'] = shape[3]+', '+choices
    if shape[2]: description['size_options_text'] = shape[1]+shape[2]
    # `Swarm of Tiny Beasts`: the type regex leaves the plural behind.
    if shape[4].strip() not in ('','s'): description['tags_text'] = shape[4].strip()
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
    ac = re.search(r'\bAC (\d+ \+ .*?)(?= HP| Initiative)',before)
    if ac: description['armor_class_text'] = ac[1]
    return {'splat_creature':description, 'splat_stats':stats, 'splat_health':{'zero_hp':'dnd2024:monster'}}


def item_fields(name,body):
    text = clean(body)
    header = text.removeprefix('# '+name).strip()
    category = re.match(r'(Wondrous Item|Armor|Weapon|Staff|Rod|Wand|Ring|Potion|Scroll)(?:\b)',header)
    data = {'category':slug(category[1]) if category else 'adventuring_gear'}
    rarity = re.match(r'(?:Wondrous Item|Armor|Weapon|Staff|Rod|Wand|Ring|Potion|Scroll)(?: \([^)]*\))?, (Common|Uncommon|Very Rare|Rare|Legendary|Artifact|Rarity Varies|Varies)\b',header)
    if rarity: data['rarity'] = rarity[1].lower().replace(' ','_')
    # `Rare (+1), Very Rare (+2), or Legendary (+3)`: one entry, several rarities.
    if rarity and re.match(r'(?: \([^)]*\))?,? (?:or )?(?:Common|Uncommon|Very Rare|Rare|Legendary|Artifact)\b',header[rarity.end():]): data['rarity'] = 'varies'
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


def gear_source_table(lines, ammunition=False):
    """Render the pinned source tables without PDF column/line-break artifacts."""
    if ammunition:
        headers = ['Type', 'Amount', 'Storage', 'Weight', 'Cost']
        skip = {'Ammunition', 'Type Amount Storage Weight Cost'}
        pattern = r'(.+?)\s+(\d+)\s+(Quiver|Case|Pouch)\s+(.+? lb\.)\s+(\d+ [GSC]P)'
    else:
        headers = ['Item', 'Weight', 'Cost']
        skip = {'Adventuring Gear', 'Item Weight Cost'}
        pattern = r'(.+?)\s+(—|Varies|[\d½/.,]+ lb\.(?: \(full\))?)\s+(Varies|[\d,]+ [GSC]P)'
    rows = []
    for line in lines:
        text = line['text']
        if text in skip: continue
        match = re.fullmatch(pattern, text)
        if not match: raise ValueError('Unexpected gear source table row: '+text)
        rows.append(match.groups())
    return ('| '+' | '.join(headers)+' |\n| '+' | '.join(['---']*len(headers))+' |\n'
            + ''.join('| '+' | '.join(row)+' |\n' for row in rows))


def source_excerpt(context, span):
    if context['slug'] not in {'ammunition_varies', 'adventuring_gear'}:
        return render(span[1:])
    # These two descriptions share p.94. The gear overview table interrupts
    # Ammunition on p.95; its own table resumes on p.96.
    prose = '\n'.join(line['text'] for line in span[1:] if line['page']==94)
    prose = re.sub(r'([a-z])-\n([a-z])', r'\1\2', prose).replace('\n', ' ')
    page = 96 if context['slug']=='ammunition_varies' else 95
    table = gear_source_table([line for line in span if line['page']==page], ammunition=page==96)
    return f'<!-- SRD page 94 -->\n\n{prose}\n\n<!-- SRD page {page} -->\n\n{table}'


def tables_in_their_sections(lines):
    """A wide table can print pages after the section that bears its name. Read it there."""
    headed={'heading','subheading','entry'}
    result=list(lines)
    for caption in [l for l in lines if l['caption'] and l['band']%2==1]:
        at=next(i for i,l in enumerate(result) if l is caption)
        start=next((i for i in range(at-1,-1,-1) if result[i]['role'] in headed and result[i]['text']==caption['text'] and result[i]['page']>=caption['page']-3),None)
        if start is None:continue
        end=next(i for i in range(start+1,len(result)) if result[i]['role'] in headed)
        if end>=at:continue
        block=[l for l in result[at:] if l['page']==caption['page'] and l['band']==caption['band']]
        rest=[l for l in result[end:] if not any(l is b for b in block)]
        result=result[:end]+block+rest
    return result


def section_sources(lines):
    """Separate every heading/feature outside the already indexed named blocks.

Each line in these sections is assigned exactly once, including tables. This is
an extraction inventory, not a claim that every table is semantically decoded.
"""
    normalized=[]
    for line in tables_in_their_sections(lines):
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
    # A black table caption is not an entry heading. Attach the inserted
    # overview page to its named section, not to the preceding item.
    overview = next(span for context,span in groups if context['slug']=='adventuring_gear')
    inserted = [line for line in selected if line['page']==95]
    if not inserted or inserted[0]['text'] != 'Adventuring Gear':
        raise ValueError('Pinned Adventuring Gear table layout changed')
    for _,span in groups:
        span[:] = [line for line in span if line['page']!=95]
    overview.extend(inserted)
    positions = {id(line): i for i,line in enumerate(selected)}
    for context,span in groups:
        spans = []
        for line in span:
            index = positions[id(line)]
            if spans and spans[-1]['first']+spans[-1]['count']==index:
                spans[-1]['count'] += 1
            else: spans.append({'first':index,'count':1})
        address = {'line_span':spans[0]} if len(spans)==1 else {'line_spans':spans}
        pages=sorted({l['page'] for l in span})
        yield {**context,'source':'dnd2024:reference/pages/'+f'{pages[0]:03}', 'pages':pages,
               'body':'# '+context['name']+'\n\n'+source_excerpt(context,span)+'\n',
               'source_path':str(SOURCE/'source.pdf'),'source_sha256':PDF_SHA256,
               **address,
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


def progression_markdown(name,keys,rows,printed):
    label=lambda key:re.sub(r'spell slots (\d)',r'Level \1 Slots',key.replace('_',' ')).title()
    header=['Level','Proficiency Bonus','Class Features']+[label(key) for key in keys]
    table=['| '+' | '.join(header)+' |','|'+' --- |'*len(header)]
    # The parser drops the unit from the Unarmored Movement column.
    unit=lambda key,value:value+' ft.' if key=='unarmored_movement' and value!='—' else value
    for row,cells in zip(rows,printed):
        table.append('| '+' | '.join([cells[0],cells[1],row['features_text'],*map(unit,keys,cells[2:])])+' |')
    return f'**{name} Features**\n\n'+'\n'.join(table)


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
            name=match[1];rows=[];printed=[]
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
                    rows.append(row);printed.append([level,'+'+pb,*values])
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
                   'body':'# '+name+' Progression\n\n'+progression_markdown(name,numeric[name],rows,printed)+'\n',
                   'fields':{'splat_rule':{'mechanic':'dnd2024:class_progression','parameters':{'class':name.lower(),'rows':rows}}}}


def generate(lines, require_complete=True):
    researched = read_research()
    readings = yaml.safe_load(Path('scripts/srd_inputs/reading.yaml').read_text())
    registry = []
    missing = []
    contracts=yaml.load(CONTRACTS.read_text(),Loader=yaml.SafeLoader)
    # The SRD prints no habitat. Every creature has a reviewed value or an explicit null.
    habitats=yaml.load(HABITATS.read_text(),Loader=yaml.SafeLoader)
    proof={}
    for path in sorted(MANUAL.rglob('*.md')):
        meta,_=load_note(path)
        if 'splat_origin' in meta:proof.setdefault(meta['splat_origin']['key'],[]).append(meta)
    outputs={};records=[];keys=set();ids=set()
    sources=list(named_sources())+list(section_sources(lines))+list(equipment_tables())+list(progression_tables())
    names = {}
    # Round 28 FG6: a section heading that ends in a colon is not a name. Each keeps its old name
    # as an alias, so links and requirements that name it still resolve.
    renamed = {
        'Barbarian Subclass:': 'Barbarian Subclass',
        'Bard Subclass:': 'Bard Subclass',
        'Druid Subclass:': 'Druid Subclass',
    }
    for entry in sources:
        if entry['name'] in renamed:
            old, new = entry['name'], renamed[entry['name']]
            entry['aliases'] = [old]
            entry['name'] = new
            if 'body' in entry:
                entry['body'] = entry['body'].replace('# '+old+'\n', '# '+new+'\n', 1)
    for entry in sources:
        category=entry['category']
        folder={'creature':'creatures','spell':'spells','magic_item':'items','glossary':'glossary','equipment':'equipment','class':'classes','subclass':'subclasses','species':'species'}.get(category,category+'s')
        key='dnd2024:'+folder+'/'+entry['slug']
        names[key]=entry['name']
        names['__path__'+key]=folder+'/'+entry['slug']+'.md'
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
        if source.get('aliases'):fields['aliases']=source['aliases']
        habitat=None
        if kind=='creature':
            if key not in habitats:raise ValueError('Habitat not reviewed: '+key)
            habitat=habitats[key]
            # Before the contracts, so a reviewed contract's closed shape includes it.
            if habitat:fields['splat_creature']={**fields['splat_creature'],'habitat_text':habitat['habitat']}
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
        for field in ['line_span','line_spans']:
            if field in source:conversion[field]=source[field]
        cited=[{'title':'SRD 5.2.1','locator':'pp. '+', '.join(map(str,source['pages'])),'url':'https://www.dndbeyond.com/srd'}]
        if habitat:cited.append({'title':MONSTER_MANUAL,'locator':f"p. {habitat['page']}, Habitat"})
        front={'splat_id':ident,'splat_kind':kind,'splat_key':key,'splat_name':name,
               'splat_origin':{'key':origin},'splat_sources':cited,
               **fields}
        if key in readings:
            reviewed = readings[key]
            if reviewed['source_sha256'] != source['source_sha256']:
                raise ValueError('Reading summary source changed: '+key)
            reading = reviewed['reading']
            if any(target not in names for target in reading.get('related', {})):
                raise ValueError('Reading summary links to an unknown definition: '+key)
            front['splat_reading'] = reading
        if required:front['splat_requires_mechanics']=required
        path=folder+'/'+s+'.md'
        body=source['body'].strip().replace('[[../pages/','[[../../srd-5.2.1/pages/')
        # Print sets a class's table beside its text, so it can fall in another section.
        if source['category']=='class' and f"**{source['name']} Features**" not in body:
            table=next(t for t in sources if t['name']==source['name']+' Progression')
            body+='\n\n'+table['body'].split('\n\n',1)[1].strip()
        outputs[path]=render_note(front, body, names, path)
        records.append({'key':key,'id':ident,'path':path,'source':origin,**conversion,'sha256':digest(outputs[path].encode())})
    if habitats.keys() - keys:
        raise ValueError('Habitats name unknown definitions: '+', '.join(sorted(habitats.keys() - keys)))
    if readings.keys() - keys:
        raise ValueError('Reading summaries name unknown definitions: '+', '.join(sorted(readings.keys() - keys)))
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
