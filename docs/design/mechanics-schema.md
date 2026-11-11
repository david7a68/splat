# Mechanics schema

Status: completed candidate field contract for the note model, under the user's
delegation of schema choices. This is documentation, not an engine or Rust
implementation. It extends [the draft](schema-draft.md); rule behavior follows
[Action resolution](action-resolution.md). [State](state-schema.md) and
[campaign/storage](campaign-schema.md) complete the other contracts.

## Types and references

All maps reject duplicate keys. A local key matches `[a-z][a-z0-9_-]*`. An EntryRef
contains exactly one of `id` or `key`, optionally `link`. A LocalRef is
`{local: key}`; an external definition selector is
`{entry: EntryRef, member: key}`. The containing field identifies whether it
expects an action, effect, form, strategy, or resource. A local definition and
its runtime instance are never interchangeable.

An expression is a scalar literal, `{literal: string}`, `{binding: key}`,
`{stat: {of: Subject, key: stat_key}}`, `{context: registered_field}`,
`{result: {step: key, field: registered_output}}`, or
`{op: registered_pure_op, args: [Expr, ...]}`. Bare strings in expression fields
are forbidden except the dice syntax in a roll definition; named identifiers
in typed selector fields are not expressions. JSON-compatible finite numbers
only; use rationals `{numerator: integer, denominator: positive_integer}` for
damage scaling and exact unit conversions. No NaN/infinity or YAML timestamps.

Subject is `actor`, `source_item`, `target` (within a target iteration), a declared
binding, or `{context: registered_field}`. `source_item` requires an item-granted
use. Context fields include `event.actor`, `event.target`, `event.resolution`,
`event.use`, `event.damage`, `event.turn`, `event.round`, `event.relationship`,
`event.application`, `application.source`, and `application.target`. Contexts
also declare fields such as `application.target_effect` and `use.category`
where appropriate. Registered dotted names can be used directly in typed
Subject fields as shorthand for `{context: name}`. Each event
declares which fields exist; reading an absent field is a type/validation error,
not a null default. An operation's outputs are available only after that step.

Pure operators are `add`, `subtract`, `multiply`, `floor`, `min`, `max`,
`ability_modifier`, comparisons `eq/lt/lte/gt/gte`, and Boolean `all/any/not`.
Divide through an explicit rational scale, with rounding stated by the rule.
No random draws, participant mutation or hidden rule execution in pure expressions.

A Predicate is `{predicate: registered_name, ...typed_parameters}`. Initial
predicate families: relation (`subject,to,value`), identity (`left,right`),
equipment_state (`subject|row,state`), free_hands (`subject,minimum`), accessible
(`subject`), within_reach (`from,to,feet`), seen/heard (`observer,subject`),
creature_type/size (`subject,allowed`), willing (`subject,for_use`), has_condition
(`subject,condition`), and test (`value: BooleanExpr`). `all`, `any`, `not` compose
predicates with three-valued known-true/known-false/unknown logic. Unknown facts
must be supplied or marked unresolved; they do not silently pass or fail.

## Rolls and tests

RollDef has `expression`, optional numeric `modifier` expression, and required
`scope: use|target|attack`. Dice strings accept integer constants and `NdM` terms
joined by `+` or `-`, with whitespace; N and M must be positive bounded integers.
Examples: `1d8 + 2`, `2d6`, `1d20`. Dice count <= 1000, die sides <= 1000000,
expression length <= 4096 are content-validation limits, not game limits.
Exceeding them is an explicit unsupported draft, never truncated dice.
For dynamic scaling use `dice: {count: IntExpr, sides: integer, add: Expr}`
instead of `expression`; exactly one is permitted.

