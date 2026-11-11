# Creature note: representative rules example

> Historical decision record. The remaining schema details are now specified in
> [Mechanics](mechanics-schema.md), [State](state-schema.md), and
> [Campaign/storage](campaign-schema.md). Those contracts supersede the older
> illustrative fields and open-detail lists here. No implementation is authorized.

Status: the shared header, resource keys, stat definitions, resource definitions,
and stable local action identifiers are agreed. Multiple recovery mechanisms
are required. Inventory table markers, columns, and supplementary initial state
are agreed, including multiple charge types per item. Resource-cost sources,
dice expressions, and named resolution operations are agreed. Container setups
and placement behavior are agreed in [Containers](containers.md).
Mend's behavior, recovery overlap, and separate use/roll controls are agreed.
Protect Ally's attack-scoped bonus, reaction revalidation, and nested reactions
are agreed. The user has delegated further fixture choices, requesting coverage
of D&D 2024 rule variants rather than approval rounds for individual examples.
The expanded [Test Warden and coverage cases](rules-coverage.md) define that
target. Detailed action schemas and remaining validation details are design work.
The next concrete draft is [Content schema](schema-draft.md), with complete
[example notes](examples/README.md), an [encounter trace](encounter-trace.md),
and a [coverage audit](schema-coverage.md). Its candidate fields supersede the
partial illustrative field shapes here, without changing the agreed behavior.
This document is not a content file, implemented schema, or complete stat block.
No data-model implementation is authorized.

This builds on [Creatures and encounters](combat.md) and
[Markdown storage](storage.md). Use a homebrew guard with an attack, a
three-use healing ability, and a reaction that protects an ally as the base
worked example. It now serves as the Test Warden in the coverage document, with
named variants and interaction scenes. The skeleton below remains a small
storage illustration, not the complete rules-coverage fixture.

## Agreed organization

- `splat_stats` groups the creature's stat definitions. Each supplied stat has
  `value`; a calculated stat instead has `derive` with a structured expression
  using supported operations. Known included contributions can be recorded
  alongside the value or calculation. Exact expression and contribution fields
  remain to be specified. The purpose is to give each stat one authoritative
  definition and retain enough detail to prevent double counting.
- `splat_resources` maps stable local keys to resource definitions. Each defines
  its `name`, `max`, and `recovery`. Recovery identifies a mechanic such as
  `dnd2024:long_rest`, whose behavior follows the campaign's rule choice.
  `restore: all` means restore that pool to its maximum when the applicable
  recovery rule is satisfied. The current amount belongs to participant state.
  A resource supports multiple recovery mechanisms through its `recovery` list.
  Each mechanism can have its own conditions and recovery behavior; the exact
  condition fields and partial-recovery encoding remain open. Recovery overlap
  follows the agreement below.
- `splat_actions` maps stable local keys to action definitions. Each action has
  a display `name`. The key stays the same when that display name changes.
  Details such as activation, targets, costs, steps, and triggers will be added
  in the next slice; they are not implicit in this skeleton.

These fields are structured in the note model, because stat definitions,
resources, and actions have structured data. On disk, the common identity and
display properties are flat. The `Stat block` tables hold the stats. A folded
Splat data callout holds the resources and actions as marked YAML blocks.

## Partial note skeleton

````markdown
---
splat_schema: 2
tags:
- homebrew
splat_id: 6b02d7d9-19a7-4a18-a5ee-8bb9f00a04bd
splat_kind: creature
splat_key: my_homebrew:creatures/guard
splat_name: Guard
---

A guard who protects allies.


## Stat block

| Stat | Value |
| --- | --- |
| Size |  |
| Type |  |
| Armor Class | 16 |
| Hit Points | 30 |
| Initiative |  |
| Speed |  |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str |  |  |  |
| Dex |  |  |  |
| Con |  |  |  |
| Int |  |  |  |
| Wis |  |  |  |
| Cha |  |  |  |


> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> strike:
>   name: Strike
> mend:
>   name: Mend
> protect:
>   name: Protect Ally
> ```
> 
> <!-- splat:yaml path=splat_resources -->
> 
> ```yaml
> healing:
>   name: Healing Uses
>   max: 3
>   recovery:
>   - mechanic: dnd2024:long_rest
>     restore: all
> ```
````

The three action definitions intentionally lack executable behavior; this
note would be an incomplete draft. An empty Stat block cell is a stat that the
skeleton does not set. Inventory, included stat contributions,
other required stats, and executable controls will appear in later slices.
The resource recovery rule is a choice for this homebrew example, not a claim
about an official creature.

## Local identity and references

Use stable keys such as `healing`, `strike`, and `protect` for definitions
within an entry. They are scoped to their containing entry, so many creature
notes can use `strike` without a global collision. An action cost can refer to
the local resource key; inline controls can refer to an action and one of its
named roll definitions once those are specified.

Renaming a display label leaves its local key unchanged. Explicitly renaming a
local key requires updating its internal references. These local keys are
distinct from the entry's globally qualified resource key and its UUID.

## Inventory tables

Explicitly marked Markdown tables are authoritative for starting inventory.
This is an agreed exception to the previous YAML-only authority for structured
data. Item mechanics remain in the referenced item definitions' YAML.

Use a marked Markdown table as the authoritative starting inventory. Do not
also store its rows as a second YAML inventory list. An explicit marker tells
Splat which table represents inventory, without inferring mechanics from
ordinary prose or unrelated tables. The marker and column names below are
agreed.

```markdown
<!-- splat:inventory -->

