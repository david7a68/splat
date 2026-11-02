---
splat_schema: 1
splat_id: 9eac7390-434c-519f-b483-0c6612df0bc3
splat_kind: creature
splat_key: dnd2024:creatures/roc
splat_name: Roc
splat_origin:
  key: dnd2024:reference/creature/roc
splat_sources:
- title: SRD 5.2.1
  locator: pp. 320
  url: https://www.dndbeyond.com/srd
---

# Roc

## Source text

<!-- SRD page 320 -->

Gargantuan Monstrosity, Unaligned\
AC 15 Initiative +8 (18)\
HP 248 (16d20 + 80)\
Speed 20 ft., Fly 120 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 28 +9 +9 Dex 10 +0 +4 Con 20 +5 +5\
Int 3 −4 −4 Wis 10 +0 +4 Cha 9 −1 −1\
Skills Perception +8\
Senses Passive Perception 18\
Languages None\
CR 11 (XP 7,200; PB +4)

### Actions

Multiattack. The roc makes two Beak attacks. It can replace one attack with a Talons attack.

Beak. Melee Attack Roll: +13, reach 10 ft. Hit: 28 (3d12 + 9) Piercing damage.

Talons. Melee Attack Roll: +13, reach 5 ft. Hit: 23 (4d6 + 9) Slashing damage. If the target is a Huge or smaller creature, it has the Grappled condition (escape DC 19) from both talons, and it has the Restrained condition until the grapple ends.

### Bonus Actions

Swoop (Recharge 5–6). If the roc has a creature Grappled, the roc flies up to half its Fly Speed without provoking Opportunity Attacks and drops that creature.

Original layout: [[../../srd-5.2.1/pages/320|page 320]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` gargantuan ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 20 ft., Fly 120 ft. ` |
| ` skills_text ` | ` Perception +8 ` |
| ` senses_text ` | ` Passive Perception 18 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 11 (XP 7,200; PB +4) ` |
| ` hit_points_text ` | ` 248 (16d20 + 80) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 28 ` | ` 9 ` |
| ` Dexterity ` | ` 10 ` | ` 4 ` |
| ` Constitution ` | ` 20 ` | ` 5 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 4 ` |
| ` Charisma ` | ` 9 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 248 ` |  |
| ` initiative ` | ` 8 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` fly_speed ` | ` 120 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/roc ` |
| ` implementation ` | ` dnd2024:creature_roc ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: beak
  count: 2
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: talons
```

##### Beak

<!-- splat:yaml path=splat_rule/parameters/features/beak -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 13
  reach_ft: 10
hit:
  damage:
  - average: 28
    roll:
      count: 3
      sides: 12
      modifier: 9
    type: piercing
```

##### Talons

<!-- splat:yaml path=splat_rule/parameters/features/talons -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 13
  reach_ft: 5
hit:
  damage:
  - average: 23
    roll:
      count: 4
      sides: 6
      modifier: 9
    type: slashing
  grapple:
    condition: grappled
    source: self
    escape_dc: 19
    maximum_size: huge
    appendage_pool:
      count: 2
      type: talons
      cost: 2
    while_grappled:
      conditions:
      - restrained
```

##### Swoop

<!-- splat:yaml path=splat_rule/parameters/features/swoop -->

```yaml
economy: bonus_action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 5
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: sequence
requires:
  has_grappled_creature: true
effects:
- operation: move
  maximum_ft:
    multiply:
    - stat: fly_speed
    - 0.5
  provoke_opportunity_attacks: false
- operation: drop_grappled_creature
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 8 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 18 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
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
