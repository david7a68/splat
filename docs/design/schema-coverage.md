# Schema coverage audit

Status: candidate field/operation contracts now exist for all 34 agreed families.
This is a design-completion audit, not a report of implemented rules or passing
engine tests. The earlier Gap/Represented list is resolved into these contracts.

- [Mechanics](mechanics-schema.md): types, expressions, rolls, operations, costs,
  budgets, effects, Concentration, spell methods and mastery.
- [State](state-schema.md): health, inventory, relationships, forms and lifecycle.
- [Campaign/storage](campaign-schema.md): party, phases, strategies, sequences,
  settings boundaries, replay fingerprints and edit recovery.
- [Resolution rules](action-resolution.md): source-backed timing and recorded
  rulings; [worked checks](schema-checks.md) state expected observations.

| Case | Contract | Validation / coverage condition |
| --- | --- | --- |
| R01 Stats | Mechanics: types/stats; State: participant | One value or derivation, known contributions and typed rebuild |
| R02 D20 tests | Mechanics: rolls and tests | Typed attack/check/save and automatic-outcome contracts |
| R03 Advantage | Mechanics: rolls and tests | Source contributions and selected dice; cancellation without source-count voting |
| R04 Modification | Mechanics: rolls and tests | Rule-granted revisions retain original and replacement results |
| R05 Economy | Mechanics: costs and budgets | Distinct economy categories, feature grants and ledgers |
| R06 Turn timing | Mechanics: budgets; Campaign: schedule | Refresh and limits identify event and subject; group members retain turns |
| R07 Attacks | Mechanics: operations | Attack/save, hit/damage and on-miss branches are distinct |
| R08 Multiple attacks | Mechanics: composition | Ordered resolution, bounded repeat and granted substitutions |
| R09 Mastery/properties | Mechanics: weapon mastery | All eight properties specify source constraints, access and limits |
| R10 Damage | Mechanics: damage | Typed components, event grouping and source-specific adjustments |
| R11 Arithmetic | Mechanics: damage; Schema checks | Published order and intermediate rounding |
| R12 Saves | Mechanics: tests/branching | Separate save subjects, ability choice and shared roll scope |
| R13 Survival | State: health | Temp HP, zero-HP policy, death saves, knockout and revival |
| R14 Conditions | Mechanics: effects | All condition keys with source-linked contributions |
| R15 Lifetime | Mechanics: duration | Elapsed/event expiry, dependent cleanup and suppression clocks |
| R16 Casting | Mechanics: spell invocation | Slot/innate/item/ritual methods, components and scaling |
| R17 Interrupts | Action resolution; Mechanics: costs | Counterspell, Ready and longer casting retain distinct expenditure outcomes |
| R18 Concentration | Mechanics: concentration; State: relations | Owner/use/target dependencies and explicit ending reasons |
| R19 Stacking | Mechanics: stacking | Whole application comparison per target and retained suppression |
| R20 Reactions | Action resolution; Mechanics: events | Completed trigger versus pending result; nested choices and stable event IDs |
| R21 Grapple | State: relationships | Body part, source, range, escape and end policies |
| R22 Recovery | Mechanics: recovery; Campaign: sequences | Multiple recoveries, recharge and explicit completed-rest events |
| R23 Items | State: inventory | Item source, hands, attunement, depletion and transfer commands |
| R24 Containers | State: inventory; Campaign: placements | Exact units, standalone setups, one holder and no cycles |
| R25 Monsters | Mechanics: special outcomes; State: health | Typed rule-granted replacement, absorption and reflection |
| R26 Extra timing | Mechanics: budgets; Action resolution | Hydra next-turn ruling, Marilith frequency, legendary uses and lockouts |
| R27 Forms | State: forms | Source-specific retained stats/equipment and revised temporary-HP policies |
| R28 Targeting | Mechanics: predicates/choices; Campaign: facts | Visibility, unknown versus hidden facts, relation and target kinds |
| R29 Phases | Campaign: phases/completion | Required arrivals, recurrence, manual outcomes and incomplete trials |
| R30 Replay | Campaign: fingerprints | Canonical semantic inputs, engine/RNG identity and deterministic ordering |
| R31 Movement | Campaign: assumptions | Explicit spatial facts only; geometry/pathfinding deferred |
| R32 Other actions | Mechanics: operations; Campaign: strategies | Typed action/check contracts with no prose execution |
| R33 Identity exceptions | State: relationships/lifecycle | Possessor, host, controller, split actors and swallowing retain identities |
| R34 Fixed damage/invalidity | Mechanics: rolls/casting/choices | Fixed damage and invalid-target apparent outcomes without secret leakage |

## Completion and scope

The notes include Reactive Warden and Controller Warden, Reinforcement Drill
and Two Exchanges alongside the original 13 notes. They exercise budgets,
source-relative effects, shared Concentration, phases and state carried between
encounters. Schema checks add hand-worked observations and failure cases.

Remaining work is review, implementation and validated rules/content, not an
unnamed schema branch. Individual official features must map to supported
contracts and retain their exact exceptions. Unknown or conflicting rules still
block affected simulation. A novel source interaction may need an explicit
rule profile or future schema revision; this does not claim every sourcebook
is already encoded or every simulator test passes.

Exclusions remain: maps/geometry, saved run state/results/logs, package updates
or distribution, arbitrary package code, and automatic interpreted readied
spells. Readied spells remain tracked in manual encounters. Live D&D Beyond
integration requires a verified adapter and stores mappings only.
