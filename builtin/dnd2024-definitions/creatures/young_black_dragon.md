---
splat_schema: 1
splat_id: 4a526619-386c-5f7a-b204-951f64027eef
splat_kind: creature
splat_key: dnd2024:creatures/young_black_dragon
splat_name: Young Black Dragon
splat_origin:
  key: dnd2024:reference/creature/young_black_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 264
  url: https://www.dndbeyond.com/srd
---

# Young Black Dragon

## Source text

<!-- SRD page 264 -->

Large Dragon (Chromatic), Chaotic Evil\
AC 18 Initiative +5 (15)\
HP 127 (15d10 + 45)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +5 Con 17 +3 +3\
Int 12 +1 +1 Wis 11 +0 +3 Cha 15 +2 +2\
Skills Perception +6, Stealth +5\
Immunities Acid\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 16\
Languages Common, Draconic\
CR 7 (XP 2,900; PB +3)\

### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 9 (2d4\
+ 4) Slashing damage plus 3 (1d6) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 14, each creature in a 30-foot-long, 5-foot-\
wide Line. Failure: 49 (14d6) Acid damage. Success:\
Half damage.\

Original layout: [[../../srd-5.2.1/pages/264|page 264]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 40 ft., Fly 80 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Perception +6, Stealth +5 ` |
| ` immunities_text ` | ` Acid ` |
| ` senses_text ` | ` Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 7 (XP 2,900; PB +3) ` |
| ` hit_points_text ` | ` 127 (15d10 + 45) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 14 ` | ` 5 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 11 ` | ` 3 ` |
| ` Charisma ` | ` 15 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 127 ` |  |
| ` initiative ` | ` 5 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/young_black_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_black_dragon ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 9
  amount:
    dice:
      count: 2
      sides: 4
    add: 4
  damage_type: slashing
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  damage_type: acid
reach_feet: 10
```

##### Acid breath

<!-- splat:yaml path=splat_rule/parameters/features/acid_breath -->

```yaml
economy: action
resource:
  capacity: 1
  initial: 1
  recharge:
    event: turn.start
    participant: self
    roll:
      dice:
        count: 1
        sides: 6
      add: 0
    succeeds_on:
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: dexterity
dc: 14
targets:
  kind: creature
  count: all
  area:
    shape: line
    size_feet: 30
    origin: self
    width_feet: 5
failure:
- op: damage
  average: 49
  amount:
    dice:
      count: 14
      sides: 6
    add: 0
  damage_type: acid
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: rend
  count: 3
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["acid"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 30 ` |
| ` 1 ` | ` darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "draconic"] ` |
| ` understands ` | ` ["common", "draconic"] ` |

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
