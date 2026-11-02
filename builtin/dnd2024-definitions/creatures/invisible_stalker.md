---
splat_schema: 1
splat_id: e2791c4d-9116-5342-8718-e3f3d9ffb2fc
splat_kind: creature
splat_key: dnd2024:creatures/invisible_stalker
splat_name: Invisible Stalker
splat_origin:
  key: dnd2024:reference/creature/invisible_stalker
splat_sources:
- title: SRD 5.2.1
  locator: pp. 301, 302
  url: https://www.dndbeyond.com/srd
---

# Invisible Stalker

## Source text

<!-- SRD page 301 -->

Large Elemental, Neutral\
AC 14 Initiative +7 (22)\
HP 97 (13d10 + 26)\
Speed 50 ft., Fly 50 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 19 +4 +4 Con 14 +2 +2\
Int 10 +0 +0 Wis 15 +2 +2 Cha 11 +0 +0\
Skills Perception +8, Stealth +10\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 18\
Languages Common, Primordial (Auran)\
CR 6 (XP 2,300; PB +3)

### Traits

Air Form. The stalker can enter an enemy’s space and stop there. It can move through a space as narrow as 1 inch without expending extra movement to do so.

Invisibility. The stalker has the Invisible condition.

### Actions

Multiattack. The stalker makes three Wind Swipe attacks. It can replace one attack with a use of Vortex.

Wind Swipe. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (2d6 + 4) Force damage.

Vortex. Constitution Saving Throw: DC 14, one Large or smaller creature in the stalker’s space. Failure: 7 (1d8 + 3) Thunder damage, and the target has the Grappled condition (escape DC 13). Until the grapple ends, the target can’t cast spells with a Verbal component and takes 7 (2d6) Thunder damage at the start of each of the stalker’s turns.

Original layout: [[../../srd-5.2.1/pages/301|page 301]], [[../../srd-5.2.1/pages/302|page 302]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 50 ft., Fly 50 ft. (hover) ` |
| ` skills_text ` | ` Perception +8, Stealth +10 ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 18 ` |
| ` languages_text ` | ` Common, Primordial (Auran) ` |
| ` cr_text ` | ` 6 (XP 2,300; PB +3) ` |
| ` hit_points_text ` | ` 97 (13d10 + 26) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 19 ` | ` 4 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 15 ` | ` 2 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 97 ` |  |
| ` initiative ` | ` 7 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/invisible_stalker ` |
| ` implementation ` | ` dnd2024:creature_invisible_stalker ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |

#### Features

##### Air form

<!-- splat:values path=splat_rule/parameters/features/air_form -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` occupy_enemy_space ` | ` true ` |
| ` minimum_gap_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |

##### Invisibility

<!-- splat:yaml path=splat_rule/parameters/features/invisibility -->

```yaml
economy: trait
operation: passive_rule
conditions:
- condition: invisible
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: wind_swipe
  count: 3
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: vortex
```

##### Wind swipe

<!-- splat:yaml path=splat_rule/parameters/features/wind_swipe -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 7
  reach_ft: 5
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 6
      modifier: 4
    type: force
```

##### Vortex

<!-- splat:yaml path=splat_rule/parameters/features/vortex -->

```yaml
economy: action
operation: saving_throw
ability: constitution
dc: 14
targets:
  kind: creature
  count: 1
  maximum_size: large
  location: self.space
failure:
  damage:
  - average: 7
    roll:
      count: 1
      sides: 8
      modifier: 3
    type: thunder
  grapple:
    condition: grappled
    source: self
    escape_dc: 13
    while_grappled:
      can_cast_verbal_spells: false
      damage_each_self_turn_start:
      - roll:
          count: 2
          sides: 6
          modifier: 0
        type: thunder
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 8 ` |
| ` stealth ` | ` 10 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 18 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- language: primordial
  dialects:
  - auran
understands: []
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
