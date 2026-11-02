---
splat_schema: 1
splat_id: d926ea81-ff38-59bb-ae90-7c055f174fb9
splat_kind: rule
splat_key: dnd2024:rules/warlock_pact_of_the_chain
splat_name: Pact of the Chain
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Pact of the Chain

## Source text

<!-- SRD page 74 -->

You learn the Find Familiar spell and can cast it as a Magic action without expending a spell slot.

When you cast the spell, you choose one of the normal forms for your familiar or one of the following special forms: Imp, Pseudodragon, Quasit, Skeleton, Sphinx of Wonder, Sprite, or Venomous Snake (see “Monsters” for the familiar’s stat block).

Additionally, when you take the Attack action, you can forgo one of your own attacks to allow your familiar to make one attack of its own with its Reaction.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_pact_of_the_chain ` |
| ` implementation ` | ` dnd2024:rules_warlock_pact_of_the_chain ` |

### Parameters

#### Learn spell

<!-- splat:values path=splat_rule/parameters/learn_spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/find_familiar ` |

#### Casting

<!-- splat:values path=splat_rule/parameters/casting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` spell_slot_cost ` | ` 0 ` |

#### Familiar forms

<!-- splat:yaml path=splat_rule/parameters/familiar_forms -->

```yaml
allow_normal: true
additional:
- key: dnd2024:creatures/imp
- key: dnd2024:creatures/pseudodragon
- key: dnd2024:creatures/quasit
- key: dnd2024:creatures/skeleton
- key: dnd2024:creatures/sphinx_of_wonder
- key: dnd2024:creatures/sprite
- key: dnd2024:creatures/venomous_snake
```

#### Attack exchange

<!-- splat:values path=splat_rule/parameters/attack_exchange -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.attack_action ` |
| ` forgo_attacks ` | ` 1 ` |
| ` familiar_attacks ` | ` 1 ` |
| ` familiar_economy ` | ` reaction ` |
