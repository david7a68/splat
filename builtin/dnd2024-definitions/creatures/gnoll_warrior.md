---
splat_schema: 2
splat_id: 4bf3d8ad-b9ff-5916-b2ee-391305fdf40e
splat_kind: creature
splat_key: dnd2024:creatures/gnoll_warrior
splat_name: Gnoll Warrior
splat_origin: dnd2024:reference/creature/gnoll_warrior
splat_sources:
- SRD 5.2.1, pp. 289, https://www.dndbeyond.com/srd
---

# Gnoll Warrior


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fiend |
| Alignment | Chaotic Evil |
| Armor Class | 15 |
| Hit Points | 27 (6d8) |
| Initiative | +1 (11) |
| Speed | 30 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Gnoll |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 12 | +1 | +1 |
| Con | 11 | +0 | +0 |
| Int | 6 | -2 | -2 |
| Wis | 10 | +0 | +0 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 289 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fiend, Chaotic Evil\
> AC 15 Initiative +1 (11)\
> HP 27 (6d8)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 12 +1 +1 Con 11 +0 +0\
> Int 6 −2 −2 Wis 10 +0 +0 Cha 7 −2 −2\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Gnoll\
> CR 1/2 (XP 100; PB +2)


### Actions

Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage.

Bone Bow. Ranged Attack Roll: +3, range 150/600 ft. Hit: 6 (1d10 + 1) Piercing damage.

### Bonus Actions

Rampage (1/Day). Immediately after dealing damage to a creature that is already Bloodied, the gnoll moves up to half its Speed, and it makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/289|page 289]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/gnoll_warrior ` |
| ` implementation ` | ` dnd2024:creature_gnoll_warrior ` |

### Parameters

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 6
    add: 2
  damage_type: piercing
reach_feet: 5
```

##### Bone bow

<!-- splat:yaml path=splat_rule/parameters/features/bone_bow -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 3
targets:
  count: 1
hit:
- op: damage
  average: 6
  amount:
    dice:
      count: 1
      sides: 10
    add: 1
  damage_type: piercing
range_feet: 150
long_range_feet: 600
```

##### Rampage

<!-- splat:yaml path=splat_rule/parameters/features/rampage -->

```yaml
economy: bonus_action
resource:
  capacity: 1
  recovery: long_rest
  cost: 1
trigger: self.damage_dealt
requires:
  target_kind: creature
  target_bloodied_before_damage: true
op: sequence
steps:
- op: move
  distance_max:
    multiply:
    - stat: speed
    - numerator: 1
      denominator: 2
- op: use_feature
  feature: rend
  count: 1
```

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["gnoll"] ` |
| ` understands ` | ` ["gnoll"] ` |

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
