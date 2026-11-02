# Markdown storage

> Character storage update: the [implemented character cache](character-cache.md)
> supersedes earlier memory-only restrictions for D&D Beyond character records.
> Encounter run state and encounter integration remain separate future work.

> Historical decision record. The remaining schema details are now specified in
> [Mechanics](mechanics-schema.md), [State](state-schema.md), and
> [Campaign/storage](campaign-schema.md). Those contracts supersede the older
> illustrative fields and open-detail lists here. No implementation is authorized.

Status: Markdown/YAML storage, readable filenames, stable UUIDs, navigation
links, seed records, package registration, campaign manifests, namespaced
frontmatter, edit preservation, and duplicate identity handling are agreed.
Metadata-based discovery, ownership changes on external moves, editable drafts,
and protection of unsupported schemas are also agreed. Flat prefixed properties
and configurable placement using uniform conventions are agreed. Organization
preserves the user's Johnny.Decimal structure. Manifest recognition, campaign
identity authority, package selections, and rule-choice fields are agreed.
Optional namespaced resource keys, owner namespaces, and UUID or key references
are agreed. Namespace uniqueness, key grammar, copy and move behavior, and
explicit key renaming are agreed. Detailed entry schemas remain under discussion.
This is a design document; no data-model implementation is authorized.

The [schema draft](schema-draft.md) now supplies candidate per-kind fields and
complete [example notes](examples/README.md). They are documentation artifacts,
not registered content or migrations of existing campaign data.

This document builds on [Packages and campaign content](packages.md) and
[Creatures and encounters](combat.md).

## Agreed storage boundaries

Content uses Obsidian-flavoured Markdown with YAML frontmatter. YAML holds
authoritative stats and executable mechanics. Inventory is an explicit
exception: marked Markdown tables hold its authoritative list, including item
references, quantities, and equipment state. The Markdown body also holds
descriptive prose, notes, examples, and inline controls. Nested YAML is
acceptable, including the need to use Source mode for structures that Obsidian's
Properties editor cannot edit.

Item definitions retain their mechanics in YAML. Do not duplicate the inventory
table as a second YAML list. Simulations copy the starting inventory into
participant state and do not update its source table. The table marker,
columns, and supplementary state fields are agreed in
[the creature example](creature-example.md).

Each inventory note has at most one unnamed root table. Other tables identify
their container with `<!-- splat:inventory container=<row-key> -->`, where the
key identifies a container row in that note. No Container column is used.
See [Containers](containers.md) for nesting and standalone setup notes.

A package is a folder with a Markdown manifest. Entries are separate notes in
that folder or its subfolders. Campaign-owned content lives within its campaign's
configured folder structure. The earlier suggestion of mandatory separate
`Content/` and `Seeds/` directories is superseded by configurable organization
that keeps related content together and preserves Johnny.Decimal folders.

The initial version saves authored content and encounter setups, including
rosters, phases, strategies, and targeting assumptions. Run state, simulation
results, and event logs remain in memory. D&D Beyond data remains live-only;
its existing campaign mapping and cached name are governed by the existing
settings behavior.

## Package locations and campaign configuration

Packages can be registered from any chosen folder, including folders already
in the user's Obsidian vault. App settings store those locations. Campaigns
refer to packages by UUID rather than by a machine-specific path.

Campaign content settings travel with the campaign folder. A campaign manifest
note holds package selections and rule choices. App settings continue to hold
the campaign registry, folder paths, and session preferences. This agreement
does not relocate credentials or change the existing D&D Beyond campaign
mapping and name cache.

Scan registered content folders for notes marked with Splat metadata.
The entry kind identifies the type, so subfolders can be organized freely.
Ordinary Markdown notes remain outside Splat's content model.

One manifest directly in each registered owner root identifies that owner.
Its filename is freely chosen; its kind identifies whether it is a package or
campaign manifest. No particular on-disk location has been selected or created.

## Package art

A package can hold a note named `Art Index.md` directly in its root. The note has one JSON
block. It is a note and not a `.json` file because a vault sync can leave out other file types.

```json
{
  "splat_art": 1,
  "entries": { "dnd2024:creatures/goblin_warrior": "Creatures/goblin-warrior.webp" },
  "links": { "https://media.dndbeyond.com/map.png": "Figures/map.webp" }
}
```

`entries` gives an image for an entry key. The Reader shows it above the entry
wherever the entry appears. `links` gives a local image for a link in a note. The
note shows the image in place of the link. Paths are relative to the package root and
cannot leave it. Images are WebP, PNG, or JPEG.

