# Markdown content layout

A note holds its data in flat properties, in the readable sections of its kind,
and in explicitly marked body sections. The reader combines them into one note
model before shape, reference and execution checks. The body is preserved
exactly; reading does not rewrite a file.

Keep identity, namespaced key, kind, name, schema version, origin, source
citations and tags in properties. Put readable descriptions in ordinary
Markdown. A creature's stats are in its `Stat block` tables. A spell's or item's
fields are in its `Spell` or `Item` table. Other game fields belong in marked
tables or local YAML blocks. Splat writes a field that has no readable form as
a YAML block in a folded **Splat data** callout at the end of the note. The
headings above marked tables and blocks have no parsing role: users can rename,
move and fold them in Obsidian.

## Optional reading summaries

`splat_reading` holds edited reading text. Splat writes it as a marked YAML
block in the Splat data callout. It does not change an
entry's identity, mechanical specification, execution status, or required
references. The library uses it when present and otherwise keeps the ordinary
reader. Published wording remains under **Source text**.

````markdown
<!-- splat:yaml path=splat_reading -->

```yaml
title: Bard Subclass
summary: |-
  Choose a Bard subclass. Your subclass grants additional features as you gain Bard levels.

  You gain all features from your chosen subclass whose required level is equal to or below your Bard level.
related_heading: Available in this package
related:
  dnd2024:subclasses/bard_college_of_lore: College of Lore
```
````

Title and summary are required nonempty strings. `related` is an optional map
from resource keys or UUIDs to display labels; `related_heading` defaults to
“Related entries”. Links resolve within the current package. Missing or duplicate
identities show as unavailable and do not become required game dependencies.
The library obtains class and level from the existing rule `class_scope`, when
present. Related links describe navigation, not the limits of a rule's choices.

## Mapped record tables

Use a `splat:table` comment to declare the collection, its stable row identities,
and how readable columns map to nested fields. The declaration is hidden in
Obsidian reading view; the table remains ordinary Markdown.

```markdown
<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order: [arrows, bolts]
columns:
  Variant: {path: id, type: symbol}
  Quantity: {path: purchase_quantity, type: integer}
  Weight: {path: mass, type: quantity}
  Cost: {path: price, type: quantity}
-->

| Variant | Quantity | Weight | Cost |
| --- | --- | --- | --- |
| Arrows | 20 | 1 lb | 1 GP |
| Bolts | 20 | 1½ lb | 1 GP |
```

This reconstructs the original record list, including nested `mass` and `price`
mappings. Column paths are relative to each row. `order` gives the output list
order and must name every row once. Sorting the visible rows or rearranging
columns does not change the data. Change `order` explicitly to change the list.

`key` names the identity column. Symbols such as “Firearm bullets” resolve to
`firearm_bullets`. A reference identity uses the last component of its resource
key. When records have no suitable identity, use a `type: key` column without a
`path`; its local key exists only to address the row. Generated keys must remain
with their rows when editing; they are not display positions.

### Column types

| Type | Cell | Stored value |
| --- | --- | --- |
| `string` | `false` | Literal string `false`; JSON quotes can protect ambiguous text |
| `symbol` | `At will` | String `at_will` |
| `integer` | `20` | Integer 20 |
| `number` | `1.5`, `1½`, `1/2` | Finite numeric value; explicit decimal notation retains float type |
| `boolean` | `false` | Boolean false |
| `quantity` | `1½ lb.`, `1 GP` | `{value: 1.5, unit: lb}`, `{value: 1, unit: gp}` |
| `reference` | Resource key, UUID, or link with identity title | `{key: ...}` or `{id: ...}` |
| `dice` | `2d4+2` | `{count: 2, sides: 4, add: 2}` |
| `range` | `1–20` or `7` | `{min: 1, max: 20}` or `{min: 7, max: 7}` |
| `yaml` | `[held, worn]` | Explicit YAML value for a small collection |
| `key` | `row_one` | Stable row address; no game field unless `path` is supplied |
| `section` | A Rules link | Navigation only; no game data |

A compound column can declare `fields`, such as `[count, sides, modifier]` for
dice or `[start, end]` for ranges. An empty `path` distributes those compound
fields directly onto the row, so a single Roll column can supply `min` and `max`.
There is no formula evaluation or user-supplied parser code.

Blank cells are omitted only when `optional: true`; otherwise they are errors.
`nullable: true` allows explicit `null`. Missing, null, false, zero, and an empty
collection stay distinct. Quoted `"null"` in a string column is literal text.
Unit names are normalized to lowercase. The owning content schema still checks
which units, values and rules are valid in its domain.

### References and row details

A navigable reference looks like:

```markdown
[Quiver](quiver_1_gp.md "dnd2024:equipment/quiver_1_gp")
```

The title holds the authoritative identity. Link text and the navigation path
do not determine identity. Renaming a file or its display text does not change
the reference. Plain resource keys and UUIDs also work.

Keep complex rules in YAML blocks addressed through the stable key:

````markdown
### Bolts

<!-- splat:yaml path=splat_rule/parameters/variants/@bolts/special -->

```yaml
requires: held
```
````

Blocks may appear before their table. Nested tables can use the same `@key`
syntax at each level. Numeric addresses under a mapped table are rejected.
A block and a column cannot both assign the same field or overlapping parents.
Unknown keys, duplicate keys, duplicate column targets, malformed values and
missing rows produce errors instead of silent overrides.

Generated Details columns link to Obsidian block anchors. Their link titles use
`section:<row_key>` to keep navigation tied to a stable row. Section links do not
supply rules; the marked YAML blocks do.

The SRD definition generator (`scripts/convert_srd_definitions.py`) selects
tables from record shapes. Variants, pack contents,
spell lists and simple roll results share this mechanism. Branching action
sequences and wide records retain YAML or the existing table layout. Checked
source-specific presentation boundaries remove duplicated source tables while
retaining descriptive prose and citations; the original pinned source remains.

## Field-name marked tables

The reader also accepts four simpler table markers. The built-in SRD notes use
the values and rows forms. Each marker must immediately precede its table, with
optional blank lines.
Paths address schema fields inside this note, not files or entry IDs. Separate
path components with `/`. Local map keys use the existing lowercase key grammar.

These markers do not supply a creature's `splat_stats`, a spell's `splat_spell`
or an item's `splat_item`. The `Stat block`, `Spell` and `Item` tables supply
those fields. The reader ignores a marker that addresses them and flags it:
`Unused data ignored`.

A values table fills a mapping. Column labels in these formats are exact field names:

```markdown
<!-- splat:values path=splat_rule/parameters/feature_grant_policy -->

| Field | Value |
| --- | --- |
| class_level_at_least_feature_level | true |
| events | ["subclass.selected", "character.level_gained"] |
| persistent | true |
```

A map table fills keyed records. This works for resource pools, charges,
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
calculation before external modifiers. Use it in a note that is not a creature;
a creature's scores and saves are in its `Stat block` tables.

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
links and inline controls do not become data without a marker. Only mapped
reference and section columns interpret links.

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
for source review. Research input files remain the generator's authored inputs;
manual changes to generated notes are protected against silent overwrite.
