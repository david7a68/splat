---
splat_schema: 1
splat_id: 0304382e-8fd5-501d-b7c9-a25fa0d8e15f
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_1_lay_on_hands
splat_name: 'Level 1: Lay On Hands'
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53, 54
  url: https://www.dndbeyond.com/srd
---

# Level 1: Lay On Hands

## Source text

<!-- SRD page 53 -->

Your blessed touch can heal wounds. You have a pool of healing power that replenishes when you finish a Long Rest. With that pool, you can restore a total number of Hit Points equal to five times your Paladin level.

**Paladin Features**

*Spell Slots per Spell Level*

| Level | Proficiency Bonus | Class Features | Channel Divinity | Prepared Spells | 1 | 2 | 3 | 4 | 5 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | +2 | Lay On Hands, Spellcasting, Weapon Mastery | — | 2 | 2 | — | — | — | — |
| 2 | +2 | Fighting Style, Paladin’s Smite | — | 3 | 2 | — | — | — | — |
| 3 | +2 | Channel Divinity, Paladin Subclass | 2 | 4 | 3 | — | — | — | — |
| 4 | +2 | Ability Score Improvement | 2 | 5 | 3 | — | — | — | — |
| 5 | +3 | Extra Attack, Faithful Steed | 2 | 6 | 4 | 2 | — | — | — |
| 6 | +3 | Aura of Protection | 2 | 6 | 4 | 2 | — | — | — |
| 7 | +3 | Subclass feature | 2 | 7 | 4 | 3 | — | — | — |
| 8 | +3 | Ability Score Improvement | 2 | 7 | 4 | 3 | — | — | — |
| 9 | +4 | Abjure Foes | 2 | 9 | 4 | 3 | 2 | — | — |
| 10 | +4 | Aura of Courage | 2 | 9 | 4 | 3 | 2 | — | — |
| 11 | +4 | Radiant Strikes | 3 | 10 | 4 | 3 | 3 | — | — |
| 12 | +4 | Ability Score Improvement | 3 | 10 | 4 | 3 | 3 | — | — |
| 13 | +5 | — | 3 | 11 | 4 | 3 | 3 | 1 | — |
| 14 | +5 | Restoring Touch | 3 | 11 | 4 | 3 | 3 | 1 | — |
| 15 | +5 | Subclass feature | 3 | 12 | 4 | 3 | 3 | 2 | — |
| 16 | +5 | Ability Score Improvement | 3 | 12 | 4 | 3 | 3 | 2 | — |
| 17 | +6 | — | 3 | 14 | 4 | 3 | 3 | 3 | 1 |
| 18 | +6 | Aura Expansion | 3 | 14 | 4 | 3 | 3 | 3 | 1 |
| 19 | +6 | Epic Boon | 3 | 15 | 4 | 3 | 3 | 3 | 2 |
| 20 | +6 | Subclass feature | 3 | 15 | 4 | 3 | 3 | 3 | 2 |

<!-- SRD page 54 -->

As a Bonus Action, you can touch a creature (which could be yourself) and draw power from the pool of healing to restore a number of Hit Points to that creature, up to the maximum amount remaining in the pool.

You can also expend 5 Hit Points from the pool of healing power to remove the Poisoned condition from the creature; those points don’t also restore Hit Points to the creature.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_1_lay_on_hands ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_1_lay_on_hands ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: lay_on_hands_pool
maximum:
  op: multiply
  args:
  - 5
  - stat: classes.paladin.level
recover:
  event: rest.long_rest_completed
  amount: all_expended
```

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` type ` | ` creature ` |
| ` range ` | ` touch ` |
| ` self_allowed ` | ` true ` |

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
chosen_points:
  type: integer
  minimum: 0
  maximum: remaining_pool
hit_points_per_pool_point: 1
```

#### Remove condition

<!-- splat:values path=splat_rule/parameters/remove_condition -->

| Field | Value |
| --- | --- |
| ` condition ` | ` poisoned ` |
| ` pool_cost ` | ` 5 ` |
| ` cost_points_also_heal ` | ` false ` |
