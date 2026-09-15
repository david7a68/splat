---
splat_schema: 1
splat_id: 4e85999e-74ec-59f9-83c4-f05e2801c75a
splat_kind: creature
splat_key: dnd2024:creatures/merfolk_skirmisher
splat_name: Merfolk Skirmisher
splat_origin:
  key: dnd2024:reference/creature/merfolk_skirmisher
splat_sources:
- title: SRD 5.2.1
  locator: pp. 308
  url: https://www.dndbeyond.com/srd
---

# Merfolk Skirmisher

## Source text

<!-- SRD page 308 -->

Medium Elemental, Neutral\
AC 11 Initiative +1 (11)\
HP 11 (2d8 + 2)\
Speed 10 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 13 +1 +1 Con 12 +1 +1\
Int 11 +0 +0 Wis 14 +2 +2 Cha 12 +1 +1\
Senses Passive Perception 12\
Languages Common, Primordial (Aquan)\
CR 1/8 (XP 25; PB +2)\

### Traits

Amphibious. The merfolk can breathe air and water.\

### Actions

Ocean Spear. Melee or Ranged Attack Roll: +2, reach 5\
ft. or range 20/60 ft. Hit: 3 (1d6) Piercing damage plus 2\
(1d4) Cold damage. If the target is a creature, its Speed\
decreases by 10 feet until the end of its next turn. Hit or\
Miss: The spear magically returns to the merfolk’s hand\
immediately after a ranged attack.\

Original layout: [[../../srd-5.2.1/pages/308|page 308]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 10 ft., Swim 40 ft. ` |
| ` senses_text ` | ` Passive Perception 12 ` |
| ` languages_text ` | ` Common, Primordial (Aquan) ` |
| ` cr_text ` | ` 1/8 (XP 25; PB +2) ` |
| ` hit_points_text ` | ` 11 (2d8 + 2) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 11 ` | ` 0 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 12 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 11 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` swim_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/merfolk_skirmisher ` |
| ` implementation ` | ` dnd2024:creature_merfolk_skirmisher ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

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
| ` known ` | ` ["common", "primordial"] ` |
| ` primordial_dialects ` | ` ["aquan"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["air", "water"] ` |

##### Ocean spear

<!-- splat:yaml path=splat_rule/parameters/features/ocean_spear -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 2
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
        sides: 6
      add: 0
    average: 3
  - type: cold
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    average: 2
- op: speed.add
  amount_ft: -10
  duration:
    until: target.next_turn_end
  when:
    target_kind: creature
after_attack:
  when: ranged_attack_hit_or_miss
  op: item.return
  destination: actor_hand
  magical: true
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
