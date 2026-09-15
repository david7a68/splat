---
splat_schema: 1
splat_id: 7d5560c6-5f2f-5d25-90b2-f22462ec433f
splat_kind: creature
splat_key: dnd2024:creatures/priest_acolyte
splat_name: Priest Acolyte
splat_origin:
  key: dnd2024:reference/creature/priest_acolyte
splat_sources:
- title: SRD 5.2.1
  locator: pp. 316
  url: https://www.dndbeyond.com/srd
---

# Priest Acolyte

## Source text

<!-- SRD page 316 -->

Medium or Small Humanoid (Cleric), Neutral\
AC 13 Initiative +0 (10)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 10 +0 +0 Con 12 +1 +1\
Int 10 +0 +0 Wis 14 +2 +2 Cha 11 +0 +0\
Skills Medicine +4, Religion +2\
Gear Chain Shirt, Holy Symbol, Mace\
Senses Passive Perception 12\
Languages Common\
CR 1/4 (XP 50; PB +2)\

### Actions

Mace. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Bludgeoning damage plus 2 (1d4) Radiant damage.\
Radiant Flame. Ranged Attack Roll: +4, range 60 ft.\
Hit: 7 (2d6) Radiant damage.\
Spellcasting. The priest casts one of the following\
spells, using Wisdom as the spellcasting ability:\
At Will: Light, Thaumaturgy\

### Bonus Actions

Divine Aid (1/Day). The priest casts Bless, Healing\
Word, or Sanctuary, using the same spellcasting ability\
as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/316|page 316]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` tags_text ` | ` (Cleric) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Medicine +4, Religion +2 ` |
| ` gear_text ` | ` Chain Shirt, Holy Symbol, Mace ` |
| ` senses_text ` | ` Passive Perception 12 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 11 (2d8 + 2) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 11 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/priest_acolyte ` |
| ` implementation ` | ` dnd2024:creature_priest_acolyte ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` medicine ` | ` 4 ` |
| ` religion ` | ` 2 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | count |
| --- | --- | --- |
| ` 0 ` | ` chain_shirt ` | ` 1 ` |
| ` 1 ` | ` holy_symbol ` | ` 1 ` |
| ` 2 ` | ` mace ` | ` 1 ` |

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
| ` known ` | ` ["common"] ` |

#### Features

##### Mace

<!-- splat:yaml path=splat_rule/parameters/features/mace -->

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
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    average: 5
  - type: radiant
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    average: 2
```

##### Radiant flame

<!-- splat:yaml path=splat_rule/parameters/features/radiant_flame -->

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
  normal: 60
  long: 60
hit:
- op: damage
  components:
  - type: radiant
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
op: spellcasting
profile:
  ability: wisdom
  waived_components: []
  spells:
  - spell:
      key: dnd2024:spells/light
    uses:
      unlimited: true
  - spell:
      key: dnd2024:spells/thaumaturgy
    uses:
      unlimited: true
```

##### Divine aid

<!-- splat:yaml path=splat_rule/parameters/features/divine_aid -->

```yaml
economy: bonus_action
op: spellcasting
profile:
  ability: wisdom
  waived_components: []
  save_dc: 12
  ability_modifier: 2
  derived_dc:
    base: 8
    ability_modifier: 2
    proficiency_bonus: 2
choose_count: 1
choices:
- op: spell.cast
  spell:
    key: dnd2024:spells/bless
- op: spell.cast
  spell:
    key: dnd2024:spells/healing_word
- op: spell.cast
  spell:
    key: dnd2024:spells/sanctuary
uses:
  count: 1
  recovery: long_rest
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