`splat:test` has `kind: attack|save|check`, `subject`, `roll`, and `against`
(DC expression or stat selector). Ability, proficiency and modifiers come from
the selected `rule` and explicit parameters; never add proficiency twice to a
supplied total. The D&D adapter differentiates attack criticals, automatic save
failure, checks, and death saves. Its outputs are `total`, `natural`,
`success`, `critical`, and `roll_id`; outputs not meaningful for a test kind
are unavailable, not fabricated. A save may specify `ability_choice: [strength,
dexterity]` and `chooser: target` rather than one fixed ability.

Advantage contributions retain source IDs and are combined by the D&D rule.
Reroll/replace is `{operation: splat:revise_roll, roll: ResultRef, mode:
reroll|replace, dice: all|selected|index, value?: Expr, rule: mechanic}`. The rule
must grant that revision at that boundary. Keep old and new dice and which is
used; repeated data delivery is not a new reroll. A fixed monster damage value
uses `amount: number`; `damage_mode: rolled|fixed` selects one definition where
both are supplied. Fixed damage under a critical requires the selected damage
rule's stated treatment; absent treatment blocks that case rather than silently
ignoring extra dice.

## Operations and branching

Every step has a unique `key` within its enclosing action, an `operation`, and
typed inputs. Keys are unique even in nested steps, so output references are
unambiguous. `when` is an optional predicate evaluated at that step's legal
boundary. A skipped step has no output. `on_success` and `on_failure` are ordered
step lists supported by tests/saves. A successful save does not automatically
negate every associated effect; the actual branches specify the results.

| Operation | Inputs | Outputs / permitted state change |
| --- | --- | --- |
| `splat:choose` | `chooser`, `selection: TargetSelector`, `bind` | A typed selected target list; player/strategy decision, not RNG by default |
| `splat:sequence` | `steps` | Ordered child resolutions |
| `splat:repeat` | `count: bounded IntExpr`, `steps` | Separate iteration scopes; no extra parent economy cost |
| `splat:test` | Test fields above | Roll and outcome, no automatic damage |
| `dnd2024:attack` | `kind`, `target`, `roll`, optional `weapon_row`, `on_hit`, `on_miss` | Attack result and source-defined reaction boundaries |
| `dnd2024:save_damage` | `target`, `ability` or `ability_choice`, `dc`, `damage`, `success.damage_scale` | Independent saves; scaled damage per target; existing compact fixture syntax |
| `splat:damage` | `target`, `components`, `rule`, `source`, optional `event_group` | Damage event with evaluated adjustments and HP outcome |
| `dnd2024:heal` | `target`, `amount`, optional `source` | Actual HP restored after restrictions, never above effective maximum |
| `splat:temporary_hp` | `target`, `amount`, `rule`, optional `source_application` | A chosen temporary-HP grant, separate from HP |
| `splat:apply_effect` | `target`, `effect: LocalRef|DefinitionSelector` or local key shorthand, optional `bindings` | One application plus its target records; captured bindings feed future effect triggers |
| `splat:end_effect` | `application` or `target_effect`, `reason` | Ends exactly the selected scope; invokes its dependent cleanup |
| `splat:revise_outcome` | `resolution`, `field`, `value`, `rule` | A rule-authorized replacement, e.g. failed save to success; original retained |
| `splat:cast` | `spell: EntryRef`, `method`, `level`, `bindings`, target selection | Full casting pipeline and cost outcomes; not an ordinary nested action debit |
| `splat:change_resource` | `subject`, `pool`, `change: spend|restore|set`, `amount`, `rule` | Atomic pool change and causal ledger event |
| `splat:inventory` | `command` and typed command inputs in State schema | Item/holder/equipment/quantity transition |
| `splat:relationship` | `command: create|end`, typed relation contract | Source/target relationship lifetime |
| `splat:form` | `command: enter|revert`, form selector, subject | Source-specific form change on same actor |
| `splat:participant` | `command: spawn|depart|return|replace`, fields in State schema | Roster/actor lifecycle change |
| `splat:health` | `command: stabilize|death_save|kill|revive|knockout`, `subject`, `rule`, parameters | Health transition governed by the selected rule |
| `splat:emit` | Registered `event`, typed payload | Explicit authored event only; cannot forge core attack/save/cost events |

