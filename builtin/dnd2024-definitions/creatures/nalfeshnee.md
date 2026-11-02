---
splat_schema: 1
splat_id: 356c36da-db91-55f8-851e-b672168583ef
splat_kind: creature
splat_key: dnd2024:creatures/nalfeshnee
splat_name: Nalfeshnee
splat_origin:
  key: dnd2024:reference/creature/nalfeshnee
splat_sources:
- title: SRD 5.2.1
  locator: pp. 310, 311
  url: https://www.dndbeyond.com/srd
---

# Nalfeshnee

## Source text

<!-- SRD page 310 -->

Large Fiend (Demon), Chaotic Evil\
AC 18 Initiative +5 (15)\
HP 184 (16d10 + 96)\
Speed 20 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 10 +0 +0 Con 22 +6 +11\
Int 19 +4 +9 Wis 12 +1 +6 Cha 15 +2 +7\
Resistances Cold, Fire, Lightning\
Immunities Poison; Frightened, Poisoned\
Senses Truesight 120 ft.; Passive Perception 11\
Languages Abyssal; telepathy 120 ft.\
CR 13 (XP 10,000; PB +5)

### Traits

Demonic Restoration. If the nalfeshnee dies outside the Abyss, its body dissolves into ichor, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Abyss.

Magic Resistance. The nalfeshnee has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The nalfeshnee makes three Rend attacks.

Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 16 (2d10 + 5) Slashing damage plus 11 (2d10) Force damage.

Teleport. The nalfeshnee teleports up to 120 feet to an unoccupied space it can see.

### Bonus Actions

Horror Nimbus (Recharge 5–6). Wisdom Saving Throw: DC 15, each creature in a 15-foot Emanation originating from the nalfeshnee. Failure: 28 (8d6) Psychic damage, and the target has the Frightened condition for 1 minute, until it takes damage, or until it ends its turn with the nalfeshnee out of line of sight. Success: The target is immune to this nalfeshnee’s Horror Nimbus for 24 hours.

<!-- SRD page 311 -->

### Reactions

Pursuit. Trigger: Another creature the nalfeshnee can see ends its move within 120 feet of the nalfeshnee. Response: The nalfeshnee uses Teleport, but its destination space must be within 10 feet of the triggering creature.

Original layout: [[../../srd-5.2.1/pages/310|page 310]], [[../../srd-5.2.1/pages/311|page 311]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Demon) ` |
| ` movement_text ` | ` 20 ft., Fly 30 ft. ` |
| ` resistances_text ` | ` Cold, Fire, Lightning ` |
| ` immunities_text ` | ` Poison; Frightened, Poisoned ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` Abyssal; telepathy 120 ft. ` |
| ` cr_text ` | ` 13 (XP 10,000; PB +5) ` |
| ` hit_points_text ` | ` 184 (16d10 + 96) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 21 ` | ` 5 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 22 ` | ` 11 ` |
| ` Intelligence ` | ` 19 ` | ` 9 ` |
| ` Wisdom ` | ` 12 ` | ` 6 ` |
| ` Charisma ` | ` 15 ` | ` 7 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 184 ` |  |
| ` initiative ` | ` 5 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` fly_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/nalfeshnee ` |
| ` implementation ` | ` dnd2024:creature_nalfeshnee ` |

### Parameters

#### Features

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

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 10
targets:
  count: 1
hit:
- op: damage
  average: 16
  amount:
    dice:
      count: 2
      sides: 10
    add: 5
  damage_type: slashing
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 10
    add: 0
  damage_type: force
reach_feet: 10
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

##### Demonic restoration

<!-- splat:yaml path=splat_rule/parameters/features/demonic_restoration -->

```yaml
economy: passive
resource:
  capacity: 1
  recovery:
  - long_rest
  cost: 1
trigger: self.died
requires:
  death_plane_not: abyss
op: revive
new_body: true
old_body: disappears
delay:
  seconds: 0
hit_points: maximum
destination:
  plane: abyss
  location_choice: gm
```

##### Teleport

<!-- splat:yaml path=splat_rule/parameters/features/teleport -->

```yaml
economy: action
op: teleport
targets: self
range_feet: 120
destination:
  unoccupied: true
  visible_to: self
```

##### Horror nimbus

<!-- splat:yaml path=splat_rule/parameters/features/horror_nimbus -->

```yaml
economy: bonus_action
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
ability: wisdom
dc: 15
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 15
    origin: self
failure:
- op: damage
  average: 28
  amount:
    dice:
      count: 8
      sides: 6
    add: 0
  damage_type: psychic
- op: condition
  condition: frightened
  ends:
    any:
    - minutes: 1
    - event: target.damage_taken
    - event: target.turn.end
      self_out_of_target_line_of_sight: true
success:
- op: feature_immunity
  source: self
  feature: horror_nimbus
  duration:
    hours: 24
```

##### Pursuit

<!-- splat:yaml path=splat_rule/parameters/features/pursuit -->

```yaml
economy: reaction
trigger: another_creature.move.end
requires:
  triggering_creature_visible: true
  triggering_creature_distance_max_feet: 120
op: use_feature
feature: teleport
destination_constraints:
  distance_to_triggering_creature_max_feet: 10
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold", "fire", "lightning"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["frightened", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

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
