# Creatures and encounters

> Historical decision record. The remaining schema details are now specified in
> [Mechanics](mechanics-schema.md), [State](state-schema.md), and
> [Campaign/storage](campaign-schema.md). Those contracts supersede the older
> illustrative fields and open-detail lists here. No implementation is authorized.

Status: the common combat model, inventory and equipment requirements, resources,
effects and stacking, multiple targets, staged action resolution, stat authority,
and fixed run inputs are agreed. Rosters can use party members, creatures, or
both. Encounter sequences, overlapping triggered phases, retained state on
return, configurable strategies, and explicit targeting assumptions are agreed.
Encounter completion, manual endings, configurable initiative, absent-participant
timing, and stable trigger ordering are also agreed. Sides, end reasons and
outcomes, initial reports, and saving authored setups are agreed. Run state and
results stay in memory. Seed records retain replay metadata and input
fingerprints. Detailed Markdown schemas remain under discussion. Mapping and
spatial simulation are deferred.
No data-model implementation is authorized.

The [schema draft](schema-draft.md) and [worked encounter trace](encounter-trace.md)
make the common action/item/effect relationships concrete. The
[coverage audit](schema-coverage.md) records remaining operation-contract gaps.

This design builds on [Packages and campaign content](packages.md).

## Agreed foundations

### Common combat model

Monsters and player characters use the same combat model: stats, actions,
resources, effects, inventory, and consumables. Character details such as class
progression can remain separate. Both sides use the same rule resolution.

The user explicitly included inventory and consumables in this scope.

### Inventory and consumables

An item definition is separate from the individual items in an inventory. For example,
a package defines a healing potion and a guard's inventory records three of
them. Identical items can share a quantity. Items with separate state, such as
two wands with different remaining charges, are tracked individually.
Equipment state belongs to the carried item.

Store starting inventory in explicitly marked Markdown tables, with stable
local row keys and references to item definitions. The tables are authoritative;
do not duplicate their inventory list in YAML. Item mechanics stay in the
referenced definitions. Simulation changes affect the participant's inventory
in memory, without writing back to the source table. The marker, columns, and
`splat_inventory_details` mapping are agreed in
[the creature example](creature-example.md). An individual item can have
several named charge pools, each with its own starting amount and recovery.

Using a consumable reduces its inventory quantity when the applicable rules
require it.

Carrying, wearing, wielding, and attunement are distinct states. An item's rules
determine which states are required for its granted actions and effects.
Those actions and effects reference the carried item, so charge use and
equipment changes affect the correct item.

Detailed equipment constraints, transfer behavior, and the timing of equipment
changes remain open. Consumption timing follows staged action resolution.

Inventory must support containers beyond a creature's carrying capacity,
including barrels, backpacks, and Bags of Holding. Containers can exist without
a creature carrying them and have their own storage rules. Container items
support rule-permitted nesting, separate capacity and carried weight, and
rule-defined access. Their storage and transfer details are the active branch in
[Containers](containers.md). This does not bring maps or spatial simulation
back into scope.

### Supplied and calculated stats

Each stat explicitly uses either a supplied value or a calculated value.
For example, AC can be supplied as 18 or calculated from armor, Dexterity,
and other contributions.

Record which bonuses are already included to prevent double counting. If an
imported value lacks that detail, identify equipment changes that cannot be
calculated reliably. Accepting a supplied value does not imply that its unknown
components can be reconstructed.

Store stat definitions under `splat_stats`. A supplied stat uses `value`; a
calculated stat uses `derive` with a structured calculation. Exact calculation
and included-contribution fields remain to be specified.

### Resources

Abilities spend named resource pools. Each pool tracks its available amount
and defines its limit and recovery rules. Multiple abilities can share a pool.
The example guard's healing ability spends from a pool with three uses.

Store resource definitions under `splat_resources` with stable local keys,
names, maxima, and a `recovery` list. Multiple recovery mechanisms per resource
are supported. A recovery mechanism follows the campaign's rule choice, such
as for `dnd2024:long_rest`. The current amount belongs to participant state.

Independent qualifying recovery mechanisms each apply once per event, capped
at the pool maximum. Rules explicitly identify alternatives or exclusions;
a long rest does not automatically trigger short-rest recovery. Specific
resource kinds, recovery conditions, and partial-recovery encoding remain
to be specified. See [Mend](creature-example.md) for the agreed example and
cost behavior. Cost timing follows the rules for staged action resolution.

