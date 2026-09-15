# Definition conversion and rule contracts

The [Markdown layout](markdown-layout.md) defines body tables, local YAML blocks
and the single-authority rule. These schema-1 fields distinguish completed authoring, informational source,
and unavailable engine behavior. See the [SRD report](../srd-definitions.md)
for the source inventory and current results.

## Conversion metadata

`splat_conversion` is a closed in-memory mapping. Generated catalogue notes
store these fields once in `coverage.json`; catalogue loading attaches them
for verification:

| Field | Meaning |
| --- | --- |
| `status` | `partial`, `specified`, or `informational`; none grants execution |
| `category` | Source definition category |
| `source_path`, `source_sha256` | Exact pinned source file |
| `pages` | Nonempty SRD page list, 1–364 |
| `conversion_gaps` | Unresolved clause keys or extraction diagnostics |
| `engine_gaps` | Required unavailable registered mechanic names |
| `execution_profiles` | Keys of separately tested bounded projections |
| `parent_name` | Optional original heading context |
| `line_span` / `line_spans` | One extraction address or an ordered list of disjoint addresses: `first`, `count`. Mutually exclusive |
| `research_path` | Authored source-research record |
| `specification_sha256` | Fingerprint of typed fields, excluding identity/status/interface metadata |

`partial` requires a conversion gap. `specified` requires no conversion gaps,
at least one required engine contract, its declared interface, research path
and specification fingerprint. `informational` permits no rule gaps, actions or
required implementations. It is reserved for headings, vocabulary and advice
without mechanical behavior. The engine-gap set equals
`splat_requires_mechanics`; each declared implementation must occur in it.

Legacy `splat_clauses` records `{key, text, sha256, status}` remain readable and
validated when present. Generated notes omit this duplicated source text. The
source excerpt appears once in the body; the inventory binds its file hash to
the pinned source. Typed fields carry behavior, never source prose.

## Typed rule interfaces

`splat_rule: {mechanic, implementation, parameters}` uses the existing rule
contract. Deep source-specific operations are valid: the registered adapter
owns the source's ordering and state transitions. Its parameters must encode
the behavior, including exceptions and choices. A script, source-text wrapper
or arbitrary foreign-system formula is not an adapter.

`splat_contracts` is attached from the generated registry during catalogue
loading. Its interfaces have exactly these fields:

- `id`: registered implementation name.
- `inputs`: names mapped to `{type, required}`. Types include participants,
  participant lists, item instances, objects, locations, integers, numbers,
  booleans, choices, effects, resolutions, events, campaigns, durations,
  definitions and text. The reader enumerates the accepted type names.
- `outputs`: named result categories.
- `events`: relevant rule event boundaries.
- `capabilities`: required engine capability families.

Inputs bind the source-specific operation. Expression bindings must be declared.
Parameters use typed maps, lists, numbers, booleans, enum symbols, quantities,
dice records, choices and entry references. Per-contract records can differ:
Hydra heads and item spell storage are not forced into one generic effect map.
Geometry, source-authorized GM decisions and narrow unresolved source cases use
explicit inputs with stated scope and constraints, never silent defaults.

Registry format 2 retains each interface, a specification fingerprint and a
closed structural schema. The specification itself lives in the note body. Object fields are exact, arrays declare their permitted
record shapes, and primitive types are checked. An independent fingerprint
binds the built-in definition to its reviewed specification. The registry
always records `implemented: false`; content cannot register executable code.
A future engine adapter must validate the source-specific semantic constraints
as well as these structural types.

## Descriptive fields retained with mechanics

Spell headers retain level, school, classes, components, material description,
consumption/value, casting time, ritual status, range and duration. Creature
headers retain supplied scores/save totals, AC, fixed HP, movement and original
labelled stat text. The source-specific contracts provide the remaining typed
traits, actions, senses, languages, defenses, resources and lifecycle behavior.
A supplied SAVE value replaces its usual calculation before additional modifiers.

Items retain category, rarity, price, weapon/armor table data and attunement
metadata, with full usage/variant rules in their contracts. Missing mass is
absent, not zero. Class progression records retain all 20 rows, column meanings,
feature references, subclass slots and resource values. `pact_slots` and
`pact_slot_level` remain separate from ordinary `spell_slots_1` through
`spell_slots_9`. A source-specific adapter consumes these exact fields.

Original source citations, UUIDs, namespaced keys and package ownership continue
to follow the existing schema. Read-only verification never writes campaign
state or claims that a specified rule already runs.
