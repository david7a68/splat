# Campaign, encounter and storage schema

> Character storage update: the [implemented character cache](character-cache.md)
> supersedes earlier memory-only restrictions for D&D Beyond character records.
> Encounter run state and encounter integration remain separate future work.

Status: completed candidate schema 1 field contract for the agreed scope.
Documentation only; existing settings, credentials and campaign folders are
unchanged. This completes [Mechanics](mechanics-schema.md) and
[State](state-schema.md).

## Owners, selections and placement

Package/campaign manifests keep their common UUID, namespace, name and optional
namespaced key. `splat_requires` contains explicit entry dependencies;
`splat_requires_mechanics` contains registered contracts. Required references
in mechanics are also traversed. Optional dependencies use
`splat_optional: [EntryRef,...]` and never enable themselves. Computed dependency
closure is runtime/index state, not a second authored selection list.

`splat_packages` retains the settled `{package: EntryRef, selection: {mode: all}}`
or `{mode: selected, entries: [EntryRef,...]}` format. Empty selected lists enable
nothing except separately required dependencies. `all` includes future local
additions. A rule group is `splat_kind: rule_group` with
`splat_rule_group: {entries: [EntryRef,...]}`; selecting it selects the whole
indivisible group. Rule-group dependency cycles are errors. No exclusion list
or update/distribution mechanism is added.

`splat_rule_choices` remains campaign-only `{mechanic_key: EntryRef}`. Compatible
rule additions declare `splat_rule: {mechanic, mode: augment, implementation,
parameters}`; mutually exclusive alternatives use `mode: replace`. A rule's
registered contract validates parameter types and compatibility. Multiple
replacements require an explicit choice; no filesystem/load order winner.
The default D&D 2024 core selection is injected during campaign creation under
the agreed design, not silently guessed from an encounter note.

Optional `splat_placement` in either owner manifest is:

```yaml
splat_placement:
  default: "."
  by_kind:
    creature: "41 Encounters"
    encounter: "41 Encounters"
    seed: "41 Encounters"
```

These are user-chosen relative folders inside that registered owner, not logical
resource keys. No mandatory folder names or Johnny.Decimal numbering are added.
Omitted defaults use the chosen current folder, then the owner root. A new-note
operation validates that its selected destination belongs to this owner; it
does not cross into a nested registered owner. Changing defaults moves nothing.
Missing defaults are reported on creation with another destination choice;
ordinary scanning does not create folders or rewrite configuration.

The app registry retains absolute owner roots, UUIDs and cached display names.
Manifest/registry mismatch is a repairable error, not an ID regeneration.
If a root disappears, its owner remains registered and unavailable; do not remove
its selections or dependencies. Duplicate root registrations, identical manifest
UUIDs at different roots, ambiguous symlink aliases and namespace collisions
require repair rather than a first-found winner. Canonical filesystem identity
prevents traversing the same root twice; follow registered symlink roots but do
not recursively follow arbitrary directory symlinks out of the owner.

## Campaign party and external identity

`splat_party` is `{members: [PartyMember,...]}`. A member requires stable `id`
(UUID) and local `key`, plus exactly one source:

- `creature: EntryRef` for an authored local definition.
- `external: {provider: dndbeyond, character_id: string}` for live stats.

An optional `name_cache` is display metadata only. `bindings` maps template party
slot keys to member UUIDs. Roster `party_slot: key` resolves through those
campaign bindings; shared encounter templates never reference private member
IDs directly. Explicit per-run party selection can supply a binding without
rewriting a shared template. A whole-party slot uses `party_slot: all` and
expands in the campaign's ordered members list; reserved `all` cannot also name
an individual binding.

External IDs are mappings, not character-sheet snapshots. D&D Beyond campaign
link and campaign-name cache live in the campaign manifest as
`splat_dndbeyond_campaign_id` and `splat_dndbeyond_campaign_name` (nullable strings).
`splat_name` supplies the local display name. App configuration retains the folder
registration and cached metadata; startup and reload read the manifest. Existing
registrations gain these fields without replacing custom paths, selections, or notes.
Credentials remain in the existing session/keyring handling, never frontmatter,
logs, source metadata or fingerprints. No new writes to D&D Beyond are authorized.

