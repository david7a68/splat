---
splat_schema: 1
splat_id: 8743edd3-aa74-5b44-99f7-b3b30b153480
splat_kind: creature
splat_key: dnd2024:creatures/balor
splat_name: Balor
splat_origin:
  key: dnd2024:reference/creature/balor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261
  url: https://www.dndbeyond.com/srd
---

# Balor

## Source text

<!-- SRD page 261 -->

Huge Fiend (Demon), Chaotic Evil\
AC 19 Initiative +14 (24)\
HP 287 (23d12 + 138)\
Speed 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 26 +8 +8 Dex 15 +2 +2 Con 22 +6 +12\
Int 20 +5 +5 Wis 16 +3 +9 Cha 22 +6 +6\
Skills Perception +9\
Resistances Cold, Lightning\
Immunities Fire, Poison; Charmed, Frightened, Poisoned\
Senses Truesight 120 ft.; Passive Perception 19\
Languages Abyssal; telepathy 120 ft.\
CR 19 (XP 22,000; PB +6)\

### Traits

Death Throes. The balor explodes when it dies. Dex-\
terity Saving Throw: DC 20, each creature in a 30-foot\
Emanation originating from the balor. Failure: 31 (9d6)\
Fire damage plus 31 (9d6) Force damage. Success: Half\
damage. Failure or Success: If the balor dies outside the\
Abyss, it gains a new body instantly, reviving with all its\
Hit Points somewhere in the Abyss.\
Fire Aura. At the end of each of the balor’s turns, each\
creature in a 5-foot Emanation originating from the ba-\
lor takes 13 (3d8) Fire damage.\
Legendary Resistance (3/Day). If the balor fails a saving\
throw, it can choose to succeed instead.\
Magic Resistance. The balor has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The balor makes one Flame Whip attack\
and one Lightning Blade attack.\
Flame Whip. Melee Attack Roll: +14, reach 30 ft. Hit:\
18 (3d6 + 8) Force damage plus 17 (5d6) Fire damage. If\
the target is a Huge or smaller creature, the balor pulls\
the target up to 25 feet straight toward itself, and the\
target has the Prone condition.\
Lightning Blade. Melee Attack Roll: +14, reach 10 ft.\
Hit: 21 (3d8 + 8) Force damage plus 22 (4d10) Light-\
ning damage, and the target can’t take Reactions until\
the start of the balor’s next turn.\

### Bonus Actions

Teleport. The balor teleports itself or a willing demon\
within 10 feet of itself up to 60 feet to an unoccupied\
space the balor can see.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Demon) ` |
| ` movement_text ` | ` 40 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +9 ` |
| ` resistances_text ` | ` Cold, Lightning ` |
| ` immunities_text ` | ` Fire, Poison; Charmed, Frightened, Poisoned ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 19 ` |
| ` languages_text ` | ` Abyssal; telepathy 120 ft. ` |
| ` cr_text ` | ` 19 (XP 22,000; PB +6) ` |
| ` hit_points_text ` | ` 287 (23d12 + 138) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 26 ` | ` 8 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 22 ` | ` 12 ` |
| ` Intelligence ` | ` 20 ` | ` 5 ` |
| ` Wisdom ` | ` 16 ` | ` 9 ` |
| ` Charisma ` | ` 22 ` | ` 6 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 19 ` |  |
| ` max_hp ` | ` 287 ` |  |
| ` initiative ` | ` 14 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/balor ` |
| ` implementation ` | ` dnd2024:creature_balor ` |

### Parameters

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
resource:
  capacity: 3
  recovery: long_rest
  cost: 1
trigger: saving_throw.failed
optional: true
op: replace_save_outcome
outcome: success
```

##### Magic resistance

<!-- splat:yaml path=splat_rule/parameters/features/magic_resistance -->

```yaml
economy: passive
advantage:
  roll: saving_throw
  against:
  - spell
  - magical_effect
```

##### Flame whip

<!-- splat:yaml path=splat_rule/parameters/features/flame_whip -->

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
  average: 18
  amount:
    dice:
      count: 3
      sides: 6
    add: 8
  damage_type: force
- op: damage
  average: 17
  amount:
    dice:
      count: 5
      sides: 6
    add: 0
  damage_type: fire
- op: if
  test:
    target_kind: creature
    target_size_max: huge
  then:
  - op: pull
    distance_max_feet: 25
    direction: straight_toward_self
  - op: condition
    condition: prone
reach_feet: 30
```

##### Lightning blade

<!-- splat:yaml path=splat_rule/parameters/features/lightning_blade -->

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
  average: 21
  amount:
    dice:
      count: 3
      sides: 8
    add: 8
  damage_type: force
- op: damage
  average: 22
  amount:
    dice:
      count: 4
      sides: 10
    add: 0
  damage_type: lightning
- op: forbid_economy
  economy:
  - reaction
  ends:
    event: turn.start
    participant: self
    occurrence: next
reach_feet: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: flame_whip
  count: 1
- op: use_feature
  feature: lightning_blade
  count: 1
```

##### Death throes

<!-- splat:yaml path=splat_rule/parameters/features/death_throes -->

```yaml
economy: passive
trigger: self.died
op: sequence
steps:
- op: save
  ability: dexterity
  dc: 20
  targets:
    kind: creature
    count: all
    area:
      shape: emanation
      size_feet: 30
      origin: self
  failure:
  - op: damage
    average: 31
    amount:
      dice:
        count: 9
        sides: 6
      add: 0
    damage_type: fire
  - op: damage
    average: 31
    amount:
      dice:
        count: 9
        sides: 6
      add: 0
    damage_type: force
  success:
  - op: damage_fraction
    numerator: 1
    denominator: 2
    round: down
- op: if
  test:
    death_plane_not: abyss
  then:
  - op: revive
    new_body: true
    hit_points: maximum
    delay:
      seconds: 0
    destination:
      plane: abyss
      location_choice: gm
```

##### Fire aura

<!-- splat:yaml path=splat_rule/parameters/features/fire_aura -->

```yaml
economy: passive
trigger: self.turn.end
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 5
    origin: self
effects:
- op: damage
  average: 13
  amount:
    dice:
      count: 3
      sides: 8
    add: 0
  damage_type: fire
```

##### Teleport

<!-- splat:yaml path=splat_rule/parameters/features/teleport -->

```yaml
economy: bonus_action
op: teleport
targets:
  choice:
  - self
  - willing_demon
  target_range_feet: 10
range_feet: 60
destination:
  unoccupied: true
  visible_to: self
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 9 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold", "lightning"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire", "poison"] ` |
| ` conditions ` | ` ["charmed", "frightened", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 19 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` truesight ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["abyssal"] ` |
| ` understands ` | ` ["abyssal"] ` |

##### Telepathy

<!-- splat:values path=splat_rule/parameters/communication/telepathy -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 120 ` |

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
