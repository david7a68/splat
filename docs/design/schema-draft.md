# Content schema draft

Status: reviewable design with a partial [read-only verifier](../srd-import.md).
The user authorized this draft and delegated routine field choices. The fields below are the note model.
`splat_schema` is the content version of a note on disk, separate from the app's existing settings schema.
Splat reads version 2 only; the example notes show that layout.

The remaining field contracts are now complete for the agreed scope:
[Mechanics](mechanics-schema.md), [State](state-schema.md), and
[Campaign/storage](campaign-schema.md). Those documents extend and, where more
specific, supersede this first worked slice. [Schema checks](schema-checks.md)
and the refreshed coverage audit distinguish design checks from engine tests.

Start with [the complete example notes](examples/README.md), then read
[the encounter trace](encounter-trace.md) and [the coverage audit](schema-coverage.md).
The [2024 action-resolution contract](action-resolution.md) governs timing and
cost outcomes; its source-specific rules supersede generic interpretations here.
Earlier design documents record decisions; this document and the example notes
specify the candidate field shapes where earlier examples were illustrative.
These design files do not register a package or change campaign settings.
Files under `examples/` remain documentation artifacts and reader fixtures.

## Boundaries and identities

| Layer | Contains | Lifetime |
| --- | --- | --- |
| User settings | Existing session preferences, UUID-to-folder registry, D&D Beyond mapping/cache | Existing app settings |
| Authored content | Owner manifests, definitions, starting inventories, encounters, strategies, assumptions | Markdown with YAML frontmatter |
| Resolved inputs | Exact definitions, selected rules, live character data, starting state | In memory for one run |
| Run state | Participants, controllers, item instances, pools, effects, choices, rolls, pending resolutions | In memory for one run |
| Seed record | Seed, engine identity, setup reference, input fingerprints | Authored metadata; no state snapshot |

Entry UUIDs identify authored notes. Namespaced keys resolve to those UUIDs.
Local keys identify actions, pools, inventory rows, or roster slots only within
their containing definition. Instantiate each roster slot and each item into a
distinct runtime UUID. A second placement of the same definition is independent.
Runtime references never use a display name or row key alone.

Keep a source address `(entry UUID, local key)` for diagnostics and provenance.
For nested setup contents it remains scoped to the referenced setup, with the
runtime placement distinguishing repeated copies. Transfers preserve runtime
IDs; they do not edit authored row keys. An explicit editor transfer between
notes assigns a collision-free destination row key and updates local references.

## Common fields and references

Every entry requires `splat_schema`, `splat_id`, `splat_kind`, and `splat_name`.
Optional fields are `splat_key`, `splat_sources` (attribution records with title,
locator and optional URL), and `splat_origin` (an optional entry reference).
Entry kinds used here are `package`, `campaign`, `creature`, `item`, `spell`,
`container`, `encounter`, `rule`, `rule_group`, `effect`, `strategy`, `sequence`,
`seed`, and source-only `reference` notes as specified in the
[import contract](../srd-import.md). A creature can represent
a PC or monster; encounter membership and zero-HP policy are separate concerns.

An entry reference is exactly one of `{id: UUID}` or `{key: namespace:local_key}`,
plus an optional navigation `link`. This rule also applies to required references
under `splat_requires`. Automatically find references in mechanics as well;
`splat_requires` adds dependencies that cannot be discovered from those fields.
Never use the list as a replacement for actual dependency traversal.

Owners can also declare `splat_requires_mechanics`, a list of required rule
contracts. This cannot enable a package or resolve a rule conflict; those remain
explicit campaign selections. It makes the examples' external core dependency
visible without inventing UUIDs for uninstalled core entries.

Engine operation names, rule mechanic names, stacking keys, and event names are
not entry references. They select registered engine contracts or rule slots;
unknown required names block the affected simulation. A rule note declares
`splat_rule: {mechanic: ..., implementation: ..., parameters: ...}`. Implementation
is an engine-registered name, never a script, path, or arbitrary package code.
Campaign `splat_rule_choices` still selects concrete rule entries for mechanics.

The examples use `dnd2024:*` mechanics as a required external rule contract,
listed in their README. They do not pretend to supply an implemented core package.

## Stats, expressions and pools

`splat_creature` contains size, type and tags. `splat_stats` maps local stat keys
to exactly one of `value` or `derive`. Supplied values may declare `includes`,
a list of contribution identities such as `inventory.armor.base` and
`ability.dexterity`. These identify already-counted contributions, not another
set of bonuses to add. Changing an included source requires a known rebuild
rule; otherwise the change is unsupported. Multiple armor formulas are choices,
not additive bonuses. The fixture declares `rebuild: dnd2024:armor_class`.

Expressions are data. A number or boolean is a literal; a dice string is accepted
only in a roll's `expression`. Structured expressions use one discriminator:

