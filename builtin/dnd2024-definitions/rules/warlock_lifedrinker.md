---
splat_schema: 1
splat_id: bca7b937-6363-525d-bcf1-41168424e5e2
splat_kind: rule
splat_key: dnd2024:rules/warlock_lifedrinker
splat_name: Lifedrinker
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Lifedrinker

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 9+ Warlock, Pact of the Blade Invocation

Once per turn when you hit a creature with your pact weapon, you can deal an extra 1d6 Necrotic, Psychic, or Radiant damage (your choice) to the creature, and you can expend one of your Hit Point Dice to roll it and regain a number of Hit Points equal to the roll plus your Constitution modifier (minimum of 1 Hit Point).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_lifedrinker ` |
| ` implementation ` | ` dnd2024:rule_warlock_lifedrinker ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` hit_creature_with_pact_weapon ` |
| ` uses_per_turn ` | ` 1 ` |

#### Prerequisite

<!-- splat:values path=splat_rule/parameters/prerequisite -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 9 ` |
| ` invocation ` | ` pact_of_the_blade ` |

#### Extra damage

<!-- splat:yaml path=splat_rule/parameters/extra_damage -->

```yaml
amount:
  dice:
    count: 1
    sides: 6
type_choice:
- necrotic
- psychic
- radiant
```

#### Optional heal

<!-- splat:yaml path=splat_rule/parameters/optional_heal -->

```yaml
spend_hit_dice: 1
amount:
  op: max
  args:
  - 1
  - op: add
    args:
    - variable: spent_hit_die_roll
    - variable: constitution_modifier
```
