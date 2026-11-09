---
splat_schema: 2
splat_id: b763d8e5-bd11-55b7-8cc4-76a939ba4a6c
splat_kind: creature
splat_key: dnd2024:creatures/bone_devil
splat_name: Bone Devil
splat_origin: dnd2024:reference/creature/bone_devil
splat_sources:
- SRD 5.2.1, pp. 267, 268, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 52, Habitat
---

# Bone Devil


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Fiend (Devil) |
| Alignment | Lawful Evil |
| Armor Class | 16 |
| Hit Points | 161 (17d10 + 68) |
| Initiative | +7 (17) |
| Speed | 40 ft., Fly 40 ft. |
| Challenge | 9 (XP 5,000; PB +4) |
| Skills | Deception +7, Insight +6 |
| Senses | Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 12 |
| Languages | Infernal; telepathy 120 ft. |
| Resistances | Cold |
| Immunities | Fire, Poison; Poisoned |
| Habitat | Planar (Nine Hells) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +8 |
| Dex | 16 | +3 | +3 |
| Con | 18 | +4 | +4 |
| Int | 13 | +1 | +5 |
| Wis | 14 | +2 | +6 |
| Cha | 16 | +3 | +7 |


## Source text

<!-- SRD page 267 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Fiend (Devil), Lawful Evil\
> AC 16 Initiative +7 (17)\
> HP 161 (17d10 + 68)\
> Speed 40 ft., Fly 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +8 Dex 16 +3 +3 Con 18 +4 +4\
> Int 13 +1 +5 Wis 14 +2 +6 Cha 16 +3 +7\
> Skills Deception +7, Insight +6\
> Resistances Cold\
> Immunities Fire, Poison; Poisoned\
> Senses Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 12\
> Languages Infernal; telepathy 120 ft.\
> CR 9 (XP 5,000; PB +4)


### Traits

Diabolical Restoration. If the devil dies outside the Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Nine Hells.

Magic Resistance. The devil has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The devil makes two Claw attacks and one Infernal Sting attack.

Claw. Melee Attack Roll: +8, reach 10 ft. Hit: 13 (2d8 + 4) Slashing damage.

Infernal Sting. Melee Attack Roll: +8, reach 10 ft. Hit: 15 (2d10 + 4) Piercing damage plus 18 (4d8) Poison damage, and the target has the Poisoned condition until the start of the devil’s next turn. While Poisoned, the target can’t regain Hit Points.

Original layout: [[../../srd-5.2.1/pages/267|page 267]], [[../../srd-5.2.1/pages/268|page 268]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/bone_devil ` |
| ` implementation ` | ` dnd2024:creature_bone_devil ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold"] ` |
| ` damage_immunities ` | ` ["fire", "poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` passive_perception ` | ` 12 ` |
| ` darkvision_ignores_magical_darkness ` | ` true ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 7 ` |
| ` insight ` | ` 6 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["infernal"] ` |

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` range_ft ` | ` 120 ` |
| ` two_way ` | ` true ` |

#### Features

##### Diabolical restoration

<!-- splat:yaml path=splat_rule/parameters/features/diabolical_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane_not: nine_hells
body: disappears
revive:
  new_body: true
  plane: nine_hells
  location_choice: gm
  hp: maximum
  delay:
    amount: 0
    unit: second
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 8
      add: 4
    average: 13
```

##### Infernal sting

<!-- splat:yaml path=splat_rule/parameters/features/infernal_sting -->

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
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 10
      add: 4
    average: 15
  - type: poison
    amount:
      dice:
        count: 4
        sides: 8
      add: 0
    average: 18
- op: condition.apply
  condition: poisoned
  duration:
    until: actor.next_turn_start
  while_active:
  - op: healing.prohibit
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
| ` row_1 ` | ` 2 ` | ` Claw ` |
| ` row_2 ` | ` 1 ` | ` Infernal sting ` |

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

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
>   value: 40
> ```