| Shape | Meaning |
| --- | --- |
| `{stat: {of: actor, key: strength}}` | Read an effective stat from the bound participant |
| `{binding: cast_level}` | Read a validated invocation parameter |
| `{context: event.target}` | Read a typed event field; not arbitrary object traversal |
| `{result: {step: attack, field: hit}}` | Read a completed operation's typed output |
| `{op: add, args: [...]}` | Apply a registered pure operation |

The initial pure-operation vocabulary is `add`, `subtract`, `multiply`, `floor`,
`min`, `max`, `ability_modifier`, `eq`, `lt`, `lte`, `gt`, `gte`, `all`, `any`,
`not`. Context fields are enumerated by each event contract. No general source
evaluation, interpolation, filesystem access, or arbitrary calls are allowed.
Cycles in derived stats or action aliases are validation errors. Dice are not
implicit in expressions: randomness always has an identified roll record.

`splat_resources` maps a local pool key to `name`, `max`, `initial` (defaults to
maximum), and `recovery`. Each recovery has a stable `key`, `mechanic`, optional
`when`, and `restore: all` or `restore: {add: expression}`. Use `roll` with a named
dice expression when recovery is random. Qualifying independent mechanisms
combine up to maximum; a mechanism processes an event once. An explicit
`exclusive_group` delegates mutually exclusive recovery selection to its rule.
Start with all pools full unless a starting-state field overrides an amount.

For actor resources the recovery subject defaults to the actor. Item recovery
must declare its subject: `root_holder` selects the creature currently carrying
the item, including through nested containers, when the recovery event completes.
It does not select an author or package owner. `owner` on an item instead means
the item's own events. This draft's homebrew wand explicitly uses `root_holder`.
Further duration-of-possession requirements belong in `when`.

Optional pool `kind: spell_slot` with `level` makes slot expenditure recognizable
to the casting rule; naming a generic pool `spell_slots_1` alone is insufficient.
Other typed budgets/pools need their own registered contracts. `splat_hit_dice`
holds count and die size; it does not overwrite a supplied maximum HP.
`splat_proficiencies` records saves, skill proficiency/expertise, armor training
and weapon categories. `splat_senses` and `splat_languages` supply perception
and communication data. Missing irrelevant languages do not block an attack;
a missing stat required by a chosen operation does.

Action economy is also tracked through runtime budgets, but is not a universal
boolean. A campaign rule supplies action, bonus-action and ordinary-reaction
budgets. Feature-granted budgets can have their own limits, refresh and allowed
uses (for example, only opportunity attacks). Legendary uses, once-per-turn
permissions and spell-slot expenditure restrictions retain separate scopes.

## Actions and composition

`splat_actions` maps local keys to a full action or an external action alias.
A full action has `name`, `activation`, optional `requirements`, `targets`,
`costs`, `rolls`, and ordered `steps`. It may define `effects` used by its steps.

`activation` declares `economy` (`action`, `bonus_action`, `reaction`, `none`),
`category` (for example `attack` or `magic`), `timing` (normally `on_turn`),
and `cost_at` (normally `commit`). A reaction instead has a `trigger` with an
event and predicates. Mandatory triggers use `economy: none` and `automatic:
true`; optional actions always require a controller/strategy decision.

An external alias has `name` and `use: {entry: reference, action: local_key,
bindings: {...}}`. Its target, economy, costs and operations come from the
referenced action; it cannot silently replace them. The referenced entry
declares typed `splat_parameters`. A binding can select an actor pool by local
key or supply a scalar, as in the spell example. Each invocation retains both
its actor-local alias and referenced source action for logs and controls.

Targets are named selectors, each with `kind`, `count: {min, max}`, and optional
`relation`, `allow_self`, `willing`, `range`, or visibility requirements. Choice
belongs to the controller/strategy. `range` is `{feet: n}` or `{touch: true}`.
The encounter supplies non-spatial facts; absence of a required fact is unknown,
not false or true by default. A binding such as `target: chosen` selects that
group; operations run per target or share rolls as their contracts specify.

Requirements use typed registered predicates, such as `equipment_state`, with
explicit subjects. Target-rule failures are distinct from known selection
errors: spell rules can allow an attempted cast whose hidden invalid target
still consumes a slot. Do not generalize Mend's prevalidation to all spells.

Costs are a list. Resource costs use `from: actor|source_item`, `resource` (a
local pool key or binding), `amount`, and `at`. Rule-owned expenditure outcomes
replace the earlier illustrative `refund` field. Consumables
use `from: source_item`, `quantity`, `at`. Validate all costs due at a stage
before spending that stage's costs together. Already paid earlier-stage costs
follow explicit interruption/refund rules; no universal rollback exists.
An action's economy cost is declared once in activation, not duplicated here.

