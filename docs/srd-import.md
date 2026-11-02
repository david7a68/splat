# Built-in SRD and read-only verification

Splat bundles the complete English **SRD 5.2.1**, which covers the revised
D&D 2024 rules. The official source is the [Wizards PDF](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf).
This import requires no D&D Beyond account or purchased books.

The package lives in `builtin/srd-5.2.1`. It includes:

| Content | Count |
| --- | ---: |
| Original PDF pages, each also stored as a Markdown page note | 364 |
| Named spell excerpts | 339 |
| Named magic item excerpts | 258 |
| Named creature stat block excerpts, including six embedded blocks | 336 |
| Named rules glossary excerpts | 155 |
| Total Markdown notes, including the package manifest | 1,453 |

All source sections are included. Character creation, classes, origins, feats,
equipment, and the gameplay toolbox are present in the page notes and full
source; they do not yet have separate notes for every named feature.

The PDF is unchanged. `source.txt` is the complete Poppler layout extraction.
Each page note preserves that page's text and spacing in a fenced block, with
trailing whitespace removed. `transcription_patches.json` lists the only exceptions:
printing errors corrected in the page notes and named notes, with the reason for
each. Verification applies this list when it compares page notes with `source.txt`. Named excerpts are for reading. They use column
reading order, one line per printed paragraph, and Markdown tables built from
the PDF's table cells. A table that spans both columns is read where it stands.
A line-end hyphen stays only for a real compound such as `5-foot-wide`. Stat
block and spell header lines keep hard line breaks. The page notes and PDF
provide the layout reference. The importer does not interpret prose as mechanics
or execute embedded text.

To change the importer, run `devenv shell -- bash scripts/regenerate_srd.sh`. It
regenerates the SRD and the definitions in place, then moves the `source_sha256`
pins in the reviewed specs from the committed notes to the new ones. It reports
each note whose words changed, and those need a review against the PDF. Commit
the specs and the notes together.

## Source-reference schema extension

`splat_kind: reference` has the normal UUID, schema, name and optional key,
plus `splat_reference`:

```yaml
splat_reference:
  source: SRD 5.2.1
  category: creature
  pages: [300]
  executable: false
```

The importer uses categories `page`, `spell`, `magic_item`, `creature`, and
`glossary`. Page references also have `section` and `headings`. The package
manifest has the same provenance metadata with category `package`.
Namespaced keys use `dnd2024:reference/<category>/<slug>`; page keys use
`dnd2024:reference/pages/001`. UUIDs are deterministically derived from the
source URL and logical key, independently of the disk path.

These are reference notes, **not** typed creature, item, spell or rule definitions.
They cannot satisfy an executable definition reference or the examples'
`dnd2024:core` engine contract. Converting source rules into checked mechanics
is separate work. Campaign selection and a library UI are not implemented by
this import. The embedded package is available through `splat::content::builtin`
and the verification CLI.

## Verification

Run from the repository:

```sh
devenv shell -- cargo run --locked --bin verify-content
devenv shell -- cargo run --locked --bin verify-content -- docs/design/examples
devenv shell -- cargo test --locked
devenv shell check
```

The CLI prints a JSON report and exits with status 1 on errors. Without paths it
verifies the embedded SRD, including pinned PDF/text hashes, exact file inventory,
per-note hashes, category counts, page ranges, attribution, and each page note
against the full extracted text. It can run without network access or the source
directory after compilation. `source_file` exposes the bundled original bytes.

With directory paths it reads marked frontmatter notes, ignores ordinary Markdown,
skips nested symlinks, and checks the notes together. It never writes to them.
Checks cover required common fields, UUID/key grammar and uniqueness, namespace
collisions, duplicate YAML mapping keys, kind payload mappings, recognized entry
reference shapes, stat value/derive alternatives, literal resource bounds,
marked inventory tables, separate item pools, container targets, and local and
cross-setup containment cycles. Missing optional dependencies are allowed.
Unknown Splat properties are retained with a warning; non-Splat metadata is retained.

This is an initial structural reader, not complete schema conformance. Nested
mechanics, ownership scopes, selections, expression types, local action bindings,
rule compatibility, replay fingerprints and game outcomes still need their
respective validators and engine. The structural report says `executable_entries: 0`. The separate
[`--executable` check](execution-proof.md) compiles the converted execution profiles.
The 17 design notes pass the supported checks; this does not establish that a
simulation can run them.

## Reproduce the import

Use Python 3 and Poppler (`pdftotext` and `pdftohtml`). The checked extraction
uses Poppler **26.08.0**. Different versions can produce different whitespace or
font metadata; `--check` reports those differences instead of accepting them.

```sh
python3 scripts/import_srd.py
python3 scripts/import_srd.py --pdf builtin/srd-5.2.1/source.pdf --check
```

The importer verifies the official PDF before extraction:

```text
SHA-256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
```

`inventory.json` records every generated Markdown path, hash, category, and
source page range. `--check` regenerates into temporary storage and compares
every output file without writing to the package. Import refuses unrecognized
stale files rather than deleting them. Do not edit generated files; put homebrew
in a separate owner directory. Font-based extraction checks expected counts and
representative entries, while the complete PDF/page inventory remains the source
coverage baseline.

## License

This work includes material from the System Reference Document 5.2.1 (“SRD 5.2.1”) by Wizards of the Coast LLC, available at https://www.dndbeyond.com/srd. The SRD 5.2.1 is licensed under the Creative Commons Attribution 4.0 International License, available at https://creativecommons.org/licenses/by/4.0/legalcode.

Splat extracts and reformats the material as Markdown. The source PDF is unchanged.
The attribution is also included in the package manifest and `LICENSE.txt`.