### Active effects and immediate changes

Ongoing effects are separate records with an origin and an end condition.
For example, a temporary AC bonus records what caused it, who it affects,
and when it ends. Applicable rules calculate effective stats without rewriting
the creature's base stats. Here, an effect's origin means its cause in play,
not the sourcebook or author described by the glossary term Source.

Damage and healing change current HP directly; they do not need to remain as
active effects.

### Stacking and effect applications

Tags describe categories, such as spell effects or AC bonuses. An explicit,
stable stacking key identifies effects that the rules treat as the same mechanic.
Equivalent entries from different packages can share a stacking key. Sharing a
broad category tag alone does not make effects conflict. The applicable rule
determines how matching contributions combine.

Retain overlapping applications even when only one contributes. Each retains
its cause, target, start time, and end condition. A suppressed effect's duration
continues. If the stronger effect ends, a weaker effect can contribute again
if it is still valid. Relationships such as who grappled whom remain available.

One spell casting or ability use groups its related effects into an effect
application. Each effect can have its own target and end condition. This lets
rules compare whole spell applications where required, without selecting
individual bonuses from competing castings. Overlap on one target does not
automatically suppress effects on other targets.

Potency comparison and grouping follow the applicable rule; a display name or
the largest numeric bonus is not sufficient for every case. Detailed matching
policies remain open.

Copying a definition preserves its stacking key by default. The copy has its
own entry UUID but initially counts as the same mechanic for stacking. Renaming
it does not change that identity. The user can explicitly declare the copy a
distinct mechanic when creating a variant that should combine differently.

#### Multiple targets

One application can affect several encounter participants, with shared
application state and individual effect state.

- The shared application identifies the caster or ability user and the spell or
  ability used. Common duration and concentration requirements belong to this
  use, where required by its rules.
- Target-specific effect records link back to that application and identify
  their target. They retain individual outcomes and end conditions. Stacking
  is evaluated for the affected target and the relevant rule, so suppression on
  one target does not suppress the application everywhere.
- Ending an effect on one target does not by itself end effects on other targets.
  Ending concentration ends the effects that depend on that concentration.
  Effects with an independent lifetime follow their own rules.

For example, a first Bless casting targets A, B, and C. A later casting by a
different caster targets B, C, and D. If both are equally potent, A uses the
first casting, B and C use the later casting, and D uses the later casting.
The first casting's effects on B and C remain recorded with their own duration.
If the later casting ends, B and C can use the first casting again if it is
still valid; D loses the later casting's benefit.

Rules explicitly specify whether a roll is shared across targets. For example,
an ability can roll damage once and use that result for several targets while
each target makes its own save. Separate target records do not automatically
cause separate damage rolls. The exact scope of each roll follows the ability's
rules and remains distinct from the number of effect records.

Applications support changing targets, such as participants entering an aura.
An ability defines target eligibility and when effects start or end. Leaving
an area can end an effect immediately or leave a lasting effect, according to
its rules. This relationship is agreed without yet choosing how maps and
positions are represented.

Mapping and spatial simulation have since been explicitly deferred. The model
still supports changing targets, but this does not require a geometry engine
in the initial scope. Simulations use explicit targeting assumptions as
described under Mapping scope.

#### Stacking constraints from the 2024 rules

These are factual constraints for the design, not agreement on a storage schema:

- Multiple castings of the same spell do not combine during overlapping
  durations. The most potent applies; equal potency uses the most recent.
  Different casters do not by themselves make the same spell combine.
- Repeated conditions retain separate durations without making the condition's
  effects worse. Exhaustion is an explicit exception with cumulative levels.
- Charmed, Frightened, and Grappled refer to the charmer, source of fear, and
  grappler respectively. The model must retain those causes and relationships.
- Resistance and Advantage/Disadvantage have their own combination rules.
  One universal method for selecting the strongest effect is insufficient.

Primary references:

