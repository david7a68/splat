#!/usr/bin/env python3
"""Reproducible, source-only SRD import. Requires Poppler; never invents mechanics."""
import argparse
import hashlib
import html
import json
import re
import shutil
import subprocess
import tempfile
import urllib.request
import uuid
from pathlib import Path
from schema2_notes import convert

URL = 'https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf'
PDF_SHA256 = '8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87'
PACKAGE_ID = uuid.uuid5(uuid.NAMESPACE_URL, URL)
ATTRIBUTION = ('This work includes material from the System Reference Document 5.2.1 '
 '(“SRD 5.2.1”) by Wizards of the Coast LLC, available at https://www.dndbeyond.com/srd. '
 'The SRD 5.2.1 is licensed under the Creative Commons Attribution 4.0 International License, '
 'available at https://creativecommons.org/licenses/by/4.0/legalcode.')
SECTIONS = [(1,'Legal Information'),(2,'Contents and Index'),(5,'Playing the Game'),
 (19,'Character Creation'),(28,'Classes'),(83,'Character Origins'),(87,'Feats'),
 (89,'Equipment'),(104,'Spells'),(176,'Rules Glossary'),(192,'Gameplay Toolbox'),
 (204,'Magic Items'),(254,'Monsters'),(344,'Animals')]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def attrs(raw):
    return dict(re.findall(r'(\w+)="([^"]*)"', raw))


CHANGES = (' Printing errors are corrected in the notes: the end of a cut sentence in Telekinesis,'
           ' from SRD 5.1, and a missing minus sign on the Young White Dragon\'s Intelligence save.')
PATCH_FILE = Path(__file__).parent/'srd_inputs'/'transcription_patches.json'
PATCHES = json.loads(PATCH_FILE.read_text())['patches']


def patched(page, text):
    """Printing errors in the PDF, corrected in the transcription. The PDF stays as it is."""
    for patch in PATCHES:
        if patch['page'] == page:
            text = text.replace(patch['find'], patch['replace'])
    return text


def plain(raw):
    return html.unescape(re.sub('<[^>]*>', '', raw)).replace('\u00ad', '')


def slug(name):
    return re.sub(r'[^a-z0-9]+', '_', name.lower()).strip('_')


def yaml_note(key, name, kind, fields, body):
    data = {'splat_schema': 1, 'splat_id': str(uuid.uuid5(PACKAGE_ID,key)),
            'splat_kind': kind, 'splat_key': 'dnd2024:'+key, 'splat_name':name, **fields}
    # JSON flow values are a strict YAML subset, avoiding custom YAML escaping.
    return convert('---\n'+''.join(k+': '+json.dumps(v,ensure_ascii=False)+'\n' for k,v in data.items())+'---\n\n'+body.rstrip()+'\n', key+'.md')


MIDDLE_FLOOR = 40
BODY_COLOR = '#231f20'
RULE_LABELS = ('Casting Time', 'Range', 'Components', 'Component', 'Duration', 'Creature Type', 'Size', 'Speed',
               'Prerequisite', 'Prerequisites', 'Category', 'Ability Scores', 'Skill Proficiencies', 'Tool Proficiency',
               'Equipment', 'Feat', 'Ability', 'Weight', 'Utilize', 'Craft', 'Variants')
SPELL_USES = re.compile(r'^(At Will|\d+/Day(?: Each)?|\d+/(?:Short|Long) Rest(?: Each)?):')
STAT_LABELS = re.compile(r'^(AC|HP|Speed|MOD SAVE|Str|Dex|Con|Int|Wis|Cha|Skills|Resistances|Vulnerabilities|Immunities|Gear|Senses|Languages|CR)\b')
RUN_IN = re.compile(r"^[A-Z][\w’'-]*(?: [A-Za-z0-9’'(),/–-]+){0,8}\.(?:\s|$)")
SIZE_LINE = re.compile(r'^(Tiny|Small|Medium|Large|Huge|Gargantuan)\b')


def faked_caps(font, bold):
    """Some sidebar titles set small capitals by hand, in two sizes of bold GillSans."""
    return bold and font['family'].endswith('+GillSans') and font['size'] in ('17', '12')


def is_gill(font):
    """Tables and sidebars are the only black GillSans text at this size."""
    return 'GillSans' in font['family'] and font['color'] == BODY_COLOR and int(font['size']) <= 15


PRINT_FIXES = (('Short sword', 'Shortsword'), ('it re appears', 'it reappears'), ('Meta magic', 'Metamagic'), ('the“', 'the “'))
MOTTO = re.compile(r"[A-Z][\w’'-]*(?: [\w’'-]+){2,}")


