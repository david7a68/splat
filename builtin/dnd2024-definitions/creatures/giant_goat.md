---
splat_schema: 1
splat_id: d51ed266-40a6-5815-a688-fc8ff8b17603
splat_kind: creature
splat_key: dnd2024:creatures/giant_goat
splat_name: Giant Goat
splat_origin:
  key: dnd2024:reference/creature/giant_goat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 351
  url: https://www.dndbeyond.com/srd
---

# Giant Goat

## Source text

<!-- SRD page 351 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 40 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +5 Dex 13 +1 +1 Con 12 +1 +1\
Int 3 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Skills Perception +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1/2 (XP 100; PB +2)

### Actions

Ram. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage. If the target is a Large or smaller creature and the goat moved 20+ feet straight toward it immediately before the hit, the target takes an extra 5 (2d4) Bludgeoning damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/351|page 351]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 40 ft., Climb 30 ft. ` |
| ` skills_text ` | ` Perception +3 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 19 (3d10 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 17 ` | ` 5 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 19 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_goat ` |
| ` implementation ` | ` dnd2024:creature_giant_goat ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Ram

<!-- splat:yaml path=splat_rule/parameters/features/ram -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 5
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 6
      add: 3
    average: 6
- op: conditional
  when:
    target_maximum_size: large
    actor_moved_straight_toward_target_ft_at_least: 20
    immediately_before_hit: true
    target_kind: creature
  effects:
  - op: damage
    components:
    - type: bludgeoning
      amount:
        dice:
          count: 2
          sides: 4
        add: 0
      average: 5
  - op: condition.apply
    condition: prone
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