Registered D&D operations can be deep operations: e.g. the attack adapter owns
its published ordering, avoiding duplication across every creature note. Its
contract must still declare inputs, outputs, permitted events and source. An
unregistered or unimplemented operation blocks use; a name is not implementation.

Attack substitutions are `splat_attack_options` entries with `key`, `granted_by`
(DefinitionSelector), `when`, and `replace: {count, with: resolution_steps}`.
Choose them within the parent Attack action, spend the replaced attack allowance,
and charge only explicitly additional costs. Do not invoke a whole full-cost
action merely to reuse its damage. Multiattack's allowed attack sequence is
source-defined, including revised Multiattack as part of the Attack action.

## Events, choices and costs

EventDef is a registry contract with `name`, typed `payload`, and allowed
`boundary: before|during|after`. Authors bind to existing rule events; a different
boundary requires a feature that grants it. Core event families are turn/round
start/end, attack roll/hit/damage, save before/after, casting start/end/interrupted,
resource change, damage applied, HP transition, condition/application end,
relationship end, equipment/holder change, participant entry/departure, and
completed rest/elapsed time. Each carries a unique deterministic event sequence,
cause, parent resolution, clock, and any applicable source/target.

A trigger has `event`, `boundary` (default after), optional `subject`,
`predicates`, and `occurrences: once_per_event|repeat_by_rule`. An offered choice
records event, eligible actor/feature instance, permitted window, and status
`offered|accepted|declined|expired`. `accept` rechecks current permission but does
not erase an already-completed trigger. Protect Ally's pending-hit eligibility
is explicitly feature-specific. Turns' controllers decide simultaneous ordering;
stable priority/initiative/roster defaults apply only where permitted.

Existing cost fields remain; add `kind: resource|quantity|budget` when required
to disambiguate. A budget cost uses `budget: local_key`. Each ledger entry records
`declared`, `reserved`, `expended`, `released`, `not_expended`, or `recovered`,
amount, subject instance, use, rule and event. Only expended events count against
the one-slot-per-turn rule. Internal reservation must not introduce a new
interrupt window. Interruption dispositions are selected by the rule operation,
not a catch-all refund flag. A rule's `not_expended` outcome never restores
unrelated costs. The earlier optional `refund` field is retired in favor of
these rule-owned outcomes.

ChoiceSpec has `kind: target|option|order|amount|accept`, `chooser`, `allowed`,
`default_strategy`, and `visibility: controller|gm|public`. An automatic run must
have a strategy for every required choice; otherwise preparation reports the
missing choice support. Readied-spell interpretation is manual-only as agreed.
The run's full state can know an invalid target while the controller sees only
the rules-permitted apparent result. Unknown engine data and hidden-but-known
facts are distinct. Never use secret facts in a restricted strategy's choice.

## Action budgets and limits

`splat_budgets` declares feature-granted permissions; core rule budgets are
injected separately. A BudgetDef contains `name`, `kind`, `capacity: IntExpr`,
`eligible` predicates, `refresh: {event, subject}`, `capacity_at:
refresh|rule_event`, and optional `replaces: mechanic_key`. Replacing a core
permission requires a selected rule/feature that authorizes replacement; it
does not add a second ordinary reaction pool accidentally.

An action's `activation.economy` requests a permission category. Eligible grants
are choices, not all automatically spent. Spending one use of one qualifying
budget pays that permission once. `activation.budget` can bind an exact grant.
`splat_limits` adds per-feature counters `{key, maximum, scope:
turn|own_turn|round|use, at: rule_event, eligible}`. Their identity includes the
grant source; a once-per-turn limit does not reset on actor departure/return.

```yaml
splat_budgets:
  extra_opportunities:
    name: Reactive Heads
    kind: reaction
    capacity:
      op: max
      args: [0, {op: subtract, args: [{stat: {of: actor, key: heads}}, 1]}]
    eligible:
      predicate: test
      value: {op: eq, args: [{context: use.category}, {literal: opportunity_attack}]}
    refresh: {event: "dnd2024:turn.start", subject: actor}
    capacity_at: refresh
```