def header_row(cells):
    """A table's first row. A few headers mix bold labels with plain ones: never numbers."""
    if all(cell['semibold'] for cell in cells):
        return True
    first = min(cells, key=lambda cell: cell['left'])
    plain_cells = [cell for cell in cells if not cell['semibold']]
    # Plain labels only on the right are the first row of a second table.
    middle = (first['left']+max(cell['left'] for cell in cells))/2
    return len(cells) >= 4 and first['semibold'] and min(cell['left'] for cell in plain_cells) < middle and all(re.fullmatch(r'[A-Z][A-Za-z ]*', cell['text'].strip()) for cell in plain_cells)


def full_width_blocks(fragments, middle):
    """Vertical ranges where a table runs across both columns.

A table on each side of the gutter also shares rows, so a block must have nothing
but table text beside it, and its two halves must not repeat one header.
"""
    labelled = lambda f: any(f['text'].strip().startswith(label+':') for label in RULE_LABELS) or re.match(r'Level \d+ [A-Z]', f['text'].strip())
    gill = sorted((f for f in fragments if f['gill'] and not labelled(f)), key=lambda f: f['top'])
    rows = []
    for f in gill:
        if rows and f['top']-rows[-1]['top'] <= 6:
            rows[-1]['cells'].append(f)
        else:
            rows.append({'top': f['top'], 'cells': [f]})
    runs, current = [], []
    for row in rows:
        left = [f for f in row['cells'] if f['left'] < middle]
        right = [f for f in row['cells'] if f['left'] >= middle]
        crossing = any(f['left'] < middle-MIDDLE_FLOOR and f['left']+f['width'] > middle+MIDDLE_FLOOR for f in row['cells'])
        if (left and right) or crossing:
            current.append(row)
        elif current and row['top']-current[-1]['top'] <= 26:
            current.append(row)
        else:
            if current:
                runs.append(current)
            current = []
    if current:
        runs.append(current)
    blocks = []
    others = [f for f in fragments if not f['gill']]
    for run in runs:
        joined = [row for row in run if len(row['cells']) >= 2]
        if len(joined) < 2:
            continue
        # A real table opens with one header row across the gutter. Two tables side by side do not.
        if not header_row(run[0]['cells']):
            continue
        top, bottom = run[0]['top'], run[-1]['top']
        # A caption above the right half names a second table.
        if any(f['caption'] and f['left'] >= middle and top-30 <= f['top'] < top for f in fragments):
            continue
        # A two-line header can have a first line on one side only.
        top = min([top]+[row['top'] for row in rows if top-30 <= row['top'] < top and all(f['semibold'] for f in row['cells'])])
        if any(top-2 <= f['top'] <= bottom+2 and f['size'] >= 14 and not f['caption'] for f in others):
            continue
        first = joined[0]['cells']
        left_words = {f['text'].strip() for f in first if f['left'] < middle}
        right_words = {f['text'].strip() for f in first if f['left'] >= middle}
        if left_words & right_words:
            continue
        blocks.append((top-1, bottom+1))
    return blocks


