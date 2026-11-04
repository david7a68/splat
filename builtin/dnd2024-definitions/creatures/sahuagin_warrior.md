---
splat_schema: 2
splat_id: 752e6d53-c0bb-5ee5-8cdb-c522a0e75279
splat_kind: creature
splat_key: dnd2024:creatures/sahuagin_warrior
splat_name: Sahuagin Warrior
splat_origin: dnd2024:reference/creature/sahuagin_warrior
splat_sources:
- SRD 5.2.1, pp. 321, https://www.dndbeyond.com/srd
---

# Sahuagin Warrior


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fiend |
| Alignment | Lawful Evil |
| Armor Class | 12 |
| Hit Points | 22 (4d8 + 4) |
| Initiative | +0 (10) |
| Speed | 30 ft., Swim 40 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Perception +5 |
| Senses | Darkvision 120 ft.; Passive Perception 15 |
| Languages | Sahuagin |
| Resistances | Acid, Cold |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 | +1 |
| Dex | 11 | +0 | +0 |
| Con | 12 | +1 | +1 |
| Int | 12 | +1 | +1 |
| Wis | 13 | +1 | +1 |
| Cha | 9 | -1 | -1 |


## Source text

<!-- SRD page 321 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fiend, Lawful Evil\
> AC 12 Initiative +0 (10)\
> HP 22 (4d8 + 4)\
> Speed 30 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 13 +1 +1 Dex 11 +0 +0 Con 12 +1 +1\
> Int 12 +1 +1 Wis 13 +1 +1 Cha 9 −1 −1\
> Skills Perception +5\
> Resistances Acid, Cold\
> Senses Darkvision 120 ft.; Passive Perception 15\
> Languages Sahuagin\
> CR 1/2 (XP 100; PB +2)


### Traits

Blood Frenzy. The sahuagin has Advantage on attack rolls against any creature that doesn’t have all its Hit Points.

Limited Amphibiousness. The sahuagin can breathe air and water, but it must be submerged at least once every 4 hours to avoid suffocating outside water.

Shark Telepathy. The sahuagin can magically control sharks within 120 feet of itself, using a special telepathy.

### Actions

Multiattack. The sahuagin makes two Claw attacks.

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Slashing damage.

### Bonus Actions

Aquatic Charge. The sahuagin swims up to its Swim Speed straight toward an enemy it can see.

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
| ` mechanic ` | ` dnd2024:creatures/sahuagin_warrior ` |
| ` implementation ` | ` dnd2024:creature_sahuagin_warrior ` |

### Parameters

#### Features

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 3
targets:
  count: 1
hit:
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 6
    add: 1
  damage_type: slashing
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: claw
  count: 2
```

##### Blood frenzy

<!-- splat:yaml path=splat_rule/parameters/features/blood_frenzy -->

```yaml
economy: passive
advantage:
  roll: attack
  target_kind: creature
  when:
    target_hp_less_than_maximum: true
```

##### Limited amphibiousness

<!-- splat:yaml path=splat_rule/parameters/features/limited_amphibiousness -->

```yaml
economy: passive
breathing_media:
- air
- water
submersion_required_every:
  hours: 4
failure:
  outside_water: suffocating
```

##### Shark telepathy

<!-- splat:yaml path=splat_rule/parameters/features/shark_telepathy -->

```yaml
economy: passive
magical: true
op: control_creatures
targets:
  kind: shark
  range_feet: 120
  count: all
communication: telepathy
```

##### Aquatic charge

<!-- splat:yaml path=splat_rule/parameters/features/aquatic_charge -->

```yaml
economy: bonus_action
op: move
mode: swim
distance_max:
  stat: swim_speed
direction: straight_toward_visible_enemy
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["acid", "cold"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["sahuagin"] ` |
| ` understands ` | ` ["sahuagin"] ` |

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> swim_speed:
>   unit: ft
>   value: 40
> ```

