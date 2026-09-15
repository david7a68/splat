---
splat_schema: 1
splat_id: 2ffcad9b-b09c-5269-97ed-5a5993a212fe
splat_kind: creature
splat_key: dnd2024:creatures/giant_crocodile
splat_name: Giant Crocodile
splat_origin:
  key: dnd2024:reference/creature/giant_crocodile
splat_sources:
- title: SRD 5.2.1
  locator: pp. 350
  url: https://www.dndbeyond.com/srd
---

# Giant Crocodile

## Source text

<!-- SRD page 350 -->

Huge Beast, Unaligned\
AC 14  Initiative −1 (9)\
HP 85 (9d12 + 27)\
Speed 30 ft., Swim 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 9 −1 −1 Con 17 +3 +3\
Int 2 −4 −4 Wis 10 +0 +0 Cha 7 −2 −2\
Skills Stealth +5\
Senses Passive Perception 10\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Hold Breath. The crocodile can hold its breath\
for 1 hour.\

### Actions

Multiattack. The crocodile makes one Bite attack and\
one Tail attack.\
Bite. Melee Attack Roll: +8, reach 5 ft. Hit: 21 (3d10\
+ 5) Piercing damage. If the target is a Large or smaller\
creature, it has the Grappled condition (escape DC 15).\
While Grappled, the target has the Restrained condition\
and can’t be targeted by the crocodile’s Tail.\
Tail. Melee Attack Roll: +8, reach 10 ft. Hit: 18 (3d8\
+ 5) Bludgeoning damage. If the target is a Large or\
smaller creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/350|page 350]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Swim 50 ft. ` |
| ` skills_text ` | ` Stealth +5 ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 85 (9d12 + 27) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 21 ` | ` 5 ` |
| ` Dexterity ` | ` 9 ` | ` -1 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 85 ` |  |
| ` initiative ` | ` -1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` swim_speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_crocodile ` |
| ` implementation ` | ` dnd2024:creature_giant_crocodile ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 5 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Hold breath

<!-- splat:yaml path=splat_rule/parameters/features/hold_breath -->

```yaml
economy: passive
op: breathing.hold
duration:
  amount: 1
  unit: hour
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 8
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
        count: 3
        sides: 10
      add: 5
    average: 21
- op: grapple
  escape_dc: 15
  when:
    target_kind: creature
    target_maximum_size: large
  while_grappled:
  - op: condition.apply
    condition: restrained
  - op: targeting.prohibit
    feature: tail
    source: actor
```

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 8
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 8
      add: 5
    average: 18
- op: condition.apply
  condition: prone
  when:
    target_kind: creature
    target_maximum_size: large
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 1
  feature: bite
- count: 1
  feature: tail
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