Keys can name entries in any package, so one art package can serve the built-in SRD
and every book without a change to their notes. Art does not join a package's
content: a campaign's selections and dependencies do not affect it. If two packages
offer art for one target, the package registered first wins. An image file that is not
present yet is ignored, until the next start, so a sync in progress causes no errors.

## Frontmatter ownership

All top-level properties managed by Splat use the `splat_` prefix, keeping them
separate from ordinary Obsidian properties such as `tags` and `aliases`.
The common fields are `splat_schema`, `splat_id`, `splat_kind`, and `splat_name`.
Complex data uses nested YAML only where it needs structure, under prefixed
properties such as `splat_actions`.

This supersedes the earlier single `splat:` wrapper. Do not maintain two
authoritative copies of the same field. The prefix rule applies to top-level
properties; nested reference objects retain the agreed `id` and `link` fields.

## Editing notes

Preserve prose, unrelated properties, YAML comments, and existing formatting
wherever possible when editing a note. If the note changes externally while
Splat has pending edits, show a conflict instead of overwriting those changes.

Incomplete or invalid notes remain editable as drafts. Show specific errors
and block simulations that require the invalid content. Other valid content
remains usable. Malformed YAML remains available for correction as raw text.

For a newer schema version Splat cannot read, allow prose and raw-source
viewing, disable structured editing and simulation for that note, and preserve
its contents without attempting an automatic downgrade.

Details for edits that cannot safely preserve existing content remain open.
No note-editing implementation is authorized by these design decisions.

## Filenames and identity

Notes use readable filenames such as `Goblin.md`. A stable UUID in frontmatter
establishes identity. Renaming or moving a note preserves that identity.
Entries with the same name can use different folders or distinguishing
filenames. Filenames are not the authority for cross-entry references.

Ownership still follows the package and campaign boundaries described in the
package design. Moving between owners is distinct from reorganizing a note
within an owner's folder.

Moving a note between owners in Obsidian changes its owner to the destination
package or campaign while preserving the UUID. Validate dependencies at the
new location. If a shared entry still depends on campaign-private content,
flag it for repair; do not move its dependencies automatically. Such a move
can leave an invalid draft, unlike the validated Share to package operation
inside Splat.

If a note is copied outside Splat and duplicates an existing UUID, report the
identity conflict and offer to give the copy a new UUID. The user identifies
which note keeps the original identity. Splat does not silently choose one
based on filename or loading order.

## Entry references

An entry reference uses either `id` for a UUID or `key` for a resource key,
with an optional `link` for Obsidian navigation. The UUID remains the entry's
stable identity. Splat resolves a resource key to the corresponding entry.
Navigation links can be repaired when a note moves or changes name and remain
separate from reference identity.

### Resource keys

The resource-key form is `namespace:local_key`, where the local key
can use slash-separated segments for logical organization. For example,
`my_homebrew:rules/grouped_initiative` identifies a rule regardless of its
filename or Johnny.Decimal folder. No part of it is resolved as a disk path.

Keep stable entry UUIDs and allow an optional `splat_key` for human-authored
references. An index resolves the key to the corresponding entry. Existing
`id` references remain available. The following illustrates the agreed fields:

```yaml
splat_schema: 1
splat_id: a9c6c5c5-b0ef-4b8d-9d56-4432ab140c8d
splat_kind: rule
splat_name: Grouped Initiative
splat_key: "my_homebrew:rules/grouped_initiative"
```

```yaml
splat_rule_choices:
  "dnd2024:initiative":
    key: "my_homebrew:rules/grouped_initiative"
```

The mapping's left side names the mechanic being configured; its right side
selects a concrete rule entry. A resource key is also separate from a stacking
key: distinct entries may deliberately share stacking behavior.

Each package or campaign has its own namespace. A resource key must identify
one entry in its resolution scope. Duplicate keys produce a clear conflict;
load order does not choose a winner. Package and campaign ownership boundaries
still apply to references.

Each reference has one authoritative target selector, `id` or `key`, with
optional `link` for navigation. Reorganizing files within an owner does not
change their resource keys.

### Key lifecycle

- Require unique namespaces among registered packages and campaigns. Declare
  an owner's namespace with `splat_namespace` in its manifest. Namespaces use
  lowercase letters, digits, underscores, and hyphens. Local key segments use
  those characters plus slash separators; segments are nonempty and the key
  is case-sensitive. Display names and filenames remain unrestricted by this
  identifier grammar.
