#!/usr/bin/env bash
# Regenerate every SRD note from the pinned PDF, re-pinning reviewed specs when only formatting moved.
set -euo pipefail
cargo build --locked --bin format-notes
python3 scripts/import_srd.py --pdf builtin/srd-5.2.1/source.pdf
# Restores from the index, so stage a hand edit to these specs before running this.
git checkout -- docs/research/srd-conversion scripts/srd_inputs
python3 scripts/repin_srd_sources.py HEAD
python3 scripts/convert_srd_definitions.py > /dev/null
python3 scripts/convert_srd.py > /dev/null
python3 scripts/import_srd.py --pdf builtin/srd-5.2.1/source.pdf --check > /dev/null
python3 scripts/convert_srd_definitions.py --check > /dev/null
echo regenerated
