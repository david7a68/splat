---
splat_schema: 1
splat_id: d587891f-84a1-59f8-aefb-70ae0130f5bc
splat_kind: creature
splat_key: dnd2024:creatures/ancient_bronze_dragon
splat_name: Ancient Bronze Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_bronze_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 271
  url: https://www.dndbeyond.com/srd
---

# Ancient Bronze Dragon

## Source text

<!-- SRD page 271 -->

Gargantuan Dragon (Metallic), Lawful Good\
AC 22 Initiative +14 (24)\
HP 444 (24d20 + 192)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 29 +9 +9 Dex 10 +0 +7 Con 27 +8 +8\
Int 18 +4 +4 Wis 17 +3 +10 Cha 25 +7 +7\
Skills Insight +10, Perception +17, Stealth +7\
Immunities Lightning\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 27\
Languages Common, Draconic\
CR 22 (XP 41,000, or 50,000 in lair; PB +7)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
It can replace one attack with a use of (A) Repulsion\
Breath or (B) Spellcasting to cast Guiding Bolt (level\
2 version).\
Rend. Melee Attack Roll: +16, reach 15 ft. Hit: 18 (2d8\
+ 9) Slashing damage plus 9 (2d8) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 23, each creature in a 120-foot-long,\
10-foot-wide Line. Failure: 82 (15d10) Lightning dam-\
age. Success: Half damage.\
Repulsion Breath. Strength Saving Throw: DC 23, each\
creature in a 30-foot Cone. Failure: The target is pushed\
up to 60 feet straight away from the dragon and has the\
Prone condition.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 22,\
+14 to hit with spell attacks):\
At Will: Detect Magic, Guiding Bolt (level 2 version),\
Shapechange (Beast or Humanoid form only, no\
Temporary Hit Points gained from the spell, and no\
Concentration or Temporary Hit Points required to\
maintain the spell), Speak with Animals, Thaumaturgy\
1/Day Each: Detect Thoughts, Control Water, Scrying,\
Water Breathing\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Guiding Light. The dragon uses Spellcasting to cast\
Guiding Bolt (level 2 version).\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\
Thunderclap. Constitution Saving Throw: DC 22, each\
creature in a 20-foot-radius Sphere centered on a point\
the dragon can see within 120 feet. Failure: 13 (3d8)\
Thunder damage, and the target has the Deafened con-\
dition until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/271|page 271]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` gargantuan ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Lawful Good ` |
| ` tags_text ` | ` (Metallic) ` |
| ` movement_text ` | ` 40 ft., Fly 80 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Insight +10, Perception +17, Stealth +7 ` |
| ` immunities_text ` | ` Lightning ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 22 (XP 41,000, or 50,000 in lair; PB +7) ` |
| ` hit_points_text ` | ` 444 (24d20 + 192) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 29 ` | ` 9 ` |
| ` Dexterity ` | ` 10 ` | ` 7 ` |
| ` Constitution ` | ` 27 ` | ` 8 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 17 ` | ` 10 ` |
| ` Charisma ` | ` 25 ` | ` 7 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 22 ` |  |
| ` max_hp ` | ` 444 ` |  |
| ` initiative ` | ` 14 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ancient_bronze_dragon ` |
| ` implementation ` | ` dnd2024:creature_ancient_bronze_dragon ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
resource:
  capacity: 4
  capacity_in_lair: 5
  recovery: long_rest
  cost: 1
trigger: saving_throw.failed
optional: true
op: replace_save_outcome
outcome: success
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 16
targets:
  count: 1
hit:
- op: damage
  average: 18
  amount:
    dice:
      count: 2
      sides: 8
    add: 9
  damage_type: slashing
- op: damage
  average: 9
  amount:
    dice:
      count: 2
      sides: 8
    add: 0
  damage_type: lightning
reach_feet: 15
```

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

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
dc: 23
targets:
  kind: creature
  count: all
  area:
    shape: line
    size_feet: 120
    origin: self
    width_feet: 10
failure:
- op: damage
  average: 82
  amount:
    dice:
      count: 15
      sides: 10
    add: 0
  damage_type: lightning
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
replacement:
  max_attacks: 1
  choices:
  - feature: repulsion_breath
  - feature: spellcasting
    spell:
      key: dnd2024:spells/guiding_bolt
    cast_level: 2
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
op: cast_spell_choice
ability: charisma
spells:
- spell:
    key: dnd2024:spells/detect_magic
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/guiding_bolt
  frequency: at_will
  cast_level: 2
- spell:
    key: dnd2024:spells/shapechange
  frequency: at_will
  overrides:
    form_types:
    - beast
    - humanoid
    temporary_hit_points_gained: 0
    concentration_required: false
    temporary_hit_points_to_maintain_required: false
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/speak_with_animals
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/thaumaturgy
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/detect_thoughts
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/control_water
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/scrying
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/water_breathing
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
components_waived:
- material
save_dc: 22
spell_attack_bonus: 14
```

##### Thunderclap

<!-- splat:yaml path=splat_rule/parameters/features/thunderclap -->

```yaml
economy: legendary_action
op: save
ability: constitution
dc: 22
targets:
  kind: creature
  count: all
  area:
    shape: sphere
    size_feet: 20
    origin: chosen_point
    origin_visible_to: self
    origin_range_feet: 120
failure:
- op: damage
  average: 13
  amount:
    dice:
      count: 3
      sides: 8
    add: 0
  damage_type: thunder
- op: condition
  condition: deafened
  ends:
    event: turn.end
    participant: target
    occurrence: next
success: []
```

##### Repulsion breath

<!-- splat:yaml path=splat_rule/parameters/features/repulsion_breath -->

```yaml
economy: action
op: save
ability: strength
dc: 23
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: push
  distance_max_feet: 60
  direction: straight_away_from_self
- op: condition
  condition: prone
success: []
```

##### Guiding light

<!-- splat:yaml path=splat_rule/parameters/features/guiding_light -->

```yaml
economy: legendary_action
op: cast_spell
spell:
  key: dnd2024:spells/guiding_bolt
via: spellcasting
cast_level: 2
```

##### Pounce

<!-- splat:yaml path=splat_rule/parameters/features/pounce -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: move
  distance_max:
    multiply:
    - stat: speed
    - numerator: 1
      denominator: 2
- op: use_feature
  feature: rend
  count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 10 ` |
| ` perception ` | ` 17 ` |
| ` stealth ` | ` 7 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["lightning"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 27 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 60 ` |
| ` 1 ` | ` darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "draconic"] ` |
| ` understands ` | ` ["common", "draconic"] ` |

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
uses_in_lair: 4
cost_per_action: 1
trigger: another_creature.turn.end
recovery:
  event: turn.start
  participant: self
  amount: all
maximum_per_trigger: 1
requires:
  self_incapacitated: false
  self_can_take_actions: true
```

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
