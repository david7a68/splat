---
splat_schema: 2
splat_id: 837bc152-ee71-5eff-bc33-6edc96f9beab
splat_kind: creature
splat_key: dnd2024:creatures/elk
splat_name: Elk
splat_origin: dnd2024:reference/creature/elk
splat_sources:
- SRD 5.2.1, pp. 348, https://www.dndbeyond.com/srd
---

# Elk


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 11 (2d10) |
| Initiative | +0 (10) |
| Speed | 50 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Perception +2 |
| Senses | Darkvision 60 ft.; Passive Perception 12 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 10 | +0 | +0 |
| Con | 11 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 348 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 10  Initiative +0 (10)\
> HP 11 (2d10)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 10 +0 +0 Con 11 +0 +0\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 6 −2 −2\
> Skills Perception +2\
> Senses Darkvision 60 ft.; Passive Perception 12\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Actions

Ram. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage. If the target is a Large or smaller creature and the elk moved 20+ feet straight toward it immediately before the hit, the target takes an extra 3 (1d6) Bludgeoning damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/348|page 348]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/elk ` |
| ` implementation ` | ` dnd2024:creature_elk ` |

### Parameters

#### Features

##### Ram

<!-- splat:yaml path=splat_rule/parameters/features/ram -->

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
  average: 6
  amount:
    dice:
      count: 1
      sides: 6
    add: 3
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: large
    immediately_preceding_movement:
      minimum_feet: 20
      direction: straight_toward_target
  then:
  - op: damage
    average: 3
    amount:
      dice:
        count: 1
        sides: 6
      add: 0
    damage_type: bludgeoning
  - op: condition
    condition: prone
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

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