A future live adapter returns in memory `{provider, external_id, retrieved_at,
normalized_definition, supported_mechanics, unresolved_mechanics, provenance}`.
The normalized definition uses the common creature contract and marks whether
each supplied stat has known contributions. Retain raw response only in memory
if necessary for diagnostics; do not write it to the vault. A local alias/member
UUID survives remote name changes. Failed/auth-expired fetches make the affected
new run unavailable; do not simulate a fabricated or silently stale sheet.
Running encounters retain their already resolved inputs. The character endpoint
and normalization are still integration work; their existence is not claimed
by these field definitions.

## Encounter definition

`splat_encounter` contains `roster`, `sides`, `assumptions`, `completion`, `limits`,
and optional `phases`, `containers`, `objects`, and `strategies`.

Roster is an ordered array of `{key, creature: EntryRef | party_slot: key,
side, controller, initial?, overrides?, initial_presence?, initiative_group?, health_policy?}`. Exactly one
source is present. `initial_presence` defaults active; pending phase arrivals
use absent. Repeating a definition means another slot/actor. A `party_slot: all`
expansion assigns stable compound placement addresses using member UUIDs, in
party order. `initial` only sets state allowed by State schema. To change a
creature everywhere, use a campaign copy. To change it in one place, use
`overrides`, which are never hidden.

### Overrides

A roster slot or an encounter group member can carry `overrides`, a mapping that is
small, closed and validated. Every field is optional, and a slot with none behaves
as if this section did not exist.

| Field | Form | The run |
| --- | --- | --- |
| `name` | One line of text | Replaces the display name. Duplicates are numbered from it |
| `max_hp` | `{set: n}` or `{add: n}` | Applies. The result must be from 1 to 1,000,000 |
| `armor_class` | `{set: n}` or `{add: n}` | Applies. The result must be from 0 to 1000 |
| `initiative_bonus` | `{set: n}` or `{add: n}` | Applies. The result must be from -1000 to 1000 |
| `attack_bonus` | `{add: n}` | Cannot apply: attacks are stat block text. Shown on the card and on a live row |
| `damage_bonus` | `{add: n}` | Cannot apply, for the same reason. Shown on the card and on a live row |
| `speed` | `{set: n}` or `{add: n}` | Cannot apply to text. Shown on the card. The result must be from 0 to 1000 |
| `side` | Text | Replaces the side. In a roster, it must be a side of that encounter. The editor sets a slot's side on its row, so this field is for a note, and the form shows it only when a note has given one |
| `conditions` | List of names | Added to the starting conditions. One already there is not added twice. A name repeated in this list is an error |
| `note` | Text | Shown on the card |

```yaml
overrides:
  name: Rival Captain
  max_hp: {add: 10}
  attack_bonus: {add: 1}
  damage_bonus: {add: 1}
```

An unknown field, a wrong form, `add: 0`, or a result out of range is an error that
names the slot, or the group's member by number. A stat with no base value accepts
`set` only. Overrides are applied once, when a participant is created. They are saved
with the run, beside the values they replaced, so a later edit of the encounter or
group file does not change that run. They are never written to the creature entry, a
package, or a campaign copy. The editor and the run's card show each base value and
changed value, marked as changed, and a row says `Changed`.

With a `max_hp` override, the participant starts at the new maximum, unless the slot's
`initial.hp` gives its HP. That value must not be above the new maximum. `initial` and
`overrides` can be on one slot. `initial.conditions` replaces the creature's starting
conditions, and `overrides.conditions` adds to the result. A roster slot's `side` is
what the card shows as the base when `overrides.side` is present. D&D Beyond sets a
linked character's maximum HP and conditions during sync, so those two overrides are
errors on a linked character. An override does not change XP, and a creature with a
mechanical override no longer counts as identical to its plain copies for initiative.
A group is a fixed list: there is no automatic scaling and there are no level variants.

### Encounter groups

An encounter group is `splat_kind: encounter_group`, a campaign-owned Markdown note.
Its body is the group's notes. Encounter groups are not sides, rosters, rule groups, or
initiative groups (`CONTEXT.md`). The app writes them under `Encounter Groups/`.

```yaml
splat_kind: encounter_group
splat_name: Rival Team
splat_encounter_group:
  side: rivals
  members:
  - creature: {key: "dnd2024:creatures/goblin_warrior"}
    count: 3
  - creature: {id: 1b671a64-40d5-491e-99b0-da01ff1f3341}
    overrides: {name: Rival Captain, max_hp: {add: 10}}
```

