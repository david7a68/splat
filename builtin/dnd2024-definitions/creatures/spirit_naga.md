---
splat_schema: 1
splat_id: 6ab88a22-af92-500a-9df9-44cebb0110d0
splat_kind: creature
splat_key: dnd2024:creatures/spirit_naga
splat_name: Spirit Naga
splat_origin:
  key: dnd2024:reference/creature/spirit_naga
splat_sources:
- title: SRD 5.2.1
  locator: pp. 329
  url: https://www.dndbeyond.com/srd
---

# Spirit Naga

## Source text

<!-- SRD page 329 -->

Large Fiend, Chaotic Evil\
AC 17 Initiative +3 (13)\
HP 135 (18d10 + 36)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 17 +3 +6 Con 14 +2 +5\
Int 16 +3 +3 Wis 15 +2 +5 Cha 16 +3 +6\
Immunities Poison; Charmed, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Abyssal, Common\
CR 8 (XP 3,900; PB +3)\

### Traits

Fiendish Restoration. If it dies, the naga returns to life\
in 1d6 days and regains all its Hit Points. Only a Wish\
spell can prevent this trait from functioning.\

### Actions

Multiattack. The naga makes three attacks, using Bite\
or Necrotic Ray in any combination.\
Bite. Melee Attack Roll: +7, reach 10 ft. Hit: 7 (1d6 + 4)\
Piercing damage plus 14 (4d6) Poison damage.\
Necrotic Ray. Ranged Attack Roll: +6, range 60 ft. Hit:\
21 (6d6) Necrotic damage.\
Spellcasting. The naga casts one of the following\
spells, requiring no Somatic or Material components\
and using Intelligence as the spellcasting ability (spell\
save DC 14):\
At Will: Detect Magic, Mage Hand, Minor Illusion, Wa-\
ter Breathing\
2/Day Each: Detect Thoughts, Dimension Door,\
Hold Person (level 3 version), Lightning Bolt (level\
4 version)\

Original layout: [[../../srd-5.2.1/pages/329|page 329]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 40 ft. ` |
| ` immunities_text ` | ` Poison; Charmed, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Abyssal, Common ` |
| ` cr_text ` | ` 8 (XP 3,900; PB +3) ` |
| ` hit_points_text ` | ` 135 (18d10 + 36) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 17 ` | ` 6 ` |
| ` Constitution ` | ` 14 ` | ` 5 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 15 ` | ` 5 ` |
| ` Charisma ` | ` 16 ` | ` 6 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 135 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/spirit_naga ` |
| ` implementation ` | ` dnd2024:creature_spirit_naga ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["charmed", "poisoned"] ` |
| ` passive_perception ` | ` 12 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "abyssal"] ` |

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 6
      add: 4
    average: 7
  - type: poison
    amount:
      dice:
        count: 4
        sides: 6
      add: 0
    average: 14
```

##### Necrotic ray

<!-- splat:yaml path=splat_rule/parameters/features/necrotic_ray -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 60
  long: 60
hit:
- op: damage
  components:
  - type: necrotic
    amount:
      dice:
        count: 6
        sides: 6
      add: 0
    average: 21
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
| ` ability ` | ` intelligence ` |
| ` waived_components ` | ` ["somatic", "material"] ` |
| ` save_dc ` | ` 14 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_magic
- mage_hand
- minor_illusion
- water_breathing
- detect_thoughts
- dimension_door
- hold_person
- lightning_bolt
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
  Cast level:
    path: cast_level
    type: integer
    optional: true
-->

| Spell | Uses unlimited | Uses count | Uses recovery | Cast level |
| --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |  |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | true |  |  |  |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | true |  |  |  |
| [Water Breathing](<../spells/water_breathing.md> "dnd2024:spells/water_breathing") | true |  |  |  |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") |  | ` 2 ` | ` Long rest ` |  |
| [Dimension Door](<../spells/dimension_door.md> "dnd2024:spells/dimension_door") |  | ` 2 ` | ` Long rest ` |  |
| [Hold Person](<../spells/hold_person.md> "dnd2024:spells/hold_person") |  | ` 2 ` | ` Long rest ` | ` 3 ` |
| [Lightning Bolt](<../spells/lightning_bolt.md> "dnd2024:spells/lightning_bolt") |  | ` 2 ` | ` Long rest ` | ` 4 ` |

##### Fiendish restoration

<!-- splat:yaml path=splat_rule/parameters/features/fiendish_restoration -->

```yaml
economy: passive
trigger: actor.died
op: revive
hp: maximum
delay:
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  unit: day
only_prevention_spell:
  key: dnd2024:spells/wish
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 3
  choices:
  - bite
  - necrotic_ray
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
