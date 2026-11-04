---
splat_schema: 2
splat_id: a3652a77-77b0-5fce-9a0a-ecff179361c8
splat_kind: rule_group
splat_key: dnd2024:species/halfling
splat_name: Halfling
splat_origin: dnd2024:reference/pages/086
splat_sources:
- SRD 5.2.1, pp. 86, https://www.dndbeyond.com/srd
---

# Halfling

## Source text

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Small (about 2–3 feet tall)\
Speed: 30 feet

As a Halfling, you have these special traits.

Brave. You have Advantage on saving throws you make to avoid or end the Frightened condition.

Halfling Nimbleness. You can move through the space of any creature that is a size larger than you, but you can’t stop in the same space.

Luck. When you roll a 1 on the d20 of a D20 Test, you can reroll the die, and you must use the new roll.

Naturally Stealthy. You can take the Hide action even when you are obscured only by a creature that is at least one size larger than you.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/halfling ` |
| ` implementation ` | ` dnd2024:species_halfling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` size ` | ` small ` |
| ` speed_feet ` | ` 30 ` |

#### Height feet

<!-- splat:values path=splat_rule/parameters/height_feet -->

| Field | Value |
| --- | --- |
| ` typical_minimum ` | ` 2 ` |
| ` typical_maximum ` | ` 3 ` |

#### Brave

<!-- splat:yaml path=splat_rule/parameters/brave -->

```yaml
save_advantage:
  condition: frightened
  purposes:
  - avoid
  - end
```

#### Nimbleness

<!-- splat:yaml path=splat_rule/parameters/nimbleness -->

```yaml
move_through_creature_space:
  minimum_size_difference: 1
stop_in_occupied_space: false
```

#### Luck

<!-- splat:values path=splat_rule/parameters/luck -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.d20_test.d20_is_1 ` |
| ` optional_reroll ` | ` true ` |
| ` must_use_new_roll ` | ` true ` |

#### Naturally stealthy

<!-- splat:yaml path=splat_rule/parameters/naturally_stealthy -->

```yaml
hide_obscurement_qualifier:
  only_obscured_by_creature: true
  minimum_size_difference: 1
```
