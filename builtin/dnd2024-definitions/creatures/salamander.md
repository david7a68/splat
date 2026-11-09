---
splat_schema: 2
splat_id: 13ac50b5-5032-5657-ba21-e3ceef91c9b6
splat_kind: creature
splat_key: dnd2024:creatures/salamander
splat_name: Salamander
splat_origin: dnd2024:reference/creature/salamander
splat_sources:
- SRD 5.2.1, pp. 321, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 267, Habitat
---

# Salamander


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Elemental |
| Alignment | Neutral Evil |
| Armor Class | 15 |
| Hit Points | 90 (12d10 + 24) |
| Initiative | +2 (12) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Primordial (Ignan) |
| Immunities | Fire |
| Vulnerabilities | Cold |
| Habitat | Underdark, Planar (Elemental Plane of Fire) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 14 | +2 | +2 |
| Con | 15 | +2 | +2 |
| Int | 11 | +0 | +0 |
| Wis | 10 | +0 | +0 |
| Cha | 12 | +1 | +1 |


## Source text

<!-- SRD page 321 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Elemental, Neutral Evil\
> AC 15  Initiative +2 (12)\
> HP 90 (12d10 + 24)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 14 +2 +2 Con 15 +2 +2\
> Int 11 +0 +0 Wis 10 +0 +0 Cha 12 +1 +1\
> Vulnerabilities Cold\
> Immunities Fire\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Primordial (Ignan)\
> CR 5 (XP 1,800; PB +3)


### Traits

Fire Aura. At the end of each of the salamander’s turns, each creature of the salamander’s choice in a 5-foot Emanation originating from the salamander takes 7 (2d6) Fire damage.

### Actions

Multiattack. The salamander makes two Flame Spear attacks. It can replace one attack with a use of Constrict.

Flame Spear. Melee or Ranged Attack Roll: +7, reach 5 ft. or range 20/60 ft. Hit: 13 (2d8 + 4) Piercing damage plus 7 (2d6) Fire damage. Hit or Miss: The spear magically returns to the salamander’s hand immediately after a ranged attack.

Constrict. Strength Saving Throw: DC 15, one Large or smaller creature the salamander can see within 10 feet. Failure: 11 (2d6 + 4) Bludgeoning damage plus 7 (2d6) Fire damage. The target has the Grappled condition (escape DC 14), and it has the Restrained condition until the grapple ends.

Original layout: [[../../srd-5.2.1/pages/321|page 321]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/salamander ` |
| ` implementation ` | ` dnd2024:creature_salamander ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_vulnerabilities ` | ` ["cold"] ` |
| ` damage_immunities ` | ` ["fire"] ` |
| ` passive_perception ` | ` 10 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

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
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["ignan"] ` |

#### Features

##### Flame spear

<!-- splat:yaml path=splat_rule/parameters/features/flame_spear -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
range_ft:
  normal: 20
  long: 60
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 8
      add: 4
    average: 13
  - type: fire
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
after_attack:
  when: ranged_attack_hit_or_miss
  op: item.return
  destination: actor_hand
  magical: true
```

##### Fire aura

<!-- splat:yaml path=splat_rule/parameters/features/fire_aura -->

```yaml
economy: passive
trigger: actor.turn_end
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    origin: actor
    size_ft: 5
  actor_selects: true
effects:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
```

##### Constrict

<!-- splat:yaml path=splat_rule/parameters/features/constrict -->

```yaml
economy: action
op: saving_throw
ability: strength
dc: 15
targets:
  kind: creature
  count: 1
  range_ft: 10
  maximum_size: large
  visible_to_actor: true
failure:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 2
        sides: 6
      add: 4
    average: 11
  - type: fire
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
- op: grapple
  escape_dc: 14
  while_grappled:
  - op: condition.apply
    condition: restrained
success: []
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: flame_spear
replace_attack:
  maximum: 1
  choices:
  - op: feature.use
    feature: constrict
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
> climb_speed:
>   unit: ft
>   value: 30
> ```