- A copied entry receives a key in the destination owner's namespace, retaining
  its local portion when available and choosing another local portion when
  needed to avoid collisions. Its UUID changes as already agreed. Copying
  within one owner also requires a distinct key. The copy's stacking key is
  preserved unless explicitly changed under the agreed stacking policy.
- Moving an entry between owners keeps its UUID but changes its key to the
  destination namespace, with collision handling as for copies. Splat-managed
  moves update known key references. External moves retain the agreed new
  ownership by destination, while stale namespaces or references are reported
  for repair. They do not trigger silent key rewrites merely on scanning.
- Treat changing a resource key as an explicit rename operation, separate from
  changing the display name or filename. Splat updates known references when
  it performs the rename. References outside available registered content
  may still need repair. Defer old-key aliases initially; UUID references
  provide stable identity through key changes.

These rules govern keys while retaining the settled UUID behavior of copying
and moving. Missing references use the existing validation rules.

## Seed records

A seed record contains the seed, encounter or sequence reference, engine
version, and fingerprints of the resolved simulation inputs. Splat uses the
fingerprints to detect changed inputs before rerunning.

Fingerprints do not contain or restore the original data. A later run is exactly
reproducible only with matching inputs and compatible engine behavior. A seed
record contains no saved run state, results, or event log. Its storage location,
exact fields, and fingerprint specification remain open.

## Agreed file examples

These examples illustrate the agreed placement policy, common header, and
reference format. No content files have been created.

### Configurable organization

Keep related content together, prefer uniform directory conventions where
possible, and make placement configurable so the existing Johnny.Decimal system
can be retained. Use the same placement conventions for packages and campaigns.
Register each owner's existing root, allow related notes of different kinds to
share a folder, and allow configurable default destinations within that root.
New notes can be placed in a selected existing folder. Changing a default does
not relocate existing notes.

For example, an existing Johnny.Decimal encounter folder could contain:

```text
41.03 Bridge Ambush/
  Bridge Ambush.md
  Bridge Guard.md
  Bridge Ambush - Trial 42.md
```

The number is illustrative, not a proposed change to the user's actual system.
The manifest belongs to the owner root; its filename can follow the user's
naming scheme. Entries are identified by metadata rather than folder names.
Each note has one owner, with another registered owner's subtree scanned
separately. Sharing private content still uses the package ownership rules.

Default destinations are optional and configurable. Their exact fields remain
to be agreed. Manifest recognition is described below. This does not include a
facility for generating Johnny.Decimal numbers.

### Common frontmatter fields

The common schema version, UUID, kind, and display name are agreed. The name
controls display independently of the filename. The schema version identifies
the content format separately from the app-settings schema. These fields use
the agreed flat `splat_` prefix:

```markdown
---
tags: [homebrew]
splat_schema: 1
splat_id: 6b02d7d9-19a7-4a18-a5ee-8bb9f00a04bd
splat_kind: creature
splat_name: Guard
---

A guard who protects allies.
```

This illustrates the shared header only. It has no combat definition and is
not presented as a simulation-ready creature. Per-kind fields follow in later
examples.

### Reference fields

Use `id` for the required target identity and `link` for the optional
Obsidian navigation link. The following is a reference object, not a complete
standalone note:

```yaml
id: 6b02d7d9-19a7-4a18-a5ee-8bb9f00a04bd
link: "[[My Homebrew/Creatures/Guard]]"
```

Quote wikilinks in YAML. The UUID remains authoritative. The exact navigable
path depends on the vault containing the notes; navigation is optional when
the package is outside that vault.

## Manifest details

The following details are agreed as design. They do not change the implemented
app settings or create content files. The UUID reference examples remain valid
alongside the resource-key reference form.

### Manifest recognition and campaign identity

Use one owner manifest directly in each registered owner root, identified
by `splat_kind: package` or `splat_kind: campaign`. Its filename remains freely
chosen. A nested registered owner's contents are handled by that owner.

The current implementation stores campaign UUIDs and names in app settings
(`src/persistence.rs`). Preserve each existing UUID when its manifest is created,
then make the manifest's UUID and display name authoritative.
The app registry keeps the UUID-to-folder association and a cached display name
for unavailable folders. A registry/manifest identity mismatch is an error, not
a reason to silently assign a new UUID. Existing D&D Beyond link settings and
credential handling are outside this change.

### Package selection fields

Use `splat_packages` as a list of package selections. Each has a `package`
reference and a `selection` with `mode: all` or `mode: selected`. Only the latter
contains an explicit `entries` list. Required dependencies are computed
separately, preserving the user's direct selections.

