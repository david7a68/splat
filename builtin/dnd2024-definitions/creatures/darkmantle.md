---
splat_schema: 1
splat_id: 08ef1af3-e4b0-5309-a741-163066ff99df
splat_kind: creature
splat_key: dnd2024:creatures/darkmantle
splat_name: Darkmantle
splat_origin:
  key: dnd2024:reference/creature/darkmantle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 278, 279
  url: https://www.dndbeyond.com/srd
---

# Darkmantle

## Source text

<!-- SRD page 278 -->

Small Aberration, Unaligned\
AC 11 Initiative +3 (13)\
HP 22 (5d6 + 5)\
Speed 10 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 12 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Stealth +3\
Senses Blindsight 60 ft.; Passive Perception 10\
Languages None\
CR 1/2 (XP 100; PB +2)

<!-- SRD page 279 -->

### Actions

Crush. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage, and the darkmantle attaches to the target. If the target is a Medium or smaller creature and the darkmantle had Advantage on the attack roll, it covers the target, which has the Blinded condition and is suffocating while the darkmantle is attached in this way.

While attached to a target, the darkmantle can attack only the target but has Advantage on its attack rolls. Its Speed becomes 0, it can’t benefit from any bonus to its Speed, and it moves with the target.

A creature can take an action to try to detach the darkmantle from itself, doing so with a successful DC 13 Strength (Athletics) check. On its turn, the darkmantle can detach itself by using 5 feet of movement.

Darkness Aura (1/Day). Magical Darkness fills a 15-foot Emanation originating from the darkmantle. This effect lasts while the darkmantle maintains Concentration on it, up to 10 minutes. Darkvision can’t penetrate this area, and no light can illuminate it.

Original layout: [[../../srd-5.2.1/pages/278|page 278]], [[../../srd-5.2.1/pages/279|page 279]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` aberration ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 10 ft., Fly 30 ft. ` |
| ` skills_text ` | ` Stealth +3 ` |
| ` senses_text ` | ` Blindsight 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 22 (5d6 + 5) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 22 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/darkmantle ` |
| ` implementation ` | ` dnd2024:creature_darkmantle ` |

### Parameters

#### Features

##### Crush

<!-- splat:yaml path=splat_rule/parameters/features/crush -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 6
    roll:
      count: 1
      sides: 6
      modifier: 3
    type: bludgeoning
  attach:
    target: hit_target
    covered_if:
      maximum_size: medium
      attack_had_advantage: true
    while_covered:
      conditions:
      - blinded
      suffocating: true
    while_attached:
      attack_targets:
      - attached_target
      attack_advantage: true
      speed: 0
      speed_bonuses: false
      moves_with_target: true
    detach_self:
      economy: movement
      cost_ft: 5
      on_own_turn: true
    detach_target:
      economy: action
      check: strength_athletics
      dc: 13
      actor: attached_target
```

##### Darkness aura

<!-- splat:yaml path=splat_rule/parameters/features/darkness_aura -->

```yaml
economy: action
resource:
  kind: daily
  uses: 1
  recovery: long_rest
operation: create_area
area:
  shape: emanation
  size_ft: 15
  origin: self
magical_darkness: true
concentration: true
duration_seconds: 600
darkvision_penetrates: false
light_illuminates: false
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` passive_perception ` | ` 10 ` |

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
