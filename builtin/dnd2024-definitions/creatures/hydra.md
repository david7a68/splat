---
splat_schema: 1
splat_id: b5a5c587-b6b9-5e9e-9dd9-ef58d6d3321d
splat_kind: creature
splat_key: dnd2024:creatures/hydra
splat_name: Hydra
splat_origin:
  key: dnd2024:reference/creature/hydra
splat_sources:
- title: SRD 5.2.1
  locator: pp. 300
  url: https://www.dndbeyond.com/srd
---

# Hydra

## Source text

<!-- SRD page 300 -->

Huge Monstrosity, Unaligned\
AC 15 Initiative +4 (14)\
HP 184 (16d12 + 80)\
Speed 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 12 +1 +1 Con 20 +5 +5\
Int 2 −4 −4 Wis 10 +0 +0 Cha 7 −2 −2\
Skills Perception +6\
Immunities Blinded, Charmed, Deafened, Frightened,\
Stunned, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 16\
Languages None\
CR 8 (XP 3,900; PB +3)\

### Traits

Hold Breath. The hydra can hold its breath for 1 hour.\
Multiple Heads. The hydra has five heads. Whenever\
the hydra takes 25 damage or more on a single turn,\
one of its heads dies. The hydra dies if all its heads are\
dead. At the end of each of its turns when it has at least\
one living head, the hydra grows two heads for each of\
its heads that died since its last turn, unless it has taken\
Fire damage since its last turn. The hydra regains 20 Hit\
Points when it grows new heads.\
Reactive Heads. For each head the hydra has beyond\
one, it gets an extra Reaction that can be used only for\
Opportunity Attacks.\

### Actions

Multiattack. The hydra makes as many Bite attacks as it\
has heads.\
Bite. Melee Attack Roll: +8, reach 10 ft. Hit: 10 (1d10\
+ 5) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/300|page 300]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 40 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Perception +6 ` |
| ` immunities_text ` | ` Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 16 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 8 (XP 3,900; PB +3) ` |
| ` hit_points_text ` | ` 184 (16d12 + 80) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 20 ` | ` 5 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 20 ` | ` 5 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 184 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/hydra ` |
| ` implementation ` | ` dnd2024:creature_hydra ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunities ` | ` ["blinded", "charmed", "deafened", "frightened", "stunned", "unconscious"] ` |

#### Features

##### Hold breath

<!-- splat:values path=splat_rule/parameters/features/hold_breath -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` hold_breath_seconds ` | ` 3600 ` |

##### Multiple heads

<!-- splat:yaml path=splat_rule/parameters/features/multiple_heads -->

```yaml
economy: trait
operation: passive_rule
initial_heads: 5
head_loss:
  trigger: damage_on_single_turn
  threshold: 25
  heads_lost: 1
dies_if_heads: 0
regrow:
  event: self.turn_end
  requires_living_head: true
  unless_fire_damage_since_last_turn: true
  heads_per_head_lost_since_last_turn: 2
  heal_per_regrowth_event: 20
```

##### Reactive heads

<!-- splat:yaml path=splat_rule/parameters/features/reactive_heads -->

```yaml
economy: trait
operation: passive_rule
extra_reaction_capacity:
  max:
  - 0
  - subtract:
    - stat: heads
    - 1
refresh_event: self.turn_start
recompute_only_at_refresh: true
allowed_uses:
- opportunity_attack
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count:
    stat: heads
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 8
  reach_ft: 10
hit:
  damage:
  - average: 10
    roll:
      count: 1
      sides: 10
      modifier: 5
    type: piercing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 16 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
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
