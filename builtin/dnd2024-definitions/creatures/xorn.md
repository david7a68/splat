---
splat_schema: 2
splat_id: e5311ec7-f6b3-509e-847d-cbae8bc3a2bd
splat_kind: creature
splat_key: dnd2024:creatures/xorn
splat_name: Xorn
splat_origin: dnd2024:reference/creature/xorn
splat_sources:
- SRD 5.2.1, pp. 343, https://www.dndbeyond.com/srd
---

# Xorn


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Elemental |
| Alignment | Neutral |
| Armor Class | 19 |
| Hit Points | 84 (8d8 + 48) |
| Initiative | +0 (10) |
| Speed | 20 ft., Burrow 20 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Perception +6, Stealth +6 |
| Senses | Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 16 |
| Languages | Primordial (Terran) |
| Immunities | Poison; Paralyzed, Petrified, Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 10 | +0 | +0 |
| Con | 22 | +6 | +6 |
| Int | 11 | +0 | +0 |
| Wis | 10 | +0 | +0 |
| Cha | 11 | +0 | +0 |


## Source text

<!-- SRD page 343 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Elemental, Neutral\
> AC 19 Initiative +0 (10)\
> HP 84 (8d8 + 48)\
> Speed 20 ft., Burrow 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 10 +0 +0 Con 22 +6 +6\
> Int 11 +0 +0 Wis 10 +0 +0 Cha 11 +0 +0\
> Skills Perception +6, Stealth +6\
> Immunities Poison; Paralyzed, Petrified, Poisoned\
> Senses Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 16\
> Languages Primordial (Terran)\
> CR 5 (XP 1,800; PB +3)


### Traits

Earth Glide. The xorn can burrow through nonmagical, unworked earth and stone. While doing so, the xorn doesn’t disturb the material it moves through.

Treasure Sense. The xorn can pinpoint the location of precious metals and stones within 60 feet of itself.

### Actions

Multiattack. The xorn makes one Bite attack and three Claw attacks.

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 17 (4d6 + 3) Piercing damage.

Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d10 + 3) Slashing damage.

### Bonus Actions

Charge. The xorn moves up to its Speed or Burrow Speed straight toward an enemy it can sense.

Original layout: [[../../srd-5.2.1/pages/343|page 343]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/xorn ` |
| ` implementation ` | ` dnd2024:creature_xorn ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["paralyzed", "petrified", "poisoned"] ` |

#### Features

##### Earth glide

<!-- splat:yaml path=splat_rule/parameters/features/earth_glide -->

```yaml
economy: trait
operation: passive_rule
burrow_materials:
- earth
- stone
requires:
  material_nonmagical: true
  material_unworked: true
disturb_material: false
```

##### Treasure sense

<!-- splat:values path=splat_rule/parameters/features/treasure_sense -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` pinpoint ` | ` ["precious_metals", "precious_stones"] ` |
| ` range_ft ` | ` 60 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count: 1
- operation: attack_sequence
  feature: claw
  count: 3
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 17
    roll:
      count: 4
      sides: 6
      modifier: 3
    type: piercing
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 8
    roll:
      count: 1
      sides: 10
      modifier: 3
    type: slashing
```

##### Charge

<!-- splat:yaml path=splat_rule/parameters/features/charge -->

```yaml
economy: bonus_action
operation: move
maximum_ft:
  choose_stat:
  - speed
  - burrow_speed
direction: straight_toward_enemy
enemy_sensed: true
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 6 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` tremorsense ` | ` 60 ` |
| ` passive_perception ` | ` 16 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- language: primordial
  dialects:
  - terran
understands: []
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> burrow_speed:
>   unit: ft
>   value: 20
> ```

