"""Compare regenerated models with a Git revision, or prove a conversion in memory."""
from pathlib import Path
from schema2_notes import convert
import argparse
import json
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--base', help='Compare generated models with this Git revision')
args = parser.parse_args()
failures = []
count = 0
for path in sorted(Path('builtin').rglob('*.md')):
    text = path.read_text()
    if 'splat_schema:' not in text:
        continue
    try:
        after = convert(text, path)
        if 'splat_schema: 2' not in after:
            raise ValueError('Not converted')
        if args.base:
            old = subprocess.run(['git', 'show', f'{args.base}:{path}'],
                                 capture_output=True, text=True, check=True).stdout
            before = json.loads(convert(old, '__entry_model__'))
            model = json.loads(convert(after, '__entry_model__'))
            if before != model:
                raise ValueError('Parsed model differs from baseline')
        count += 1
    except ValueError as error:
        failures.append(f'{path}: {error}')
print(json.dumps({'compared': count, 'differences_or_refusals': failures}, indent=2))
raise SystemExit(bool(failures))
