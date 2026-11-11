"""Write and read notes through Splat's own checked schema-2 writer and reader."""
import atexit
import json
from pathlib import Path
import subprocess

_process = None


def _ask(request, path):
    global _process
    if _process is None:
        binary = Path('target/debug/format-notes')
        if not binary.is_file():
            raise RuntimeError('Build the generator bridge: cargo build --locked --bin format-notes')
        _process = subprocess.Popen([str(binary)], stdin=subprocess.PIPE,
                                    stdout=subprocess.PIPE, text=True)
        atexit.register(_process.terminate)
    _process.stdin.write(json.dumps(request, ensure_ascii=False) + '\n')
    _process.stdin.flush()
    result = json.loads(_process.stdout.readline())
    if 'Err' in result:
        raise ValueError(f'{path}: {result["Err"]}')
    return result['Ok']


def write(fields, body, path='generated.md'):
    """Lay out a note from its fields and the body below its properties, which can hold marked data."""
    return _ask({'write': {'path': str(path), 'fields': fields, 'body': body}}, path)


def read(text, path='note.md'):
    """Read a schema-2 note into its model: fields (without the schema), id, key, name and kind."""
    return json.loads(_ask({'read': {'path': str(path), 'text': text}}, path))