def positioned_lines(xml):
    fonts = {d['id']:d for raw in re.findall(r'<fontspec ([^>]+)/>',xml) for d in [attrs(raw)]}
    result=[]
    pages=re.findall(r'<page ([^>]+)>(.*?)</page>',xml,re.S)
    if len(pages)!=364:
        raise ValueError(f'Expected 364 pages, found {len(pages)}')
    for page_attrs,body in pages:
        page=attrs(page_attrs);n=int(page['number']); height=int(page['height']);width=int(page['width'])
        fragments=[]
        for raw,text in re.findall(r'<text ([^>]+)>(.*?)</text>',body,re.S):
            d=attrs(raw);font=fonts[d['font']];top=int(d['top']);left=int(d['left'])
            if top>height*.94: continue # repeated footer only; full layout pages retain it
            # A printed run-in title or list lead is bold: `<i><b>Rend. </b></i>`, `<b>Rituals</b>.`
            titled=bool(re.match(r'\s*(?:<i>)?<b>',text))
            italic=bool(re.fullmatch(r'\s*<i>[^<]*</i>\s*',text))
            text_bold=text.lstrip().startswith('<b>')
            # In body type the fraction glyph extracts as digits: the print has `1½`.
            text=re.sub(r'\b11/2 (?=pints|times)','1½ ',plain(text))
            # The text layer has stray or missing spaces that the print does not.
            for wrong,right in PRINT_FIXES:text=text.replace(wrong,right)
            if not text.strip():continue
            red=font['color']=='#88191f'
            role='stat' if red and font['size']=='23' else 'entry' if red and 'SemiBold' in font['family'] and font['size'] in ['18','13'] else 'heading' if red and int(font['size'])>=27 else 'subheading' if red else 'body'
            fragments.append({'page':n,'col':int(left>width/2),'top':top,'left':left,
                              'width':int(d['width']),'text':text,'role':role,'size':int(font['size']), 'smallcaps':'SC700' in font['family'] or faked_caps(font,text_bold),
                              'gill':is_gill(font) and not faked_caps(font,text_bold),'caption':'SemiBold' in font['family'] and 'SC700' not in font['family'] and int(font['size'])==16 and font['color']==BODY_COLOR,
                              'titled':titled,'italic':italic,'semibold':'SemiBold' in font['family'],'header':font['color'] != BODY_COLOR and not red,'family':font['family']})
        # A full-width table is read where it stands, not one half per column.
        blocks=full_width_blocks(fragments,width/2)
        for f in fragments:
            f['band']=sum(2 for start,end in blocks if f['top']>end)
            if any(start<=f['top']<=end for start,end in blocks):
                f['band']=sum(2 for start,end in blocks if end<f['top'])+1
                f['col']=0
        # A wide table's caption can print at the foot of the left column. It stays with its table.
        for index,_ in enumerate(blocks):
            above=[f for f in fragments if f['band']==2*index]
            left=[f for f in above if f['col']==0];right=sorted((f for f in above if f['col']==1),key=lambda f:f['top'])
            foot=max((f['top'] for f in left),default=None)
            captions=[f for f in left if f['top']==foot and f['caption']]
            if not captions:continue
            for f in captions:f['band']=2*index+1
            # The table comes before the right column, unless that column continues the
            # left one or is the entry that names the table.
            name=' '.join(f['text'].strip() for f in captions)
            if right and right[0]['text'].strip()[0]!='•' and not right[0]['text'].strip()[0].islower() and name+' table' not in ' '.join(f['text'].strip() for f in right):
                for f in right:f['band']=2*index+2
        fragments.sort(key=lambda d:(d['band'],d['col'],d['top'],d['left']))
        lines=[]
        for f in fragments:
            candidates=[l for l in lines[-12:] if l['band']==f['band'] and l['col']==f['col'] and abs(l['top']-f['top'])<=6 and l['role']==f['role']]
            if candidates:
                candidates[-1]['parts'].append(f)
            else:lines.append({**f,'parts':[f]})
        lines.sort(key=lambda d:(d['band'],d['col'],d['top'],d['left']))
        for line in lines:
            parts=sorted(line.pop('parts'),key=lambda d:d['left']);text='';right=None
            for f in parts:
                gap=f['left']-right if right is not None else 0
                if text and not text[-1].isspace() and not f['text'][0].isspace() and (gap>3 or f['text'][0]=='“'):text+=' '
                text+=f['text'];right=f['left']+f['width']
            line['raw']=parts[0]['text']
            line['titled']=parts[0]['titled']
            # Fragments that touch are one cell in two type styles.
            cells=[]
            for f in parts:
                if cells and f['left']-(cells[-1]['left']+cells[-1]['width'])<=6:
                    piece=f['text'].strip()
                    # Touching type with no printed space is one word: `Cure Wound` + `s`.
                    tight=piece=='-' or (f['left']-(cells[-1]['left']+cells[-1]['width'])<=2 and not f['text'][0].isspace() and not cells[-1]['open'])
                    cells[-1]['text']=(cells[-1]['text']+piece) if tight else (cells[-1]['text']+' '+piece).strip()
                    cells[-1]['open']=f['text'][-1].isspace()
                    cells[-1]['width']=f['left']+f['width']-cells[-1]['left']
                    cells[-1]['semibold']=cells[-1]['semibold'] and f['semibold']
                else:
                    cells.append({'left':f['left'],'width':f['width'],'text':f['text'].strip(),'semibold':f['semibold'],'italic':f['italic'],'open':f['text'][-1].isspace()})
            line['cells']=cells
            line['gill']=sum(f['gill'] for f in parts)*2>=len(parts)
            line['caption']=any(f['caption'] for f in parts)
            line['header']=any(f['header'] for f in parts)
            line['width']=(parts[-1]['left']+parts[-1]['width'])-parts[0]['left']
            line['text']=patched(n,text.strip().title() if any(f['smallcaps'] for f in parts) else text.strip());line['size']=max(f['size'] for f in parts)
            if line['role']=='entry' and line['size']<18:line['role']='body'
            # Join a genuinely wrapped entry title (not the next prose line).
            if result and line['role']=='entry' and result[-1]['role']=='entry' and line['page']==result[-1]['page'] and line['col']==result[-1]['col'] and 7<line['top']-result[-1]['top']<=24:
                result[-1]['text']+=' '+line['text']
            else: result.append(line)
    attest(result)
    return result


