---
splat_schema: 1
splat_id: 717bc686-4fb8-5665-b30a-c6a2b4a22fa4
splat_kind: creature
splat_key: dnd2024:creatures/skeleton
splat_name: Skeleton
splat_origin:
  key: dnd2024:reference/creature/skeleton
splat_sources:
- title: SRD 5.2.1
  locator: pp. 325, 326
  url: https://www.dndbeyond.com/srd
---

# Skeleton

## Source text

<!-- SRD page 325 -->

Medium Undead, Lawful Evil\
AC 14  Initiative +3 (13)\
HP 13 (2d8 + 4)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 16 +3 +3 Con 15 +2 +2\
Int 6 −2 −2 Wis 8 −1 −1 Cha 5 −3 −3\
Vulnerabilities Bludgeoning\
Immunities Poison; Exhaustion, Poisoned\
Gear Shortbow, Shortsword\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Understands Common plus one other language but can’t speak\
CR 1/4 (XP 50; PB +2)

<!-- SRD page 326 -->

### Actions

Shortsword. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Piercing damage.

Shortbow. Ranged Attack Roll: +5, range 80/320 ft. Hit: 6 (1d6 + 3) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/325|page 325]], [[../../srd-5.2.1/pages/326|page 326]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Lawful Evil ` |
| ` movement_text ` | ` 30 ft. ` |
| ` vulnerabilities_text ` | ` Bludgeoning ` |
| ` immunities_text ` | ` Poison; Exhaustion, Poisoned ` |
| ` gear_text ` | ` Shortbow, Shortsword ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 9 ` |
| ` languages_text ` | ` Understands Common plus one other language but can’t speak ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 13 (2d8 + 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 6 ` | ` -2 ` |
| ` Wisdom ` | ` 8 ` | ` -1 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 13 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/skeleton ` |
| ` implementation ` | ` dnd2024:creature_skeleton ` |

### Parameters

#### Features

##### Shortsword

<!-- splat:yaml path=splat_rule/parameters/features/shortsword -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 6
  amount:
    dice:
      count: 1
      sides: 6
    add: 3
  damage_type: piercing
reach_feet: 5
```

##### Shortbow

<!-- splat:yaml path=splat_rule/parameters/features/shortbow -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 6
  amount:
    dice:
      count: 1
      sides: 6
    add: 3
  damage_type: piercing
range_feet: 80
long_range_feet: 320
```

#### Defenses

##### Vulnerabilities

<!-- splat:values path=splat_rule/parameters/defenses/vulnerabilities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["bludgeoning"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["exhaustion", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

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
| ` row_1 ` | ` Shortbow ` | ` 1 ` |
| ` row_2 ` | ` Shortsword ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` understands ` | ` ["common"] ` |
| ` can_speak ` | ` false ` |

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` false ` |
| ` can_understand ` | ` true ` |

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
