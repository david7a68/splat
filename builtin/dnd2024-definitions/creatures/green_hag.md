---
splat_schema: 1
splat_id: 5e3caea4-c0c7-545a-aa9c-9f1c87c777e9
splat_kind: creature
splat_key: dnd2024:creatures/green_hag
splat_name: Green Hag
splat_origin:
  key: dnd2024:reference/creature/green_hag
splat_sources:
- title: SRD 5.2.1
  locator: pp. 295
  url: https://www.dndbeyond.com/srd
---

# Green Hag

## Source text

<!-- SRD page 295 -->

Medium Fey, Neutral Evil\
AC 17 Initiative +1 (11)\
HP 82 (11d8 + 33)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con 16 +3 +3\
Int 13 +1 +1 Wis 14 +2 +2 Cha 14 +2 +2\
Skills Arcana +5, Deception +4, Perception +4, Stealth +3\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Common, Elvish, Sylvan\
CR 3 (XP 700; PB +2)\

### Traits

Amphibious. The hag can breathe air and water.\
Coven Magic. While within 30 feet of at least two hag\
allies, the hag can cast one of the following spells,\
requiring no Material components, using the spell’s\
normal casting time, and using Intelligence as the spell-\
casting ability (spell save DC 11): Augury, Find Familiar,\
Identify, Locate Object, Scrying, or Unseen Servant.\
The hag must finish a Long Rest before using this trait to\
cast that spell again.\
Mimicry. The hag can mimic animal sounds and hu-\
manoid voices. A creature that hears the sounds can\
tell they are imitations only with a successful DC 14\
Wisdom (Insight) check.\

### Actions

Multiattack. The hag makes two Claw attacks.\
Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 + 4)\
Slashing damage plus 3 (1d6) Poison damage.\
Spellcasting. The hag casts one of the following spells,\
requiring no Material components and using Wisdom\
as the spellcasting ability (spell save DC 12, +4 to hit\
with spell attacks):\
At Will: Dancing Lights, Disguise Self (24-hour du-\
ration), Invisibility (self only, and the hag leaves no\
tracks while Invisible), Minor Illusion, Ray of Sickness\
(level 3 version)\

Original layout: [[../../srd-5.2.1/pages/295|page 295]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft., Swim 30 ft. ` |
| ` skills_text ` | ` Arcana +5, Deception +4, Perception +4, Stealth +3 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Common, Elvish, Sylvan ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 82 (11d8 + 33) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 13 ` | ` 1 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 14 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 82 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` swim_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/green_hag ` |
| ` implementation ` | ` dnd2024:creature_green_hag ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 8
  amount:
    dice:
      count: 1
      sides: 8
    add: 4
  damage_type: slashing
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: claw
  count: 2
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` wisdom ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 12 ` |
| ` spell_attack_bonus ` | ` 4 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- dancing_lights
- disguise_self
- invisibility
- minor_illusion
- ray_of_sickness
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Overrides duration hours:
    path: overrides/duration/hours
    type: integer
    optional: true
  Overrides targets:
    path: overrides/targets
    type: symbol
    optional: true
  Overrides while invisible leaves tracks:
    path: overrides/while_invisible/leaves_tracks
    type: boolean
    optional: true
  Details:
    type: section
    optional: true
-->

| Spell | Frequency | Overrides duration hours | Overrides targets | Overrides while invisible leaves tracks | Details |
| --- | --- | --- | --- | --- | --- |
| [Dancing Lights](<../spells/dancing_lights.md> "dnd2024:spells/dancing_lights") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-dancing_lights "section:dancing_lights") |
| [Disguise Self](<../spells/disguise_self.md> "dnd2024:spells/disguise_self") | ` At will ` | ` 24 ` |  |  | [Rules](#^splat-68a383d1-disguise_self "section:disguise_self") |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | ` At will ` |  | ` Self ` | false | [Rules](#^splat-68a383d1-invisibility "section:invisibility") |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-minor_illusion "section:minor_illusion") |
| [Ray of Sickness](<../spells/ray_of_sickness.md> "dnd2024:spells/ray_of_sickness") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-ray_of_sickness "section:ray_of_sickness") |

###### Dancing lights

^splat-68a383d1-dancing_lights

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@dancing_lights/cast_level -->

```yaml
spell_minimum
```

###### Disguise self

^splat-68a383d1-disguise_self

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@disguise_self/cast_level -->

```yaml
spell_minimum
```

###### Invisibility

^splat-68a383d1-invisibility

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@invisibility/cast_level -->

```yaml
spell_minimum
```

###### Minor illusion

^splat-68a383d1-minor_illusion

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@minor_illusion/cast_level -->

```yaml
spell_minimum
```

###### Ray of sickness

^splat-68a383d1-ray_of_sickness

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@ray_of_sickness/cast_level -->

```yaml
3
```

##### Mimicry

<!-- splat:yaml path=splat_rule/parameters/features/mimicry -->

```yaml
economy: passive
imitations:
- animal_sound
- humanoid_voice
discern:
  ability: wisdom
  skill: insight
  dc: 14
  requires_hearing: true
```

##### Coven magic

<!-- splat:values path=splat_rule/parameters/features/coven_magic -->

| Field | Value |
| --- | --- |
| ` economy ` | ` spell_normal_casting_time ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` intelligence ` |
| ` save_dc ` | ` 11 ` |
| ` components_waived ` | ` ["material"] ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/coven_magic/requires -->

| Field | Value |
| --- | --- |
| ` hag_allies_within_feet ` | ` 30 ` |
| ` minimum_hag_allies ` | ` 2 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/coven_magic/choices
key: Spell
order:
- augury
- find_familiar
- identify
- locate_object
- scrying
- unseen_servant
columns:
  Spell:
    path: spell
    type: reference
  Resource capacity:
    path: resource/capacity
    type: integer
  Resource cost:
    path: resource/cost
    type: integer
  Resource recovery:
    path: resource/recovery
    type: symbol
  Resource scope:
    path: resource/scope
    type: symbol
  Cast level:
    path: cast_level
    type: symbol
-->

| Spell | Resource capacity | Resource cost | Resource recovery | Resource scope | Cast level |
| --- | --- | --- | --- | --- | --- |
| [Augury](<../spells/augury.md> "dnd2024:spells/augury") | ` 1 ` | ` 1 ` | ` Long rest ` | ` Per spell ` | ` Spell minimum ` |
| [Find Familiar](<../spells/find_familiar.md> "dnd2024:spells/find_familiar") | ` 1 ` | ` 1 ` | ` Long rest ` | ` Per spell ` | ` Spell minimum ` |
| [Identify](<../spells/identify.md> "dnd2024:spells/identify") | ` 1 ` | ` 1 ` | ` Long rest ` | ` Per spell ` | ` Spell minimum ` |
| [Locate Object](<../spells/locate_object.md> "dnd2024:spells/locate_object") | ` 1 ` | ` 1 ` | ` Long rest ` | ` Per spell ` | ` Spell minimum ` |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` 1 ` | ` 1 ` | ` Long rest ` | ` Per spell ` | ` Spell minimum ` |
| [Unseen Servant](<../spells/unseen_servant.md> "dnd2024:spells/unseen_servant") | ` 1 ` | ` 1 ` | ` Long rest ` | ` Per spell ` | ` Spell minimum ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 5 ` |
| ` deception ` | ` 4 ` |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 3 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "elvish", "sylvan"] ` |
| ` understands ` | ` ["common", "elvish", "sylvan"] ` |

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
