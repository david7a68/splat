---
splat_schema: 1
splat_id: 8b52c7cd-6dc1-5f02-949a-6df4b15a852c
splat_kind: creature
splat_key: dnd2024:creatures/flesh_golem
splat_name: Flesh Golem
splat_origin:
  key: dnd2024:reference/creature/flesh_golem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 285
  url: https://www.dndbeyond.com/srd
---

# Flesh Golem

## Source text

<!-- SRD page 285 -->

Medium Construct, Neutral\
AC 9 Initiative −1 (9)\
HP 127 (15d8 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 9 −1 −1 Con 18 +4 +4\
Int 6 −2 −2 Wis 10 +0 +0 Cha 5 −3 −3\
Immunities Lightning, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Understands Common plus one other language\
but can’t speak\
CR 5 (XP 1,800; PB +3)\

### Traits

Aversion to Fire. If the golem takes Fire damage, it has\
Disadvantage on attack rolls and ability checks until the\
end of its next turn.\
Berserk. Whenever the golem starts its turn Bloodied,\
roll 1d6. On a 6, the golem goes berserk. On each of\
its turns while berserk, the golem attacks the nearest\
creature it can see. If no creature is near enough to\
move to and attack, the golem attacks an object. Once\
the golem goes berserk, it remains so until it is de-\
stroyed or it is no longer Bloodied.\
The golem’s creator, if within 60 feet of the berserk\
golem, can try to calm it by taking an action to make a\
DC 15 Charisma (Persuasion) check; the golem must be\
able to hear its creator. If this check succeeds, the go-\
lem ceases being berserk until the start of its next turn,\
at which point it resumes rolling for the Berserk trait\
again if it is still Bloodied.\
Immutable Form. The golem can’t shape-shift.\
Lightning Absorption. Whenever the golem is sub-\
jected to Lightning damage, it regains a number of Hit\
Points equal to the Lightning damage dealt.\
Magic Resistance. The golem has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The golem makes two Slam attacks.\
Slam. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 +\
4) Bludgeoning damage plus 4 (1d8) Lightning damage.\

Original layout: [[../../srd-5.2.1/pages/285|page 285]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` construct ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Lightning, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Understands Common plus one other language but can’t speak ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 127 (15d8 + 60) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 9 ` | ` -1 ` |
| ` Constitution ` | ` 18 ` | ` 4 ` |
| ` Intelligence ` | ` 6 ` | ` -2 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 9 ` |  |
| ` max_hp ` | ` 127 ` |  |
| ` initiative ` | ` -1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/flesh_golem ` |
| ` implementation ` | ` dnd2024:creature_flesh_golem ` |

### Parameters

#### Features

##### Immutable form

<!-- splat:values path=splat_rule/parameters/features/immutable_form -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` shape_shift_allowed ` | ` false ` |

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

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

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
  average: 13
  amount:
    dice:
      count: 2
      sides: 8
    add: 4
  damage_type: bludgeoning
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 8
    add: 0
  damage_type: lightning
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: slam
  count: 2
```

##### Lightning absorption

<!-- splat:yaml path=splat_rule/parameters/features/lightning_absorption -->

```yaml
economy: passive
trigger: damage.subjected
damage_type: lightning
op: heal
amount:
  event_value: incoming_damage_of_matching_type
```

##### Aversion to fire

<!-- splat:yaml path=splat_rule/parameters/features/aversion_to_fire -->

```yaml
economy: passive
trigger: self.damage_taken
requires:
  damage_type: fire
disadvantage:
  rolls:
  - attack
  - ability_check
  ends:
    event: turn.end
    participant: self
    occurrence: next
```

##### Berserk

<!-- splat:values path=splat_rule/parameters/features/berserk -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` trigger ` | ` self.turn.start ` |
| ` state ` | ` berserk ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/berserk/requires -->

| Field | Value |
| --- | --- |
| ` self_bloodied ` | ` true ` |

###### Activation roll

<!-- splat:yaml path=splat_rule/parameters/features/berserk/activation_roll -->

```yaml
roll:
  dice:
    count: 1
    sides: 6
  add: 0
success:
- 6
```

###### While active

<!-- splat:yaml path=splat_rule/parameters/features/berserk/while_active -->

```yaml
turn_behavior:
  attack_target: nearest_visible_creature
  requires_reachable_with_move_and_attack: true
  fallback_target_kind: object
```

###### Ends any

<!-- splat:table
path: splat_rule/parameters/features/berserk/ends_any
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Self destroyed:
    path: self_destroyed
    type: boolean
    optional: true
  Self bloodied:
    path: self_bloodied
    type: boolean
    optional: true
-->

| Key | Self destroyed | Self bloodied |
| --- | --- | --- |
| ` row_1 ` | true |  |
| ` row_2 ` |  | false |

###### Suppress

<!-- splat:yaml path=splat_rule/parameters/features/berserk/suppress -->

```yaml
actor: creator
economy: action
range_feet: 60
self_can_hear_creator: true
check:
  ability: charisma
  skill: persuasion
  dc: 15
on_success:
  suppress_until:
    event: turn.start
    participant: self
    occurrence: next
  resume_activation_rolls_if_bloodied: true
```

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["lightning", "poison"] ` |
| ` conditions ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` understands ` | ` ["common"] ` |
| ` can_speak ` | ` false ` |

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` false ` |
| ` can_understand ` | ` true ` |

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
