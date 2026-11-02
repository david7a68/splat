---
splat_schema: 1
splat_id: 58e39c3e-e714-5a0d-8b07-8f3519179edb
splat_kind: creature
splat_key: dnd2024:creatures/vampire_familiar
splat_name: Vampire Familiar
splat_origin:
  key: dnd2024:reference/creature/vampire_familiar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 334
  url: https://www.dndbeyond.com/srd
---

# Vampire Familiar

## Source text

<!-- SRD page 334 -->

Medium or Small Humanoid, Neutral Evil\
AC 15  Initiative +5 (15)\
HP 65 (10d8 + 20)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 16 +3 +5 Con 15 +2 +2\
Int 10 +0 +0 Wis 10 +0 +2 Cha 14 +2 +2\
Skills Perception +4, Persuasion +4, Stealth +7\
Resistances Necrotic\
Immunities Charmed (except from its vampire master)\
Gear Daggers (10)\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Common plus one other language\
CR 3 (XP 700; PB +2)

### Traits

Vampiric Connection. While the familiar and its vampire master are on the same plane of existence, the vampire can communicate with the familiar telepathically, and the vampire can perceive through the familiar’s senses.

### Actions

Multiattack. The familiar makes two Umbral Dagger attacks.

Umbral Dagger. Melee or Ranged Attack Roll: +5, reach 5 ft. or range 20/60 ft. Hit: 5 (1d4 + 3) Piercing damage plus 7 (3d4) Necrotic damage. If the target is reduced to 0 Hit Points by this attack, the target becomes Stable but has the Poisoned condition for 1 hour. While it has the Poisoned condition, the target has the Paralyzed condition.

### Bonus Actions

Deathless Agility. The familiar takes the Dash or Disengage action.

Original layout: [[../../srd-5.2.1/pages/334|page 334]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral Evil ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` skills_text ` | ` Perception +4, Persuasion +4, Stealth +7 ` |
| ` resistances_text ` | ` Necrotic ` |
| ` immunities_text ` | ` Charmed (except from its vampire master) ` |
| ` gear_text ` | ` Daggers (10) ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 65 (10d8 + 20) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 17 ` | ` 3 ` |
| ` Dexterity ` | ` 16 ` | ` 5 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 2 ` |
| ` Charisma ` | ` 14 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 65 ` |  |
| ` initiative ` | ` 5 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/vampire_familiar ` |
| ` implementation ` | ` dnd2024:creature_vampire_familiar ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["necrotic"] ` |
| ` passive_perception ` | ` 14 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` persuasion ` | ` 4 ` |
| ` stealth ` | ` 7 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Condition immunities

<!-- splat:rows path=splat_rule/parameters/condition_immunities -->

| Row | condition | except_source |
| --- | --- | --- |
| ` 0 ` | ` charmed ` | ` vampire_master ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | count |
| --- | --- | --- |
| ` 0 ` | ` daggers ` | ` 10 ` |

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
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 1 ` |

#### Features

##### Umbral dagger

<!-- splat:yaml path=splat_rule/parameters/features/umbral_dagger -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 5
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
      add: 3
    average: 5
  - type: necrotic
    amount:
      dice:
        count: 3
        sides: 4
      add: 0
    average: 7
on_reduce_target_to_zero_hp:
  effects:
  - op: stable.set
    value: true
  - op: condition.apply
    condition: poisoned
    duration:
      amount: 1
      unit: hour
    while_active:
    - op: condition.apply
      condition: paralyzed
```

##### Deathless agility

<!-- splat:values path=splat_rule/parameters/features/deathless_agility -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` action.choose ` |
| ` choices ` | ` ["dash", "disengage"] ` |

##### Vampiric connection

<!-- splat:yaml path=splat_rule/parameters/features/vampiric_connection -->

```yaml
economy: passive
when:
  same_plane_as: vampire_master
master_can:
  telepathic_communication: true
  perceive_through_actor_senses: true
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: umbral_dagger
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
