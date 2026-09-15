---
splat_schema: 1
splat_id: ae3f8349-3c70-579f-abc5-99126ce42f69
splat_kind: creature
splat_key: dnd2024:creatures/cloud_giant
splat_name: Cloud Giant
splat_origin:
  key: dnd2024:reference/creature/cloud_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 275
  url: https://www.dndbeyond.com/srd
---

# Cloud Giant

## Source text

<!-- SRD page 275 -->

Huge Giant, Neutral\
AC 14 Initiative +4 (14)\
HP 200 (16d12 + 96)\
Speed 40 ft., Fly 20 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 10 +0 +0 Con 22 +6 +10\
Int 12 +1 +1 Wis 16 +3 +7 Cha 16 +3 +3\
Skills Insight +7, Perception +11\
Senses Passive Perception 21\
Languages Common, Giant\
CR 9 (XP 5,000; PB +4)\

### Actions

Multiattack. The giant makes two attacks, using Thun-\
derous Mace or Thundercloud in any combination. It\
can replace one attack with a use of Spellcasting to cast\
Fog Cloud.\
Thunderous Mace. Melee Attack Roll: +12, reach 10\
ft. Hit: 21 (3d8 + 8) Bludgeoning damage plus 7 (2d6)\
Thunder damage.\
Thundercloud. Ranged Attack Roll: +12, range 240\
ft. Hit: 18 (3d6 + 8) Thunder damage, and the target\
has the Incapacitated condition until the end of its\
next turn.\
Spellcasting. The giant casts one of the following spells,\
requiring no Material components and using Charisma\
as the spellcasting ability (spell save DC 15):\
At Will: Detect Magic, Fog Cloud, Light\
1/Day Each: Control Weather, Gaseous Form,\
Telekinesis\

### Bonus Actions

Misty Step. The giant casts the Misty Step spell, using\
the same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/275|page 275]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` giant ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 40 ft., Fly 20 ft. (hover) ` |
| ` skills_text ` | ` Insight +7, Perception +11 ` |
| ` senses_text ` | ` Passive Perception 21 ` |
| ` languages_text ` | ` Common, Giant ` |
| ` cr_text ` | ` 9 (XP 5,000; PB +4) ` |
| ` hit_points_text ` | ` 200 (16d12 + 96) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 27 ` | ` 8 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 22 ` | ` 10 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 16 ` | ` 7 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 200 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/cloud_giant ` |
| ` implementation ` | ` dnd2024:creature_cloud_giant ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 21 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 7 ` |
| ` perception ` | ` 11 ` |

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
| ` known ` | ` ["common", "giant"] ` |

#### Features

##### Thunderous mace

<!-- splat:yaml path=splat_rule/parameters/features/thunderous_mace -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 12
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 8
      add: 8
    average: 21
  - type: thunder
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
```

##### Thundercloud

<!-- splat:yaml path=splat_rule/parameters/features/thundercloud -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 12
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 240
  long: 240
hit:
- op: damage
  components:
  - type: thunder
    amount:
      dice:
        count: 3
        sides: 6
      add: 8
    average: 18
- op: condition.apply
  condition: incapacitated
  duration:
    until: target.next_turn_end
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
op: spellcasting
profile:
  ability: charisma
  waived_components:
  - material
  spells:
  - spell:
      key: dnd2024:spells/detect_magic
    uses:
      unlimited: true
  - spell:
      key: dnd2024:spells/fog_cloud
    uses:
      unlimited: true
  - spell:
      key: dnd2024:spells/light
    uses:
      unlimited: true
  - spell:
      key: dnd2024:spells/control_weather
    uses:
      count: 1
      recovery: long_rest
  - spell:
      key: dnd2024:spells/gaseous_form
    uses:
      count: 1
      recovery: long_rest
  - spell:
      key: dnd2024:spells/telekinesis
    uses:
      count: 1
      recovery: long_rest
  save_dc: 15
```

##### Misty step

<!-- splat:yaml path=splat_rule/parameters/features/misty_step -->

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
    key: dnd2024:spells/misty_step
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 2
  choices:
  - thunderous_mace
  - thundercloud
  combination: any
replace_attack:
  maximum: 1
  choices:
  - op: spell.cast
    spell:
      key: dnd2024:spells/fog_cloud
    profile: spellcasting
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
