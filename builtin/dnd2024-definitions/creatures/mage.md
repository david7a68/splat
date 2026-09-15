---
splat_schema: 1
splat_id: d74914a1-1960-56f1-b633-6cc3a498362d
splat_kind: creature
splat_key: dnd2024:creatures/mage
splat_name: Mage
splat_origin:
  key: dnd2024:reference/creature/mage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 305
  url: https://www.dndbeyond.com/srd
---

# Mage

## Source text

<!-- SRD page 305 -->

Medium or Small Humanoid (Wizard), Neutral\
AC 15 Initiative +2 (12)\
HP 81 (18d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 9 −1 −1 Dex 14 +2 +2 Con 11 +0 +0\
Int 17 +3 +6 Wis 12 +1 +4 Cha 11 +0 +0\
Skills Arcana +6, History +6, Perception +4\
Gear Wand\
Senses Passive Perception 14\
Languages Common plus three other languages\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The mage makes three Arcane\
Burst attacks.\
Arcane Burst. Melee or Ranged Attack Roll: +6, reach\
5 ft. or range 120 ft. Hit: 16 (3d8 + 3) Force damage.\
Spellcasting. The mage casts one of the following\
spells, using Intelligence as the spellcasting ability (spell\
save DC 14):\
At Will: Detect Magic, Light, Mage Armor (included in\
AC), Mage Hand, Prestidigitation\
2/Day Each: Fireball (level 4 version), Invisibility\
1/Day Each: Cone of Cold, Fly\

### Bonus Actions

Misty Step (3/Day). The mage casts Misty Step, using\
the same spellcasting ability as Spellcasting.\

### Reactions

Protective Magic (3/Day). The mage casts Counterspell\
or Shield in response to the spell’s trigger, using the\
same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/305|page 305]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` tags_text ` | ` (Wizard) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Arcana +6, History +6, Perception +4 ` |
| ` gear_text ` | ` Wand ` |
| ` senses_text ` | ` Passive Perception 14 ` |
| ` languages_text ` | ` Common plus three other languages ` |
| ` cr_text ` | ` 6 (XP 2,300; PB +3) ` |
| ` hit_points_text ` | ` 81 (18d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 9 ` | ` -1 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 17 ` | ` 6 ` |
| ` Wisdom ` | ` 12 ` | ` 4 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 81 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/mage ` |
| ` implementation ` | ` dnd2024:creature_mage ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: arcane_burst
  count: 3
```

##### Arcane burst

<!-- splat:yaml path=splat_rule/parameters/features/arcane_burst -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
  range_ft: 120
  long_range_ft: null
hit:
  damage:
  - average: 16
    roll:
      count: 3
      sides: 8
      modifier: 3
    type: force
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` intelligence ` |
| ` save_dc ` | ` 14 ` |
| ` material_components_available ` | ` true ` |

###### Components waived

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/components_waived -->

```yaml
[]
```

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_magic
- light
- mage_armor
- mage_hand
- prestidigitation
- fireball
- invisibility
- cone_of_cold
- fly
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: usage/kind
    type: symbol
  Included in header ac:
    path: included_in_header_ac
    type: boolean
    optional: true
  Uses:
    path: usage/uses
    type: integer
    optional: true
  Use pool:
    path: usage/pool
    type: symbol
    optional: true
  Recovery:
    path: usage/recovery
    type: symbol
    optional: true
  Cast level:
    path: cast_level
    type: integer
    optional: true
-->

| Spell | Frequency | Included in header ac | Uses | Use pool | Recovery | Cast level |
| --- | --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  |  |  |
| [Light](<../spells/light.md> "dnd2024:spells/light") | ` At will ` |  |  |  |  |  |
| [Mage Armor](<../spells/mage_armor.md> "dnd2024:spells/mage_armor") | ` At will ` | true |  |  |  |  |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | ` At will ` |  |  |  |  |  |
| [Prestidigitation](<../spells/prestidigitation.md> "dnd2024:spells/prestidigitation") | ` At will ` |  |  |  |  |  |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") | ` Daily ` |  | ` 2 ` | ` Per spell ` | ` Long rest ` | ` 4 ` |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | ` Daily ` |  | ` 2 ` | ` Per spell ` | ` Long rest ` |  |
| [Cone of Cold](<../spells/cone_of_cold.md> "dnd2024:spells/cone_of_cold") | ` Daily ` |  | ` 1 ` | ` Per spell ` | ` Long rest ` |  |
| [Fly](<../spells/fly.md> "dnd2024:spells/fly") | ` Daily ` |  | ` 1 ` | ` Per spell ` | ` Long rest ` |  |

##### Misty step

<!-- splat:yaml path=splat_rule/parameters/features/misty_step -->

```yaml
economy: bonus_action
resource:
  kind: daily
  uses: 3
  recovery: long_rest
operation: cast_spell
spell:
  key: dnd2024:spells/misty_step
ability: intelligence
```

##### Protective magic

<!-- splat:yaml path=splat_rule/parameters/features/protective_magic -->

```yaml
economy: reaction
resource:
  kind: daily
  uses: 3
  recovery: long_rest
operation: spell_choice
options:
- operation: cast_spell
  spell:
    key: dnd2024:spells/counterspell
- operation: cast_spell
  spell:
    key: dnd2024:spells/shield
trigger: chosen_spell.normal_trigger
ability: intelligence
save_dc: 14
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 6 ` |
| ` history ` | ` 6 ` |
| ` perception ` | ` 4 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | quantity |
| --- | --- | --- |
| ` 0 ` | ` wand ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
additional_languages:
  count: 3
  chooser: creator
  can_speak: true
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
