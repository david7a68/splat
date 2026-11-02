---
splat_schema: 1
splat_id: 735cd1e4-7c04-5899-8994-59dcb0679174
splat_kind: creature
splat_key: dnd2024:creatures/efreeti
splat_name: Efreeti
splat_origin:
  key: dnd2024:reference/creature/efreeti
splat_sources:
- title: SRD 5.2.1
  locator: pp. 283
  url: https://www.dndbeyond.com/srd
---

# Efreeti

## Source text

<!-- SRD page 283 -->

Large Elemental (Genie), Neutral\
AC 17 Initiative +1 (11)\
HP 212 (17d10 + 119)\
Speed 40 ft., Fly 60 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 12 +1 +1 Con 24 +7 +7\
Int 16 +3 +3 Wis 15 +2 +6 Cha 19 +4 +8\
Immunities Fire\
Senses Darkvision 120 ft.; Passive Perception 12\
Languages Primordial (Ignan)\
CR 11 (XP 7,200; PB +4)

### Traits

Elemental Restoration. If the efreeti dies outside the Elemental Plane of Fire, its body dissolves into ash, and it gains a new body in 1d4 days, reviving with all its Hit Points somewhere on the Plane of Fire.

Magic Resistance. The efreeti has Advantage on saving throws against spells and other magical effects.

Wishes. The efreeti has a 30 percent chance of knowing the Wish spell. If the efreeti knows it, the efreeti can cast it only on behalf of a non-genie creature who communicates a wish in a way the efreeti can understand. If the efreeti casts the spell for the creature, the efreeti suffers none of the spell’s stress. Once the efreeti has cast it three times, the efreeti can’t do so again for 365 days.

### Actions

Multiattack. The efreeti makes three attacks, using Heated Blade or Hurl Flame in any combination.

Heated Blade. Melee Attack Roll: +10, reach 5 ft. Hit: 13 (2d6 + 6) Slashing damage plus 13 (2d12) Fire damage.

Hurl Flame. Ranged Attack Roll: +8, range 120 ft. Hit: 24 (7d6) Fire damage.

Spellcasting. The efreeti casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 16):

At Will: Detect Magic, Elementalism\
1/Day Each: Gaseous Form, Invisibility, Major Image, Plane Shift, Tongues, Wall of Fire (level 7 version)

Original layout: [[../../srd-5.2.1/pages/283|page 283]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral ` |
| ` tags_text ` | ` (Genie) ` |
| ` movement_text ` | ` 40 ft., Fly 60 ft. (hover) ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Primordial (Ignan) ` |
| ` cr_text ` | ` 11 (XP 7,200; PB +4) ` |
| ` hit_points_text ` | ` 212 (17d10 + 119) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 22 ` | ` 6 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 24 ` | ` 7 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 15 ` | ` 6 ` |
| ` Charisma ` | ` 19 ` | ` 8 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 212 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/efreeti ` |
| ` implementation ` | ` dnd2024:creature_efreeti ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |
| ` passive_perception ` | ` 12 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["ignan"] ` |

#### Features

##### Elemental restoration

<!-- splat:yaml path=splat_rule/parameters/features/elemental_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane_not: elemental_plane_of_fire
body: disappears
revive:
  new_body: true
  plane: elemental_plane_of_fire
  location_choice: gm
  hp: maximum
  delay:
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    unit: day
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Wishes

<!-- splat:yaml path=splat_rule/parameters/features/wishes -->

```yaml
economy: passive
known_chance_percent: 30
spell:
  key: dnd2024:spells/wish
requester:
  excluded_subtype: genie
  wish_communicated_and_understood: true
caster_stress: false
uses: 3
after_third_cast_lockout:
  amount: 365
  unit: day
reset_uses_after_lockout: true
```

##### Heated blade

<!-- splat:yaml path=splat_rule/parameters/features/heated_blade -->

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
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 6
      add: 6
    average: 13
  - type: fire
    amount:
      dice:
        count: 2
        sides: 12
      add: 0
    average: 13
```

##### Hurl flame

<!-- splat:yaml path=splat_rule/parameters/features/hurl_flame -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 8
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 120
  long: 120
hit:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 7
        sides: 6
      add: 0
    average: 24
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
| ` ability ` | ` charisma ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 16 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_magic
- elementalism
- gaseous_form
- invisibility
- major_image
- plane_shift
- tongues
- wall_of_fire
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
| [Elementalism](<../spells/elementalism.md> "dnd2024:spells/elementalism") | true |  |  |  |
| [Gaseous Form](<../spells/gaseous_form.md> "dnd2024:spells/gaseous_form") |  | ` 1 ` | ` Long rest ` |  |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") |  | ` 1 ` | ` Long rest ` |  |
| [Major Image](<../spells/major_image.md> "dnd2024:spells/major_image") |  | ` 1 ` | ` Long rest ` |  |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") |  | ` 1 ` | ` Long rest ` |  |
| [Tongues](<../spells/tongues.md> "dnd2024:spells/tongues") |  | ` 1 ` | ` Long rest ` |  |
| [Wall of Fire](<../spells/wall_of_fire.md> "dnd2024:spells/wall_of_fire") |  | ` 1 ` | ` Long rest ` | ` 7 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 3
  choices:
  - heated_blade
  - hurl_flame
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
