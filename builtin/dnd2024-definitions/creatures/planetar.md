---
splat_schema: 1
splat_id: 354c2bcd-4b81-50ec-8696-b361255d6067
splat_kind: creature
splat_key: dnd2024:creatures/planetar
splat_name: Planetar
splat_origin:
  key: dnd2024:reference/creature/planetar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 315
  url: https://www.dndbeyond.com/srd
---

# Planetar

## Source text

<!-- SRD page 315 -->

Large Celestial (Angel), Lawful Good\
AC 19 Initiative +10 (20)\
HP 262 (21d10 + 147)\
Speed 40 ft., Fly 120 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 24 +7 +12 Dex 20 +5 +5 Con 24 +7 +12\
Int 19 +4 +4 Wis 22 +6 +11 Cha 25 +7 +12\
Skills Perception +11\
Resistances Radiant\
Immunities Charmed, Exhaustion, Frightened\
Senses Truesight 120 ft.; Passive Perception 21\
Languages All; telepathy 120 ft.\
CR 16 (XP 15,000; PB +5)

### Traits

Divine Awareness. The planetar knows if it hears a lie.

Exalted Restoration. If the planetar dies outside Mount Celestia, its body disappears, and it gains a new body instantly, reviving with all its Hit Points somewhere in Mount Celestia.

Magic Resistance. The planetar has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The planetar makes three Radiant Sword attacks or uses Holy Burst twice.

Radiant Sword. Melee Attack Roll: +12, reach 10 ft. Hit: 14 (2d6 + 7) Slashing damage plus 18 (4d8) Radiant damage.

Holy Burst. Dexterity Saving Throw: DC 20, each enemy in a 20-foot-radius Sphere centered on a point the planetar can see within 120 feet. Failure: 24 (7d6) Radiant damage. Success: Half damage.

Spellcasting. The planetar casts one of the following spells, requiring no Material components and using Charisma as spellcasting ability (spell save DC 20):

At Will: Detect Evil and Good\
1/Day Each: Commune, Control Weather, Dispel Evil and Good, Raise Dead

### Bonus Actions

Divine Aid (2/Day). The planetar casts Cure Wounds, Invisibility, Lesser Restoration, or Remove Curse, using the same spellcasting ability as Spellcasting.

Original layout: [[../../srd-5.2.1/pages/315|page 315]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Lawful Good ` |
| ` tags_text ` | ` (Angel) ` |
| ` movement_text ` | ` 40 ft., Fly 120 ft. (hover) ` |
| ` skills_text ` | ` Perception +11 ` |
| ` resistances_text ` | ` Radiant ` |
| ` immunities_text ` | ` Charmed, Exhaustion, Frightened ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 21 ` |
| ` languages_text ` | ` All; telepathy 120 ft. ` |
| ` cr_text ` | ` 16 (XP 15,000; PB +5) ` |
| ` hit_points_text ` | ` 262 (21d10 + 147) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 24 ` | ` 12 ` |
| ` Dexterity ` | ` 20 ` | ` 5 ` |
| ` Constitution ` | ` 24 ` | ` 12 ` |
| ` Intelligence ` | ` 19 ` | ` 4 ` |
| ` Wisdom ` | ` 22 ` | ` 11 ` |
| ` Charisma ` | ` 25 ` | ` 12 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 19 ` |  |
| ` max_hp ` | ` 262 ` |  |
| ` initiative ` | ` 10 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 120 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/planetar ` |
| ` implementation ` | ` dnd2024:creature_planetar ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["radiant"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened"] ` |

#### Features

##### Divine awareness

<!-- splat:yaml path=splat_rule/parameters/features/divine_awareness -->

```yaml
economy: trait
operation: passive_rule
detect_lies:
  requires: heard
```

##### Exalted restoration

<!-- splat:yaml path=splat_rule/parameters/features/exalted_restoration -->

```yaml
economy: trait
operation: passive_rule
trigger: self.dies
unless:
  plane: mount_celestia
remove_old_body: true
new_body:
  delay_seconds: 0
  plane: mount_celestia
  hp: maximum
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: choice
  options:
  - operation: attack_sequence
    feature: radiant_sword
    count: 3
  - operation: repeat
    count: 2
    effect:
      operation: use_feature
      feature: holy_burst
```

##### Radiant sword

<!-- splat:values path=splat_rule/parameters/features/radiant_sword -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/radiant_sword/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 12
reach_ft: 10
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/radiant_sword/hit/damage
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
| ` row_1 ` | ` 14 ` | ` 2 ` | ` 6 ` | ` 7 ` | ` Slashing ` |
| ` row_2 ` | ` 18 ` | ` 4 ` | ` 8 ` | ` 0 ` | ` Radiant ` |

##### Holy burst

<!-- splat:yaml path=splat_rule/parameters/features/holy_burst -->

```yaml
economy: action
operation: saving_throw
ability: dexterity
dc: 20
targets:
  kind: enemy
  selection: each
  area:
    shape: sphere
    radius_ft: 20
    center:
      visible: true
      range_ft: 120
failure:
  damage:
  - average: 24
    roll:
      count: 7
      sides: 6
      modifier: 0
    type: radiant
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 20 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_evil_and_good
- commune
- control_weather
- dispel_evil_and_good
- raise_dead
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: usage/kind
    type: symbol
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
-->

| Spell | Frequency | Uses | Use pool | Recovery |
| --- | --- | --- | --- | --- |
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | ` At will ` |  |  |  |
| [Commune](<../spells/commune.md> "dnd2024:spells/commune") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Control Weather](<../spells/control_weather.md> "dnd2024:spells/control_weather") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Dispel Evil and Good](<../spells/dispel_evil_and_good.md> "dnd2024:spells/dispel_evil_and_good") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Raise Dead](<../spells/raise_dead.md> "dnd2024:spells/raise_dead") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Divine aid

<!-- splat:yaml path=splat_rule/parameters/features/divine_aid -->

```yaml
economy: bonus_action
resource:
  kind: daily
  uses: 2
  recovery: long_rest
operation: spell_choice
options:
- operation: cast_spell
  spell:
    key: dnd2024:spells/cure_wounds
- operation: cast_spell
  spell:
    key: dnd2024:spells/invisibility
- operation: cast_spell
  spell:
    key: dnd2024:spells/lesser_restoration
- operation: cast_spell
  spell:
    key: dnd2024:spells/remove_curse
ability: charisma
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 11 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` truesight ` | ` 120 ` |
| ` passive_perception ` | ` 21 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: all
understands: []
telepathy_range_ft: 120
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