Keep a cost ledger with reserved, paid and reversed amounts and the rule that
caused each change. Preserve the distinction between a slot not expended and a
slot spent then recovered. Counterspell's explicit non-expenditure outcome is
not a general refund instruction; it does not count as slot expenditure for
the turn and does not restore the wasted casting action or arbitrary other
costs. Reservations or provisional debits are internal bookkeeping, not new
game timing. See the source-backed casting and Ready cases in the contract.

The stage names are `commit`, `resolution`, and named operation boundaries
`before:<step>` / `after:<step>`. They are checkpoints, not a claim that every
ability follows the same timing. `at` can instead name a registered rule-managed
cost point, such as `dnd2024:spell_slot_expenditure`. The casting operation then
applies its specific expenditure outcomes; the entry does not invent a refund
callback for Counterspell. Rule events expose finer opportunities such
as `dnd2024:attack.hit_pending` and `dnd2024:spell.cast_pending`. The former is
provisional, before damage. Changing its effective AC re-evaluates the original
attack roll. Event IDs distinguish repeated delivery from a new event.
The normal Reaction follows its completed trigger immediately. Pending-result
events apply only where a particular rule permits an interrupt or result change.
Do not revalidate all past triggers or discard them after unrelated state changes.

Bind a reaction use to its triggering event and resolution when creating that
use. Nested events cannot replace this binding with a mutable global 'current
event'; an effect's expiry and target must still name the original resolution.

An Attack action can contain several attack operations, with new selections
between them. Use an ordered `sequence` or bounded `repeat` over declared
resolution steps. Do not invoke a full action and pay its economy again.
Extra Attack, Multiattack and attack substitutions supply their own permissions.
Reaction opportunities can nest; resume the parent only after permitted child
resolution and its state changes finish. The run limit produces an incomplete
result on an unbounded chain, never a silent partial success.

## Rolls, operations and effects

`rolls` maps local names to an expression, `scope` (`use`, `target`, `attack`),
and, when needed, a `modifier`. Roll only when a step needs that roll, after
relevant choices/interrupts. Cache by the declared scope, not globally by name.
Keep dice, modifiers, selected result, original/rerolled results and causes.
Critical-hit rules transform eligible damage dice, not flat modifiers.

| Step operation | Principal inputs and results |
| --- | --- |
| `dnd2024:attack` | Target, kind, roll, optional weapon row, hit damage; outputs target-specific hit/critical and damage results; opens rule-required reactions |
| `dnd2024:save_damage` | Target group, ability/DC, damage components and success scale; independent saves, shared or per-target damage by roll scope |
| `dnd2024:heal` | Target and amount; applies healing restrictions and maximum HP |
| `splat:apply_effect` | Target and local effect definition; creates an application linked to the use |
| `splat:sequence` / `splat:repeat` | Ordered steps or bounded repeated resolution; keeps choices and event ancestry |

Damage components have a `type`, `amount: {roll: key}` or numeric amount, and
critical eligibility where it differs from the attack rule. Components are
grouped into one damage event per affected target unless the ability says
otherwise. Reduction, immunity, resistance, vulnerability, rounding, threshold,
concentration checks and on-damage triggers follow the selected rule contracts.

Effects have `stacking: {key, policy}`, `duration`, and `modifiers` or rule-defined
conditions. Policies are registered rule contracts, not one universal maximum.
Duration can bind to `event.resolution` or declare an expiry event and subject,
elapsed time, concentration owner, or explicit removal condition. Preserve
suppressed applications, their causes and independent expiry. Applying an
effect to many targets creates target records linked to one application.
Source-relative relationships and concentration ownership are explicit runtime
references, not encoded in tags. Complex condition rules remain rule contracts.

The concrete effect below lasts for `event.resolution` and adds 2 to effective
AC only when `scope: event.resolution`; later attacks do not see it. Reaching
the resolution's end removes it even when the attack misses or is cancelled.

Inline controls use `splat:use <action_key>` and
`splat:roll <action_key>.<roll_key>`. Each local key follows the row-key grammar
below. An action alias resolves its inherited roll keys. An item note's control
needs an exact item instance in play; a library view only previews it. A reaction
control requires a live eligible opportunity. A standalone roll uses its named
expression but never applies damage, pays costs, or supplies a later action's
result. If a preview lacks required bindings or participant stats, request those
inputs or leave it unavailable; never fabricate them.

## Items, inventory and containers

`splat_item` on an item definition supplies category, mass, stackability,
equipment behavior, and optional containment. On a container setup it is an
entry reference to the underlying container item, as already agreed; entry kind
disambiguates the two shapes. Items may have their own resources and actions.
Those actions bind `source_item` to a particular instance.

