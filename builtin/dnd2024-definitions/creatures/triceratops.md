---
splat_schema: 2
splat_id: 95a127ec-d2fb-5908-8987-dd8846ede461
splat_kind: creature
splat_key: dnd2024:creatures/triceratops
splat_name: Triceratops
splat_origin: dnd2024:reference/creature/triceratops
splat_sources:
- SRD 5.2.1, pp. 363, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 372, Habitat
---

# Triceratops


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Beast (Dinosaur) |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 114 (12d12 + 36) |
| Initiative | -1 (9) |
| Speed | 50 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Senses | Passive Perception 10 |
| Languages | None |
| Habitat | Grassland |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 22 | +6 | +6 |
| Dex | 9 | -1 | -1 |
| Con | 17 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 363 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Beast (Dinosaur), Unaligned\
> AC 14  Initiative −1 (9)\
> HP 114 (12d12 + 36)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 22 +6 +6 Dex 9 −1 −1 Con 17 +3 +3\
> Int 2 −4 −4 Wis 11 +0 +0 Cha  5 −3 −3\
> Senses Passive Perception 10\
> Languages None\
> CR 5 (XP 1,800; PB +3)


### Actions

Multiattack. The triceratops makes two Gore attacks.

Gore. Melee Attack Roll: +9, reach 5 ft. Hit: 19 (2d12 + 6) Piercing damage. If the target is Huge or smaller and the triceratops moved 20+ feet straight toward it immediately before the hit, the target takes an extra 9 (2d8) Piercing damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/363|page 363]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/triceratops ` |
| ` implementation ` | ` dnd2024:creature_triceratops ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 9
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
        sides: 12
      add: 6
    average: 19
- op: conditional
  when:
    target_maximum_size: huge
    actor_moved_straight_toward_target_ft_at_least: 20
    immediately_before_hit: true
  effects:
  - op: damage
    components:
    - type: piercing
      amount:
        dice:
          count: 2
          sides: 8
        add: 0
      average: 9
  - op: condition.apply
    condition: prone
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: gore
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
