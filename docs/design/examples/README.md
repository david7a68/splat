# Worked content notes

These are complete notes for the candidate schema in [Schema draft](../schema-draft.md).
They are documentation, not registered campaign content or implemented fixtures.
Do not load them into the current app. The folders illustrate ownership only;
real package/campaign roots and filenames remain configurable.

- [Test Warden](library/Test%20Warden.md): stats, inventory, pools, attack, healing,
  reaction and a referenced spell.
- [Ember Wave](library/Ember%20Wave.md): homebrew spell with shared damage and
  individual saves and a caller-bound slot pool.
- [Dual Wand](library/Dual%20Wand.md), [Healing Draught](library/Healing%20Draught.md):
  two item pools and a quantity-consuming Bonus Action.
- [Supply Pack](library/Supply%20Pack.md): reusable container setup; all supporting
  equipment definitions live beside these notes.
- [Training Exchange](campaign/Training%20Exchange.md): four independent Warden
  placements, declared non-spatial assumptions and manual completion.
- [Encounter trace](../encounter-trace.md): reproducible design decisions and
  controlled dice, with state changes at each step.
- [Reactive Warden](library/Reactive%20Warden.md) and
  [Controller Warden](library/Controller%20Warden.md): independent copies with a
  restricted reaction budget and source-linked, Concentration-dependent fear.
- [Reinforcement Drill](campaign/Reinforcement%20Drill.md) and
  [Two Exchanges](campaign/Two%20Exchanges.md): phased arrival and sequence bindings.

The completed [mechanics](../mechanics-schema.md), [state](../state-schema.md),
and [campaign](../campaign-schema.md) contracts define the added fields.
[Schema checks](../schema-checks.md) gives expected outcomes without claiming
these notes have been executed by a simulator.

The Library manifest owns `my_homebrew`; the Campaign manifest owns `test_campaign`.
The Warden's original UUID and resource key are retained. Other UUIDs identify
only these examples and must not be mistaken for installed content.

The selected core contract is D&D 2024. Referenced `dnd2024:*` operations,
predicates and mechanics must be supplied by a future core rule package and
engine: attack/save/healing, ordinary action economy, armor rebuild, resting,
spell commitment and components, inventory access and transfers, and stacking.
`splat:*` names identify generic engine operations, never code in a package.
All entry references resolve within these examples; required engine/rule
contracts are explicitly external and currently unimplemented.

Homebrew numbers and abilities are chosen to expose model behavior. In
particular Strike deals 1d8 + 3 independently of the ordinary spear damage; it
requires a wielded spear but is a named homebrew ability, not the default weapon
attack. Ember Wave is not an official spell. The training pack capacities are
fixture values, not claims about published equipment.

The draft extends the base Warden with spellcasting and its two-slot pool so one
trace can exercise reusable spells. It is not a class-legal character. The other
named variants are examined in [the coverage audit](../schema-coverage.md).