The example uses the user's explicit next-turn Hydra ruling, not an asserted
printed head-change algorithm. `use.category` is available in budget eligibility
contexts. Marilith instead replaces ordinary reaction frequency through its
registered trait, with a one-use refresh on every turn; it does not get unlimited
responses within a turn. Legendary actions use their own budget, an after-other-
turn timing selector and per-option lockouts, with Incapacitated restrictions.

Recovery extends the existing list with `subject`, `when`, and optional
`rolls`. `restore.add` can reference a local recovery roll. At most one entry
per exclusive group is chosen for an event; independent entries all run. Recharge
uses a `turn.start` rule check plus short/long-rest recovery. X/Day binds to Long
Rest per the source, not midnight. Calendar recovery, if authored, requires a
campaign clock event and explicit recurrence. Failed/interrupted rest is not a
completed-rest event. Item `root_holder` recovery remains as explicitly authored.

## Effects, durations and Concentration

Effects may be action-local, under entry-wide `splat_effects`, or in a reusable
`splat_kind: effect` note with `splat_effect`. References always select the
definition; applications get separate IDs. EffectDef has `name`, `stacking`,
`duration`, optional typed `parameters`, `conditions`, `modifiers`, `triggers`, `relationships`,
`concentration`, and `remove_when`. No duplicate prose mechanics.

```yaml
splat_effects:
  fearful:
    name: Frightened of the source
    parameters:
      save_dc: {type: integer, minimum: 1, required: true}
    stacking:
      key: "my_homebrew:fearful"
      policy: "dnd2024:same_effect"
      compare: {potency: 1, equal: newest}
      scope: application_per_target
    duration:
      any:
        - {elapsed: {seconds: 60}}
        - {event: "dnd2024:concentration.ended", subject: application.source}
    conditions:
      - {mechanic: "dnd2024:conditions/frightened", source: application.source}
    concentration: {owner: application.source, required: true}
    triggers:
      resist:
        event: "dnd2024:turn.end"
        subject: application.target
        steps:
          - key: save
            operation: "splat:test"
            kind: save
            subject: application.target
            ability: wisdom
            against: {binding: save_dc}
            rule: "dnd2024:saving_throw"
            on_success:
              - key: end
                operation: "splat:end_effect"
                target_effect: {context: application.target_effect}
                reason: successful_save
```

For `splat:test` with `rule: dnd2024:saving_throw`, `ability` and `against` are
enough: the adapter supplies its named D20 roll from the subject's stats.
An explicit roll overrides only a rule-authorized roll definition, not automatic
failures. Effect triggers use their own action-like parameter bindings and
step scope. Bindings such as `save_dc` are captured when applying the effect.

Duration is a tagged expression: `{elapsed: {seconds}}`,
`{event, subject, occurrence: next|positive_integer}`, `{until: resolution_end,
of: event.resolution}`, `{permanent: true}`, or `{any: [Duration,...]}`.
Event-based expiry is measured from application creation; the same event that
created it is excluded unless `include_current: true` is stated by its rule.
Subjects bind to instance IDs at creation. `remove_when` predicates are evaluated
on declared `observe` events, never uncontrolled continuous polling. The record
can remain suppressed while its independent clocks/checks continue.

Modifiers declare `selector` (stat, test, damage, eligibility or budget),
`operation: add|scale|set|grant|deny`, typed `value`, `when`, and optional scope.
Legacy `{stat, operation, value, scope}` modifier syntax in Protect Ally is the
stat-selector shorthand. Rule-defined precedence resolves replacement formulas,
immunity, Advantage and numeric changes; author list order is not a universal
stacking winner. A comparison policy must return an ordering or a declared
controller choice; incomparable applications cannot be silently sorted by UUID.

