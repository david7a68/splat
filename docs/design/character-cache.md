# Imported character records

Status: implemented for the party viewer, campaign content, and encounter roster.
Campaign copies use the [shared character content schema](character-content.md).
Simulation execution remains separate work.

## Shared creature definition and fetched state

`character::EntitySnapshot` holds these parts:

- `definition`: note-model `splat_kind: creature`, stable `splat_id`, name,
  `splat_creature`, `splat_stats`, `splat_health`, and `splat_resources`.
  Creature, Stat, Health, and Pool reuse the engine's existing types.
- `initial`: current HP, temporary HP, death saves, conditions, inspiration,
  remaining resources, and spent hit dice. Current amounts do not replace pool
  maxima or template stats.
- `inventory`: stable item-instance and definition UUIDs, quantity, source
  container, equipment and attunement state, item definition and source details.
  Two identical items remain separate instances.
- `character`: level, species, classes, checks and proficiency/advantage flags,
  attacks, casting values, senses, defenses, features, and spells.
- `source`: provider IDs, separate retrieval times, provider modification time
  when supplied, a source fingerprint, and complete raw-v5 and SCDS records.
- `coverage`: what was supplied, what was normalized locally, and remaining gaps.

The creature UUID derives from the provider and character ID, not the name,
account, or campaign. Item-instance UUIDs include their character and source
instance IDs. Renaming does not change identity.

`EntitySnapshot::entry()` emits the normal Markdown/YAML `Entry` shape and runs
it through the existing creature reader. `splat_initial` carries starting state;
`splat_external` identifies the provider and read-only source. The content reader
recognizes this provenance field. The execution compiler continues to reject
unsupported imported state and provenance rather than treating the record as an
executable character. The manual encounter roster already copies starting state into a run; simulation
requires separate mechanics coverage.

## Data parity

The adapter reads two first-party services:

1. Character v5 supplies descriptions, choices, inventory, slot usage, class
   progression, feature uses, custom values, and other detailed source fields.
2. SCDS supplies calculated abilities and saves, AC, HP, skills and flags,
   initiative, passive senses, speeds, attacks, defenses, and casting values.

SCDS values are stored as supplied stats with `dndbeyond:supplied_total` in
`includes`. The viewer does not recalculate or add modifiers to those totals.
It retains all source fields, including fields not shown in a dedicated panel.

Single-class slot pools use v5 class progression and usage. Feature uses follow
v5 limited-use fields: fixed maxima, ability/proficiency additions or multipliers,
and number used. Source recovery
rules are preserved, but rest execution is not implemented. Multiclass resource
normalization and arbitrary formula-based feature uses remain explicitly marked
as gaps. Computed attacks and spellcasting totals still work for such characters.

The services return independent responses and provide no shared revision token.
A successful combined read is not proof that both services reflect the same edit.
The record keeps both retrieval times; follow-up reads after events allow delayed
computed results to catch up. Missing or invalid required data fails the refresh
and preserves the previous local copy. No guessed stat replaces a missing total.

## Local storage

Application-owned records live at:

```text
$XDG_DATA_HOME/splat/characters/<local-campaign-uuid>/<remote-campaign-id>/<character-id>.json
```

On Linux, the default root is `~/.local/share/splat/characters`. Each JSON file is
a versioned snapshot envelope containing the shared creature definition and its
current state, details, and source records. The envelope's `version` is 2. Splat does
not read a record with a lower version, and does not check the rest of it: it leaves the
file alone, shows no sheet for it, and the next sync writes a current record over it. A record with a higher version, or one
that Splat cannot parse, is kept and never replaced. The app also writes a campaign Markdown copy as described below. Neither form
introduces a second authoritative inventory table.
A `roster-index` records the most recently fetched membership; removed members
remain on disk but no longer appear when that roster is loaded offline.

The portrait a player chose on D&D Beyond is kept next to the record, as a square
PNG named `<character-id>-<hash of its address>.png`. A sheet update downloads it once,
without the session, and only from D&D Beyond's own hosts. A new portrait replaces the old
one. If there is no portrait, Party shows the first letter of the character's name. A
failed download shows a warning, and the sheet still updates.

Writes use a private temporary file, flush, and atomic replacement. Invalid or
unsupported existing records are retained and reported. Credentials remain in
the keyring and memory, never in these records. Cached characters can be viewed
offline. The UI distinguishes cached copies from records checked in this session.
An account/campaign change discards active requests and subscriptions; it does not
delete the local campaign's saved records.

## Live updates

While D&D Beyond is connected and the active campaign is linked, Splat reads
the party on every page. A character update event triggers a read, then a
second read to catch delayed computed values. A read about once a minute
covers missed events. When the stream is down and Party is open, Splat
polls every 15 seconds. Provider errors can delay these reads.

The Rust tests use synthetic data for cache round trips and event routing.
The ignored `live_parity_cache_and_stream` test checks normalization and cache
round trips using live HTTP reads and a simulated stream notification. It uses
the desktop keyring and the active campaign and writes the local character cache.

```sh
devenv shell -- cargo test --locked -p splat-party --lib live_parity_cache_and_stream -- --ignored --nocapture
```

[Party transport](../../src/party_sync.rs) · [Combat sync transport](../../src/combat_sync.rs)

## User-side campaign copies

Cache loads and successful sheet reads publish `Characters/D&D Beyond/<creature UUID>.md`
in the configured campaign folder. Filenames stay stable across character renames.
The note's model uses the existing creature definition, `splat_initial`, and
`splat_external`. The body includes readable attacks, features, spells and authoritative inventory
tables. Separate item and spell notes supply referenced definitions. Typed
`splat_character` and `splat_import` fields retain sheet details and conversion
gaps. Full provider responses remain only in the private JSON cache. See the
[character content contract](character-content.md) for schema extensions and
publication behavior.

Each file has a checksum for generated content and a Personal notes section.
Refreshes preserve everything after the checksum. A file that fails its checksum, or an
unmanaged file, blocks replacement and produces a visible warning. A copy that Splat cannot
read follows the same checksum, and its properties say which note it is. When they name the note
that the refresh writes there and the generated text is unchanged, the refresh writes it again.
Any other file that Splat cannot read counts as edited, whatever its checksum. An unchanged former
copy that Splat cannot read, without personal notes, goes to its archive. One case stops **Force read** too: a file
that Splat cannot read and that mentions the character, when its properties do not name the
character. Move it aside or repair it. A file that only
changed during a save is not a conflict: the save reads it again. This covers two Splat
writers at the same time. To resume updates, move the edited file aside or use
**Force read** on the warning. Force read fetches the character again, keeps each edited
file's exact bytes in an archive, and then replaces it. Personal notes carry over when
the checksum line is still readable. Files without edits do not change their personal
notes, and nothing is written until every check passes. Writes use a temporary file
and atomic replacement, with a check for edits before replacement. External editors can still race the
final rename. Campaign/account cancellation uses the same gate as cache writes.
Removed party members' files remain as historical copies, with their fetch time;
only the current roster appears in the viewer. These files update when records
are loaded or fetched, not while the app is closed.

To publish an existing cache without network access:
`cargo run --example export_campaign_characters -- <local-campaign-uuid> <ddb-campaign-id> <absolute-campaign-folder>`.
