---
splat_schema: 1
splat_id: eab8c6aa-2a19-5d7b-a4d0-a327cfe7bc90
splat_kind: creature
splat_key: dnd2024:creatures/sea_hag
splat_name: Sea Hag
splat_origin:
  key: dnd2024:reference/creature/sea_hag
splat_sources:
- title: SRD 5.2.1
  locator: pp. 322
  url: https://www.dndbeyond.com/srd
---

# Sea Hag

## Source text

<!-- SRD page 322 -->

Medium Fey, Chaotic Evil\
AC 14  Initiative +1 (11)\
HP 52 (7d8 + 21)\
Speed 30 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 16 +3 +3\
Int 12 +1 +1 Wis 12 +1 +1 Cha 13 +1 +1\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common, Giant, Primordial (Aquan)\
CR 2 (XP 450; PB +2)\

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
Vile Appearance. Wisdom Saving Throw: DC 11, any\
Beast or Humanoid that starts its turn within 30 feet of\
the hag and can see the hag’s true form. Failure: The\
target has the Frightened condition until the start of its\
next turn. Success: The target is immune to this hag’s\
Vile Appearance for 24 hours.\

### Actions

Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage.\
Death Glare (Recharge 5–6). Wisdom Saving Throw:\
DC 11, one Frightened creature the hag can see within\
30 feet. Failure: If the target has 20 Hit Points or fewer,\
it drops to 0 Hit Points. Otherwise, the target takes 13\
(3d8) Psychic damage.\
Illusory Appearance. The hag casts Disguise Self, using\
Constitution as the spellcasting ability (spell save DC\
13). The spell’s duration is 24 hours.\

Original layout: [[../../srd-5.2.1/pages/322|page 322]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 30 ft., Swim 40 ft. ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` Common, Giant, Primordial (Aquan) ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 52 (7d8 + 21) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 13 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 52 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` swim_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/sea_hag ` |
| ` implementation ` | ` dnd2024:creature_sea_hag ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

##### Coven magic

<!-- splat:values path=splat_rule/parameters/features/coven_magic -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` intelligence ` |
| ` save_dc ` | ` 11 ` |
| ` components_waived ` | ` ["material"] ` |
| ` casting_time ` | ` spell_normal ` |
| ` material_components_available ` | ` true ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/coven_magic/requires -->

| Field | Value |
| --- | --- |
| ` hag_allies_within_ft ` | ` 30 ` |
| ` minimum_allies ` | ` 2 ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/coven_magic/options
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
  Frequency:
    path: usage/kind
    type: symbol
  Uses:
    path: usage/uses
    type: integer
  Recovery:
    path: usage/recovery
    type: symbol
  Use pool:
    path: usage/pool
    type: symbol
-->

| Spell | Frequency | Uses | Recovery | Use pool |
| --- | --- | --- | --- | --- |
| [Augury](<../spells/augury.md> "dnd2024:spells/augury") | ` Rest ` | ` 1 ` | ` Long rest ` | ` Per spell ` |
| [Find Familiar](<../spells/find_familiar.md> "dnd2024:spells/find_familiar") | ` Rest ` | ` 1 ` | ` Long rest ` | ` Per spell ` |
| [Identify](<../spells/identify.md> "dnd2024:spells/identify") | ` Rest ` | ` 1 ` | ` Long rest ` | ` Per spell ` |
| [Locate Object](<../spells/locate_object.md> "dnd2024:spells/locate_object") | ` Rest ` | ` 1 ` | ` Long rest ` | ` Per spell ` |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` Rest ` | ` 1 ` | ` Long rest ` | ` Per spell ` |
| [Unseen Servant](<../spells/unseen_servant.md> "dnd2024:spells/unseen_servant") | ` Rest ` | ` 1 ` | ` Long rest ` | ` Per spell ` |

##### Vile appearance

<!-- splat:yaml path=splat_rule/parameters/features/vile_appearance -->

```yaml
economy: trait
operation: saving_throw
ability: wisdom
dc: 11
targets:
  kind: creature
  count: 1
  range_ft: 30
  types:
  - beast
  - humanoid
  can_see_true_form: true
failure:
  conditions:
  - condition: frightened
    until: target.next_turn_start
trigger: target.turn_start
success:
  immunity:
    effect: this_vile_appearance
    duration_seconds: 86400
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 10
    roll:
      count: 2
      sides: 6
      modifier: 3
    type: slashing
```

##### Death glare

<!-- splat:yaml path=splat_rule/parameters/features/death_glare -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 5
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: wisdom
dc: 11
targets:
  kind: creature
  count: 1
  range_ft: 30
  visible: true
  required_conditions:
  - frightened
failure:
  if:
    target_current_hp_at_most: 20
  then:
    set_hp: 0
  else:
    damage:
    - roll:
        count: 3
        sides: 8
        modifier: 0
      type: psychic
success:
  effects: []
```

##### Illusory appearance

<!-- splat:yaml path=splat_rule/parameters/features/illusory_appearance -->

```yaml
economy: action
operation: cast_spell
spell:
  key: dnd2024:spells/disguise_self
ability: constitution
save_dc: 13
duration_seconds: 86400
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- giant
- language: primordial
  dialects:
  - aquan
understands: []
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
