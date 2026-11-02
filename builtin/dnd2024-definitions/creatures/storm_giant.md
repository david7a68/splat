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
Senses Darkvision 120 ft., Truesight 30 ft.; Passive Perception 20\
Languages Common, Giant\
CR 13 (XP 10,000; PB +5)

<!-- SRD page 331 -->

### Traits

Amphibious. The giant can breathe air and water.

### Actions

Multiattack. The giant makes two attacks, using Storm Sword or Thunderbolt in any combination.

Storm Sword. Melee Attack Roll: +14, reach 10 ft. Hit: 23 (4d6 + 9) Slashing damage plus 13 (3d8) Lightning damage.

Thunderbolt. Ranged Attack Roll: +14, range 500 ft. Hit: 22 (2d12 + 9) Lightning damage, and the target has the Blinded and Deafened conditions until the start of the giant’s next turn.

Lightning Storm (Recharge 5–6). Dexterity Saving Throw: DC 18, each creature in a 10-foot-radius, 40-foot-high Cylinder originating from a point the giant can see within 500 feet. Failure: 55 (10d10) Lightning damage. Success: Half damage.

Spellcasting. The giant casts one of the following spells, requiring no Material components and using Wisdom as the spellcasting ability (spell save DC 18):

At Will: Detect Magic, Light\
1/Day: Control Weather

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

<!-- splat:values path=splat_rule/parameters/features/storm_sword -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/storm_sword/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 14
reach_ft: 10
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/storm_sword/hit/damage
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
| ` row_1 ` | ` 23 ` | ` 4 ` | ` 6 ` | ` 9 ` | ` Slashing ` |
| ` row_2 ` | ` 13 ` | ` 3 ` | ` 8 ` | ` 0 ` | ` Lightning ` |

##### Thunderbolt

<!-- splat:values path=splat_rule/parameters/features/thunderbolt -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/thunderbolt/attack -->

```yaml
mode: ranged
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 14
range_ft: 500
long_range_ft: null
```

###### Hit

###### Damage

<!-- splat:rows path=splat_rule/parameters/features/thunderbolt/hit/damage -->

| Row | average | type |
| --- | --- | --- |
| ` 0 ` | ` 22 ` | ` lightning ` |

###### Roll

<!-- splat:values path=splat_rule/parameters/features/thunderbolt/hit/damage/0/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sides ` | ` 12 ` |
| ` modifier ` | ` 9 ` |

###### Conditions

<!-- splat:table
path: splat_rule/parameters/features/thunderbolt/hit/conditions
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Condition:
    path: condition
    type: symbol
  Until:
    path: until
    type: string
-->

| Key | Condition | Until |
| --- | --- | --- |
| ` row_1 ` | ` Blinded ` | ` self.next_turn_start ` |
| ` row_2 ` | ` Deafened ` | ` self.next_turn_start ` |

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

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` wisdom ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 18 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_magic
- light
- control_weather
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
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  |
| [Light](<../spells/light.md> "dnd2024:spells/light") | ` At will ` |  |  |  |
| [Control Weather](<../spells/control_weather.md> "dnd2024:spells/control_weather") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

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
