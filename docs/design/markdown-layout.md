# Markdown content layout

Schema 1 accepts data in frontmatter or explicitly marked body sections. The
reader combines them before shape, reference and execution checks. Older notes
remain valid. The body is preserved exactly; reading does not rewrite a file.

Keep identity, namespaced key, kind, name, schema version, origin, source
citations and tags in frontmatter. Put readable descriptions in ordinary
Markdown. Game fields belong in marked tables or local YAML blocks. Headings
have no parsing role: users can rename, move and fold them in Obsidian.

## Marked tables

Each marker must immediately precede its table, with optional blank lines.
Paths address schema fields inside this note, not files or entry IDs. Separate
path components with `/`. Local map keys use the existing lowercase key grammar.

A values table fills a mapping. Column labels are exact schema syntax:

```markdown
<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| level | 3 |
| school | evocation |
| concentration | false |
| classes | [sorcerer, wizard] |
```

A map table fills keyed records. This works for stats, resource pools, charges,
and other maps with simple columns. Empty cells omit optional fields.

```markdown
<!-- splat:map path=splat_resources -->

| Key | max | initial |
| --- | --- | --- |
| charges | 7 | 7 |
| absorption | 20 | 0 |
```

An abilities table supplies scores and optional total save overrides. A blank
save leaves it absent; it does not set zero. Supplied saves replace the normal
calculation before external modifiers.

```markdown
<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| Intelligence | 6 | +2 |
| Strength | 18 | |
```

A rows table fills an ordered list of records. `Row` is a zero-based address,
not an actor ID or display order. Rows can appear in any physical order, but
their addresses must be unique and contiguous. This supports class progression,
random outcomes and encounter rosters. Other columns use exact field names.

```markdown
<!-- splat:rows path=splat_rule/parameters/rows -->

| Row | level | proficiency_bonus |
| --- | --- | --- |
| 0 | 1 | 2 |
| 1 | 2 | 2 |
```

Nested data for a row can go in a separate block at, for example,
`splat_rule/parameters/rows/0/grants`. Do not also supply that field in a cell.
Changing list addresses changes the association with those blocks; a future
editor must update them together. Keyed maps are preferable where the schema
already gives records stable local keys.

Table values are YAML values. Quote ambiguous strings such as `"true"`, `"007"`
and `"null"`; unquoted forms have YAML types. Generated cells use inline code
to protect their text from Markdown. JSON escapes preserve newlines, backslashes
and pipes inside quoted strings (`"a\u007cb"` stores `a|b`). Blank optional cells
mean absent; explicit `null`, `false`, `0`, `[]` and `{}` remain distinct.

The existing `splat:inventory` tables and container markers keep their original
columns and semantics. They can coexist with these sections.

## Local mechanics blocks

Use a marked YAML fence for nested triggers, targeting, recovery and effect
rules. The fence is data, never executable code. Standard YAML highlighting
works in Obsidian without a plugin.

````markdown
### Recharge

<!-- splat:yaml path=splat_resources/charges/recovery -->

```yaml
- key: dawn
  mechanic: dnd2024:dawn
  amount: 1d6+1
```
````

This example illustrates placement; the owning rule still defines the recovery
record's required fields and semantics. A marker can also bind an entire field,
including an empty mapping or list.

Every field has one authoritative location. Duplicate keys, overlapping parent
and child assignments, conflicting frontmatter, invalid paths, missing/extra
cells, row gaps, detached markers and unclosed YAML fences are errors. No
last-write-wins merge occurs. Ordinary tables, code examples, prose, Obsidian
links and inline controls do not become data without a marker.

## Generated reports and source text

The SRD catalogue stores conversion status, provenance hashes, adjudications
and engine gaps in `coverage.json`. Contract registry format 2 stores interfaces,
capabilities, closed structural schemas and specification fingerprints. It no
longer repeats the full specification: the note body holds that data.

Catalogue loading attaches these reports in memory and validates the resolved
fields. A standalone note can be read without a report; its rule implementation
still identifies an unavailable adapter and cannot pass as an executable profile.
Directory validation is structural; use `verify-content --definitions` for the
embedded catalogue's strict report, provenance and fingerprint checks.

The source excerpt appears once in the body under Source text. It is reference
material, not a second editable mechanics store. The old `splat_clauses.text`
copy is omitted. Source excerpts and the pinned PDF remain available separately
for source review. Research input files remain the converter's authored inputs;
manual changes to generated notes are protected against silent overwrite.
