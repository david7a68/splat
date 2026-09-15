---
splat_schema: 1
splat_id: a2dd7d7d-f61c-5fa6-a5d9-253d6a4e52de
splat_kind: creature
splat_key: dnd2024:creatures/stone_golem
splat_name: Stone Golem
splat_origin:
  key: dnd2024:reference/creature/stone_golem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 330
  url: https://www.dndbeyond.com/srd
---

# Stone Golem

## Source text

<!-- SRD page 330 -->

Large Construct, Unaligned\
AC 18 Initiative +3 (13)\
HP 220 (21d10 + 105)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 9 −1 −1 Con 20 +5 +5\
Int 3 −4 −4 Wis 11 +0 +0 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned\
Senses Darkvision 120 ft.; Passive Perception 10\
Languages Understands Common plus two other\
languages but can’t speak\
CR 10 (XP 5,900; PB +4)\

### Traits

Immutable Form. The golem can’t shape-shift.\
Magic Resistance. The golem has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The golem makes two attacks, using Slam\
or Force Bolt in any combination.\
Slam. Melee Attack Roll: +10, reach 5 ft. Hit: 15 (2d8\
+ 6) Bludgeoning damage plus 9 (2d8) Force damage.\
Force Bolt. Ranged Attack Roll: +9, range 120 ft. Hit:\
22 (4d10) Force damage.\

### Bonus Actions

Slow (Recharge 5–6). The golem casts the Slow spell,\
requiring no spell components and using Constitution\
as the spellcasting ability (spell save DC 17).\

Original layout: [[../../srd-5.2.1/pages/330|page 330]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` construct ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Understands Common plus two other languages but can’t speak ` |
| ` cr_text ` | ` 10 (XP 5,900; PB +4) ` |
| ` hit_points_text ` | ` 220 (21d10 + 105) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 22 ` | ` 6 ` |
| ` Dexterity ` | ` 9 ` | ` -1 ` |
| ` Constitution ` | ` 20 ` | ` 5 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 1 ` | ` -5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 220 ` |  |
| ` initiative ` | ` 3 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/stone_golem ` |
| ` implementation ` | ` dnd2024:creature_stone_golem ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |
| ` passive_perception ` | ` 10 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` false ` |
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 2 ` |

#### Features

##### Immutable form

<!-- splat:values path=splat_rule/parameters/features/immutable_form -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` shapechange.prohibit ` |

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 10
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
      add: 6
    average: 15
  - type: force
    amount:
      dice:
        count: 2
        sides: 8
      add: 0
    average: 9
```

##### Force bolt

<!-- splat:yaml path=splat_rule/parameters/features/force_bolt -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 9
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 120
  long: 120
hit:
- op: damage
  components:
  - type: force
    amount:
      dice:
        count: 4
        sides: 10
      add: 0
    average: 22
```

##### Slow

<!-- splat:yaml path=splat_rule/parameters/features/slow -->

```yaml
economy: bonus_action
op: spellcasting
profile:
  ability: constitution
  waived_components:
  - verbal
  - somatic
  - material
  save_dc: 17
choose_count: 1
choices:
- op: spell.cast
  spell:
    key: dnd2024:spells/slow
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
  - slam
  - force_bolt
  combination: any
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
