---
splat_schema: 2
splat_id: 95b24f2f-1a9c-5078-a67b-46d413130acd
splat_kind: creature
splat_key: dnd2024:creatures/troll_limb
splat_name: Troll Limb
splat_origin: dnd2024:reference/creature/troll_limb
splat_sources:
- SRD 5.2.1, pp. 333, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 310, Habitat
---

# Troll Limb


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Giant |
| Alignment | Chaotic Evil |
| Armor Class | 13 |
| Hit Points | 14 (4d6) |
| Initiative | +1 (11) |
| Speed | 20 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 9 |
| Languages | None |
| Habitat | Arctic, Forest, Hill, Mountain, Swamp, Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 12 | +1 | +1 |
| Con | 10 | +0 | +0 |
| Int | 1 | -5 | -5 |
| Wis | 9 | -1 | -1 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 333 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Giant, Chaotic Evil\
> AC 13  Initiative +1 (11)\
> HP 14 (4d6)\
> Speed 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 12 +1 +1 Con 10 +0 +0\
> Int 1 −5 −5 Wis 9 −1 −1 Cha 1 −5 −5\
> Senses Darkvision 60 ft.; Passive Perception 9\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Traits

Regeneration. The limb regains 5 Hit Points at the start of each of its turns. If the limb takes Acid or Fire damage, this trait doesn’t function on the limb’s next turn. The limb dies only if it starts its turn with 0 Hit Points and doesn’t regenerate.

Troll Spawn. The limb uncannily has the same senses as a whole troll. If the limb isn’t destroyed within 24 hours, roll 1d12. On a 12, the limb turns into a Troll. Otherwise, the limb withers away.

### Actions

Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4 + 4) Slashing damage.

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
| ` mechanic ` | ` dnd2024:creatures/troll_limb ` |
| ` implementation ` | ` dnd2024:creature_troll_limb ` |

### Parameters

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
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
reach_feet: 5
```

##### Regeneration

<!-- splat:yaml path=splat_rule/parameters/features/regeneration -->

```yaml
economy: passive
trigger: self.turn.start
op: heal
amount:
  constant: 5
suppression:
  trigger: self.damage_taken
  damage_type_any:
  - acid
  - fire
  skip: next_turn
death:
  trigger: self.turn.start
  requires:
    hit_points: 0
    regeneration_occurs: false
  exclusive: true
```

##### Troll spawn

<!-- splat:values path=splat_rule/parameters/features/troll_spawn -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` trigger ` | ` self.exists_for_24_hours ` |

###### Senses as

<!-- splat:values path=splat_rule/parameters/features/troll_spawn/senses_as -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/troll ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/troll_spawn/requires -->

| Field | Value |
| --- | --- |
| ` not_destroyed ` | ` true ` |

###### Roll

<!-- splat:yaml path=splat_rule/parameters/features/troll_spawn/roll -->

```yaml
dice:
  count: 1
  sides: 12
add: 0
```

###### Table

<!-- splat:table
path: splat_rule/parameters/features/troll_spawn/table
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Roll min:
    path: roll_min
    type: integer
  Roll max:
    path: roll_max
    type: integer
  Effect op:
    path: effect/op
    type: symbol
  Effect definition:
    path: effect/definition
    type: reference
    optional: true
-->

| Key | Roll min | Roll max | Effect op | Effect definition |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 12 ` | ` 12 ` | ` Replace stat block ` | [Troll](<troll.md> "dnd2024:creatures/troll") |
| ` row_2 ` | ` 1 ` | ` 11 ` | ` Wither away ` |  |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Understands

<!-- splat:yaml path=splat_rule/parameters/communication/understands -->

```yaml
[]
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