ATTESTED = set()


def attest(lines):
    """Words the SRD prints whole. They decide whether a line-end hyphen was a real one.
The two halves of a word cut at a line end are not words."""
    ATTESTED.clear()
    cut = False
    for line in lines:
        words = re.findall(r"[A-Za-z’']+", line['text'])
        ends_cut = line['text'].endswith('-')
        if cut:
            words = words[1:]
        if ends_cut:
            words = words[:-1]
        cut = ends_cut
        ATTESTED.update(word.lower() for word in words)

# Whole words that the SRD prints only across a line end, with halves that are also words.
CUT_WORDS = {'battlefield', 'conforms', 'lightweight', 'nickname', 'pathways'}


def joined(before, after):
    """Join wrapped lines. A printed hyphen goes when the whole word exists without it."""
    if not before:
        return after
    if re.search(r'[A-Za-z]-$', before) and after[:1].isdigit():
        return before+after
    if before.endswith('-') and not before.endswith(' -') and after[:1].isalpha():
        left = re.search(r"[A-Za-z’']+-$", before)
        right = re.match(r"[A-Za-z’']+", after)
        if left and right:
            whole = (left[0][:-1]+right[0]).lower()
            compound = left[0][:-1].lower() in ATTESTED and right[0].lower() in ATTESTED
            if whole in ATTESTED or whole.removesuffix('s') in ATTESTED or whole in CUT_WORDS or (not compound and after[0].islower()):
                return before[:-1]+after
        return before+after
    return before+' '+after


SPANNING = re.compile(r'[—–-]+[A-Za-z].*[—–-]+|Distance Traveled Per …')
VALUE = re.compile(r'[+−-]?\d[\d,/]*|—')
WEIGHT_COST = re.compile(r'(\S+ lb\.|\d+ oz\.|—) (\d[\d,]* [GSCE]P)')
NAME_DAMAGE = re.compile(r'(\D+?) +(\d+d\d+ [A-Z][a-z]+)')
ROLL_CELL = re.compile(r'(\d{1,3}(?:–\d{1,3})?) (\S.*)')


def split_rolls(rows, head):
    """Print can join a roll and its result into one fragment: `01–20 You cast a spell`.

Only a table keyed by a die or AC has such cells, and only a repeated header has a second
pair in the row. In other tables `30 days` is one value.
"""
    words = head.split()
    if not words or not re.fullmatch(r'\d*d\d+|AC', words[0]):
        return rows
    paired = words.count(words[0]) > 1
    def split(cells):
        result = []
        for index, cell in enumerate(cells):
            match = None if cell['semibold'] else ROLL_CELL.fullmatch(cell['text'])
            if match and (index == 0 or paired):
                step = cell['width']/len(cell['text'])
                at = int(cell['left']+step*(len(match[1])+1))
                result.append({**cell, 'width': int(step*len(match[1])), 'text': match[1]})
                result.append({**cell, 'left': at, 'width': cell['left']+cell['width']-at, 'text': match[2]})
            else:
                result.append(cell)
        return result
    return [split(cells) for cells in rows]