| Key | Item | Quantity | Equipment | Attuned |
| --- | --- | --- | --- | --- |
| potions | `dnd2024:items/potion_of_healing` | 3 | carried | false |
| shield | `dnd2024:items/shield` | 1 | wielded | false |
```

The resource keys in this example are illustrative; they do not assert that
these package entries already exist. Item references resolve to definitions
using the same identity and ownership rules as other references. The `Key`
column provides stable local identity, so sorting rows or having two separate
instances of the same item does not merge their state.

The table holds quantities and equipment state. Item definitions retain their
structured mechanics in YAML. Complex per-item starting settings live in
`splat_inventory_details`, keyed by row identity, with non-overlapping fields.
Distinct charged items retain separate individual state as already agreed.

An encounter participant gets its own current inventory from the starting
setup. A simulation's consumption or equipment changes do not write back into
this definition table. Saving run state remains out of scope.

Simple lists such as encounter rosters might later use the same marked-table
approach. This agreement does not automatically convert other structured data
or make arbitrary Markdown tables executable.

### Table encoding

- Use `<!-- splat:inventory -->` immediately before a table, allowing blank
  lines between the marker and table. The comment is hidden in normal rendered
  Markdown. At most one table in a note is an unnamed root table. Each additional
  table names its container with `<!-- splat:inventory container=<row-key> -->`.
  Row keys remain unique across the note. The marker supplies containment, so
  there is no Container column. Unmarked tables are ordinary prose content.
  This format is agreed in [Containers](containers.md).
- Required columns are `Key`, `Item`, `Quantity`, and `Equipment`. `Item` accepts
  a resource key or UUID, optionally formatted as inline code. `Attuned` is an
  optional boolean column, defaulting to false when omitted. An optional `Link`
  column carries an Obsidian navigation link. Equipment labels and allowed
  combinations follow the applicable equipment rules; the schema must distinguish
  carrying, wearing, and wielding. Exact cell grammar remains to be specified.
- Complex per-item starting settings use `splat_inventory_details`, keyed by
  inventory row key. These fields supplement the table without duplicating its
  quantities or equipment columns. In particular, named resource amounts can
  initialize the participant's item state. Stateful individual items use
  separate rows with quantity one. Missing row keys and invalid resource
  names are validation errors, not instructions to create another item.

An item supports multiple named charge pools. Each is defined by the item with
its own limit and recovery mechanisms. Starting amounts are recorded separately
under `resources`. For example, a row named `wand_a` can specify two pools:

```yaml
splat_inventory_details:
  wand_a:
    resources:
      fire: 4
      healing: 2
```

The resources `fire` and `healing` must be defined by the referenced wand. These
are illustrative homebrew pools, not claims about an official item. This is
starting setup data, not saved run state. Consumption during a run still changes
only the participant's in-memory inventory. Pools on two different carried
instances of the same item remain separate.

## Action encoding

The following encoding choices are agreed. Detailed operation and timing
schemas remain open.

### Cost sources

Represent costs as a list. Each resource cost specifies `from`, `resource`,
`amount`, and `at`. `from: actor` uses the acting participant's pool;
`from: source_item` uses the carried item that grants the action. The latter
binds to that individual item, so it does not select a wand by display name
or spend another item's identically named pool.

```yaml
costs:
  - from: source_item
    resource: fire
    amount: 1
    at: activation
