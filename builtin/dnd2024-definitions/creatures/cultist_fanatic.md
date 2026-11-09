---
splat_schema: 2
splat_id: 49c2f231-18d1-5fb3-bc11-02824f17e924
splat_kind: creature
splat_key: dnd2024:creatures/cultist_fanatic
splat_name: Cultist Fanatic
splat_origin: dnd2024:reference/creature/cultist_fanatic
splat_sources:
- SRD 5.2.1, pp. 278, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 85, Habitat
---

# Cultist Fanatic


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 13 |
| Hit Points | 44 (8d8 + 8) |
| Initiative | +2 (12) |
| Speed | 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Deception +3, Persuasion +3, Religion +2 |
| Senses | Passive Perception 12 |
| Languages | Common |
| Gear | Holy Symbol, Leather Armor |
| Habitat | Any |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 14 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 10 | +0 | +0 |
| Wis | 14 | +2 | +4 |
| Cha | 13 | +1 | +1 |


## Source text

<!-- SRD page 278 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 13 Initiative +2 (12)\
> HP 44 (8d8 + 8)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 11 +0 +0 Dex 14 +2 +2 Con 12 +1 +1\
> Int 10 +0 +0 Wis 14 +2 +4 Cha 13 +1 +1\
> Skills Deception +3, Persuasion +3, Religion +2\
> Gear Holy Symbol, Leather Armor\
> Senses Passive Perception 12\
> Languages Common\
> CR 2 (XP 450; PB +2)


### Actions

Pact Blade. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage plus 7 (2d6) Necrotic damage.

Spellcasting. The cultist casts one of the following spells, using Wisdom as the spellcasting ability (spell save DC 12, +4 to hit with spell attacks):

At Will: Light, Thaumaturgy\
2/Day: Command\
1/Day: Hold Person

### Bonus Actions

Spiritual Weapon (2/Day). The cultist casts the Spiritual Weapon spell, using the same spellcasting ability as Spellcasting.

Original layout: [[../../srd-5.2.1/pages/278|page 278]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/cultist_fanatic ` |
| ` implementation ` | ` dnd2024:creature_cultist_fanatic ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 3 ` |
| ` persuasion ` | ` 3 ` |
| ` religion ` | ` 2 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
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
| ` row_1 ` | ` Holy symbol ` | ` 1 ` |
| ` row_2 ` | ` Leather armor ` | ` 1 ` |

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

##### Pact blade

<!-- splat:yaml path=splat_rule/parameters/features/pact_blade -->

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
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 8
      add: 2
    average: 6
  - type: necrotic
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
| ` save_dc ` | ` 12 ` |
| ` attack_bonus ` | ` 4 ` |

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
- command
- hold_person
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
    optional: true
  Uses count:
    path: uses/count
    type: integer
    optional: true
  Uses recovery:
    path: uses/recovery
    type: symbol
    optional: true
-->

| Spell | Uses unlimited | Uses count | Uses recovery |
| --- | --- | --- | --- |
| [Light](<../spells/light.md> "dnd2024:spells/light") | true |  |  |
| [Thaumaturgy](<../spells/thaumaturgy.md> "dnd2024:spells/thaumaturgy") | true |  |  |
| [Command](<../spells/command.md> "dnd2024:spells/command") |  | ` 2 ` | ` Long rest ` |
| [Hold Person](<../spells/hold_person.md> "dnd2024:spells/hold_person") |  | ` 1 ` | ` Long rest ` |

##### Spiritual weapon

<!-- splat:yaml path=splat_rule/parameters/features/spiritual_weapon -->

```yaml
economy: bonus_action
op: spellcasting
profile:
  ability: same_as_spellcasting
  waived_components: []
choose_count: 1
choices:
- op: spell.cast
  spell:
    key: dnd2024:spells/spiritual_weapon
uses:
  count: 2
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