```yaml
splat_packages:
  - package:
      id: 62f54b31-1ba8-4e0f-a5f9-2df25ae4b997
    selection:
      mode: all
  - package:
      id: 7b31c6e4-47af-41c6-8b8d-42f1bac31229
    selection:
      mode: selected
      entries:
        - id: 6b02d7d9-19a7-4a18-a5ee-8bb9f00a04bd
```

These illustrative UUIDs identify a core package, a homebrew package, and its
Guard entry respectively. They do not refer to existing registered content.
`all` automatically includes later additions; `selected` records only the
explicit entries. This encodes previously agreed selection behavior.

### Rule choice fields

Use `splat_rule_choices` as a mapping from a stable mechanic key to a rule
entry reference, for example:

```yaml
splat_rule_choices:
  "dnd2024:initiative":
    id: a9c6c5c5-b0ef-4b8d-9d56-4432ab140c8d
```

Here the illustrative rule entry supplies the chosen initiative method. The
mechanic key identifies what is being chosen; the UUID identifies the exact
rule definition. Display-name changes do not change either identity. Precise
mechanic-key conventions and rule-entry schemas remain to be specified.

## Design tree

- Markdown/YAML content storage and package manifest notes: agreed.
- Readable filenames and stable UUIDs in frontmatter: agreed.
- UUID entry references with optional Obsidian navigation links: agreed.
- Saved seed metadata with input fingerprints, without snapshots or results:
  agreed.
- Register packages from any folder, retaining paths in app settings and using
  package UUIDs in campaigns: agreed.
- Campaign manifest notes carry package selections and rule choices: agreed.
- Use flat `splat_` properties for all top-level Splat fields, with nested YAML
  where the data needs structure: agreed; supersedes the single-block wrapper.
- Preserve existing note content and formatting where possible; detect
  conflicting external edits before saving: agreed.
- Duplicate UUIDs are reported for explicit resolution rather than silently
  choosing a note: agreed.
- Discover marked notes by metadata within registered content folders, with
  freely organized subfolders: agreed.
- External moves change ownership by destination, preserve identity, and
  validate dependencies without moving them automatically: agreed.
- Invalid and incomplete notes remain editable drafts, with specific errors
  and affected simulations blocked: agreed.
- Newer unsupported schemas permit viewing, disable structured editing and
  simulation, and are preserved without automatic downgrade: agreed.
- Organization must be configurable, preserve Johnny.Decimal folders, keep
  related content together, and prefer uniform conventions: required.
- Apply the same placement conventions to packages and campaigns, with optional
  configurable defaults and no relocation when a default changes: agreed.
- Common schema version, UUID, kind, and authoritative display name: agreed.
- Reference objects use either `id` or `key`, plus optional `link`: agreed.
- Entry UUIDs remain stable identities; optional `splat_key` values provide
  readable references: agreed.
- Each package or campaign has a namespace; duplicate keys are reported as
  conflicts rather than resolved by load order: agreed.
- One root manifest recognized by its kind, with freely chosen filename:
  agreed.
- Preserve campaign UUIDs and use the manifest's identity and display name as
  authoritative, retaining registry paths and cached names: agreed.
- `splat_packages` stores package references and explicit `all` or `selected`
  modes, with direct entry selections separate from computed dependencies:
  agreed.
- `splat_rule_choices` maps stable mechanic keys to rule references: agreed.
- Namespace names are unique among registered packages and campaigns and are
  declared with `splat_namespace`; identifier grammar uses lowercase letters,
  digits, underscores, hyphens, and slash-separated local segments: agreed.
- Copies and moves adapt keys to the destination namespace with collision
  handling; UUID behavior and stacking identity remain as previously agreed.
- Resource-key changes are explicit renames that update known references;
  aliases for old keys are deferred: agreed.
- Next work:
  - Validate entry-schema needs against [the rule coverage cases](rules-coverage.md)
    and [the creature example](creature-example.md). Example choices are delegated.
- Subsequent detail:
  - Container inherited state, cross-note row-key scope, and encounter placement
    references in [Containers](containers.md).
  - Placement-default fields, additional manifest content, seed record fields,
    and fingerprint format.
  - Entry-kind schemas and concrete Markdown/YAML examples.
  - Link repair behavior.
  - Inline-control grammar and validation.
  - Link repair, absent folders, and ambiguous ownership boundaries.
- Deferred:
  - Aliases for renamed resource keys.

Open decisions are not implementation instructions.
