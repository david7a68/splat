---
splat_schema: 2
splat_id: 5a1e2ab8-e9aa-5c8b-ba09-8d5ccbbfc4da
splat_kind: creature
splat_key: dnd2024:creatures/troll
splat_name: Troll
splat_origin: dnd2024:reference/creature/troll
splat_sources:
- SRD 5.2.1, pp. 333, https://www.dndbeyond.com/srd
---

# Troll


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Giant |
| Alignment | Chaotic Evil |
| Armor Class | 15 |
| Hit Points | 94 (9d10 + 45) |
| Initiative | +1 (11) |
| Speed | 30 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Perception +5 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | Giant |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 13 | +1 | +1 |
| Con | 20 | +5 | +5 |
| Int | 7 | -2 | -2 |
| Wis | 9 | -1 | -1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 333 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Giant, Chaotic Evil\
> AC 15 Initiative +1 (11)\
> HP 94 (9d10 + 45)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 13 +1 +1 Con 20 +5 +5\
> Int 7 −2 −2 Wis 9 −1 −1 Cha 7 −2 −2\
> Skills Perception +5\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages Giant\
> CR 5 (XP 1,800; PB +3)


### Traits

Loathsome Limbs (4/Day). If the troll ends any turn Bloodied and took 15+ Slashing damage during that turn, one of the troll’s limbs is severed, falls into the troll’s space, and becomes a Troll Limb. The limb acts immediately after the troll’s turn. The troll has 1 Exhaustion level for each missing limb, and it grows replacement limbs the next time it regains Hit Points.

Regeneration. The troll regains 15 Hit Points at the start of each of its turns. If the troll takes Acid or Fire damage, this trait doesn’t function on the troll’s next turn. The troll dies only if it starts its turn with 0 Hit Points and doesn’t regenerate.

### Actions

Multiattack. The troll makes three Rend attacks.

Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6 + 4) Slashing damage.

### Bonus Actions

Charge. The troll moves up to half its Speed straight toward an enemy it can see.

Original layout: [[../../srd-5.2.1/pages/333|page 333]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/troll ` |
| ` implementation ` | ` dnd2024:creature_troll ` |

### Parameters

#### Features

##### Loathsome limbs

<!-- splat:yaml path=splat_rule/parameters/features/loathsome_limbs -->

```yaml
economy: trait
resource:
  kind: daily
  uses: 4
  recovery: long_rest
operation: passive_rule
trigger: any.turn_end
requires:
  self_bloodied: true
  slashing_damage_this_turn_at_least: 15
sever_limb:
  spawn:
    key: dnd2024:creatures/troll_limb
  location: self.space
  turn: immediately_after_self
exhaustion_per_missing_limb: 1
regrow:
  event: self.hp_regained
  all_missing: true
```

##### Regeneration

<!-- splat:yaml path=splat_rule/parameters/features/regeneration -->

```yaml
economy: trait
operation: passive_rule
trigger: self.turn_start
heal: 15
suppressed_next_turn_by:
- acid_damage
- fire_damage
death_rule:
  only_if:
    turn_start_hp: 0
    does_not_regenerate: true
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: rend
  count: 3
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 7
  reach_ft: 10
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 6
      modifier: 4
    type: slashing
```

##### Charge

<!-- splat:yaml path=splat_rule/parameters/features/charge -->

```yaml
economy: bonus_action
operation: move
maximum_ft:
  multiply:
  - stat: speed
  - 0.5
direction: straight_toward_enemy
enemy_visible: true
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 15 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
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
