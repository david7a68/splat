---
splat_schema: 1
splat_id: 6cdc34e3-3dda-5178-8857-75a5d374f66a
splat_kind: creature
splat_key: dnd2024:creatures/panther
splat_name: Panther
splat_origin:
  key: dnd2024:reference/creature/panther
splat_sources:
- title: SRD 5.2.1
  locator: pp. 358
  url: https://www.dndbeyond.com/srd
---

# Panther

## Source text

<!-- SRD page 358 -->

Medium Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 13 (3d8)\
Speed 50 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  14 +2 +2 Dex 16 +3 +3 Con 10 +0 +0\
Int 3 −4 −4 Wis 14 +2 +2 Cha 7 −2 −2\
Skills Perception +4, Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3)\
Slashing damage.\

### Bonus Actions

Nimble Escape. The panther takes the Disengage or\
Hide action.\

Original layout: [[../../srd-5.2.1/pages/358|page 358]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 50 ft., Climb 40 ft. ` |
| ` skills_text ` | ` Perception +4, Stealth +7 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 13 (3d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 13 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/panther ` |
| ` implementation ` | ` dnd2024:creature_panther ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 7 ` |

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

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 6
      add: 3
    average: 6
```

##### Nimble escape

<!-- splat:values path=splat_rule/parameters/features/nimble_escape -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` action.choose ` |
| ` choices ` | ` ["disengage", "hide"] ` |

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
