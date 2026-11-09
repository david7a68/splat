---
splat_schema: 2
splat_id: 9bb302df-2d68-5311-9182-d160654ad979
splat_kind: creature
splat_key: dnd2024:creatures/warhorse
splat_name: Warhorse
splat_origin: dnd2024:reference/creature/warhorse
splat_sources:
- SRD 5.2.1, pp. 364, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 373, Habitat
---

# Warhorse


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 19 (3d10 + 3) |
| Initiative | +1 (11) |
| Speed | 60 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Senses | Passive Perception 11 |
| Languages | None |
| Habitat | Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 12 | +1 | +1 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +3 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 364 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 11  Initiative +1 (11)\
> HP 19 (3d10 + 3)\
> Speed 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 12 +1 +1 Con 13 +1 +1\
> Int 2 −4 −4 Wis 12 +1 +3 Cha 7 −2 −2\
> Senses Passive Perception 11\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4 + 4) Bludgeoning damage. If the target is a Large or smaller creature and the horse moved 20+ feet straight toward it immediately before the hit, the target takes an extra 5 (2d4) Bludgeoning damage and has the Prone condition.

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
| ` mechanic ` | ` dnd2024:creatures/warhorse ` |
| ` implementation ` | ` dnd2024:creature_warhorse ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

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

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 2
        sides: 4
      add: 4
    average: 9
- op: conditional
  when:
    target_maximum_size: large
    actor_moved_straight_toward_target_ft_at_least: 20
    immediately_before_hit: true
    target_kind: creature
  effects:
  - op: damage
    components:
    - type: bludgeoning
      amount:
        dice:
          count: 2
          sides: 4
        add: 0
      average: 5
  - op: condition.apply
    condition: prone
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
