# Imported character content

Imported characters are ordinary schema-1 creature entries. Matching SRD items,
spells, and features use the shared catalogue's canonical UUID references. Other
definitions follow their [import route](#routes-and-the-campaign-package). The catalogue,
content editor, dependency traversal, and encounter roster use the same reader
as authored content. Structural validity does not grant simulation support.

The private `EntitySnapshot` JSON remains a lossless provider cache, not a second
simulation format. Both fresh responses and old version-1 caches go through
`content_notes()` when published. Campaign Markdown never embeds that snapshot.
The provider cache retains choices, custom fields, and source data for future
conversion; incomplete semantics are reported rather than inferred from prose.
The [rule conversion registry](character-rule-conversion.md) now supplies typed
behavior for the source definitions in the cached party.

## Character fields

Existing fields retain their contracts: `splat_stats` contains supplied totals,
`splat_health` selects character zero-HP policy, `splat_resources` contains limits
and known rest recovery, and `splat_initial` holds fetched current amounts.
`splat_proficiencies` contains training and skill proficiency; languages use
`splat_languages`. Multiclass hit dice use `splat_hit_dice.classes`, mapping a
local class key to `{count, die}`. Starting `hit_dice_spent` uses the same keys.
The original single-pool `{count, die}` shape remains valid for authored creatures.

The typed `splat_character` extension contains:

- `level`, `species`, and `classes` (`name`, `level`, optional `subclass`).
- `checks`: names, existing stat keys, proficiency/expertise/half-proficiency,
  and supplied advantage/disadvantage flags.
- `senses`: names and supplied distance text; `defenses`: resistance, immunity,
  and vulnerability names.
- `casting`: supplied ability modifiers, save DCs and attack bonuses, with the
  source class/feature names associated with each total.
- `attacks`: stable local keys for supplied attack profiles, including attack
  bonus, dice/fixed damage, damage type, range and unresolved-save indication.
  Duplicate source rows remain separate. Profile data is not an ability use.
- `features`: named descriptions, known activation, optional resource-pool key,
  and source identity. Descriptions do not implement rules.
- `spells`: normal spell entry references, preparation flags, slot requirement,
  casting ability, supplied override DC, cast level, and restriction text.
- `campaign_options`: inactive, unselected, active, or unknown Dark Bargain,
  Character Thread, and Runestone selections. Active choices have source IDs
  and references to the converted benefits and costs.
- `background`: the background's name, `custom` when the player wrote it, its
  source key, printed description, `feats` it grants, and `feature`, a key in
  `features` for a background feature that is not a feat (2014 backgrounds). The
  background's proficiencies, choices, feats and spells come in with the rest of
  the sheet, so they are not repeated here.
- `details`: player, alignment, faith, lifestyle, gender, age, height, weight,
  eyes, hair, skin, and experience points when the sheet has some.
- `characteristics`: personality traits, ideals, bonds, flaws and appearance.
- `story`: the sheet's notes: backstory, allies, enemies, organizations,
  personal possessions, other holdings and other notes.
- `coins`: cp, sp, ep, gp and pp.
- `companions`: the sheet's Extras, such as a familiar, mount or Wild Shape form.
  Each has a name, creature, group, active flag, description, the stat block's
  printed AC and HP, and the damage and temporary HP the sheet records. The stat
  block is not converted, so a companion is not an encounter participant.
- `custom_items`: items the player wrote on the sheet, with quantity, weight,
  cost, description and notes. They have no item definition, so they have no
  inventory placement.

Custom actions become `features` entries (`custom_action_ID`) with their gaps.
Empty fields are left out. These parts come from the private cache's character
v5 record, so an old cache gains them on its next publication with no new read.
Alignment, lifestyle and creature group names use the IDs from D&D Beyond's
`api/config/json`; an ID that Splat cannot name is a gap, and the source value
stays in the cache. Sheet text in the Markdown body is escaped: each line's
first ASCII punctuation mark gets a backslash, leading spaces go, and `<!--`
becomes `&lt;!--`. So no sheet text can start a heading, code fence, HTML block,
quote, list or table, or hide or end the generated part. One-line values such
as names and gap reasons also lose their line breaks. The frontmatter keeps the
text exactly as written.

A weapon attack with sufficient data also produces `splat_actions` with normal
activation, targets, rolls and attack steps. Its source-matched rule bindings specify
equipment and feature interactions; unresolved encounter equipment choices are
required inputs. Spell attack totals never create free
weapon actions: casting costs and effects come from their separate spell rules.

`content::character::Sheet` is the strict serialized contract. The entry reader
checks its shape, local keys, stat/resource references and positive levels.
Normal reference validation resolves all spell and inventory dependencies.

## Inventory and dependencies

One marked Markdown inventory table is authoritative for starting placements.
Contained items use `<!-- splat:inventory container=item_SOURCE_ID -->` tables.
Each row retains quantity and attunement. `splat_inventory_details` maps row keys
to stable `instance_id`, provider `source_id`, `source_equipped`, and nullable
`charges_used`. An unknown charge maximum does not become an invented pool.
Beyond's equipped flag does not establish hand allocation; ambiguous items are
carried with an explicit required input. Known equipped body armor is worn.

Item entries contain category, known mass, equipment facts, described properties,
known armor/weapon values and containment facts. The import extends `splat_item`
with descriptive `properties` (`name`, `description`), `armor` (`value`,
`contribution: base|bonus`, optional strength requirement), `weapon` (dice,
modifier/fixed damage, type and ranges), and containment `capacity_weight_lb`
and `description`. These are supplied facts, not granted effects or rebuild rules.

Spell entries contain level, school, components/material text, concentration,
ritual, duration, casting-time amount/unit, range and readable description. A
referenced spell does not imply that its action is implemented.

Characters share one definition when their source definitions generate the same note.
A different version of a provider item is a different definition, so independently
refreshed characters cannot conflict. Repeated instances of the same definition within
a sheet share a definition but retain separate inventory row/instance identities.
Matching SRD definitions are not routed. Item rows keep their separate instance
identities, quantities, equipment, and attunement while sharing one definition; spell
bindings keep character-specific casting details.

## Routes and the campaign package

`Campaign Package/Package.md` is a package manifest in the campaign's folder. Its
`splat_import_routes` records where each definition lives. Definitions with no package note
live under `Campaign Package/{Items,Spells,Rules}/`. The catalogue lists this package, and its
entries are private to the campaign.

A package note says which D&D Beyond definitions it holds:

```yaml
splat_dndbeyond_ids:
- feature:8429477
- action:5119104
```

The IDs are `feature:ID`, `action:ID`, `item:KEY`, and `spell:ID`, the same identities as the
[conversion registry](character-rule-conversion.md). A species note can list every trait it
holds. An ID says where a definition's text lives. It does not prove what the rule does:
rule data stays in the registry, pinned to the source text. So a note with many traits takes
only the IDs. A note that holds one item or spell also takes that definition's rule data and
missing facts in its frontmatter. Its existing values and words stay, and each save adds them
again if a regenerated package lost them.

A definition with no route takes one when a sheet is saved:

1. One selected note in an enabled package lists its ID: the route goes to that note.
2. Otherwise, or if it already has a note in the former `Definitions/<character UUID>/` folder,
   the route goes to the campaign package. The former folder's notes retire as other former
   copies do. Characters share a campaign package note when their definitions generate the same
   note.

A route never changes during a save. **Package reconciliation** (`campaign_package::check`)
compares the routes with the enabled packages when a campaign opens, when selections are
saved, when Campaign > Packages reloads, and every five minutes. The sidebar shows how many
changes wait in Campaign > Packages. For a definition that no note lists, the check suggests a
note by its words: the source text of an item or spell must end with them, and a rule's words
must be in the character's species note, or in exactly one other note. Words never move
anything by themselves. `campaign_package::apply` carries out the ticked changes:

- **To a package**: the note gets the ID, the route changes, and each sheet saves again. A
  campaign package note goes to an archive once a move leaves no sheet using it.
- **Back to the campaign package**: the route changes, and each sheet saves again. The package
  note keeps its ID and data, because other campaigns can use them. Another package that lists
  the ID stays a choice, so the check offers it.
- **Remove**: no entry refers to the definition. The route goes, and any note goes to an archive.

Nothing leaves until every sheet has saved, so a failed apply can run again. A note that Splat
cannot read blocks removals, because it may use anything. A package whose manifest is not the one
registered takes no routes or writes, and a route's note takes no writes while the campaign does
not select it. A partial selection counts the entries that its chosen entries require. One save
at a time reads and replaces the manifest, within the app.

To map a definition that the check cannot suggest, add its ID to the right note by hand. An ID on
two notes is blocked until one note has it.

A move is blocked, with its reason shown, when the campaign package's note has edits or
personal notes, when a note that Splat does not generate refers to it, or when a sheet that
uses it has no saved D&D Beyond data.

## Coverage and publication

`splat_import` version 2 records incomplete conversion, unavailable engine adapters,
and required inputs separately, with a fingerprint of the converted mechanics.
Version 1 (`{version: 1, gaps: [{path, reason}]}`) remains readable. Gaps record incomplete conversion
at specific fields. It is valid on creatures, spells and items, and is separate
from the SRD catalogue's source-specific `splat_conversion` contract.
Unknown import versions or malformed gap records fail parsing. Import metadata
cannot enable execution; the current proof compiler rejects it.

All generated entries are parsed and their references validated against local
definitions plus the shared catalogue before publication. The app already loads
the SRD catalogue and pulls dependencies of campaign creatures into its selection
automatically; users do not need to select the entire SRD package. Offline checks
use `verify-content --with-builtin DIRECTORY`. Every existing file's generated checksum is checked before any
replacement. Dependencies publish before the creature, with atomic replacement
per file. Personal notes survive refresh, and edited generated content blocks the
whole character's publication. A disk failure or an editor racing the writes can
still leave some updated dependencies; this is not a multi-file transaction.
The importer retires the former per-character definitions, SRD copies, and matched
item/spell wrappers by saving their exact
bytes as `.md.splat-archive` files, which the content reader does not index.
It keeps copies with personal annotations or references from other current
campaign entries at their original paths, including dependencies of retained
copies. Edited generated content or conflicting archives block publication.
All retirement checks run before writes; archives are created only after the
replacement creature has been saved. This migration never edits shared SRD files.
**Force read** uses the same checks and order. It archives each edited note before it
replaces that note. It archives an edited former copy, unless another entry needs that
copy or the copy has personal notes. Such a copy stays, and its edits become its
generated content. A conflicting archive never changes: the new bytes take the next
unused name, `.md.splat-archive.1` and so on.

Loading an old private cache automatically creates the new content on its next
publication. The offline export example also performs this migration without
fetching source data. Existing creature UUIDs and note paths stay unchanged.

## Sheet presentation

The generated page puts spells and equipment before the longer feature reference.
Descriptions retain paragraph and list breaks. Shared spell links open the official
SRD PDF at the source page; other spell links point to the note that holds the
definition. The structured spell bindings still use canonical catalogue UUIDs.

The Splat content reader displays named imported inventory tables without the
storage keys and UUID columns. Those columns remain in Markdown source because
they carry the inventory schema. Tables without item names retain all columns.
The Party viewer groups spells by level, keeps casting details visible under each
name, and opens full descriptions on demand. Source and conversion details remain
on the Source & parity tab.
