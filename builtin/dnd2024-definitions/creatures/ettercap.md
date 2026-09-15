---
splat_schema: 1
splat_id: 18a69e7f-9d87-5f7d-aacb-add6f23f142d
splat_kind: creature
splat_key: dnd2024:creatures/ettercap
splat_name: Ettercap
splat_origin:
  key: dnd2024:reference/creature/ettercap
splat_sources:
- title: SRD 5.2.1
  locator: pp. 284
  url: https://www.dndbeyond.com/srd
---

# Ettercap

## Source text

<!-- SRD page 284 -->

Medium Monstrosity, Neutral Evil\
AC 13  Initiative +2 (12)\
HP 44 (8d8 + 8)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 15 +2 +2 Con 13 +1 +1\
Int 7 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +3, Stealth +4, Survival +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Spider Climb. The ettercap can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Web Walker. The ettercap ignores movement re-\
strictions caused by webs, and the ettercap knows\
the location of any other creature in contact with\
the same web.\

### Actions

Multiattack. The ettercap makes one Bite attack and\
one Claw attack.\
Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage plus 2 (1d4) Poison damage, and the\
target has the Poisoned condition until the start of the\
ettercap’s next turn.\
Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2)\
Slashing damage.\
Web Strand (Recharge 5–6). Dexterity Saving Throw:\
DC 12, one Large or smaller creature the ettercap can\
see within 30 feet. Failure: The target has the Restrained\
condition until the web is destroyed (AC 10; HP 5;\
Vulnerability to Fire damage; Immunity to Bludgeoning,\
Poison, and Psychic damage).\

### Bonus Actions

Reel. The ettercap pulls one creature within 30 feet of\
itself that is Restrained by its Web Strand up to 25 feet\
straight toward itself.\

Original layout: [[../../srd-5.2.1/pages/284|page 284]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` skills_text ` | ` Perception +3, Stealth +4, Survival +3 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 44 (8d8 + 8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 7 ` | ` -2 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 8 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 44 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ettercap ` |
| ` implementation ` | ` dnd2024:creature_ettercap ` |

### Parameters

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` climb_difficult_surfaces ` | ` true ` |
| ` climb_ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Web walker

<!-- splat:values path=splat_rule/parameters/features/web_walker -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` ignore_web_movement_restrictions ` | ` true ` |
| ` locate_creatures_on_same_web ` | ` true ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count: 1
- operation: attack_sequence
  feature: claw
  count: 1
```

##### Bite

<!-- splat:values path=splat_rule/parameters/features/bite -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/bite/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 4
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/bite/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 1 ` | ` 6 ` | ` 2 ` | ` Piercing ` |
| ` row_2 ` | ` 2 ` | ` 1 ` | ` 4 ` | ` 0 ` | ` Poison ` |

###### Conditions

<!-- splat:rows path=splat_rule/parameters/features/bite/hit/conditions -->

| Row | condition | until |
| --- | --- | --- |
| ` 0 ` | ` poisoned ` | ` self.next_turn_start ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 7
    roll:
      count: 2
      sides: 4
      modifier: 2
    type: slashing
```

##### Web strand

<!-- splat:yaml path=splat_rule/parameters/features/web_strand -->

```yaml
economy: action
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
operation: saving_throw
ability: dexterity
dc: 12
targets:
  kind: creature
  count: 1
  range_ft: 30
  visible: true
  maximum_size: large
failure:
  conditions:
  - condition: restrained
    until: web.destroyed
  web_object:
    ac: 10
    hp: 5
    damage_vulnerabilities:
    - fire
    damage_immunities:
    - bludgeoning
    - poison
    - psychic
success:
  effects: []
```

##### Reel

<!-- splat:yaml path=splat_rule/parameters/features/reel -->

```yaml
economy: bonus_action
operation: pull
targets:
  kind: creature
  count: 1
  range_ft: 30
  restrained_by: self.web_strand
maximum_ft: 25
direction: straight_toward_self
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 4 ` |
| ` survival ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 13 ` |

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