Stacking policy inputs are source/use IDs, target, stacking key, potency data,
timestamps and applicable rules. Return contributing/suppressed applications,
not deleted records. `scope: application_per_target` compares whole applications
at their overlapping targets; it cannot cherry-pick individual benefits from
the same spell. `compare` is required only when the rule cannot derive potency.
Copies preserve stacking keys unless explicitly made a distinct mechanic.

All 15 condition keys are registry contracts: blinded, charmed, deafened,
exhaustion, frightened, grappled, incapacitated, invisible, paralyzed, petrified,
poisoned, prone, restrained, stunned, unconscious. Condition applications keep
their causes; repeated ordinary conditions do not worsen the condition.
Exhaustion instead has a level counter with source-aware changes. Conditions
that imply other conditions create dependent contributions, not independent
permanent flags. Ending Unconscious removes its own linked contributions but
does not erase an independently persisting Prone condition. Condition/damage
immunity is tested by the applicable rule, not a broad tag match.

Concentration is one owner relationship linking a participant to a use or held
spell and its dependent effects. Beginning another Concentration effect ends
the old relationship at the rule's start boundary. Damage creates source-
appropriate saves with the D&D DC rule; failure ends dependents, not every
unrelated effect on every target. Incapacitated, death, voluntary ending and
expiry use explicit reasons. A per-target end need not end the whole use.
Readied spells keep their manual trigger/target record and expire by the agreed
rules; they do not become an automated strategy through this schema.

## Spell invocation contracts

`splat_spell` contains required `level: 0..9`, `school`, `components`,
`concentration: bool`, and `duration`; optional `casting_time`, `materials`,
`ritual` and `scaling`. The existing action's activation supplies casting time
when it is one Action/Bonus Action/Reaction. Longer casting uses
`casting_time: {seconds: positive_integer, rule: dnd2024:longer_casting}`.
Components are verbal/somatic/material. Material requirements are an ordered
list `{key, item?: EntryRef, description, cost?: Currency, consumed: bool,
quantity: positive_integer}`; a focus substitutes only where the rule permits.
Currency is exact `{amount: nonnegative_integer, unit: cp|sp|ep|gp|pp}`.

`splat_parameters` fields declare `type`, optional `minimum/maximum`, `required`
and `default`. Types include integer, number, boolean, string enum, actor_resource,
item_resource, entry reference and participant selector. A missing required
binding is an error. Bounds constrain legal bindings, not a way to grant spell
level eligibility. Referenced action aliases bind a method explicitly or inherit
the referenced cast action's method. Current Ember Wave uses the slot-pool cost
as the slot-method shorthand.

CastMethod is exactly one of:

- `{kind: slot, pool: PoolSelector, level: IntExpr}`.
- `{kind: innate, uses: PoolSelector|at_will, level: IntExpr}`.
- `{kind: item, item: Subject, costs: [Cost,...], level: IntExpr}`.
- `{kind: ritual, rule: mechanic, level: IntExpr}`.

The grant defines eligibility and component exceptions in `requirements`; method
alone never waives components. Cantrips use a slot-free innate/at-will method
with level zero or their registered class grant. `scaling` maps local roll or
parameter keys to expressions in `cast_level`, `actor_level`, `class_level` or
other declared bindings. Dynamic dice use RollDef's structured count. Each
spell states which scaling source applies; an upcast level and total character
level cannot be substituted for one another.

Casting exposes rule-owned start, eligible interrupt, completion, held-energy
and expenditure events. Cost provenance includes spell level, slot source and
turn. One-slot expenditure, invalid targets, component consumption, Counterspell,
longer-cast failure and Ready use the source-backed contract. Automatic invalid
target display can show apparent save success without drawing a save die.
Readied casting stores manual declared targets or unresolved choices separately
from release, never an automatic rerun of the casting action.

## Weapon mastery contracts

