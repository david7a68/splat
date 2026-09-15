---
splat_schema: 1
splat_id: 7a623dd0-f983-5f68-adfa-2b9262f43ec8
splat_kind: creature
splat_key: dnd2024:creatures/doppelganger
splat_name: Doppelganger
splat_origin:
  key: dnd2024:reference/creature/doppelganger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 280
  url: https://www.dndbeyond.com/srd
---

# Doppelganger

## Source text

<!-- SRD page 280 -->

Medium Monstrosity, Neutral\
AC 14 Initiative +4 (14)\
HP 52 (8d8 + 16)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 18 +4 +4 Con 14 +2 +2\
Int 11 +0 +0 Wis 12 +1 +1 Cha 14 +2 +2\
Skills Deception +6, Insight +3\
Immunities Charmed\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common plus three other languages\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The doppelganger makes two Slam attacks\
and uses Unsettling Visage if available.\
Slam. Melee Attack Roll: +6 (with Advantage during the\
first round of each combat), reach 5 ft. Hit: 11 (2d6 + 4)\
Bludgeoning damage.\
Read Thoughts. The doppelganger casts Detect\
Thoughts, requiring no spell components and using\
Charisma as the spellcasting ability (spell save DC 12).\
Unsettling Visage (Recharge 6). Wisdom Saving Throw:\
DC 12, each creature in a 15-foot Emanation originating\
from the doppelganger that can see the doppelganger.\
Failure: The target has the Frightened condition and re-\
peats the save at the end of each of its turns, ending the\
effect on itself on a success. After 1 minute, it succeeds\
automatically.\

### Bonus Actions

Shape-Shift. The doppelganger shape-shifts into a Me-\
dium or Small Humanoid, or it returns to its true form.\
Its game statistics, other than its size, are the same in\
each form. Any equipment it is wearing or carrying isn’t\
transformed.\

Original layout: [[../../srd-5.2.1/pages/280|page 280]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Deception +6, Insight +3 ` |
| ` immunities_text ` | ` Charmed ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` Common plus three other languages ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 52 (8d8 + 16) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 11 ` | ` 0 ` |
| ` Dexterity ` | ` 18 ` | ` 4 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 11 ` | ` 0 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 14 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 52 ` |  |
| ` initiative ` | ` 4 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/doppelganger ` |
| ` implementation ` | ` dnd2024:creature_doppelganger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunities ` | ` ["charmed"] ` |

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: slam
  count: 2
- operation: use_feature
  feature: unsettling_visage
  if_available: true
```

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

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
  advantage_if:
    combat_round: 1
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 6
      modifier: 4
    type: bludgeoning
```

##### Read thoughts

<!-- splat:yaml path=splat_rule/parameters/features/read_thoughts -->

```yaml
economy: action
operation: cast_spell
spell:
  key: dnd2024:spells/detect_thoughts
ability: charisma
save_dc: 12
components_waived:
- verbal
- somatic
- material
```

##### Unsettling visage

<!-- splat:yaml path=splat_rule/parameters/features/unsettling_visage -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: wisdom
dc: 12
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 15
    origin: self
  can_see_self: true
failure:
  conditions:
  - condition: frightened
  repeat_save:
    event: target.turn_end
    success: end_effect
  automatic_success_after_seconds: 60
success:
  effects: []
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` operation ` | ` shape_shift ` |
| ` statistics_preserved_except ` | ` ["size"] ` |
| ` equipment_transformed ` | ` false ` |

###### Forms

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/forms
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Type:
    path: type
    type: symbol
    optional: true
  Size:
    path: size
    type: yaml
    optional: true
  Form:
    path: form
    type: symbol
    optional: true
-->

| Key | Type | Size | Form |
| --- | --- | --- | --- |
| ` row_1 ` | ` Humanoid ` | ` ["medium", "small"] ` |  |
| ` row_2 ` |  |  | ` True form ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 6 ` |
| ` insight ` | ` 3 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
additional_languages:
  count: 3
  chooser: creator
  can_speak: true
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
