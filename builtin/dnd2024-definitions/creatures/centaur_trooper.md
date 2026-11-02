---
splat_schema: 1
splat_id: b3e09ba9-787f-5cd5-9e07-4bd1e256147c
splat_kind: creature
splat_key: dnd2024:creatures/centaur_trooper
splat_name: Centaur Trooper
splat_origin:
  key: dnd2024:reference/creature/centaur_trooper
splat_sources:
- title: SRD 5.2.1
  locator: pp. 272
  url: https://www.dndbeyond.com/srd
---

# Centaur Trooper

## Source text

<!-- SRD page 272 -->

Large Fey, Neutral Good\
AC 16 Initiative +2 (12)\
HP 45 (6d10 + 12)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 14 +2 +2\
Int 9 −1 −1 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Athletics +6, Perception +3\
Gear Breastplate, Longbow, Pike\
Senses Passive Perception 13\
Languages Elvish, Sylvan\
CR 2 (XP 450; PB +2)

### Actions

Multiattack. The centaur makes two attacks, using Pike or Longbow in any combination.

Pike. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10 + 4) Piercing damage.

Longbow. Ranged Attack Roll: +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage.

### Bonus Actions

Trampling Charge (Recharge 5–6). The centaur moves up to its Speed without provoking Opportunity Attacks and can move through the spaces of Medium or smaller creatures. Each creature whose space the centaur enters is targeted once by the following effect. Strength Saving Throw: DC 14. Failure: 7 (1d6 + 4) Bludgeoning damage, and the target has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/272|page 272]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Neutral Good ` |
| ` movement_text ` | ` 50 ft. ` |
| ` skills_text ` | ` Athletics +6, Perception +3 ` |
| ` gear_text ` | ` Breastplate, Longbow, Pike ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` Elvish, Sylvan ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 45 (6d10 + 12) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 9 ` | ` -1 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 45 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/centaur_trooper ` |
| ` implementation ` | ` dnd2024:creature_centaur_trooper ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 6 ` |
| ` perception ` | ` 3 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Item | Count |
| --- | --- | --- |
| ` row_1 ` | ` Breastplate ` | ` 1 ` |
| ` row_2 ` | ` Longbow ` | ` 1 ` |
| ` row_3 ` | ` Pike ` | ` 1 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["elvish", "sylvan"] ` |

#### Features

##### Pike

<!-- splat:yaml path=splat_rule/parameters/features/pike -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 10
      add: 4
    average: 9
```

##### Longbow

<!-- splat:yaml path=splat_rule/parameters/features/longbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 150
  long: 600
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 8
      add: 2
    average: 6
```

##### Trampling charge

<!-- splat:yaml path=splat_rule/parameters/features/trampling_charge -->

```yaml
economy: bonus_action
op: movement
maximum:
  multiply:
  - 1
  - actor_stat: speed
opportunity_attacks: false
through_creature_maximum_size: medium
on_enter_creature_space:
  maximum_per_creature: 1
  effect:
    op: saving_throw
    ability: strength
    dc: 14
    targets:
      kind: entered_creature
    failure:
    - op: damage
      components:
      - type: bludgeoning
        amount:
          dice:
            count: 1
            sides: 6
          add: 4
        average: 7
    - op: condition.apply
      condition: prone
    success: []
recharge:
  die_sides: 6
  success_min: 5
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 2
  choices:
  - pike
  - longbow
  combination: any
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