Item `weapon.mastery` identifies a property, not access to it. A grant under
`splat_traits` or a referenced feature authorizes using it with specified weapons.
All eight adapters consume ordinary typed hits/damage/rolls, target relations,
limits and effects from this schema. Their source constraints are:

| Mastery | Trigger, operation and limits |
| --- | --- |
| Cleave | Melee attack roll hits a creature; optional attack with same weapon against a second creature within 5 feet of the first and within attacker reach. Once per turn; omit ability modifier from extra damage unless negative |
| Graze | Weapon attack roll misses a creature; optional damage equal to the attack ability modifier, weapon damage type; only increasing that modifier can increase this damage |
| Nick | Light-property extra attack moves into Attack action instead of Bonus Action; it remains the same once-per-turn extra attack, not a second one |
| Push | On hit, optional push up to 10 feet straight away; target Large or smaller; no save in this property |
| Sap | On hit, Disadvantage on target's next attack roll before start of attacker's next turn |
| Slow | Hit and damage; optional Speed reduction of 10 feet until start of attacker's next turn; repeated Slow-weapon hits do not increase the reduction beyond 10 |
| Topple | On hit, optional Constitution save, DC 8 + attack ability modifier + proficiency bonus; failure Prone; no property-specific size cap |
| Vex | Hit and damage; Advantage on attacker's next attack against that creature before end of attacker's next turn |

These use distinct `hit` and `damage_applied` events. Expiring 'next roll' grants
is consumption at that qualifying roll, plus an expiry deadline; a general
one-minute effect is not equivalent. Push/Cleave use declared spatial facts,
otherwise the affected choice is unresolved. Registered adapters cannot infer
geometry. Core item properties (finesse, light, heavy, thrown, loading, ammunition,
versatile and two-handed) expose typed weapon parameters and eligibility, including
ability requirements for revised Heavy and exact equipment interaction rules.

## Damage and special outcomes

Damage component input is `{key, type, amount, source?, when?, critical?}`.
`key` is optional when the component is never referenced individually; an
unkeyed component keeps its ordered position in the enclosing definition.
Keep raw, scaled and adjusted values with rule/source annotations in the event.
The D&D adapter applies modifiers, resistance and vulnerability in the published
order, with rounding at the specified points. Identical resistance sources
do not halve damage repeatedly. Save-half and resistance remain separate
operations. One source's mixed components are not automatically many separate
damage events or Concentration checks; the feature/source defines grouping.

Damage type selectors can be a finite list or `all`, plus typed predicates on
the attack/spell/source, rather than legacy blanket assumptions about magic.
An event's `source` links to the originating action use; reflected damage or a
reflected spell carries both original cause and new rule-defined target/caster.
Reflection is `splat:revise_outcome` on the pending spell resolution with a
registered `spell_reflection` contract; it is not a second slot-paying cast.

Rules such as Evasion, Legendary Resistance, regeneration suppression, damage
absorption and Undead Fortitude use typed outcome modifiers/triggers and
registered predicates. Their contracts identify the exact pending outcome,
eligibility (damage type, critical, condition, feature use), replacement and
cost. No generic success override is available without a granting rule.
Additional source-specific mechanics fit these slots but must provide supported
contracts before simulation; schema expressibility is not content completeness.

## Verified rule sources

- [SRD 5.2.1](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf), checked 2026-09-15: mastery pp89–90; damage pp16–18; Concentration p179; condition definitions and monster-specific source entries.
- [Weapon mastery](https://www.dndbeyond.com/sources/dnd/free-rules/equipment#MasteryProperties).
- [Damage order](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game#OrderofApplication).
- [Condition stacking](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game#ConditionsDontStack).
- [Concentration](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary#Concentration).

The general Concentration text does not supply a universal subdivision algorithm
for every simultaneous multi-hit effect. Preserve source-defined damage events;
neither one check per damage type nor one check per whole spell is a universal
fallback. An unresolved source interaction remains an explicit rule dependency,
not a missing schema field or an implicit house rule.
