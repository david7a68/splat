---
splat_schema: 1
splat_id: 433c4671-5681-5c3f-9a38-f7eca923b6d5
splat_kind: creature
splat_key: dnd2024:creatures/otyugh
splat_name: Otyugh
splat_origin:
  key: dnd2024:reference/creature/otyugh
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313
  url: https://www.dndbeyond.com/srd
---

# Otyugh

## Source text

<!-- SRD page 313 -->

Large Aberration, Neutral\
AC 14 Initiative +0 (10)\
HP 104 (11d10 + 44)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 11 +0 +0 Con 19 +4 +7\
Int 6 −2 −2 Wis 13 +1 +1 Cha 6 −2 −2\
Senses Darkvision 120 ft.; Passive Perception 11\
Languages Otyugh; telepathy 120 ft. (doesn’t allow the\
receiving creature to respond telepathically)\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The otyugh makes one Bite attack and two\
Tentacle attacks.\
Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 12 (2d8 + 3)\
Piercing damage, and the target has the Poisoned con-\
dition. Whenever the Poisoned target finishes a Long\
Rest, it is subjected to the following effect. Constitution\
Saving Throw: DC 15. Failure: The target’s Hit Point\
maximum decreases by 5 (1d10) and doesn’t return to\
normal until the Poisoned condition ends on the target.\
Success: The Poisoned condition ends.\
Tentacle. Melee Attack Roll: +6, reach 10 ft. Hit: 12\
(2d8 + 3) Piercing damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 13) from one of two tentacles.\
Tentacle Slam. Constitution Saving Throw: DC 14, each\
creature Grappled by the otyugh. Failure: 16 (3d8 + 3)\
Bludgeoning damage, and the target has the Stunned\
condition until the start of the otyugh’s next turn. Suc-\
cess: Half damage only.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` aberration ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 30 ft. ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` Otyugh; telepathy 120 ft. (doesn’t allow the receiving creature to respond telepathically) ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 104 (11d10 + 44) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 19 ` | ` 7 ` |
| ` Intelligence ` | ` 6 ` | ` -2 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 104 ` |  |
| ` initiative ` | ` 0 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/otyugh ` |
| ` implementation ` | ` dnd2024:creature_otyugh ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["otyugh"] ` |

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` range_ft ` | ` 120 ` |
| ` two_way ` | ` false ` |

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 8
      add: 3
    average: 12
- op: condition.apply
  condition: poisoned
  on_target_long_rest_finished:
    op: saving_throw
    ability: constitution
    dc: 15
    targets:
      kind: target
    failure:
    - op: hp_maximum.reduce
      amount:
        dice:
          count: 1
          sides: 10
        add: 0
      average: 5
      recovery: this_poisoned_condition_ends
    success:
    - op: effect.end
```

##### Tentacle

<!-- splat:yaml path=splat_rule/parameters/features/tentacle -->

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
        count: 2
        sides: 8
      add: 3
    average: 12
- op: grapple
  escape_dc: 13
  when:
    target_kind: creature
    target_maximum_size: medium
  limb_slots: 2
  slots_per_target: 1
```

##### Tentacle slam

<!-- splat:yaml path=splat_rule/parameters/features/tentacle_slam -->

```yaml
economy: action
op: saving_throw
ability: constitution
dc: 14
targets:
  kind: creature
  count: all
  grappled_by: actor
failure:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 8
      add: 3
    average: 16
- op: condition.apply
  condition: stunned
  duration:
    until: actor.next_turn_start
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 1
  feature: bite
- count: 2
  feature: tentacle
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
