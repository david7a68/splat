---
splat_schema: 2
splat_id: a12c9816-5379-5649-92c8-960c7dbf2989
splat_kind: creature
splat_key: dnd2024:creatures/grick
splat_name: Grick
splat_origin: dnd2024:reference/creature/grick
splat_sources:
- SRD 5.2.1, pp. 295, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 158, Habitat
---

# Grick


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Aberration |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 54 (12d8) |
| Initiative | +2 (12) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 12 |
| Languages | None |
| Habitat | Forest, Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 14 | +2 | +2 |
| Con | 11 | +0 | +0 |
| Int | 3 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 295 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Aberration, Unaligned\
> AC 14 Initiative +2 (12)\
> HP 54 (12d8)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 14 +2 +2 Con 11 +0 +0\
> Int 3 −4 −4 Wis 14 +2 +2 Cha 5 −3 −3\
> Skills Stealth +4\
> Senses Darkvision 60 ft.; Passive Perception 12\
> Languages None\
> CR 2 (XP 450; PB +2)


### Actions

Multiattack. The grick makes one Beak attack and one Tentacles attack.

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2) Piercing damage.

Tentacles. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10 + 2) Slashing damage. If the target is a Medium or smaller creature, it has the Grappled condition (escape DC 12) from all four tentacles.

Original layout: [[../../srd-5.2.1/pages/295|page 295]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/grick ` |
| ` implementation ` | ` dnd2024:creature_grick ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 4 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Beak

<!-- splat:yaml path=splat_rule/parameters/features/beak -->

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
        count: 2
        sides: 6
      add: 2
    average: 9
```

##### Tentacles

<!-- splat:yaml path=splat_rule/parameters/features/tentacles -->

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
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 10
      add: 2
    average: 7
- op: grapple
  escape_dc: 12
  when:
    target_kind: creature
    target_maximum_size: medium
  limb_slots: 4
  slots_per_target: 4
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Count:
    path: count
    type: integer
  Feature:
    path: feature
    type: symbol
-->

| Key | Count | Feature |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` Beak ` |
| ` row_2 ` | ` 1 ` | ` Tentacles ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 30
> ```

