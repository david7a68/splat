---
splat_schema: 2
splat_id: 9e0eea97-c939-58d8-b586-c85d883f58eb
splat_kind: creature
splat_key: dnd2024:creatures/blood_hawk
splat_name: Blood Hawk
splat_origin: dnd2024:reference/creature/blood_hawk
splat_sources:
- SRD 5.2.1, pp. 345, 346, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 350, Habitat
---

# Blood Hawk


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 7 (2d6) |
| Initiative | +2 (12) |
| Speed | 10 ft., Fly 60 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Skills | Perception +6 |
| Senses | Passive Perception 16 |
| Languages | None |
| Habitat | Arctic, Coastal, Forest, Grassland, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 6 | -2 | -2 |
| Dex | 14 | +2 | +2 |
| Con | 10 | +0 | +0 |
| Int | 3 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 345 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 7 (2d6)\
> Speed 10 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 6 −2 −2 Dex 14 +2 +2 Con 10 +0 +0\
> Int 3 −4 −4 Wis 14 +2 +2 Cha 5 −3 −3\
> Skills Perception +6\
> Senses Passive Perception 16\
> Languages None\
> CR 1/8 (XP 25; PB +2)


<!-- SRD page 346 -->

### Traits

Pack Tactics. The hawk has Advantage on an attack roll against a creature if at least one of the hawk’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage, or 6 (1d8 + 2) Piercing damage if the target is Bloodied.

Original layout: [[../../srd-5.2.1/pages/345|page 345]], [[../../srd-5.2.1/pages/346|page 346]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/blood_hawk ` |
| ` implementation ` | ` dnd2024:creature_blood_hawk ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
op: roll.advantage
rolls:
- attack
target_kind: creature
when:
  ally_within_target_ft: 5
  ally_excluded_conditions:
  - incapacitated
```

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
        count: 1
        sides: 4
      add: 2
    average: 4
  replacement_when:
    if:
      target_bloodied: true
    components:
    - type: piercing
      amount:
        dice:
          count: 1
          sides: 8
        add: 2
      average: 6
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> ```

