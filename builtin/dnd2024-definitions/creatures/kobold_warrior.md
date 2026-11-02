---
splat_schema: 1
splat_id: 8ceed437-0cda-5a4c-8c01-dd70cebae23e
splat_kind: creature
splat_key: dnd2024:creatures/kobold_warrior
splat_name: Kobold Warrior
splat_origin:
  key: dnd2024:reference/creature/kobold_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 302
  url: https://www.dndbeyond.com/srd
---

# Kobold Warrior

## Source text

<!-- SRD page 302 -->

Small Dragon, Neutral\
AC 14 Initiative +2 (12)\
HP 7 (3d6 − 3)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 15 +2 +2 Con 9 −1 −1\
Int 8 −1 −1 Wis 7 −2 −2 Cha 8 −1 −1\
Gear Daggers (3)\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Common, Draconic\
CR 1/8 (XP 25; PB +2)

### Traits

Pack Tactics. The kobold has Advantage on an attack roll against a creature if at least one of the kobold’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

Sunlight Sensitivity. While in sunlight, the kobold has Disadvantage on ability checks and attack rolls.

### Actions

Dagger. Melee or Ranged Attack Roll: +4, reach 5 ft. or range 20/60 ft. Hit: 4 (1d4 + 2) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/302|page 302]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Daggers (3) ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 8 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 1/8 (XP 25; PB +2) ` |
| ` hit_points_text ` | ` 7 (3d6 - 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 7 ` | ` -2 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 9 ` | ` -1 ` |
| ` Intelligence ` | ` 8 ` | ` -1 ` |
| ` Wisdom ` | ` 7 ` | ` -2 ` |
| ` Charisma ` | ` 8 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 7 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/kobold_warrior ` |
| ` implementation ` | ` dnd2024:creature_kobold_warrior ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 8 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | count |
| --- | --- | --- |
| ` 0 ` | ` daggers ` | ` 3 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "draconic"] ` |

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
op: roll.advantage
rolls:
- attack
target_kind: creature
when:
  ally_within_target_ft: 5
  ally_excluded_conditions:
  - incapacitated
```

##### Sunlight sensitivity

<!-- splat:yaml path=splat_rule/parameters/features/sunlight_sensitivity -->

```yaml
economy: passive
op: roll.disadvantage
when:
  in_sunlight: true
rolls:
- ability_check
- attack
```

##### Dagger

<!-- splat:yaml path=splat_rule/parameters/features/dagger -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
range_ft:
  normal: 20
  long: 60
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 4
      add: 2
    average: 4
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
