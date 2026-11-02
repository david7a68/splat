---
splat_schema: 1
splat_id: 5ff9859e-7b55-5d3c-9841-f7f6300087bb
splat_kind: creature
splat_key: dnd2024:creatures/sprite
splat_name: Sprite
splat_origin:
  key: dnd2024:reference/creature/sprite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 329
  url: https://www.dndbeyond.com/srd
---

# Sprite

## Source text

<!-- SRD page 329 -->

Tiny Fey, Neutral Good\
AC 15  Initiative +4 (14)\
HP 10 (4d4)\
Speed 10 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 18 +4 +4 Con 10 +0 +0\
Int 14 +2 +2 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Perception +3, Stealth +8\
Senses Passive Perception 13\
Languages Common, Elvish, Sylvan\
CR 1/4 (XP 50; PB +2)

### Actions

Needle Sword. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4) Piercing damage.

Enchanting Bow. Ranged Attack Roll: +6, range 40/160 ft. Hit: 1 Piercing damage, and the target has the Charmed condition until the start of the sprite’s next turn.

Heart Sight. Charisma Saving Throw: DC 10, one creature within 5 feet the sprite can see (Celestials, Fiends, and Undead automatically fail the save). Failure: The sprite knows the target’s emotions and alignment.

Invisibility. The sprite casts Invisibility on itself, requiring no spell components and using Charisma as the spellcasting ability.

Original layout: [[../../srd-5.2.1/pages/329|page 329]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Neutral Good ` |
| ` movement_text ` | ` 10 ft., Fly 40 ft. ` |
| ` skills_text ` | ` Perception +3, Stealth +8 ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` Common, Elvish, Sylvan ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 10 (4d4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 3 ` | ` -4 ` |
| ` Dexterity ` | ` 18 ` | ` 4 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 14 ` | ` 2 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 10 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/sprite ` |
| ` implementation ` | ` dnd2024:creature_sprite ` |

### Parameters

#### Features

##### Needle sword

<!-- splat:yaml path=splat_rule/parameters/features/needle_sword -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 6
    roll:
      count: 1
      sides: 4
      modifier: 4
    type: piercing
```

##### Enchanting bow

<!-- splat:yaml path=splat_rule/parameters/features/enchanting_bow -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  range_ft: 40
  long_range_ft: 160
hit:
  damage:
  - average: 1
    roll:
      constant: 1
    type: piercing
  conditions:
  - condition: charmed
    until: self.next_turn_start
```

##### Heart sight

<!-- splat:yaml path=splat_rule/parameters/features/heart_sight -->

```yaml
economy: action
operation: saving_throw
ability: charisma
dc: 10
targets:
  kind: creature
  count: 1
  range_ft: 5
  visible: true
failure:
  learn:
  - target.emotions
  - target.alignment
automatic_failure_types:
- celestial
- fiend
- undead
success:
  effects: []
```

##### Invisibility

<!-- splat:yaml path=splat_rule/parameters/features/invisibility -->

```yaml
economy: action
operation: cast_spell
spell:
  key: dnd2024:spells/invisibility
ability: charisma
targets:
- self
components_waived:
- verbal
- somatic
- material
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 8 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- elvish
- sylvan
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