```

Several costs can draw from different pools, including actor and item pools,
at their specified stages. This is a resource-cost example, not a full cost
schema: action economy, consumable quantities, legality, interruption behavior,
and payment ordering still need their exact fields. The phase name `activation`
is illustrative until the timing vocabulary is settled.

### Dice notation

Use readable strings such as `1d8 + 2` for simple dice expressions, parsed
as restricted dice operations. They do not execute arbitrary code or interpret
prose. Conditional behavior remains structured YAML. Dynamic stat references,
critical-hit handling, and the exact expression grammar remain open.

### Resolution operations

Use named engine operations for common behavior such as attacks, saves,
damage, and healing, with parameters and explicit branches where needed.
These operations participate in the previously agreed action stages and
reaction opportunities. Standard rules are supplied through the campaign's
rule choices; entry authors do not need to restate every step of attack
resolution. Local roll identifiers still allow inline controls to point to
the attack or damage part of an action. Exact operation fields remain open.

## Worked action: Mend

The behavior in this section is agreed. Field names below illustrate it; they
are not a finalized operation schema or an implemented format.

### Action and targeting

For this homebrew example, Mend takes one action, targets the
guard or one willing creature within touch reach, spends one Healing Use,
and restores `1d8 + 2` HP through the campaign's healing rules.

```yaml
splat_actions:
  mend:
    name: Mend
    activation:
      economy: action
    targets:
      count: 1
      kind: creature
      willing: true
      allow_self: true
      range: touch
    costs:
      - from: actor
        resource: healing
        amount: 1
        at: activation
    steps:
      - key: healing
        operation: heal
        target: selected_targets
        amount: "1d8 + 2"
```

Validate the chosen target and costs before committing the action. Cancelling
target selection spends nothing. Once committed, spend the action and use;
later interruption does not automatically refund them. Specific rules can
change payment or refund behavior. Resolution still exposes the agreed
reaction opportunities. Touch reach uses an explicit targeting assumption
until spatial simulation is supported. Exact event names and stage ordering
remain for the timing design.

### Recovery overlap

Distinct recovery mechanisms each apply when their rules are
satisfied, with the resulting pool capped at its maximum. Process a particular
mechanism only once per qualifying event. Alternatives or mutually exclusive
recoveries must be stated explicitly by their rules; do not infer them from
similar display names. A long rest does not automatically trigger a short-rest
recovery unless the applicable rules say it does.

For example, if two independent mechanisms restore one and two uses on the
same event, an empty three-use pool reaches three. Rules still determine
eligibility and ordering. The exact encoding for partial amounts, conditions,
and exclusivity remains open. Mend's existing long-rest recovery stays as shown
in the skeleton; this example does not add another recovery to that ability.

### Inline controls

Use two controls with distinct behavior:

- `splat:use mend` starts the full action, chooses a target, checks legality,
  pays costs, and resolves healing in an active run.
- `splat:roll mend.healing` rolls the named healing expression and displays
  the result without changing HP or spending resources.

The prose can therefore contain:

```markdown
**Mend.** Restore an ally's strength. `splat:use mend`

