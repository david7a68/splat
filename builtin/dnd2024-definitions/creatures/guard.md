---
splat_schema: 2
splat_id: 4ad4d8a0-ac33-5953-a26d-9f48e8e902f7
splat_kind: creature
splat_key: dnd2024:creatures/guard
splat_name: Guard
splat_origin: dnd2024:reference/creature/guard
splat_sources:
- SRD 5.2.1, pp. 296, https://www.dndbeyond.com/srd
---

# Guard


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 16 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +1 (11) |
| Speed | 30 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Skills | Perception +2 |
| Senses | Passive Perception 12 |
| Languages | Common |
| Gear | Chain Shirt, Shield, Spear |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 | +1 |
| Dex | 12 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 10 | +0 | +0 |
| Wis | 11 | +0 | +0 |
| Cha | 10 | +0 | +0 |


## Source text

<!-- SRD page 296 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 16 Initiative +1 (11)\
> HP 11 (2d8 + 2)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 13 +1 +1 Dex 12 +1 +1 Con 12 +1 +1\
> Int 10 +0 +0 Wis 11 +0 +0 Cha 10 +0 +0\
> Skills Perception +2\
> Gear Chain Shirt, Shield, Spear\
> Senses Passive Perception 12\
> Languages Common\
> CR 1/8 (XP 25; PB +2)


### Actions

Spear. Melee or Ranged Attack Roll: +3, reach 5 ft. or range 20/60 ft. Hit: 4 (1d6 + 1) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/296|page 296]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/guard ` |
| ` implementation ` | ` dnd2024:creature_guard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Item | Count |
| --- | --- | --- |
| ` row_1 ` | ` Chain shirt ` | ` 1 ` |
| ` row_2 ` | ` Shield ` | ` 1 ` |
| ` row_3 ` | ` Spear ` | ` 1 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common"] ` |

#### Features

##### Spear

<!-- splat:yaml path=splat_rule/parameters/features/spear -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
range_ft:
  normal: 20
  long: 60
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 6
      add: 1
    average: 4
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