Inventory columns stay `Key`, `Item`, `Quantity`, `Equipment`, optional `Attuned`
and `Link`. Exact Equipment values in this draft are `carried`, `worn`, `wielded`,
`stored`. Location comes solely from the table marker; `stored` means not worn
or wielded while inside a container. A root creature inventory uses carried,
worn or wielded. A standalone container's root holds stored items. Item-specific
rules validate legal states, hands, attunement and transitions; table editing
does not bypass those constraints during a run.

Row keys match `[a-z][a-z0-9_-]*`, unique across the note. Quantity is a positive
integer; individually stateful items and populated containers use one. Item
cells contain a UUID or namespaced key, optionally one code span. Attuned is
`true`/`false`; omission is false. Supplementary `splat_inventory_details` may
set pool amounts but not duplicate quantity, equipment or containment.

Use `<!-- splat:inventory -->` once at most and
`<!-- splat:inventory container=row_key -->` for local contents. A setup reference
and a local table for that same row are mutually exclusive. Referenced setup
contents retain their own local scope. Reject reference cycles as well as local
containment cycles. A setup can define its own initial item pools under
`splat_initial.resources`; placed-row overrides may change those amounts, but
cannot implicitly merge or replace its contents.

Mass/volume quantities use `{value, unit}`, initially `lb`/`kg` and `ft3`/`l`;
count is unitless. Convert explicitly for comparisons. Capacity is independent
of contributed mass. `carried_mass` is `self_and_contents` or a specified fixed
quantity; other magic behavior requires a named rule. A capacity dimension not
declared by the item imposes no limit; an unknown measurement for a declared
limit blocks that check. Transfers are atomic in state but their action costs
and failure consequences still follow the applicable rules.

## Encounters, rules and runtime state

`splat_encounter` contains ordered `roster`, `sides`, `assumptions`, `completion`,
`limits`, and optional `phases`. Roster rows have a stable `key`, definition
reference or party binding, side, controller strategy, and starting-state
overrides. Each is one placement; repeating a creature means another row.
Overrides alter initial HP/pools/equipment only; they are not a hidden copy of
the definition's rules. Mechanic changes require a variant entry or campaign
rule choice. The campaign retains authority over initiative and other rules.

Phases define a key, trigger, priority, repeat policy and operations to arrive,
depart or replace a roster participant. Return retains identity and state;
replacement creates a new participant. Sequence entries contain ordered
encounter references, explicit elapsed time, and rest events. Strategies are
named registered policies with parameters; `manual` is valid only in interactive
play or a design trace, not an unattended simulation with missing decisions.

Minimum in-memory records for the trace:

| Record | Required relationships/state |
| --- | --- |
| Run | Input fingerprint, rule selection, roster order, clock/turn schedule, RNG, limits, end reason and outcome |
| Participant | Runtime UUID, roster/source address, side, controller, HP/temp HP, budgets, pools, inventory and active effects |
| Item instance | Runtime UUID, source definition/address, holder, equipment/attunement, quantity or individual pools, contents |
| Action use | Runtime UUID, actor, source item if any, definition/alias, bindings, choices, costs paid, parent resolution |
| Readied spell | Actor and casting use, stated trigger, declared or undecided targets, held energy, Concentration dependency, spent costs, expiry and release/cancellation state; manual controller decides interpreted targeting |
| Resolution/event | Stable runtime ID, stage, cause/parent, target, pending outcome, reaction opportunities and processed deliveries |
| Effect application | Use/source, target records, duration, stacking identity, suppression, concentration/relationship links |
| Roll | Use and local roll key, scope owner, dice/modifiers, replacements, effective result |

These are conceptual records, not Rust types or saved JSON schemas. Runtime
controller changes must not overwrite actor identity. Item holder is separate
from package ownership. Party membership is separate from encounter side.

Readied-spell interpretation is tracked in manual encounters, including for
creatures. Automatic strategies for that behavior are deferred; a required
unresolved manual decision cannot be silently simulated. Hydra's extra reaction
capacity is recomputed at its start-of-turn refresh under the user's selected
ruling, not immediately after each head-count change. See
[Action resolution](action-resolution.md) for both decisions.

## Validation and remaining review

Validate syntax, schema version, IDs, owner/key uniqueness, typed references,
local bindings, expression types, dependency cycles, target/cardinality rules,
roll references, resource names, costs, inventory markers, and starting amounts.
At run preparation resolve rule contracts and required assumptions. Unknown
mechanics or incompatible selections block only affected runs. During execution
check rule legality and current state again after interrupts.

Preserve existing edit-safety decisions: malformed notes remain raw-editable,
unsupported future schemas are view-only, and external edits cause conflicts
rather than overwrites. This draft does not weaken those boundaries.

The first worked slice and the three completion contracts are ready for review.
The coverage audit maps every agreed family to its candidate contract. No claim
of implemented 5.5e simulation support follows from a schema field existing.
