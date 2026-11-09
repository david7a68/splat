---
splat_schema: 2
splat_id: 5e3982d6-f3ff-5853-9522-57c4d3de919a
splat_kind: creature
splat_key: dnd2024:creatures/wight
splat_name: Wight
splat_origin: dnd2024:reference/creature/wight
splat_sources:
- SRD 5.2.1, pp. 341, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 332, Habitat
---

# Wight


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Undead |
| Alignment | Neutral Evil |
| Armor Class | 14 |
| Hit Points | 82 (11d8 + 33) |
| Initiative | +4 (14) |
| Speed | 30 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Perception +3, Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | Common plus one other language |
| Gear | Studded Leather Armor |
| Resistances | Necrotic |
| Immunities | Poison; Exhaustion, Poisoned |
| Habitat | Desert, Swamp, Underdark, Urban, Planar (Shadowfell) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 14 | +2 | +2 |
| Con | 16 | +3 | +3 |
| Int | 10 | +0 | +0 |
| Wis | 13 | +1 | +1 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 341 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Undead, Neutral Evil\
> AC 14 Initiative +4 (14)\
> HP 82 (11d8 + 33)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 14 +2 +2 Con 16 +3 +3\
> Int 10 +0 +0 Wis 13 +1 +1 Cha 15 +2 +2\
> Skills Perception +3, Stealth +4\
> Resistances Necrotic\
> Immunities Poison; Exhaustion, Poisoned\
> Gear Studded Leather Armor\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages Common plus one other language\
> CR 3 (XP 700; PB +2)


### Traits

Sunlight Sensitivity. While in sunlight, the wight has Disadvantage on ability checks and attack rolls.

### Actions

Multiattack. The wight makes two attacks, using Necrotic Sword or Necrotic Bow in any combination. It can replace one attack with a use of Life Drain.

Necrotic Sword. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage plus 4 (1d8) Necrotic damage.

Necrotic Bow. Ranged Attack Roll: +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage plus 4 (1d8) Necrotic damage.

Life Drain. Constitution Saving Throw: DC 13, one creature within 5 feet. Failure: 6 (1d8 + 2) Necrotic damage, and the target’s Hit Point maximum decreases by an amount equal to the damage taken.

A Humanoid slain by this attack rises 24 hours later as a Zombie under the wight’s control, unless the Humanoid is restored to life or its body is destroyed. The wight can have no more than twelve zombies under its control at a time.

Original layout: [[../../srd-5.2.1/pages/341|page 341]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/wight ` |
| ` implementation ` | ` dnd2024:creature_wight ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["necrotic"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "poisoned"] ` |
| ` passive_perception ` | ` 13 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 4 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | count |
| --- | --- | --- |
| ` 0 ` | ` studded_leather_armor ` | ` 1 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 1 ` |

#### Features

##### Sunlight sensitivity

<!-- splat:yaml path=splat_rule/parameters/features/sunlight_sensitivity -->

```yaml
economy: passive
op: roll.disadvantage
when:
  in_sunlight: true
rolls:
- ability_check
- attack
```

##### Necrotic sword

<!-- splat:yaml path=splat_rule/parameters/features/necrotic_sword -->

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
        sides: 8
      add: 2
    average: 6
  - type: necrotic
    amount:
      dice:
        count: 1
        sides: 8
      add: 0
    average: 4
```

##### Necrotic bow

<!-- splat:yaml path=splat_rule/parameters/features/necrotic_bow -->

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
  - type: necrotic
    amount:
      dice:
        count: 1
        sides: 8
      add: 0
    average: 4
```

##### Life drain

<!-- splat:yaml path=splat_rule/parameters/features/life_drain -->

```yaml
economy: action
op: saving_throw
ability: constitution
dc: 13
targets:
  kind: creature
  count: 1
  range_ft: 5
failure:
- op: damage
  components:
  - type: necrotic
    amount:
      dice:
        count: 1
        sides: 8
      add: 2
    average: 6
- op: hp_maximum.reduce
  amount:
    event_field: damage_taken
success: []
on_humanoid_slain:
  op: undead.create
  definition:
    key: dnd2024:creatures/zombie
  delay:
    amount: 24
    unit: hour
  unless_any:
  - restored_to_life
  - body_destroyed
  controller: actor
  control_maximum: 12
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 2
  choices:
  - necrotic_sword
  - necrotic_bow
  combination: any
replace_attack:
  maximum: 1
  choices:
  - op: feature.use
    feature: life_drain
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
