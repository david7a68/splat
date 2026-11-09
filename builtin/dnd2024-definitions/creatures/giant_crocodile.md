---
splat_schema: 2
splat_id: 2ffcad9b-b09c-5269-97ed-5a5993a212fe
splat_kind: creature
splat_key: dnd2024:creatures/giant_crocodile
splat_name: Giant Crocodile
splat_origin: dnd2024:reference/creature/giant_crocodile
splat_sources:
- SRD 5.2.1, pp. 350, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 356, Habitat
---

# Giant Crocodile


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 85 (9d12 + 27) |
| Initiative | -1 (9) |
| Speed | 30 ft., Swim 50 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Stealth +5 |
| Senses | Passive Perception 10 |
| Languages | None |
| Habitat | Coastal, Swamp |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 21 | +5 | +5 |
| Dex | 9 | -1 | -1 |
| Con | 17 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 350 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Beast, Unaligned\
> AC 14  Initiative −1 (9)\
> HP 85 (9d12 + 27)\
> Speed 30 ft., Swim 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 21 +5 +5 Dex 9 −1 −1 Con 17 +3 +3\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 7 −2 −2\
> Skills Stealth +5\
> Senses Passive Perception 10\
> Languages None\
> CR 5 (XP 1,800; PB +3)


### Traits

Hold Breath. The crocodile can hold its breath for 1 hour.

### Actions

Multiattack. The crocodile makes one Bite attack and one Tail attack.

Bite. Melee Attack Roll: +8, reach 5 ft. Hit: 21 (3d10 + 5) Piercing damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 15). While Grappled, the target has the Restrained condition and can’t be targeted by the crocodile’s Tail.

Tail. Melee Attack Roll: +8, reach 10 ft. Hit: 18 (3d8 + 5) Bludgeoning damage. If the target is a Large or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/350|page 350]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_crocodile ` |
| ` implementation ` | ` dnd2024:creature_giant_crocodile ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 5 ` |

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

##### Hold breath

<!-- splat:yaml path=splat_rule/parameters/features/hold_breath -->

```yaml
economy: passive
op: breathing.hold
duration:
  amount: 1
  unit: hour
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 8
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
        count: 3
        sides: 10
      add: 5
    average: 21
- op: grapple
  escape_dc: 15
  when:
    target_kind: creature
    target_maximum_size: large
  while_grappled:
  - op: condition.apply
    condition: restrained
  - op: targeting.prohibit
    feature: tail
    source: actor
```

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 8
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 8
      add: 5
    average: 18
- op: condition.apply
  condition: prone
  when:
    target_kind: creature
    target_maximum_size: large
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
| ` row_1 ` | ` 1 ` | ` Bite ` |
| ` row_2 ` | ` 1 ` | ` Tail ` |

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
> swim_speed:
>   unit: ft
>   value: 50
> ```

