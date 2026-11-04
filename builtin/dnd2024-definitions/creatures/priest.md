---
splat_schema: 2
splat_id: 6fd4ab02-bb66-5826-9e5f-16e92b75e672
splat_kind: creature
splat_key: dnd2024:creatures/priest
splat_name: Priest
splat_origin: dnd2024:reference/creature/priest
splat_sources:
- SRD 5.2.1, pp. 316, https://www.dndbeyond.com/srd
---

# Priest


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid (Cleric) |
| Alignment | Neutral |
| Armor Class | 13 |
| Hit Points | 38 (7d8 + 7) |
| Initiative | +0 (10) |
| Speed | 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Medicine +7, Perception +5, Religion +5 |
| Senses | Passive Perception 15 |
| Languages | Common plus one other language |
| Gear | Chain Shirt, Holy Symbol, Mace |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 10 | +0 | +0 |
| Con | 12 | +1 | +1 |
| Int | 13 | +1 | +1 |
| Wis | 16 | +3 | +3 |
| Cha | 13 | +1 | +1 |


## Source text

<!-- SRD page 316 -->

Medium or Small Humanoid (Cleric), Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 13 Initiative +0 (10)\
> HP 38 (7d8 + 7)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 10 +0 +0 Con 12 +1 +1\
> Int 13 +1 +1 Wis 16 +3 +3 Cha 13 +1 +1\
> Skills Medicine +7, Perception +5, Religion +5\
> Gear Chain Shirt, Holy Symbol, Mace\
> Senses Passive Perception 15\
> Languages Common plus one other language\
> CR 2 (XP 450; PB +2)


### Actions

Multiattack. The priest makes two attacks, using Mace or Radiant Flame in any combination.

Mace. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage plus 5 (2d4) Radiant damage.

Radiant Flame. Ranged Attack Roll: +5, range 60 ft. Hit: 11 (2d10) Radiant damage.

Spellcasting. The priest casts one of the following spells, using Wisdom as the spellcasting ability (spell save DC 13):

At Will: Light, Thaumaturgy\
1/Day: Spirit Guardians

### Bonus Actions

Divine Aid (3/Day). The priest casts Bless, Dispel Magic, Healing Word, or Lesser Restoration, using the same spellcasting ability as Spellcasting.

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
| ` mechanic ` | ` dnd2024:creatures/priest ` |
| ` implementation ` | ` dnd2024:creature_priest ` |

### Parameters

#### Features

##### Mace

<!-- splat:yaml path=splat_rule/parameters/features/mace -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 6
  amount:
    dice:
      count: 1
      sides: 6
    add: 3
  damage_type: bludgeoning
- op: damage
  average: 5
  amount:
    dice:
      count: 2
      sides: 4
    add: 0
  damage_type: radiant
reach_feet: 5
```

##### Radiant flame

<!-- splat:yaml path=splat_rule/parameters/features/radiant_flame -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 10
    add: 0
  damage_type: radiant
range_feet: 60
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 2 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Mace ` |
| ` row_2 ` | ` Radiant flame ` |

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` wisdom ` |
| ` save_dc ` | ` 13 ` |
| ` required_material_components_available ` | ` true ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- light
- thaumaturgy
- spirit_guardians
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Cast level:
    path: cast_level
    type: symbol
  Uses:
    path: uses
    type: integer
    optional: true
  Resource scope:
    path: resource_scope
    type: symbol
    optional: true
  Recovery:
    path: recovery
    type: symbol
    optional: true
-->

| Spell | Frequency | Cast level | Uses | Resource scope | Recovery |
| --- | --- | --- | --- | --- | --- |
| [Light](<../spells/light.md> "dnd2024:spells/light") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Thaumaturgy](<../spells/thaumaturgy.md> "dnd2024:spells/thaumaturgy") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Spirit Guardians](<../spells/spirit_guardians.md> "dnd2024:spells/spirit_guardians") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

###### Components waived

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/components_waived -->

```yaml
[]
```

##### Divine aid

<!-- splat:values path=splat_rule/parameters/features/divine_aid -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` wisdom ` |
| ` save_dc ` | ` 13 ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/divine_aid/resource -->

| Field | Value |
| --- | --- |
| ` capacity ` | ` 3 ` |
| ` cost ` | ` 1 ` |
| ` recovery ` | ` long_rest ` |
| ` scope ` | ` shared_choices ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/divine_aid/choices
key: Spell
order:
- bless
- dispel_magic
- healing_word
- lesser_restoration
columns:
  Spell:
    path: spell
    type: reference
-->

| Spell |
| --- |
| [Bless](<../spells/bless.md> "dnd2024:spells/bless") |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") |
| [Healing Word](<../spells/healing_word.md> "dnd2024:spells/healing_word") |
| [Lesser Restoration](<../spells/lesser_restoration.md> "dnd2024:spells/lesser_restoration") |

###### Components waived

<!-- splat:yaml path=splat_rule/parameters/features/divine_aid/components_waived -->

```yaml
[]
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` medicine ` | ` 7 ` |
| ` perception ` | ` 5 ` |
| ` religion ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

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
  Item name:
    path: item_name
    type: string
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item name | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Chain Shirt ` | ` 1 ` |
| ` row_2 ` | ` Holy Symbol ` | ` 1 ` |
| ` row_3 ` | ` Mace ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
| ` can_understand ` | ` true ` |

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```
