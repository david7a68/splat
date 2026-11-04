---
splat_schema: 2
splat_id: bcb4f9e2-334c-5464-9a54-c62ae20ff5ba
splat_kind: creature
splat_key: dnd2024:creatures/wolf
splat_name: Wolf
splat_origin: dnd2024:reference/creature/wolf
splat_sources:
- SRD 5.2.1, pp. 364, https://www.dndbeyond.com/srd
---

# Wolf


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +2 (12) |
| Speed | 40 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Perception +5, Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 15 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 364 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 11 (2d8 + 2)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  14 +2 +2 Dex 15 +2 +2 Con 12 +1 +1\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
> Skills Perception +5, Stealth +4\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Traits

Pack Tactics. The wolf has Advantage on attack rolls against a creature if at least one of the wolf’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage. If the target is a Medium or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/364|page 364]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/wolf ` |
| ` implementation ` | ` dnd2024:creature_wolf ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
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

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
- op: condition.apply
  condition: prone
  when:
    target_kind: creature
    target_maximum_size: medium
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
