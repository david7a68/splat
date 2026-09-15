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


def plain(raw):
    return html.unescape(re.sub('<[^>]*>', '', raw)).replace('\u00ad', '')


def slug(name):
    return re.sub(r'[^a-z0-9]+', '_', name.lower()).strip('_')


def yaml_note(key, name, kind, fields, body):
    data = {'splat_schema': 1, 'splat_id': str(uuid.uuid5(PACKAGE_ID,key)),
            'splat_kind': kind, 'splat_key': 'dnd2024:'+key, 'splat_name':name, **fields}
    # JSON flow values are a strict YAML subset, avoiding custom YAML escaping.
    return '---\n'+''.join(k+': '+json.dumps(v,ensure_ascii=False)+'\n' for k,v in data.items())+'---\n\n'+body.rstrip()+'\n'


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
            text=plain(text)
            if not text.strip():continue
            red=font['color']=='#88191f'
            role='stat' if red and font['size']=='23' else 'entry' if red and 'SemiBold' in font['family'] and font['size'] in ['18','13'] else 'heading' if red and int(font['size'])>=27 else 'subheading' if red else 'body'
            fragments.append({'page':n,'col':int(left>width/2),'top':top,'left':left,
                              'width':int(d['width']),'text':text,'role':role,'size':int(font['size']), 'smallcaps':'SC700' in font['family']})
        fragments.sort(key=lambda d:(d['col'],d['top'],d['left']))
        lines=[]
        for f in fragments:
            candidates=[l for l in lines[-12:] if l['col']==f['col'] and abs(l['top']-f['top'])<=6 and l['role']==f['role']]
            if candidates:
                candidates[-1]['parts'].append(f)
            else:lines.append({**f,'parts':[f]})
        lines.sort(key=lambda d:(d['col'],d['top'],d['left']))
        for line in lines:
            parts=sorted(line.pop('parts'),key=lambda d:d['left']);text='';right=None
            for f in parts:
                gap=f['left']-right if right is not None else 0
                if text and not text[-1].isspace() and not f['text'][0].isspace() and gap>3:text+=' '
                text+=f['text'];right=f['left']+f['width']
            line['text']=text.strip().title() if any(f['smallcaps'] for f in parts) else text.strip();line['size']=max(f['size'] for f in parts)
            if line['role']=='entry' and line['size']<18:line['role']='body'
            # Join a genuinely wrapped entry title (not the next prose line).
            if result and line['role']=='entry' and result[-1]['role']=='entry' and line['page']==result[-1]['page'] and line['col']==result[-1]['col'] and 7<line['top']-result[-1]['top']<=24:
                result[-1]['text']+=' '+line['text']
            else: result.append(line)
    return result


def render(lines):
    # Preserve logical column order and line breaks. Do not guess whether a printed
    # hyphen is semantic. Facsimile page notes/PDF retain tables in their full layout.
    out=[];last_page=None
    for line in lines:
        if line['page']!=last_page:
            out.append(f'\n<!-- SRD page {line["page"]} -->\n');last_page=line['page']
        if line['role'] in ['entry','stat','heading','subheading']:
            out.append('\n### '+line['text']+'\n')
        else:out.append(line['text']+'\\')
    return '\n'.join(out).strip()


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
        files={'source.pdf':pdf.read_bytes(),'source.txt':text.encode(),
               'LICENSE.txt':(ATTRIBUTION+'\n\nExtracted and reformatted as Markdown by Splat. Source PDF is unmodified.\n').encode()}
        records=[]
        def add(path,content,category,page_numbers):
            raw=content.encode();files[path]=raw
            records.append({'path':path,'sha256':digest(raw),'category':category,'pages':page_numbers})
        for n,page in enumerate(pages,1):
            section=next(title for start,title in reversed(SECTIONS) if n>=start)
            headings=[l['text'] for l in lines if l['page']==n and l['role']!='body']
            body=f'# {section} — page {n}\n\nFull extracted page layout; use the bundled source PDF for original typography.\n\n```text\n'+'\n'.join(line.rstrip() for line in page.rstrip().splitlines())+'\n```\n'
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
                span=lines[start:end];pnums=sorted({l['page'] for l in span})
                key=f'reference/{category}/{slug(name)}';path=f'{category}/{slug(name)}.md'
                if path in files:raise ValueError(f'Duplicate entry name {category}/{name}')
                ref={'source':'SRD 5.2.1','category':category,'pages':pnums,'executable':False}
                body='# '+name+'\n\n'+render(span[1:])+'\n\nOriginal layout: '+', '.join(f'[[../pages/{p:03}|page {p}]]' for p in pnums)+'.\n'
                add(path,yaml_note(key,name,'reference',{'splat_reference':ref},body),category,pnums)
        manifest=yaml_note('package/srd','System Reference Document 5.2.1','package',
                           {'splat_namespace':'dnd2024','splat_reference':{'source':'SRD 5.2.1','category':'package','pages':list(range(1,365)),'executable':False}},
                           '# System Reference Document 5.2.1\n\n'+ATTRIBUTION+'\n\nComplete source-reference package. No executable mechanics are inferred from prose.\n')
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
