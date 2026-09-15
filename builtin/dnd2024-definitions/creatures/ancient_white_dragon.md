---
splat_schema: 1
splat_id: 50fc4fa1-a370-5a94-8557-2cabebc8e410
splat_kind: creature
splat_key: dnd2024:creatures/ancient_white_dragon
splat_name: Ancient White Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_white_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 341
  url: https://www.dndbeyond.com/srd
---

# Ancient White Dragon

## Source text

<!-- SRD page 341 -->

Gargantuan Dragon (Chromatic), Chaotic Evil\
AC 20 Initiative +12 (22)\
HP 333 (18d20 + 144)\
Speed 40 ft., Burrow 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 26 +8 +8 Dex 10 +0 +6 Con26 +8 +8\
Int 10 +0 +0 Wis 13 +1 +7 Cha 18 +4 +4\
Skills Perception +13, Stealth +6\
Immunities Cold\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 23\
Languages Common, Draconic\
CR 20 (XP 25,000, or 33,000 in lair; PB +6)\

### Traits

Ice Walk. The dragon can move across and climb icy\
surfaces without needing to make an ability check.\
Additionally, Difficult Terrain composed of ice or snow\
doesn’t cost it extra movement.\
Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +14, reach 15 ft. Hit: 17 (2d8\
+ 8) Slashing damage plus 7 (2d6) Cold damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 22, each creature in a 90-foot Cone. Fail-\
ure: 63 (14d8) Cold damage. Success: Half damage.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Freezing Burst. Constitution Saving Throw: DC 20,\
each creature in a 30-foot-radius Sphere centered on\
a point the dragon can see within 120 feet. Failure: 14\
(4d6) Cold damage, and the target’s Speed is 0 until\
the end of the target’s next turn. Failure or Success: The\
dragon can’t take this action again until the start of its\
next turn.\
Frightful Presence. The dragon casts Fear, requiring no\
Material components and using Charisma as the spell-\
casting ability (spell save DC 18). The dragon can’t take\
this action again until the start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/341|page 341]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` gargantuan ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 40 ft., Burrow 40 ft., Fly 80 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Perception +13, Stealth +6 ` |
| ` immunities_text ` | ` Cold ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 23 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 20 (XP 25,000, or 33,000 in lair; PB +6) ` |
| ` hit_points_text ` | ` 333 (18d20 + 144) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 26 ` | ` 8 ` |
| ` Dexterity ` | ` 10 ` | ` 6 ` |
| ` Constitution ` | ` 26 ` | ` 8 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 13 ` | ` 7 ` |
| ` Charisma ` | ` 18 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 20 ` |  |
| ` max_hp ` | ` 333 ` |  |
| ` initiative ` | ` 12 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` burrow_speed ` | ` 40 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/ancient_white_dragon ` |
| ` implementation ` | ` dnd2024:creature_ancient_white_dragon ` |

### Parameters

#### Features

##### Ice walk

<!-- splat:yaml path=splat_rule/parameters/features/ice_walk -->

```yaml
economy: passive
movement_checks_waived:
  surfaces:
  - ice
  modes:
  - walk
  - climb
difficult_terrain_extra_cost_ignored:
- ice
- snow
```

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
attack_bonus: 14
targets:
  count: 1
hit:
- op: damage
  average: 17
  amount:
    dice:
      count: 2
      sides: 8
    add: 8
  damage_type: slashing
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: cold
reach_feet: 15
```

##### Cold breath

<!-- splat:yaml path=splat_rule/parameters/features/cold_breath -->

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
ability: constitution
dc: 22
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 90
    origin: self
failure:
- op: damage
  average: 63
  amount:
    dice:
      count: 14
      sides: 8
    add: 0
  damage_type: cold
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

##### Freezing burst

<!-- splat:yaml path=splat_rule/parameters/features/freezing_burst -->

```yaml
economy: legendary_action
op: save
ability: constitution
dc: 20
targets:
  kind: creature
  count: all
  area:
    shape: sphere
    size_feet: 30
    origin: chosen_point
    origin_visible_to: self
    origin_range_feet: 120
failure:
- op: damage
  average: 14
  amount:
    dice:
      count: 4
      sides: 6
    add: 0
  damage_type: cold
- op: set_speed
  feet: 0
  ends:
    event: turn.end
    participant: target
    occurrence: next
success: []
cooldown_ends:
  event: turn.start
  participant: self
  occurrence: next
```

##### Frightful presence

<!-- splat:yaml path=splat_rule/parameters/features/frightful_presence -->

```yaml
economy: legendary_action
op: cast_spell
spell:
  key: dnd2024:spells/fear
ability: charisma
save_dc: 18
components_waived:
- material
cooldown_ends:
  event: turn.start
  participant: self
  occurrence: next
cast_level: spell_minimum
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
| ` perception ` | ` 13 ` |
| ` stealth ` | ` 6 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 23 ` |

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