`side` is required. Because a group can arrive in any encounter and a run keeps no
hostility table, it can be any name, and a side the run has not seen uses the "other"
mark. `members` is an ordered, non-empty list. `count` defaults to 1, up to 100. A
reference follows the same dependency rules as a roster's, so it can reach a creature
in any enabled package. An empty group, an unknown field, or an invalid override stops
the note from parsing, with the reason. A reference that does not resolve is reported
as a content issue, and the group says why it cannot be added. A linked D&D Beyond
character cannot be a member, because one sheet cannot be two participants.

Adding a group to a run is one step of the run. It works during setup and during combat,
and the same group can be added again. Each addition creates new participants with fresh
keys, separate HP, and full HP on the group's side. They keep any temporary HP and
conditions from the creature's `splat_initial`. Names already in the fight do not change:
new duplicates continue from the highest number. A creature with no maximum HP needs a
`max_hp` override, and a run cannot grow past the 2,000 participants a save can hold. A
step that fails leaves the run and its dice as they were. Undo is linear and lasts one
session, so it removes a group only by stepping back through later steps. After that,
mark its members absent.

A typed total applies to every member under any rule. Otherwise the campaign's initiative
grouping decides:

| Grouping | Adding the group | A later NPC roll in setup |
| --- | --- | --- |
| `identical` | One shared roll, even for different creatures | The addition stays one roll. It joins neither its like nor another addition |
| `individual` | Each member rolls alone | Each rolls alone |
| `by_side` | Takes the initiative an NPC on its side has. A side with none rolls once | One roll for each side |
| `explicit` | One shared roll, as its own initiative group | Its own initiative group |

A shared roll uses the first member's bonus, as other groups do. In combat, the group
takes its place on that total. A tie goes after those already on it. The current turn stays
current. A place after the current turn acts this round, and a place before it waits for the
next round.

Sides map keys to `{hostile_to: [side_key,...]}`. Treat hostility as directed;
if both sides are hostile, both declare it. Same-side participants are allies
unless a source-specific relation says otherwise; different-side participants
not declared hostile are neutral. Side changes are explicit and do not infer
party membership. A strategy selects among legal relations, not name strings.

`containers` is an ordered list `{key, setup: EntryRef}` for standalone container
placements, each instantiated independently. `objects` uses `{key, item: EntryRef,
health?: InitialHealth, rule: mechanic}` for targetable world items/objects;
object AC/HP/damage thresholds use an explicit object-health contract, not a
fake creature with free turns. A carried object still has an item identity and
does not gain a second standalone placement by being referenced as a target.

Assumptions are an ordered set `{key, predicate, participants|objects, value,
scope?, visibility?}`. Pairwise reach/sight facts apply to explicitly named
participants; the existing `maximum_feet` shortcut asserts reach eligibility
for the named group at all requested distances up to that value, including
touch. It is not a measured distance. General precise form uses Predicate
parameters and `value: true|false|unknown`. Conflicting active facts are errors;
an explicit source/event replacement can retire an earlier fact.

Scripted spatial input is `{event, at: ClockSelector, facts: [Assumption,...]}`
under `splat_encounter.assumption_events`. This supplies movement/visibility/area
membership facts without geometry. An area target selector can specify
`assumption: key` and `count`; if no supported rule or fact supplies the members,
the affected simulation is blocked. Missing geometry does not disable all
non-spatial conditions. Unknown data and hidden data remain distinct.

## Strategies, initiative and phase events

StrategyRef is either a local `strategies` key or an EntryRef to
`splat_kind: strategy` with `splat_strategy`. StrategyDef has `implementation`
(registered policy), `parameters`, `knowledge: controller|gm`, and ordered
`priorities: [{when, choice, rank}]`. `choice` is a typed action/target/order
selector; no executable prose or script. Stable fallback after equal priorities
is roster order then local option key order, only within rule-permitted choices.
Manual is an explicit strategy mode; unattended trials reject required manual
choices, including interpreted readied spells. Strategy choice never creates
a legal action that the creature lacks.

Campaign initiative rule parameters contain `grouping: identical|individual|
by_side|explicit`, `within_group: roster|controller`, and `tie: core_rule|roster`.
These are selected rule parameters, not claims that every variant is official.
The D&D default follows the source rule for identical monsters and player/GM
tie choices; alternatives such as all enemies together are explicit campaign
choices. A group has one initiative position but each participant has its own
turn-start/end and refresh events. `controller` order is a game choice made by
the designated controller/strategy. Returning actors keep initiative and do
not acquire an additional turn. First arrivals join at the next eligible
schedule position under the chosen rule.

