"""Publish generated notes through Splat's model-checked schema-2 converter."""
import atexit
import json
from pathlib import Path
import subprocess

_process = None


def convert(text, path='generated.md'):
    global _process
    if _process is None:
        binary = Path('target/debug/format-notes')
        if not binary.is_file():
            raise RuntimeError('Build the generator bridge: cargo build --locked --bin format-notes')
        _process = subprocess.Popen([str(binary)], stdin=subprocess.PIPE,
                                    stdout=subprocess.PIPE, text=True)
        atexit.register(_process.terminate)
    _process.stdin.write(json.dumps([str(path), text]) + '\n')
    _process.stdin.flush()
    result = json.loads(_process.stdout.readline())
    if 'Err' in result:
        raise ValueError(f'{path}: {result["Err"]}')
    return result['Ok']