- [Combining Spell Effects](https://www.dndbeyond.com/sources/dnd/free-rules/spells#CombiningSpellEffects)
- [Conditions Don't Stack](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game#ConditionsDontStack)
- [Exhaustion](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary#ExhaustionCondition)
- [Charmed](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary#CharmedCondition),
  [Frightened](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary#FrightenedCondition),
  and [Grappled](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary#GrappledCondition)
- [Resistance and Vulnerability](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game#ResistanceandVulnerability)
- [Advantage/Disadvantage](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game#AdvantageDisadvantage)

### Encounter templates and runs

An encounter template defines the roster and starting setup. Each encounter run
has its own participants, current HP, spent resources, and active effects.
Running an encounter does not change its template.

An encounter roster can consist entirely of party members, entirely of creatures,
or a combination. Party membership is optional. Reusable templates can have an
optional party slot supplied by the campaign when preparing a run, as well as
explicit creature selections. A shared template does not require references to
a particular campaign's private characters.

Each participant has an explicit encounter side, independent of party membership
or creature type. A creature can fight alongside the party. An encounter can
contain several opposing sides.

When a run starts, capture its resolved stats, rules, and inventory in memory.
The run keeps those starting definitions while its combat state changes through
play. Editing a creature definition or refreshing a D&D Beyond character affects
new runs, without changing the inputs of an existing run.

The initial version saves authored setups while keeping run state and results
in memory. Saving and resuming runs is deferred. Authored content, rosters,
phases, strategies, and targeting assumptions are saved. Seed retention is the
limited addition requested by the user; seed records hold the metadata described
below. The existing live-only boundary for D&D Beyond data remains in place.

### Encounter sequences and phases

Simulations support explicit sequences of encounters. HP, resources, inventory,
and ongoing effects carry forward within a sequence. Elapsed time and rests
are explicit parts of that sequence. Each independent trial starts from the
same initial state.

An encounter can also have phases in which creatures enter or leave combat
when specified triggers occur. This is part of one encounter, distinct from
moving to another encounter in a sequence.

Phases have independent triggers and can overlap. For example, reinforcements
can enter at the start of round three while a separate trigger makes a boss
retreat below a specified HP threshold. Triggers fire once by default;
repetition must be enabled explicitly.

A participant that leaves and returns retains its identity, HP, inventory,
and spent resources. Time continues and effects expire according to their
rules. Leaving combat does not count as death or provide a rest. A replacement
creature is a new participant.

Absent participants take no voluntary actions unless the encounter explicitly
schedules them. Durations and required events, such as recurring damage or
saving throws, continue at their normal times. A departed participant retains
its place in the timing schedule for these checks.

Detailed trigger vocabulary remains open.

### Encounter completion and manual endings

Encounters have explicit completion conditions, such as defeating an opposing
force, surviving five rounds, or completing another objective. Required
reinforcement phases must finish or be cancelled before the default victory
condition succeeds. An empty battlefield between waves does not automatically
end the encounter.

Manual ending is also supported; many actual encounters will be ended by the
user. It does not require automatic completion conditions to be satisfied.

Record the end reason separately from the outcome. A manual ending can include
a user-selected outcome. Simulation limits, such as a maximum round count,
produce an incomplete result. Reports show incomplete trials separately from
wins and losses. Detailed side relationships, outcome fields, and limit values
remain to be specified.

### Initiative and participant entry

Initiative is a configurable campaign mechanic. Support alternative methods,
including grouping all enemies at one initiative position. Grouped participants
retain their individual identities and combat state.

For individual initiative, roll on first entry and retain that initiative on
return. Grouped initiative uses the selected method's group assignment and
initiative. Arrival happens when triggered, with the next turn at the next
eligible position in the turn order. Leaving and returning cannot grant an
extra turn. Entry, return, and timing checks must use the selected initiative
method rather than assume that every participant rolls separately.

Exact group formation and within-group turn rules remain to be specified with
the relevant initiative methods.

### Simultaneous encounter triggers

Apply game rules that specify the order first, then any explicit encounter
priority. When these leave a tie, use initiative order, then roster order.
The user permits any stable fallback and suggested this sequence; it is the
chosen default rather than attempting to calculate which order most benefits
the players.

If neither initiative nor a participant's roster position applies, retain a
stable encounter-defined trigger order to resolve the remaining tie. This is
an explicit order in the encounter, independent of filesystem traversal or
collection iteration. Tied triggers do not require user intervention merely
because no custom priority was supplied.

### Mapping scope

Mapping is out of scope for now. Do not require maps, creature tokens,
coordinates or grid placement, terrain, cover calculation, or a spatial
simulation engine for the initial design. The earlier coordinate proposal
was not accepted.

Range and area information can remain part of a rule's definition, and the
accepted model can represent effects on changing targets.

The initial simulator uses explicit targeting assumptions. Examples include
"all active enemies are reachable" and "this area effect hits three eligible
targets." These are examples of scenario assumptions, not universal defaults.
Results show the assumptions used.

This is an agreed, explicit simplification of spatial rules. Mechanics that
still cannot be resolved remain blocked; it does not authorize silently
ignoring other unsupported behavior. Detailed assumption fields and their
application to individual mechanics remain to be specified.

### Local and reusable actions

A creature can define its own actions in its own note. A creature's
attack or reaction does not need a separate note. Reusable definitions, such as
spells, can be separate entries referenced by multiple creatures.

The local keys used by inline controls refer to these structured definitions.
Store local actions under `splat_actions`, with stable local keys and display
names. Changing a display name does not change its key.
Detailed action, cost, trigger, and effect schemas remain open.

### Staged action resolution

Abilities resolve in stages, with explicit reaction opportunities and cost
timing. Rules specify when targets are chosen, resources are spent, rolls occur,
and effects apply. Reactions can interrupt at defined points. Each cost
specifies what happens if the ability is interrupted.

These are rule-defined stages, not one universal sequence for all abilities.
Exact timing vocabulary remains open. Simultaneous encounter triggers use the
ordering described above; rules for reaction opportunities still apply.

Reaction effects can be scoped to one attack resolution, as in the agreed
[Protect Ally example](creature-example.md). That homebrew feature rechecks its
pending-hit eligibility. General 2024 reactions happen immediately after their
trigger unless the feature says otherwise; do not erase completed triggers
merely because later state changes. Nested reactions resolve
at the point permitted by their rules before the interrupted resolution resumes.
Do not offer the same event/ability opportunity repeatedly through duplicate
processing. A configurable resolution limit stops unbounded chains with a
diagnostic and an incomplete trial, rather than silently skipping reactions.
The [source-backed action-resolution contract](action-resolution.md) governs
these distinctions, controller choices and rule-specific expenditure outcomes.

Resource costs use a list whose entries identify `from` (the acting participant
or the source item), `resource`, `amount`, and `at` (the payment stage).
An action can incur multiple costs. Source-item costs bind to the particular
item granting the action, including its separate named charge pools.

Simple dice expressions use readable strings such as `1d8 + 2`, parsed into
restricted dice operations. Conditions and branching remain structured YAML.
Named engine operations resolve attacks, saves, damage, and healing using the
campaign's rules and exposing the applicable reaction opportunities.

### Choices and rule resolution

Rules determine which choices are allowed and resolve their outcomes. A person
or simulation strategy selects a choice. For example, the rules determine when
a guard can protect an ally; the decision to spend the reaction belongs to the
person or strategy controlling that guard.

Changing a simulation strategy does not change the creature's rules.

The initial simulator uses named strategies with configurable priorities per
participant. Examples include preferred targets, when to heal, and when to spend
limited resources. More advanced planning is deferred.

### Simulation results and saved seeds

Initial simulation results contain win rates, encounter length, remaining HP,
and resource use across trials. Individual trials have a readable event log
and a seed so they can be repeated with the same inputs and engine version.
Full results and event logs remain in memory in the initial scope.

Retain seeds to support regeneration. A seed initializes the random choices;
it does not contain the starting state, content definitions, or decisions made
outside the simulator. Exact regeneration requires the same resolved starting
state, content, rules, roster order, strategies, assumptions, and compatible
engine behavior. All automatic random choices and resolution order must be
deterministic for those inputs.

Because source data stays live and snapshots are not saved, a later change to
a D&D Beyond character or local content may prevent exact regeneration. A seed
still permits another trial with current inputs, but that is a new evaluation.
Manual play would also require the recorded decisions to reproduce its history;
a saved seed alone cannot replay those choices.

Save a small seed record containing the seed, encounter or sequence reference,
engine version, and fingerprints of the resolved simulation inputs. Splat uses
these to detect changed inputs before rerunning. Fingerprints detect changes;
they do not store or restore the original data. Seed records contain replay
metadata without saved run state or results.

Readable Markdown filenames, stable UUIDs in frontmatter, and references with
optional Obsidian navigation links are agreed in
[Markdown storage](storage.md). Exact field names and the seed record location
remain to be specified there.

## Working example

Use a homebrew guard with an attack, a three-use healing ability, and a reaction
that protects an ally to test the relationships. Give the guard three identical
healing potions and two wands with different remaining charges to test inventory
quantities and individual item state. This is a design example, not a claim
about an official D&D stat block.

The user has delegated example details and requested a creature representative
of D&D 2024 rule variants. The guard is now the base Test Warden, with named
variants and companion scenarios in [Rules coverage](rules-coverage.md). That
document supplies mechanic families, source checks, and expected observations.
Use it to expose missing model capabilities rather than asking the user to
approve each example. It does not authorize implementation or add spatial
simulation to the scope.

## Design tree and remaining work

- Common combat model, including inventory and consumables: agreed.
- Encounter template/run separation: agreed.
- Creature-local actions and references to reusable definitions: agreed.
- Combat choices separate from rule resolution: agreed.
- Item definitions separate from inventory quantities and individual item state:
  agreed.
- Named resource pools, shared ability costs, and recovery rules: agreed.
- Active effects with origins and end conditions, separate from immediate HP
  changes: agreed.
- Run inputs fixed in memory at the start: agreed.
- Stacking keys separate from broad category tags: agreed.
- Retain overlapping applications with their causes and continuing durations:
  agreed.
- Group effects from one casting or ability use, retaining each effect's target
  and end condition: agreed.
- Shared application state and individual target effect state: agreed.
- Rules explicitly specify shared and individual rolls: agreed.
- Applications support changing targets and rule-defined effect lifetimes:
  agreed.
- Staged action resolution, explicit reaction opportunities, and rule-defined
  cost timing and interruption behavior: agreed.
- Equipment-granted actions and effects follow item requirements and retain a
  reference to the carried item: agreed.
- Each stat explicitly uses a supplied or calculated value, with included
  contributions identified when known: agreed.
- Copies preserve stacking identity unless explicitly declared a distinct
  mechanic: agreed.
- Encounter rosters can use party members, creatures, or both, with an optional
  party binding for reusable templates: agreed.
- Explicit encounter sequences retain state and include elapsed time and rests;
  independent trials start from the same initial state: agreed.
- Triggered encounter phases can bring participants into or out of combat:
  agreed.
- Phases have independent triggers and can overlap; triggers fire once unless
  repetition is explicitly enabled: agreed.
- Returning participants retain their identity and state; time and effect
  durations continue, with no automatic rest on departure: agreed.
- Named simulation strategies with configurable priorities per participant:
  agreed; advanced planning deferred.
- Mapping and spatial simulation: explicitly deferred.
- Explicit targeting assumptions are shown with simulation results; unresolved
  mechanics still block affected simulations: agreed.
- Explicit completion conditions account for required future phases, with
  manual encounter endings also supported: agreed.
- Configurable initiative methods include individual and grouped initiative;
  entry and return do not grant extra turns: agreed.
- Absent participants retain required timing checks and take voluntary actions
  only when explicitly scheduled: agreed.
- Simultaneous triggers use applicable rules, explicit priority, then stable
  initiative and roster order as the default fallback: agreed.
- Encounter sides are explicit and independent of party membership or creature
  type; multiple opposing sides are supported: agreed.
- End reasons and outcomes are separate; limits yield incomplete results shown
  separately from wins and losses: agreed.
- Initial reports contain win rates, duration, remaining HP, resource use,
  individual logs, and seeds: agreed.
- Save authored setups; keep run state, results, and event logs in memory;
  saving and resuming runs is deferred: agreed.
- Seed records contain seeds, setup references, engine versions, and input
  fingerprints to detect changed inputs without storing them: agreed.
- Readable Markdown filenames and UUID references with optional navigation
  links: agreed in [Markdown storage](storage.md).
- Next work:
  - Refine the model against [the representative coverage cases](rules-coverage.md),
    including revised Multiattack, extra reactions, spell costs, and monster
    exceptions. Example choices are delegated; no per-example approval needed.
- Subsequent detail:
  - Remaining container state, capacity units, and validation in
    [the container design](containers.md).
  - File layout, seed record location, and concrete entry schemas.
  - Detailed targeting assumptions and their interaction with mechanics.
  - Potency comparison and policies for individual mechanics.
  - Detailed action timing, targeting, durations, and equipment transitions.
  - Entry fields and references, including character identity and provenance.
  - Party state between encounters and detailed remote-data boundaries.
  - Concrete Markdown/YAML examples and the inline control grammar.
- Deferred:
  - Saving and resuming run state, results, and event logs.
  - Maps, creature tokens, coordinates, terrain, cover calculation, and spatial
    simulation.
  - Advanced tactical planning.

Open questions are not agreed behavior or implementation instructions.
