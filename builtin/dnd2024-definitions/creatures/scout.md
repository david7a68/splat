---
splat_schema: 2
splat_id: 05dd58d8-2e0e-5ff9-81e5-ee246a24acf4
splat_kind: creature
splat_key: dnd2024:creatures/scout
splat_name: Scout
splat_origin: dnd2024:reference/creature/scout
splat_sources:
- SRD 5.2.1, pp. 322, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 270, Habitat
---

# Scout


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 13 |
| Hit Points | 16 (3d8 + 3) |
| Initiative | +2 (12) |
| Speed | 30 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Nature +4, Perception +5, Stealth +6, Survival +5 |
| Senses | Passive Perception 15 |
| Languages | Common plus one other language |
| Gear | Leather Armor, Longbow, Shortsword |
| Habitat | Any |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 14 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 11 | +0 | +0 |
| Wis | 13 | +1 | +1 |
| Cha | 11 | +0 | +0 |


## Source text

<!-- SRD page 322 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 13  Initiative +2 (12)\
> HP 16 (3d8 + 3)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 11 +0 +0 Dex 14 +2 +2 Con 12 +1 +1\
> Int 11 +0 +0 Wis 13 +1 +1 Cha 11 +0 +0\
> Skills Nature +4, Perception +5, Stealth +6, Survival +5\
> Gear Leather Armor, Longbow, Shortsword\
> Senses Passive Perception 15\
> Languages Common plus one other language\
> CR 1/2 (XP 100; PB +2)


### Actions

Multiattack. The scout makes two attacks, using Shortsword and Longbow in any combination.

Shortsword. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage.

Longbow. Ranged Attack Roll: +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/322|page 322]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/scout ` |
| ` implementation ` | ` dnd2024:creature_scout ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` nature ` | ` 4 ` |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 6 ` |
| ` survival ` | ` 5 ` |

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
| ` row_1 ` | ` Leather armor ` | ` 1 ` |
| ` row_2 ` | ` Longbow ` | ` 1 ` |
| ` row_3 ` | ` Shortsword ` | ` 1 ` |

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
| ` additional_choice_count ` | ` 1 ` |

#### Features

##### Shortsword

<!-- splat:yaml path=splat_rule/parameters/features/shortsword -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    average: 5
```

##### Longbow

<!-- splat:yaml path=splat_rule/parameters/features/longbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 150
  long: 600
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 8
      add: 2
    average: 6
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 2
  choices:
  - shortsword
  - longbow
  combination: any
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
