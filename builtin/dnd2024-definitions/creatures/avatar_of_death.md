---
splat_schema: 1
splat_id: 661e684a-bd4e-5a77-9411-28c2f8881c4d
splat_kind: creature
splat_key: dnd2024:creatures/avatar_of_death
splat_name: Avatar of Death
splat_origin:
  key: dnd2024:reference/creature/avatar_of_death
splat_sources:
- title: SRD 5.2.1
  locator: pp. 232
  url: https://www.dndbeyond.com/srd
---

# Avatar of Death

## Source text

<!-- SRD page 232 -->

Medium Undead, Neutral evil\
AC 20  Initiative +3 (13)\
HP Half the HP maximum of its summoner\
Speed 60 ft., Fly 60 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 16 +3 +3 Con 16 +3 +3\
Int 16 +3 +3 Wis 16 +3 +3 Cha 16 +3 +3\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned, Unconscious\
Senses Truesight 60 ft., Passive Perception 13\
Languages All languages known to its summoner\
CR None (XP 0; PB equals its summoner’s)\

### Traits

Incorporeal Movement. The avatar can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\

### Actions

Multiattack. The avatar makes a number of Reaping\
Scythe attacks equal to half the summoner’s Proficiency\
Bonus (rounded up).\
Reaping Scythe. Melee Attack Roll: Automatic hit,\
reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage plus 4 (1d8)\
Necrotic damage.\

Original layout: [[../../srd-5.2.1/pages/232|page 232]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Neutral evil ` |
| ` movement_text ` | ` 60 ft., Fly 60 ft. (hover) ` |
| ` immunities_text ` | ` Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned, Unconscious ` |
| ` senses_text ` | ` Truesight 60 ft., Passive Perception 13 ` |
| ` languages_text ` | ` All languages known to its summoner ` |
| ` cr_text ` | ` None (XP 0; PB equals its summoner’s) ` |
| ` hit_points_text ` | ` Half the HP maximum of its summoner ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 16 ` | ` 3 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 20 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 60 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/avatar_of_death ` |
| ` implementation ` | ` dnd2024:creature_avatar_of_death ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["necrotic", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned", "unconscious"] ` |

#### Features

##### Incorporeal movement

<!-- splat:yaml path=splat_rule/parameters/features/incorporeal_movement -->

```yaml
economy: trait
operation: passive_rule
pass_through:
- creatures
- objects
movement_terrain: difficult
end_turn_inside_object_damage:
  roll:
    count: 1
    sides: 10
    modifier: 0
  type: force
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: reaping_scythe
  count:
    ceil:
      divide:
      - stat: summoner.proficiency_bonus
      - 2
```

##### Reaping scythe

<!-- splat:yaml path=splat_rule/parameters/features/reaping_scythe -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  automatic_hit: true
  reach_ft: 5
hit:
  damage:
  - average: 7
    roll:
      count: 1
      sides: 8
      modifier: 3
    type: slashing
  - average: 4
    roll:
      count: 1
      sides: 8
      modifier: 0
    type: necrotic
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` truesight ` | ` 60 ` |
| ` passive_perception ` | ` 13 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
  copy: summoner.languages
understands: []
```

#### Derived statistics

<!-- splat:yaml path=splat_rule/parameters/derived_statistics -->

```yaml
hp:
  divide:
  - stat: summoner.maximum_hp
  - 2
proficiency_bonus:
  stat: summoner.proficiency_bonus
languages:
  stat: summoner.languages
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
