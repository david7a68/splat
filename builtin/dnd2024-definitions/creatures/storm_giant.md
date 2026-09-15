---
splat_schema: 1
splat_id: e042044b-02ee-5924-a25f-5af0cc5317c0
splat_kind: creature
splat_key: dnd2024:creatures/storm_giant
splat_name: Storm Giant
splat_origin:
  key: dnd2024:reference/creature/storm_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 330, 331
  url: https://www.dndbeyond.com/srd
---

# Storm Giant

## Source text

<!-- SRD page 330 -->

Huge Giant, Chaotic Good\
AC 16 Initiative +7 (17)\
HP 230 (20d12 + 100)\
Speed 50 ft., Fly 25 ft. (hover), Swim 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 29 +9 +14 Dex 14 +2 +2 Con 20 +5 +10\
Int 16 +3 +3 Wis 20 +5 +10 Cha 18 +4 +9\
Skills Arcana +8, Athletics +14, History +8, Perception +10\
Resistances Cold\
Immunities Lightning, Thunder\
Senses Darkvision 120 ft., Truesight 30 ft.;\
Passive Perception 20\
Languages Common, Giant\
CR 13 (XP 10,000; PB +5)\

<!-- SRD page 331 -->


### Traits

Amphibious. The giant can breathe air and water.\

### Actions

Multiattack. The giant makes two attacks, using Storm\
Sword or Thunderbolt in any combination.\
Storm Sword. Melee Attack Roll: +14, reach 10 ft.\
Hit: 23 (4d6 + 9) Slashing damage plus 13 (3d8) Light-\
ning damage.\
Thunderbolt. Ranged Attack Roll: +14, range 500 ft.\
Hit: 22 (2d12 + 9) Lightning damage, and the target has\
the Blinded and Deafened conditions until the start of\
the giant’s next turn.\
Lightning Storm (Recharge 5–6). Dexterity Saving\
Throw: DC 18, each creature in a 10-foot-radius,\
40-foot-high Cylinder originating from a point the giant\
can see within 500 feet. Failure: 55 (10d10) Lightning\
damage. Success: Half damage.\
Spellcasting. The giant casts one of the following spells,\
requiring no Material components and using Wisdom\
as the spellcasting ability (spell save DC 18):\
At Will: Detect Magic, Light\
1/Day: Control Weather\

Original layout: [[../../srd-5.2.1/pages/330|page 330]], [[../../srd-5.2.1/pages/331|page 331]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` giant ` |
| ` alignment ` | ` Chaotic Good ` |
| ` movement_text ` | ` 50 ft., Fly 25 ft. (hover), Swim 50 ft. ` |
| ` skills_text ` | ` Arcana +8, Athletics +14, History +8, Perception +10 ` |
| ` resistances_text ` | ` Cold ` |
| ` immunities_text ` | ` Lightning, Thunder ` |
| ` senses_text ` | ` Darkvision 120 ft., Truesight 30 ft.; Passive Perception 20 ` |
| ` languages_text ` | ` Common, Giant ` |
| ` cr_text ` | ` 13 (XP 10,000; PB +5) ` |
| ` hit_points_text ` | ` 230 (20d12 + 100) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 29 ` | ` 14 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 20 ` | ` 10 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 20 ` | ` 10 ` |
| ` Charisma ` | ` 18 ` | ` 9 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 230 ` |  |
| ` initiative ` | ` 7 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |
| ` fly_speed ` | ` 25 ` | ` ft ` |
| ` swim_speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/storm_giant ` |
| ` implementation ` | ` dnd2024:creature_storm_giant ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold"] ` |
| ` damage_immunities ` | ` ["lightning", "thunder"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - storm_sword
  - thunderbolt
```

##### Storm sword

<!-- splat:yaml path=splat_rule/parameters/features/storm_sword -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 14
  reach_ft: 10
hit:
  damage:
  - average: 23
    roll:
      count: 4
      sides: 6
      modifier: 9
    type: slashing
  - average: 13
    roll:
      count: 3
      sides: 8
      modifier: 0
    type: lightning
```

##### Thunderbolt

<!-- splat:yaml path=splat_rule/parameters/features/thunderbolt -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 14
  range_ft: 500
  long_range_ft: null
hit:
  damage:
  - average: 22
    roll:
      count: 2
      sides: 12
      modifier: 9
    type: lightning
  conditions:
  - condition: blinded
    until: self.next_turn_start
  - condition: deafened
    until: self.next_turn_start
```

##### Lightning storm

<!-- splat:yaml path=splat_rule/parameters/features/lightning_storm -->

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
ability: dexterity
dc: 18
targets:
  kind: creature
  selection: each
  area:
    shape: cylinder
    radius_ft: 10
    height_ft: 40
    origin:
      visible: true
      range_ft: 500
failure:
  damage:
  - average: 55
    roll:
      count: 10
      sides: 10
      modifier: 0
    type: lightning
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
operation: spellcasting
ability: wisdom
components_waived:
- material
save_dc: 18
options:
- spell:
    key: dnd2024:spells/detect_magic
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/light
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/control_weather
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
material_components_available: true
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 8 ` |
| ` athletics ` | ` 14 ` |
| ` history ` | ` 8 ` |
| ` perception ` | ` 10 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` truesight ` | ` 30 ` |
| ` passive_perception ` | ` 20 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- giant
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
