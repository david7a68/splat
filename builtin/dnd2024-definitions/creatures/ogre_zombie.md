---
splat_schema: 1
splat_id: 7a6ad303-6841-5a1d-ae28-fe80b260ced7
splat_kind: creature
splat_key: dnd2024:creatures/ogre_zombie
splat_name: Ogre Zombie
splat_origin:
  key: dnd2024:reference/creature/ogre_zombie
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
---

# Ogre Zombie

## Source text

<!-- SRD page 344 -->

Large Undead, Neutral Evil\
AC 8 Initiative −2 (8)\
HP 85 (9d10 + 36)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 6 −2 −2 Con 18 +4 +4\
Int 3 −4 −4 Wis 6 −2 +0 Cha 5 −3 −3\
Immunities Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Understands Common and Giant but can’t speak\
CR 2 (XP 450; PB +2)

### Traits

Undead Fortitude. If damage reduces the zombie to 0 Hit Points, it makes a Constitution saving throw (DC 5 plus the damage taken) unless the damage is Radiant or from a Critical Hit. On a successful save, the zombie drops to 1 Hit Point instead.

### Actions

Slam. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/344|page 344]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Poison; Exhaustion, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 8 ` |
| ` languages_text ` | ` Understands Common and Giant but can’t speak ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 85 (9d10 + 36) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 6 ` | ` -2 ` |
| ` Constitution ` | ` 18 ` | ` 4 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 6 ` | ` 0 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 8 ` |  |
| ` max_hp ` | ` 85 ` |  |
| ` initiative ` | ` -2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ogre_zombie ` |
| ` implementation ` | ` dnd2024:creature_ogre_zombie ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "poisoned"] ` |
| ` passive_perception ` | ` 8 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` false ` |
| ` known ` | ` ["common", "giant"] ` |

#### Features

##### Undead fortitude

<!-- splat:yaml path=splat_rule/parameters/features/undead_fortitude -->

```yaml
economy: passive
trigger: damage.reduces_actor_to_zero_hp
exclude:
  damage_type: radiant
  critical_hit: true
save:
  ability: constitution
  dc:
    add:
    - 5
    - event_field: damage_taken
  success:
    hp_instead: 1
```

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
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
        count: 2
        sides: 8
      add: 4
    average: 13
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
