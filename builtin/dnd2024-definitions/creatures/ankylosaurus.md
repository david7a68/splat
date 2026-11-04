---
splat_schema: 2
splat_id: 6675cf17-efcf-52a9-a89f-e9b7a8bac680
splat_kind: creature
splat_key: dnd2024:creatures/ankylosaurus
splat_name: Ankylosaurus
splat_origin: dnd2024:reference/creature/ankylosaurus
splat_sources:
- SRD 5.2.1, pp. 344, https://www.dndbeyond.com/srd
---

# Ankylosaurus


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Beast (Dinosaur) |
| Alignment | Unaligned |
| Armor Class | 15 |
| Hit Points | 68 (8d12 + 16) |
| Initiative | +0 (10) |
| Speed | 30 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Passive Perception 11 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +6 |
| Dex | 11 | +0 | +0 |
| Con | 15 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 344 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Beast (Dinosaur), Unaligned\
> AC 15   Initiative +0 (10)\
> HP 68 (8d12 + 16)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +6 Dex 11 +0 +0 Con 15 +2 +2\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
> Senses Passive Perception 11\
> Languages None\
> CR 3 (XP 700; PB +2)


### Actions

Multiattack. The ankylosaurus makes two Tail attacks.

Tail. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10 + 4) Bludgeoning damage. If the target is a Huge or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/344|page 344]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ankylosaurus ` |
| ` implementation ` | ` dnd2024:creature_ankylosaurus ` |

### Parameters

#### Features

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

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
      count: 1
      sides: 10
    add: 4
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: huge
  then:
  - op: condition
    condition: prone
reach_feet: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: tail
  count: 2
```

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

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
