---
splat_schema: 1
splat_id: e10dcd2a-10f4-535b-a264-adc2b5243358
splat_kind: creature
splat_key: dnd2024:creatures/specter
splat_name: Specter
splat_origin:
  key: dnd2024:reference/creature/specter
splat_sources:
- title: SRD 5.2.1
  locator: pp. 327
  url: https://www.dndbeyond.com/srd
---

# Specter

## Source text

<!-- SRD page 327 -->

Medium Undead, Chaotic Evil\
AC 12 Initiative +2 (12)\
HP 22 (5d8)\
Speed 30 ft., Fly 50 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 14 +2 +2 Con 11 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 11 +0 +0\
Resistances Acid, Bludgeoning, Cold, Fire, Lightning,\
Piercing, Slashing, Thunder\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Grappled, Paralyzed, Petrified, Poisoned, Prone,\
Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Understands Common plus one other language\
but can’t speak\
CR 1 (XP 200; PB +2)\

### Traits

Incorporeal Movement. The specter can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\
Sunlight Sensitivity. While in sunlight, the specter has\
Disadvantage on ability checks and attack rolls.\

### Actions

Life Drain. Melee Attack Roll: +4, reach 5 ft. Hit: 7\
(2d6) Necrotic damage. If the target is a creature, its Hit\
Point maximum decreases by an amount equal to the\
damage taken.\

Original layout: [[../../srd-5.2.1/pages/327|page 327]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 30 ft., Fly 50 ft. (hover) ` |
| ` resistances_text ` | ` Acid, Bludgeoning, Cold, Fire, Lightning, Piercing, Slashing, Thunder ` |
| ` immunities_text ` | ` Necrotic, Poison; Charmed, Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Understands Common plus one other language but can’t speak ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 22 (5d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 1 ` | ` -5 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 22 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/specter ` |
| ` implementation ` | ` dnd2024:creature_specter ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["acid", "bludgeoning", "cold", "fire", "lightning", "piercing", "slashing", "thunder"] ` |
| ` damage_immunities ` | ` ["necrotic", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |

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

##### Sunlight sensitivity

<!-- splat:yaml path=splat_rule/parameters/features/sunlight_sensitivity -->

```yaml
economy: trait
operation: passive_rule
while:
  illumination: sunlight
disadvantage_on:
- ability_checks
- attack_rolls
```

##### Life drain

<!-- splat:yaml path=splat_rule/parameters/features/life_drain -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 7
    roll:
      count: 2
      sides: 6
      modifier: 0
    type: necrotic
  maximum_hp_reduction:
    requires_target_kind: creature
    equal: damage_taken
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- common
additional_languages:
  count: 1
  chooser: creator
  can_speak: false
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