Phase is `{key, trigger: Trigger, priority: integer, repeat: RepeatPolicy,
required_for_completion: bool, steps: [Step,...]}`. Defaults: priority 0,
repeat `{mode: once}`, required false. Repeat can be `{mode: each_event,
maximum: positive_integer}` or `{mode: rule, rule: mechanic}`. An explicit
maximum prevents authored accidental loops; runtime event limits remain a
separate incomplete-result safeguard. Phase states are
`waiting|running|completed|cancelled`. Empty hostile rosters cannot imply
victory while a required phase still waits.

Trigger uses Mechanics schema event/subject/predicates. ClockSelector is
`{round: positive_integer, boundary: start|end}`, `{turn: roster_key, boundary,
occurrence: positive_integer}`, or `{elapsed_seconds: nonnegative_integer}`.
Clock selectors compile into normal rule-visible events; phase ordering honors
specific rules, controller-given simultaneous order, then configured priority
and the settled stable fallback. No collection iteration order determines play.

ParticipantSelector is an encounter-local `{roster: key}`, a declared sequence
member binding, or a typed runtime Subject supplied by an operation. A dead
actor is still addressable until its lifecycle rule removes it. Objective
`participant_health` uses `participant: roster_key` as the local shorthand.
Predicate composition is `{predicate: all|any, predicates: [Predicate,...]}`
or `{predicate: not, operand: Predicate}`. This does not evaluate arbitrary text.

```yaml
phases:
  - key: reinforcement
    trigger:
      event: "dnd2024:round.start"
      predicates:
        - predicate: test
          value: {op: eq, args: [{context: event.round}, 3]}
    priority: 0
    repeat: {mode: once}
    required_for_completion: true
    steps:
      - key: arrive
        operation: "splat:participant"
        command: return
        participants: [{roster: reinforcement}]
        reason: scheduled_arrival
```

The example presumes an authored absent roster slot `reinforcement`; return on
its first scheduled arrival creates its first active turn eligibility without
resetting state. `spawn` is used instead when no authored placement existed.

## Completion, sequences and results

Completion is `{mode: manual}` or `{mode: objectives, objectives: [Objective,...]}`.
An Objective is `{key, when: Predicate, outcome, requires_phases: all_required|
[phase_keys], terminal: bool}`. Outcome is `{kind: victory|defeat|draw|undecided,
sides?: [side_keys]}`. Simultaneous terminal objectives use a declared
`conflict: {rule|chooser}`; absent resolution produces an incomplete diagnostic,
not an arbitrary first objective. Core helper predicates include
`no_active_hostiles`, `all_required_phases_settled`, `elapsed_rounds`,
`participant_health`, and typed Boolean composition. A manual end is allowed
regardless of automatic objectives, with an optional selected outcome.

Limits default to `{rounds: 1000, resolution_events: 100000, reaction_depth: 64}`
unless authored/configured otherwise; the existing example's lower explicit
limits remain. Limits are engine controls, not rules saying combat must end.
Hitting one yields `{end_reason: limit, outcome: undecided, completeness:
incomplete, diagnostic}`. Manual ending is `{end_reason: manual, ...}`; objectives
use `objective`; fatal unsupported runtime conditions use `unresolved_rule`.
Report incomplete trials separately from win/loss denominators, showing their
count and reasons. Completed-run metrics retain HP, resources, item use, duration
and event log in memory only.

A `sequence` note has `splat_sequence: {steps: [...]}`. Each keyed step is exactly
one of `{encounter: EntryRef, bindings?}`, `{elapsed: {seconds}}`, or
`{rest: {mechanic, participants: all|[binding_keys], duration_seconds}}`.
Roster `bindings` map a next encounter's placement to a continuing participant
ID through a stable sequence member key, never by display name. Unbound slots
instantiate fresh actors. State and elapsed effects carry across bound members;
each independent trial starts the whole sequence fresh. The rest operation
checks completion/interruption and emits recovery only when its rule permits.
On the first use of a sequence member key, instantiate it from that roster slot;
later uses bind the existing actor and validate compatible identity rather than
reapplying authored starting HP. `health_policy` on a roster slot can select the
character treatment for a monster as permitted by the core rules.

## Seeds and fingerprints

A seed note uses `splat_kind: seed` and `splat_seed` with required fields:

