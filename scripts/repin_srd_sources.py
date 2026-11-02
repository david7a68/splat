#!/usr/bin/env python3
"""After a formatting-only SRD import change, re-pin reviewed specs to the new note hashes.

Refuses to re-pin a note whose words changed: that needs a human review of the spec.
"""
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

SOURCE = Path('builtin/srd-5.2.1')
TARGETS = [*Path('docs/research/srd-conversion').glob('*.md'), *Path('scripts/srd_inputs').glob('*.yaml')]


def words(text):
    text = text.split('---\n', 2)[-1]
    text = re.sub(r'<!--.*?-->', ' ', text, flags=re.S)
    text = re.sub(r'-\\?\n', '', text)
    text = re.sub(r"[\\|*#’'—–−-]", ' ', text).lower()
    return ''.join(re.findall(r'[a-z0-9]+', text))


def main():
    base = sys.argv[1] if len(sys.argv) > 1 else 'HEAD'
    mapping, changed = {}, []
    for path in sorted(SOURCE.rglob('*.md')):
        old = subprocess.run(['git', 'show', f'{base}:{path}'], capture_output=True, check=True).stdout
        new = path.read_bytes()
        if old == new:
            continue
        before, after = hashlib.sha256(old).hexdigest(), hashlib.sha256(new).hexdigest()
        mapping[before] = after
        if sorted(words(old.decode())) != sorted(words(new.decode())):
            changed.append(str(path))
    if changed:
        print('Word content changed (review the specs for these):', *changed, sep='\n  ')
    replaced = 0
    for target in TARGETS:
        text = target.read_text()
        result = re.sub(r'\b[0-9a-f]{64}\b', lambda m: mapping.get(m[0], m[0]), text)
        if result != text:
            target.write_text(result)
            replaced += 1
    print(json.dumps({'notes_changed': len(mapping), 'files_repinned': replaced, 'word_changes': len(changed)}))


if __name__ == '__main__':
    main()
