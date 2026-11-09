---
splat_schema: 2
splat_id: 30866ef2-a238-5b10-be76-9bf148aaf82f
splat_kind: creature
splat_key: dnd2024:creatures/ogre
splat_name: Ogre
splat_origin: dnd2024:reference/creature/ogre
splat_sources:
- SRD 5.2.1, pp. 312, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 231, Habitat
---

# Ogre


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Giant |
| Alignment | Chaotic Evil |
| Armor Class | 11 |
| Hit Points | 68 (8d10 + 24) |
| Initiative | -1 (9) |
| Speed | 40 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 8 |
| Languages | Common, Giant |
| Gear | Greatclub, Javelins (3) |
| Habitat | Arctic, Desert, Forest, Grassland, Hill, Mountain, Swamp, Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 8 | -1 | -1 |
| Con | 16 | +3 | +3 |
| Int | 5 | -3 | -3 |
| Wis | 7 | -2 | -2 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 312 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Giant, Chaotic Evil\
> AC 11 Initiative −1 (9)\
> HP 68 (8d10 + 24)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 8 −1 −1 Con 16 +3 +3\
> Int 5 −3 −3 Wis 7 −2 −2 Cha 7 −2 −2\
> Gear Greatclub, Javelins (3)\
> Senses Darkvision 60 ft.; Passive Perception 8\
> Languages Common, Giant\
> CR 2 (XP 450; PB +2)


### Actions

Greatclub. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage.

Javelin. Melee or Ranged Attack Roll: +6, reach 5 ft. or range 30/120 ft. Hit: 11 (2d6 + 4) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/312|page 312]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ogre ` |
| ` implementation ` | ` dnd2024:creature_ogre ` |

### Parameters

#### Features

##### Greatclub

<!-- splat:yaml path=splat_rule/parameters/features/greatclub -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 13
  amount:
    dice:
      count: 2
      sides: 8
    add: 4
  damage_type: bludgeoning
reach_feet: 5
```

##### Javelin

<!-- splat:yaml path=splat_rule/parameters/features/javelin -->

```yaml
economy: action
op: attack
attack_modes:
- melee
- ranged
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 6
    add: 4
  damage_type: piercing
reach_feet: 5
range_feet: 30
long_range_feet: 120
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
| ` passive_perception ` | ` 8 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Item name:
    path: item_name
    type: string
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item name | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Greatclub ` | ` 1 ` |
| ` row_2 ` | ` Javelins ` | ` 3 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "giant"] ` |
| ` understands ` | ` ["common", "giant"] ` |

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