def table_markdown(block):
    """Rebuild a table from its cells. Each cell is a separate PDF text fragment."""
    rows = split_rolls([line['cells'] for line in block], block[0]['text'])
    # A footnote under the table is prose, not a row.
    foot = next((i for i, cells in enumerate(rows) if cells[0]['text'].startswith('*')), None)
    footnote = ''
    if foot is not None:
        for cells in rows[foot:]:
            footnote = joined(footnote, ' '.join(cell['text'] for cell in cells))
        rows = rows[:foot]
    widest = max(rows, key=len)
    anchors = [(cell['left'], cell['left']+cell['width']/2) for cell in widest]
    def column(cell):
        center = cell['left']+cell['width']/2
        return min(range(len(anchors)), key=lambda i: min(abs(anchors[i][0]-cell['left']), abs(anchors[i][1]-center)))
    grid, notes, header_rows = [], [], 0
    def split_spanning(cells):
        """Print joined a short header pair: `1d10 Damage Type` over two columns."""
        result = []
        for index, cell in enumerate(cells):
            if SPANNING.fullmatch(cell['text']):
                result.append(cell)
                continue
            covered = [i for i, (left, _) in enumerate(anchors) if cell['left']-6 <= left < cell['left']+cell['width']]
            parts = cell['text'].split()
            sentence = None
            weighed = WEIGHT_COST.fullmatch(cell['text'])
            named = None if cell['semibold'] else NAME_DAMAGE.fullmatch(cell['text'])
            if named:
                step = cell['width']/len(cell['text'])
                result += [{**cell, 'width': 1, 'text': named[1]},
                           {**cell, 'left': int(cell['left']+step*named.start(2)), 'width': 1, 'text': named[2]}]
                continue
            # Runs of values share a fragment: `2 — —`, or `65 lb. 1,500 GP`.
            if not cell['semibold'] and (weighed or (len(covered) >= 2 and len(parts) == len(covered) and all(VALUE.fullmatch(part) for part in parts))):
                values = weighed.groups() if weighed else parts
                at = [anchors[i][0] for i in covered] if not weighed or len(covered) == 2 else [cell['left'], cell['left']+cell['width']//2]
                result += [{**cell, 'left': left, 'width': 1, 'text': value} for left, value in zip(at, values)]
                continue
            if cell['semibold'] and len(covered) >= 2 and len(parts) == len(covered):
                result += [{**cell, 'left': anchors[i][0], 'width': 1, 'text': part} for i, part in zip(covered, parts)]
                continue
            if not cell['semibold'] and len(covered) == 2:
                # One fragment holds two cells that each end in a full sentence.
                at = (anchors[covered[1]][0]-cell['left'])/cell['width']*len(cell['text'])
                sentence = next((m.end() for m in re.finditer(r'\. ', cell['text']) if abs(m.end()-at) <= 8), None)
                # Or a short row name runs straight into the next cell: `Disengage Your movement`.
                if not sentence and index == 0 and covered[0] == 0 and at < 24 and ' ' in cell['text'] and not cell.get('italic'):
                    sentence = min((m.end() for m in re.finditer(' ', cell['text'])), key=lambda end: abs(end-at))
            if cell['semibold'] and len(covered) == 2 and len(parts) >= 2:
                # The header word nearest the second column starts that column.
                at = (anchors[covered[1]][0]-cell['left'])/cell['width']*len(cell['text'])
                cut = min((m.start() for m in re.finditer(' ', cell['text'])), key=lambda start: abs(start-at))
                first = {**cell, 'left': anchors[covered[0]][0], 'width': 1, 'text': cell['text'][:cut]}
                second = {**cell, 'left': anchors[covered[1]][0], 'width': 1, 'text': cell['text'][cut+1:]}
                result += [first, second]
            elif sentence:
                first = {**cell, 'left': anchors[covered[0]][0], 'width': 1, 'text': cell['text'][:sentence].strip()}
                second = {**cell, 'left': anchors[covered[1]][0], 'width': 1, 'text': cell['text'][sentence:].strip()}
                result += [first, second]
            else:
                result.append(cell)
        return result
    tops = [line.get('top') for line in block]
    previous = None
    for cells, top in zip(rows, tops):
        wrapped = top is not None and previous is not None and 16 <= top-previous <= 19
        previous = top
        cells = split_spanning(cells)
        spanning = [c for c in cells if SPANNING.fullmatch(c['text'])]
        if spanning:
            notes.append(spanning[0]['text'].strip('—–-… '))
            cells = [c for c in cells if c not in spanning]
            if not cells:
                continue
        row = ['']*len(anchors)
        for cell in cells:
            at = column(cell)
            row[at] = joined(row[at], cell['text']) if row[at] else cell['text']
        semibold = all(c['semibold'] for c in cells) or (not grid and header_row(cells))
        if grid and semibold and header_rows == len(grid):
            # A header cell wrapped onto a second line.
            grid[-1] = [joined(a, b) if b else a for a, b in zip(grid[-1], row)]
        elif grid and not semibold and ((wrapped and header_rows) or not row[0] or (sum(1 for c in row if c) == 1 and len(anchors) > 2 and (row[0].startswith('(') or row[0][:1].islower()))):
            # A long cell wrapped. The row continues.
            grid[-1] = [joined(a, b) if b else a for a, b in zip(grid[-1], row)]
        else:
            grid.append(row)
            if semibold and header_rows == len(grid)-1:
                header_rows = len(grid)
    used = [i for i in range(len(anchors)) if any(row[i] for row in grid)]
    if not grid or not used:
        return ''
    if len(used) == 1:
        return '\n'.join(row[used[0]].replace('|', '\\|') for row in grid if row[used[0]])
    cell = lambda text: text.replace('|', '\\|')
    if not header_rows:
        grid.insert(0, ['']*len(anchors))
    text = ['| '+' | '.join(cell(grid[0][i]) for i in used)+' |', '|'+' --- |'*len(used)]
    text += ['| '+' | '.join(cell(row[i]) for i in used)+' |' for row in grid[1:]]
    prefix = ''.join(f'*{note}*\n\n' for note in notes)
    return prefix+'\n'.join(text)+(f'\n\n{footnote}' if footnote else '')


def sidebar_title(line):
    return line['role'] == 'body' and any(c for c in [line.get('smallcaps')]) and len(line['text']) > 5 and not STAT_LABELS.match(line['text'])


def sidebar_paragraphs(block):
    paragraphs, paragraph = [], ''
    for item in block:
        if (item['raw'].startswith('  ') or item['titled'] or item['text'].startswith('•')) and paragraph:
            paragraphs.append(paragraph)
            paragraph = ''
        paragraph = joined(paragraph, item['text'])
    return paragraphs+([paragraph] if paragraph else [])


def is_sidebar(block):
    # A table can have one printed cell per row, but it opens with a header row.
    if all(cell['semibold'] for cell in block[0]['cells']):
        return False
    single = [line for line in block if len(line['cells']) == 1]
    long_lines = [line for line in single if line['width'] >= 150]
    return len(single)*10 >= len(block)*7 and len(long_lines)*2 >= len(block)


def render(lines):
    """Readable Markdown from positioned lines: one paragraph per printed paragraph,
real tables, and sidebars moved out of the sentences they interrupt. Column order,
wording and hyphenated compounds come from the PDF."""
    out, held = [], []
    words, label_lines = '', []
    page = None
    state = {'previous': None, 'marked': None}

    def flush():
        nonlocal words, label_lines
        if label_lines or words:
            text = '\\\n'.join(label_lines+([words] if words else []))
            out.append(text)
        words, label_lines = '', []
        out.extend(held)
        held.clear()

    def mark(number):
        if state['marked'] != number and not words and not label_lines:
            out.append(f'<!-- SRD page {number} -->')
            state['marked'] = number

    index = 0
    in_stat_header = False
    while index < len(lines):
        line = lines[index]
        previous = state['previous']
        if line['role'] in ['entry', 'stat', 'heading', 'subheading']:
            flush()
            mark(line['page'])
            out.append('### '+line['text'])
            state['previous'] = None
            in_stat_header = False
            index += 1
            continue
        if line['header'] and not line['gill'] and (STAT_LABELS.match(line['text']) or SIZE_LINE.match(line['text']) or in_stat_header or line['smallcaps'] or line['size'] <= 15 and re.match(r'^[+−-]?\d', line['text'])):
            # A stat block's header lines stay lines. Wrapped values join their label.
            if not in_stat_header:
                flush()
                mark(line['page'])
            in_stat_header = True
            starts_row = STAT_LABELS.match(line['text']) or SIZE_LINE.match(line['text']) or not label_lines
            # The print sets `Con` tight against its score.
            stat_text = re.sub(r'\b(Str|Dex|Con|Int|Wis|Cha)(\d)', r'\1 \2', line['text'])
            if starts_row:
                label_lines.append(stat_text)
            else:
                label_lines[-1] = joined(label_lines[-1], stat_text)
            state['previous'] = line
            index += 1
            continue
        if in_stat_header:
            flush()
            in_stat_header = False
        if line['gill'] and any(line['text'].startswith(label+':') for label in RULE_LABELS):
            # A spell, background or tool header: labelled lines in the table font.
            mark(line['page'])
            if words:
                label_lines.append(words)
                words = ''
            while index < len(lines) and lines[index]['gill'] and lines[index]['role'] == 'body':
                item = lines[index]
                item_text = re.sub(' +', ' ', item['text'])
                if any(item_text.startswith(label+':') for label in RULE_LABELS) or not label_lines:
                    label_lines.append(item_text)
                else:
                    label_lines[-1] = joined(label_lines[-1], item_text)
                index += 1
                if item['text'].startswith('Duration:'):
                    break
            state['previous'] = None
            flush_after_labels = index >= len(lines) or not lines[index]['gill']
            if flush_after_labels:
                flush()
            continue
        if line['gill'] or line['caption']:
            block = []
            caption = None
            if line['caption']:
                caption = line['text']
                index += 1
            while index < len(lines) and lines[index]['gill'] and lines[index]['role'] == 'body' and lines[index]['page'] == line['page'] and lines[index]['band'] == line['band'] and lines[index]['col'] == line['col']:
                block.append(lines[index])
                index += 1
            text = []
            if caption:
                text.append(f'**{caption}**')
            if block and is_sidebar(block) and not caption:
                text.extend(sidebar_paragraphs(block))
            elif block:
                text.append(table_markdown(block))
            chunk = [t for t in text if t]
            # A table or sidebar can stand in the middle of a sentence on the page.
            if words and not re.search(r'[.!?:”)]$', words):
                held.extend(chunk)
            else:
                flush()
                mark(line['page'])
                out.extend(chunk)
            state['previous'] = None
            continue
        if line.get('smallcaps') and sidebar_title(line):
            title = '### '+line['text']
            if words and not re.search(r'[.!?:”)]$', words):
                held.append(title)
                index += 1
                # The sidebar body follows in GillSans and is held with its title.
                block = []
                while index < len(lines) and lines[index]['gill'] and lines[index]['role'] == 'body' and lines[index]['page'] == line['page']:
                    block.append(lines[index])
                    index += 1
                held.extend(sidebar_paragraphs(block))
                continue
            flush()
            mark(line['page'])
            out.append(title)
            state['previous'] = None
            index += 1
            continue
        text = line['text']
        # A subclass's motto opens its section, in italic, with no comma or full stop.
        if line['italic'] and not words and not label_lines and all(part.startswith('<!--') for part in out) and MOTTO.fullmatch(text):
            mark(line['page'])
            out.append('### '+text)
            state['previous'] = None
            index += 1
            continue
        labelled = any(text.startswith(label+':') for label in RULE_LABELS) or SPELL_USES.match(text)
        gap = line['top']-previous['top'] if previous and previous['page'] == line['page'] and previous['col'] == line['col'] and previous['band'] == line['band'] else None
        # A long title can wrap, and then its second line is titled too.
        titled = line['titled'] and (not words or re.search(r'[.!?:”)]$', words) or (words.startswith('•') and RUN_IN.match(text)) or ' = ' in text)
        new_block = line['raw'].startswith('  ') or titled or text.startswith('•') or (re.match(r'^\d+[.:]\s', text) and (not words or re.search(r'[.!?:”)]$', words))) or (gap is not None and gap >= 22)
        if not words and label_lines and not labelled:
            new_block = True
        if gap is None and previous is not None and re.search(r'[.!?]$', words) and 'Optima' in line['family'] and RUN_IN.match(text):
            new_block = True
        if labelled and (label_lines or words):
            if not label_lines and len(words.split()) > 14:
                flush()
                mark(line['page'])
            else:
                if words:
                    label_lines.append(words)
                words = text
                state['previous'] = line
                index += 1
                continue
        if new_block and (words or label_lines):
            flush()
        mark(line['page'])
        words = joined(words, text)
        if label_lines and new_block is False:
            pass
        state['previous'] = line
        index += 1
    flush()
    return '\n\n'.join(part for part in out if part.strip()).strip()


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--pdf',type=Path,help='Use a local copy of the pinned official PDF')
    ap.add_argument('--output',type=Path,default=Path('builtin/srd-5.2.1'))
    ap.add_argument('--check',action='store_true',help='Regenerate in memory/temp and compare without edits')
    args=ap.parse_args()
    with tempfile.TemporaryDirectory(prefix='splat-srd-') as scratch:
        temp=Path(scratch);pdf=temp/'source.pdf'
        if args.pdf:shutil.copyfile(args.pdf,pdf)
        else:
            with urllib.request.urlopen(URL,timeout=60) as response:pdf.write_bytes(response.read())
        if digest(pdf.read_bytes())!=PDF_SHA256:raise ValueError('Official source hash differs from pinned SRD 5.2.1')
        subprocess.run(['pdftotext','-layout','-enc','UTF-8',str(pdf),str(temp/'source.txt')],check=True)
        subprocess.run(['pdftohtml','-xml','-hidden','-i',str(pdf),str(temp/'source.xml')],check=True,stdout=subprocess.DEVNULL)
        text=(temp/'source.txt').read_text();pages=text.split('\f')
        if pages[-1].strip():raise ValueError('Missing terminal PDF page break')
        pages=pages[:-1]
        if len(pages)!=364 or any(not p.strip() for p in pages):raise ValueError('Incomplete full-page extraction')
        lines=positioned_lines((temp/'source.xml').read_text())
        for patch in PATCHES:
            if patch['find'] not in pages[patch['page']-1]:raise ValueError('Transcription patch not found: '+patch['entry'])
        files={'source.pdf':pdf.read_bytes(),'source.txt':text.encode(),'transcription_patches.json':PATCH_FILE.read_bytes(),
               'LICENSE.txt':(ATTRIBUTION+'\n\nExtracted and reformatted as Markdown by Splat. Source PDF is unmodified.'+CHANGES+'\n').encode()}
        records=[]
        def add(path,content,category,page_numbers):
            raw=content.encode();files[path]=raw
            records.append({'path':path,'sha256':digest(raw),'category':category,'pages':page_numbers})
        for n,page in enumerate(pages,1):
            section=next(title for start,title in reversed(SECTIONS) if n>=start)
            headings=[l['text'] for l in lines if l['page']==n and l['role']!='body']
            body=f'# {section} — page {n}\n\nFull extracted page layout; use the bundled source PDF for original typography.\n\n```text\n'+'\n'.join(line.rstrip() for line in patched(n,page).rstrip().splitlines())+'\n```\n'
            ref={'source':'SRD 5.2.1','category':'page','pages':[n],'section':section,'headings':headings,'executable':False}
            add(f'pages/{n:03}.md',yaml_note(f'reference/pages/{n:03}',f'{section} — {n}','reference',{'splat_reference':ref},body),'page',[n])
        groups=[('spell',lambda l:107<=l['page']<=175 and l['role']=='entry',339),
                ('magic_item',lambda l:209<=l['page']<=253 and l['role']=='entry',258),
                ('creature',lambda l:l['role']=='stat',336),
                ('glossary',lambda l:176<=l['page']<=191 and l['role']=='entry',155)]
        counts={}
        for category,predicate,expected in groups:
            starts=[i for i,l in enumerate(lines) if predicate(l)]
            if len(starts)!=expected:raise ValueError(f'{category}: expected {expected}, found {len(starts)}; '+str([lines[i]['text'] for i in starts[:8]]))
            counts[category]=len(starts)
            for pos,start in enumerate(starts):
                first=lines[start];name=first['text'];end=starts[pos+1] if pos+1<len(starts) else len(lines)
                # Entries may span columns/pages. Stop at the chapter boundary;
                # embedded stat blocks also stop at a later outer entry heading.
                boundary={'spell':176,'magic_item':254,'creature':365,'glossary':192}[category]
                for i in range(start+1,end):
                    if lines[i]['page']>=boundary or (category=='creature' and lines[i]['role'] in ['entry','heading']):end=i;break
                    # A stat block is in Optima. Inside a spell or item, body type ends it.
                    if category=='creature' and lines[i]['page']<254 and 'Cambria' in lines[i]['family']:end=i;break
                span=lines[start:end];pnums=sorted({l['page'] for l in span})
                key=f'reference/{category}/{slug(name)}';path=f'{category}/{slug(name)}.md'
                if path in files:raise ValueError(f'Duplicate entry name {category}/{name}')
                ref={'source':'SRD 5.2.1','category':category,'pages':pnums,'executable':False}
                body='# '+name+'\n\n'+render(span[1:])+'\n\nOriginal layout: '+', '.join(f'[[../pages/{p:03}|page {p}]]' for p in pnums)+'.\n'
                add(path,yaml_note(key,name,'reference',{'splat_reference':ref},body),category,pnums)
        manifest=yaml_note('package/srd','System Reference Document 5.2.1','package',
                           {'splat_namespace':'dnd2024','splat_reference':{'source':'SRD 5.2.1','category':'package','pages':list(range(1,365)),'executable':False}},
                           '# System Reference Document 5.2.1\n\n'+ATTRIBUTION+'\n\nComplete source-reference package. No executable mechanics are inferred from prose.'+CHANGES+'\n')
        add('Package.md',manifest,'package',list(range(1,365)))
        audit={'format':1,'source_url':URL,'source_sha256':PDF_SHA256,'source_text_sha256':digest(text.encode()),
               'pages':364,'counts':counts,'files':sorted(records,key=lambda r:r['path'])}
        files['inventory.json']=(json.dumps(audit,ensure_ascii=False,indent=2)+'\n').encode()
        if args.check:
            actual={str(p.relative_to(args.output)):p.read_bytes() for p in args.output.rglob('*') if p.is_file()}
            changed=sorted(k for k in set(actual)|set(files) if actual.get(k)!=files.get(k))
            if changed:raise ValueError('Regeneration differs: '+', '.join(changed[:20]))
        else:
            args.output.mkdir(parents=True,exist_ok=True)
            # Do not delete unknown user files. Refuse stale generated output instead.
            stale={str(p.relative_to(args.output)) for p in args.output.rglob('*') if p.is_file()}-files.keys()
            if stale:raise ValueError('Unexpected files in output: '+', '.join(sorted(stale)))
            for name,data in files.items():
                dest=args.output/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
        print(json.dumps({'pages':364,'entries':counts,'markdown_notes':len(records),'source_sha256':PDF_SHA256,'checked':args.check}))

if __name__=='__main__':main()
