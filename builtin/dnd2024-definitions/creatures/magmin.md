---
splat_schema: 1
splat_id: 70753f51-9152-503e-972c-dff51c4d5701
splat_kind: creature
splat_key: dnd2024:creatures/magmin
splat_name: Magmin
splat_origin:
  key: dnd2024:reference/creature/magmin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 305, 306
  url: https://www.dndbeyond.com/srd
---

# Magmin

## Source text

<!-- SRD page 305 -->

Small Elemental, Chaotic Neutral\
AC 14 Initiative +2 (12)\
HP 13 (3d6 + 3)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 15 +2 +2 Con 12 +1 +1\
Int 8 −1 −1 Wis 11 +0 +0 Cha 10 +0 +0\
Immunities Fire\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Ignan)\
CR 1/2 (XP 100; PB +2)\

### Traits

Death Burst. The magmin explodes when it dies. Dex-\
terity Saving Throw: DC 11, each creature in a 10-foot\
Emanation originating from the magmin. Failure: 7 (2d6)\
Fire damage. Success: Half damage.\

### Actions

Touch. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 +\
2) Fire damage. If the target is a creature or a flammable\
object that isn’t being worn or carried, it starts burning.\

<!-- SRD page 306 -->


### Bonus Actions

Ignited Illumination. The magmin sets itself ablaze\
or extinguishes its flames. While ablaze, the magmin\
sheds Bright Light in a 10-foot radius and Dim Light for\
an additional 10 feet.\

Original layout: [[../../srd-5.2.1/pages/305|page 305]], [[../../srd-5.2.1/pages/306|page 306]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Chaotic Neutral ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Primordial (Ignan) ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 13 (3d6 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 7 ` | ` -2 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 8 ` | ` -1 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 13 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/magmin ` |
| ` implementation ` | ` dnd2024:creature_magmin ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |
| ` passive_perception ` | ` 10 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

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
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["ignan"] ` |

#### Features

##### Touch

<!-- splat:yaml path=splat_rule/parameters/features/touch -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 2
        sides: 4
      add: 2
    average: 7
- op: burning.start
  when_any:
  - target_kind: creature
  - target_kind: object
    flammable: true
    worn_or_carried: false
```

##### Death burst

<!-- splat:yaml path=splat_rule/parameters/features/death_burst -->

```yaml
economy: passive
op: saving_throw
ability: dexterity
dc: 11
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    origin: actor
    size_ft: 10
failure:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
trigger: actor.died
```

##### Ignited illumination

<!-- splat:yaml path=splat_rule/parameters/features/ignited_illumination -->

```yaml
economy: bonus_action
op: state.choose
parameter: ablaze
choices:
- true
- false
while_true:
  bright_radius_ft: 10
  dim_additional_ft: 10
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
