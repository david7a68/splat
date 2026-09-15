---
splat_schema: 1
splat_id: 77fa8634-4077-5e9f-970a-36508e19b9f4
splat_kind: creature
splat_key: dnd2024:creatures/glabrezu
splat_name: Glabrezu
splat_origin:
  key: dnd2024:reference/creature/glabrezu
splat_sources:
- title: SRD 5.2.1
  locator: pp. 289
  url: https://www.dndbeyond.com/srd
---

# Glabrezu

## Source text

<!-- SRD page 289 -->

Large Fiend (Demon), Chaotic Evil\
AC 17 Initiative +6 (16)\
HP 189 (18d10 + 90)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +9 Dex 15 +2 +2 Con 21 +5 +9\
Int 19 +4 +4 Wis 17 +3 +7 Cha 16 +3 +7\
Skills Deception +7, Perception +7\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Truesight 120 ft.; Passive Perception 17\
Languages Abyssal; telepathy 120 ft.\
CR 9 (XP 5,000; PB +4)\

### Traits

Demonic Restoration. If the glabrezu dies outside the\
Abyss, its body dissolves into ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Abyss.\
Magic Resistance. The glabrezu has Advantage on sav-\
ing throws against spells and other magical effects.\

### Actions

Multiattack. The glabrezu makes two Pincer attacks\
and uses Pummel or Spellcasting.\
Pincer. Melee Attack Roll: +9, reach 10 ft. Hit: 16 (2d10\
+ 5) Slashing damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 15) from one of two pincers.\
Pummel. Dexterity Saving Throw: DC 17, one creature\
Grappled by the glabrezu. Failure: 15 (3d6 + 5) Blud-\
geoning damage. Success: Half damage.\
Spellcasting. The glabrezu casts one of the following\
spells, requiring no Material components and using In-\
telligence as the spellcasting ability (spell save DC 16):\
At Will: Darkness, Detect Magic, Dispel Magic\
1/Day Each: Confusion, Fly, Power Word Stun\

Original layout: [[../../srd-5.2.1/pages/289|page 289]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Demon) ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Deception +7, Perception +7 ` |
| ` resistances_text ` | ` Cold, Fire, Lightning ` |
| ` immunities_text ` | ` Poison; Poisoned ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 17 ` |
| ` languages_text ` | ` Abyssal; telepathy 120 ft. ` |
| ` cr_text ` | ` 9 (XP 5,000; PB +4) ` |
| ` hit_points_text ` | ` 189 (18d10 + 90) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 20 ` | ` 9 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 21 ` | ` 9 ` |
| ` Intelligence ` | ` 19 ` | ` 4 ` |
| ` Wisdom ` | ` 17 ` | ` 7 ` |
| ` Charisma ` | ` 16 ` | ` 7 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 189 ` |  |
| ` initiative ` | ` 6 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/glabrezu ` |
| ` implementation ` | ` dnd2024:creature_glabrezu ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire", "lightning"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` passive_perception ` | ` 17 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 7 ` |
| ` perception ` | ` 7 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` truesight ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["abyssal"] ` |

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` range_ft ` | ` 120 ` |
| ` two_way ` | ` true ` |

#### Features

##### Demonic restoration

<!-- splat:yaml path=splat_rule/parameters/features/demonic_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane_not: abyss
body: disappears
revive:
  new_body: true
  plane: abyss
  location_choice: gm
  hp: maximum
  delay:
    amount: 0
    unit: second
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Pincer

<!-- splat:yaml path=splat_rule/parameters/features/pincer -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 9
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 10
      add: 5
    average: 16
- op: grapple
  escape_dc: 15
  when:
    target_kind: creature
    target_maximum_size: medium
  limb_slots: 2
  slots_per_target: 1
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
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 16 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- darkness
- detect_magic
- dispel_magic
- confusion
- fly
- power_word_stun
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
| [Darkness](<../spells/darkness.md> "dnd2024:spells/darkness") | true |  |  |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") | true |  |  |
| [Confusion](<../spells/confusion.md> "dnd2024:spells/confusion") |  | ` 1 ` | ` Long rest ` |
| [Fly](<../spells/fly.md> "dnd2024:spells/fly") |  | ` 1 ` | ` Long rest ` |
| [Power Word Stun](<../spells/power_word_stun.md> "dnd2024:spells/power_word_stun") |  | ` 1 ` | ` Long rest ` |

##### Pummel

<!-- splat:yaml path=splat_rule/parameters/features/pummel -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 17
targets:
  kind: creature
  count: 1
  grappled_by: actor
failure:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 6
      add: 5
    average: 15
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
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
    optional: true
  Count:
    path: count
    type: integer
  Choose:
    path: choose
    type: yaml
    optional: true
-->

| Key | Feature | Count | Choose |
| --- | --- | --- | --- |
| ` row_1 ` | ` Pincer ` | ` 2 ` |  |
| ` row_2 ` |  | ` 1 ` | ` ["pummel", "spellcasting"] ` |

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
