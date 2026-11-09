---
splat_schema: 2
splat_id: 7d5560c6-5f2f-5d25-90b2-f22462ec433f
splat_kind: creature
splat_key: dnd2024:creatures/priest_acolyte
splat_name: Priest Acolyte
splat_origin: dnd2024:reference/creature/priest_acolyte
splat_sources:
- SRD 5.2.1, pp. 316, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 247, Habitat
---

# Priest Acolyte


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid (Cleric) |
| Alignment | Neutral |
| Armor Class | 13 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +0 (10) |
| Speed | 30 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Medicine +4, Religion +2 |
| Senses | Passive Perception 12 |
| Languages | Common |
| Gear | Chain Shirt, Holy Symbol, Mace |
| Habitat | Any |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 10 | +0 | +0 |
| Con | 12 | +1 | +1 |
| Int | 10 | +0 | +0 |
| Wis | 14 | +2 | +2 |
| Cha | 11 | +0 | +0 |


## Source text

<!-- SRD page 316 -->

Medium or Small Humanoid (Cleric), Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 13 Initiative +0 (10)\
> HP 11 (2d8 + 2)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 10 +0 +0 Con 12 +1 +1\
> Int 10 +0 +0 Wis 14 +2 +2 Cha 11 +0 +0\
> Skills Medicine +4, Religion +2\
> Gear Chain Shirt, Holy Symbol, Mace\
> Senses Passive Perception 12\
> Languages Common\
> CR 1/4 (XP 50; PB +2)


### Actions

Mace. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Bludgeoning damage plus 2 (1d4) Radiant damage.

Radiant Flame. Ranged Attack Roll: +4, range 60 ft. Hit: 7 (2d6) Radiant damage.

Spellcasting. The priest casts one of the following spells, using Wisdom as the spellcasting ability:

At Will: Light, Thaumaturgy

### Bonus Actions

Divine Aid (1/Day). The priest casts Bless, Healing Word, or Sanctuary, using the same spellcasting ability as Spellcasting.

Original layout: [[../../srd-5.2.1/pages/316|page 316]].







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
| ` row_1 ` | ` Chain shirt ` | ` 1 ` |
| ` row_2 ` | ` Holy symbol ` | ` 1 ` |
| ` row_3 ` | ` Mace ` | ` 1 ` |

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

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` spellcasting ` |

###### Profile

<!-- splat:values path=splat_rule/parameters/features/spellcasting/profile -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |

###### Waived components

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/profile/waived_components -->

```yaml
[]
```

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- light
- thaumaturgy
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
-->

| Spell | Uses unlimited |
| --- | --- |
| [Light](<../spells/light.md> "dnd2024:spells/light") | true |
| [Thaumaturgy](<../spells/thaumaturgy.md> "dnd2024:spells/thaumaturgy") | true |

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
