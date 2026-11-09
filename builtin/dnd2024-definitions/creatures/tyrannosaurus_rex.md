---
splat_schema: 2
splat_id: 4ab35862-e512-5bcd-8355-32675ab13c81
splat_kind: creature
splat_key: dnd2024:creatures/tyrannosaurus_rex
splat_name: Tyrannosaurus Rex
splat_origin: dnd2024:reference/creature/tyrannosaurus_rex
splat_sources:
- SRD 5.2.1, pp. 363, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 372, Habitat
---

# Tyrannosaurus Rex


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Beast (Dinosaur) |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 136 (13d12 + 52) |
| Initiative | +3 (13) |
| Speed | 50 ft. |
| Challenge | 8 (XP 3,900; PB +3) |
| Skills | Perception +4 |
| Senses | Passive Perception 14 |
| Languages | None |
| Habitat | Grassland |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 25 | +7 | +10 |
| Dex | 10 | +0 | +0 |
| Con | 19 | +4 | +4 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +4 |
| Cha | 9 | -1 | -1 |


## Source text

<!-- SRD page 363 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Beast (Dinosaur), Unaligned\
> AC 13 Initiative +3 (13)\
> HP 136 (13d12 + 52)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  25 +7 +10 Dex 10 +0 +0 Con 19 +4 +4\
> Int 2 −4 −4 Wis 12 +1 +4 Cha 9 −1 −1\
> Skills Perception +4\
> Senses Passive Perception 14\
> Languages None\
> CR 8 (XP 3,900; PB +3)


### Actions

Multiattack. The tyrannosaurus makes one Bite attack and one Tail attack.

Bite. Melee Attack Roll: +10, reach 10 ft. Hit: 33 (4d12 + 7) Piercing damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 17). While Grappled, the target has the Restrained condition and can’t be targeted by the tyrannosaurus’s Tail.

Tail. Melee Attack Roll: +10, reach 15 ft. Hit: 25 (4d8 + 7) Bludgeoning damage. If the target is a Huge or smaller creature, it has the Prone condition.

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
| ` mechanic ` | ` dnd2024:creatures/tyrannosaurus_rex ` |
| ` implementation ` | ` dnd2024:creature_tyrannosaurus_rex ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |

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

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 10
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 4
        sides: 12
      add: 7
    average: 33
- op: grapple
  escape_dc: 17
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
attack_bonus: 10
target:
  kind: creature_or_object
  count: 1
reach_ft: 15
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 4
        sides: 8
      add: 7
    average: 25
- op: condition.apply
  condition: prone
  when:
    target_kind: creature
    target_maximum_size: huge
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