Healing roll: `splat:roll mend.healing`
```

The YAML remains the authority for the mechanics. A standalone roll is not
silently reused by a later full action. Library views can preview the action
and roll expression without changing a template or participant. Full action
execution requires an active participant in a run. Exact control grammar and
the treatment of expressions that need missing runtime context remain open.

## Worked reaction: Protect Ally

The following behavior is agreed. This is a homebrew example, not an
official feature. No new operation field names are settled here.

### Trigger and scope

When another ally within 5 feet is hit by an attack, the guard
can spend its reaction to grant that ally +2 AC against that attack only.
Open the opportunity after the hit is determined and before damage. Recheck
the hit using the existing attack roll and campaign rules; do not reroll.
The bonus is scoped to that particular attack resolution and ends when it
ends, so it cannot leak into the next attack or another target's resolution.
Reachability uses explicit assumptions while mapping remains deferred.

This tests the general requirement that triggers identify a specific event
and stage, and effects can last for that event's resolution. It does not
assume that all reactions occur before their triggering event; each reaction
states its own timing.

### Choice and revalidation

Eligibility offers a choice rather than automatically using a
reaction. The participant's controller decides during manual play; its
configured strategy decides during simulation. Both use the same legality
checks. Mandatory triggered effects are distinct from optional reactions.

When several participants can react, use the agreed rule and stable ordering.
Honor a rule-given controller choice before applying a stable default. For this
homebrew pending-hit feature, recheck each later choice before spending costs.
For example, if the first guard turns the hit into a miss, a second guard
cannot spend Protect Ally against a hit that no longer qualifies.
This is not a universal 2024 rule for all already-triggered reactions. Ordinary
reactions follow their completed trigger unless their rule says otherwise; see
[Action resolution](action-resolution.md).

### Nested reactions

A reaction can produce an event that permits another reaction.
Resolve the new opportunity before resuming the interrupted resolution, where
the applicable timing rules permit this. Identify each opportunity by its
triggering event and reacting ability instance, so processing the same event
again does not offer the same opportunity repeatedly. Explicit repeat rules
can permit otherwise valid repetition.

Set a configurable resolution limit to detect unbounded reaction chains. If
reached, stop with a diagnostic and mark the trial incomplete under the agreed
result policy. Do not silently discard reactions or report a completed trial.
Exact limits and timing vocabulary remain open.

## Worked attack: Strike

The following choices are selected under the user's delegation of example
details. The illustrative homebrew Strike takes one action,
targets one creature within 5 feet, has a supplied attack bonus of +5, and deals
`1d8 + 3` piercing damage on a hit. Campaign rules determine attack resolution,
including critical hits and reaction opportunities. These numbers do not claim
to describe an official creature. Exact operation fields remain open.

### Roll results

Retain individual die results, modifiers and their causes, the
selected dice, and the resulting total in the run's in-memory roll records.
Rerolls or replacements retain the earlier result and identify which result
is now used. This permits a critical-hit rule to inspect a natural die result
and a reaction to modify a roll without reconstructing it from a total.

Tie each record to its action resolution and named local roll. Preserve the
agreed roll scope: one shared roll can serve several targets, while target-
specific evaluations remain separate. A changed target modifier does not
require a new random draw unless its rule says to reroll. These are temporary
run records; saving logs or run state remains deferred.

### Damage components

Represent damage as typed components grouped into a damage event.
Strike starts with one piercing component. A variant could add a separate fire
component to the same hit. Keep each component's expression, type, and source
identifiable so rules can apply critical-hit behavior, bonuses, resistance,
immunity, and other adjustments correctly.

The applicable rules determine calculation order, rounding, and whether a
modifier applies to a component or the combined event. Do not assume that every
component creates a separate damage event or that every bonus die qualifies
for a critical hit. Grouping must preserve effects that trigger once when
damage is taken. Separate targets retain their own damage evaluation, even
when they use the same initial roll.

### Multiple attacks

Represent a multiattack as one ability use containing separate,
ordered attack resolutions. Each attack has its own target, roll, reactions,
and outcome. The ability defines its permitted attacks and target restrictions.
Where rules allow, select the next target after the earlier attack resolves;
do not require all targets to be locked when the ability starts.

For example, a two-Strike ability could defeat one enemy with the first attack
and choose another for the second. The parent ability pays its declared action
cost once; contained attacks incur only the additional costs its rules require.
This avoids charging an action for each use of Strike's resolution. An
interrupted sequence follows its rules for which remaining attacks, if any,
can still occur. The Martial variant exercises this behavior. Revised monster
Multiattack is part of the Attack action (SRD 5.2.1 p257); do not encode the
older assumption that it is a separate action category. Extra Attack and
Multiattack retain their own granting rules and permitted substitutions.

## Design tree

- Structured stat definitions under `splat_stats`, with `value` or `derive`:
  agreed.
- Resource definitions under `splat_resources`, with `max` and mechanic-based
  recovery, separate from the participant's remaining amount: agreed.
- Multiple recovery mechanisms per resource: required.
- Actions under `splat_actions` keyed by stable local identifiers, with mutable
  display names: agreed.
- Authoritative marked Markdown tables for inventory, with item mechanics in
  YAML and no duplicate inventory list: agreed.
- The inventory marker, required and optional columns, and supplementary
  initial state under `splat_inventory_details` are agreed.
- Items can have multiple named charge pools with independent starting amounts,
  limits, and recovery mechanisms: required.
- Resource-cost lists identify the actor or source item, pool, amount, and
  payment stage: agreed; exact stage names remain open.
- Readable dice expressions parsed as restricted dice operations: agreed.
- Named engine operations for attacks, saves, damage, and healing, using
  campaign rules and reaction opportunities: agreed.
- Container setups, independent placements, and one source for starting
  contents: agreed in [Containers](containers.md).
- Mend's target selection, commitment, payment, and healing behavior: agreed.
- Independent qualifying recoveries combine up to the pool maximum; process
  each mechanism once per event unless its rules specify otherwise: agreed.
- Full action execution uses `splat:use`; standalone `splat:roll` changes no
  participant state and is not automatically reused by an action: agreed.
- Protect Ally can change a pending hit through an attack-scoped AC bonus:
  agreed.
- Reactions are chosen by a controller or strategy; current permission is
  checked, while trigger lifetime and revalidation follow the actual feature.
  Protect Ally's pending-hit example is not a universal reaction rule.
- Nested reactions follow their trigger timing; duplicate opportunities are
  prevented, and a resolution limit produces an incomplete trial: agreed.
- Structured roll results, typed damage components, and separate attack
  resolutions within a multiattack: selected under delegated example choices.
- Next work: use [the coverage cases](rules-coverage.md) to refine schema needs
  and record source-specific exceptions without per-example approval rounds.
- Subsequent detail: cell grammar and validation, resource conditions, recovery
  amount encoding, action timing, and full operation schemas.

Complete the attack, then settle the detailed operation
schemas and timing vocabulary. Those details remain open.
