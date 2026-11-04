---
splat_schema: 2
splat_id: 3690316d-fb14-5b76-b9f8-5230a0c69e47
splat_kind: creature
splat_key: dnd2024:creatures/knight
splat_name: Knight
splat_origin: dnd2024:reference/creature/knight
splat_sources:
- SRD 5.2.1, pp. 302, https://www.dndbeyond.com/srd
---

# Knight


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 18 |
| Hit Points | 52 (8d8 + 16) |
| Initiative | +0 (10) |
| Speed | 30 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Passive Perception 10 |
| Languages | Common plus one other language |
| Gear | Greatsword, Heavy Crossbow, Plate Armor |
| Immunities | Frightened |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 11 | +0 | +0 |
| Con | 14 | +2 | +4 |
| Int | 11 | +0 | +0 |
| Wis | 11 | +0 | +2 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 302 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 18 Initiative +0 (10)\
> HP 52 (8d8 + 16)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  16 +3 +3 Dex 11 +0 +0 Con 14 +2 +4\
> Int  11 +0 +0 Wis  11 +0 +2 Cha 15 +2 +2\
> Immunities Frightened\
> Gear Greatsword, Heavy Crossbow, Plate Armor\
> Senses Passive Perception 10\
> Languages Common plus one other language\
> CR 3 (XP 700; PB +2)


### Actions

Multiattack. The knight makes two attacks, using Greatsword or Heavy Crossbow in any combination.

Greatsword. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage plus 4 (1d8) Radiant damage.

Heavy Crossbow. Ranged Attack Roll: +2, range 100/400 ft. Hit: 11 (2d10) Piercing damage plus 4 (1d8) Radiant damage.

### Reactions

Parry. Trigger: The knight is hit by a melee attack roll while holding a weapon. Response: The knight adds 2 to its AC against that attack, possibly causing it to miss.

Original layout: [[../../srd-5.2.1/pages/302|page 302]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/knight ` |
| ` implementation ` | ` dnd2024:creature_knight ` |

### Parameters

#### Features

##### Greatsword

<!-- splat:yaml path=splat_rule/parameters/features/greatsword -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 10
  amount:
    dice:
      count: 2
      sides: 6
    add: 3
  damage_type: slashing
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 8
    add: 0
  damage_type: radiant
reach_feet: 5
```

##### Heavy crossbow

<!-- splat:yaml path=splat_rule/parameters/features/heavy_crossbow -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 2
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 10
    add: 0
  damage_type: piercing
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 8
    add: 0
  damage_type: radiant
range_feet: 100
long_range_feet: 400
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 2 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Greatsword ` |
| ` row_2 ` | ` Heavy crossbow ` |

##### Parry

<!-- splat:yaml path=splat_rule/parameters/features/parry -->

```yaml
economy: reaction
trigger: melee_attack.hits_self
requires:
  holding_weapon: true
effects:
- op: modify_ac
  add: 2
  scope: triggering_attack
  reevaluate_hit: true
```

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["frightened"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Item name:
    path: item_name
    type: string
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item name | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Greatsword ` | ` 1 ` |
| ` row_2 ` | ` Heavy Crossbow ` | ` 1 ` |
| ` row_3 ` | ` Plate Armor ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
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