| Field | Shape / meaning |
| --- | --- |
| `value` | 64 lowercase hexadecimal characters, 256 bits of seed material |
| `setup` | EntryRef to encounter or sequence |
| `engine` | `{version: string, build: string, semantics: string}`; build identifies exact engine implementation |
| `rng` | `{algorithm: string, revision: positive_integer}`; supported engine contract, never chosen from a filename |
| `fingerprints` | `{format: "splat-inputs-1", algorithm: sha256, aggregate: 64hex, entries: [{id: UUID, digest: 64hex}], external: [{provider, id, digest: 64hex}], rules: 64hex}` |
| `created_at` | Quoted RFC3339 UTC string, metadata only |

The RNG algorithm/revision must match a supported engine build for an exact
replay claim. Selecting its implementation is implementation work, not an
unresolved content shape. Never claim that an arbitrary seed reproduces the
hand-worked trace. A seed record is emitted only with resolved input digests;
an authored incomplete seed draft is editable but not replayable.

Fingerprint format `splat-inputs-1` hashes a resolved semantic projection:
normalize EntryRefs to UUIDs; include engine semantics, applicable rule profiles
and parameters, initial actor/item data, ordered roster/steps, party bindings,
strategies and knowledge policy, assumption facts, limits, and selected
operations. Include only enabled/required entries relevant to that run. Exclude
file paths, filenames, prose, navigation links, sourcebook prose, labels,
ordinary Obsidian tags, retrieval timestamps and credentials. Mechanically used
tags under Splat fields are included. Display renames must not change mechanics.

A roster slot's `overrides` are part of its projection, in whole. This is a stated exception
to excluding labels and prose: `name` and `note` count too, so any changed override changes
the fingerprint. A slot with no overrides adds nothing, so its digest stays as it was and the
format name does not change. `content::overrides::roster_fingerprint` computes this roster
component. No seed writer exists yet, so nothing calls it at runtime. An encounter group added
by hand is not a seed input, because manual encounters cannot be regenerated from a seed alone.

Normalize finite decimal measurements to exact reduced rationals in canonical
units (kg, l, feet, seconds), preserving unknown as a typed unknown rather than
zero. Integers in canonical JSON must be within the exactly representable
range ±(2^53−1); larger required integers use an explicit decimal string type.
Canonicalize the projection using RFC 8785 JSON, with all sets sorted by stable
source identity and all semantically ordered lists preserved. Hash UTF-8 bytes
with SHA-256. Aggregate includes component identity and digest, not concatenated
unlabeled hashes. Resource-key spelling is not an extra identity after resolution.

The seed initializes game randomness, not runtime identity allocation. Runtime
UUIDs never determine order or affect rolls: order comes from roster, steps and
event sequence. Use a separate ID allocator; exact reproduction depends on stable
creation addresses, not matching arbitrary run UUIDs. RNG consumption is tied
to actual rule-requested draws; duplicate events, previews and diagnostics must
not advance the run's stream. These are engine compatibility requirements.

Re-fetch live inputs before replay. Compare fingerprints and engine/RNG identity.
On mismatch, report changed/missing components and allow a new evaluation with
current inputs, but never label it an exact replay. Fingerprints cannot restore
old local or remote data. No saved runtime state, logs or manual choices are
added. Manual encounters still cannot be regenerated from seed alone.

## Editing and recovery

Preserve raw source and unknown non-Splat fields. Unknown Splat fields in a
supported version produce a diagnostic and disable affected structured edits
until understood; never drop them. Newer versions remain view-only as agreed.
Use patch-preserving edits where possible. If a structured edit cannot preserve
the note faithfully, keep the pending change in memory, show the exact rewrite
diff, and require the user to choose that rewrite or continue raw editing;
do not silently normalize the whole document.

Atomic save checks the read fingerprint immediately before replacing the file;
an external change creates a conflict with both pending/local text available.
No background merge discards user changes. Reference repair uses UUID/key
identity; optional links can be offered a corrected path after a unique match.
An ambiguous link, unavailable owner or duplicate identity is not an instruction
to guess. Repair choices are reopen a root, correct a reference, choose a unique
identity, or explicitly create an independent copy under the existing rules.

Deleting/disabling referenced content leaves repairable drafts and blocks only
affected uses. Sharing/moving validates private dependencies before mutation;
external filesystem moves remain detectable repair cases. Read-only indexing
does not rewrite UUIDs, keys, paths, manifests or credentials.
