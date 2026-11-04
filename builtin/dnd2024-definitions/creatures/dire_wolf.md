---
splat_schema: 2
splat_id: d8c7cab5-1a7f-5f44-99f9-2e2de6e3a51d
splat_kind: creature
splat_key: dnd2024:creatures/dire_wolf
splat_name: Dire Wolf
splat_origin: dnd2024:reference/creature/dire_wolf
splat_sources:
- SRD 5.2.1, pp. 347, https://www.dndbeyond.com/srd
---

# Dire Wolf


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 22 (3d10 + 6) |
| Initiative | +2 (12) |
| Speed | 50 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +5, Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 15 | +2 | +2 |
| Con | 15 | +2 | +2 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 347 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 14  Initiative +2 (12)\
> HP 22 (3d10 + 6)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 15 +2 +2 Con 15 +2 +2\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +5, Stealth +4\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages None\
> CR 1 (XP 200; PB +2)


### Traits

Pack Tactics. The wolf has Advantage on an attack roll against a creature if at least one of the wolf’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Piercing damage. If the target is a Large or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/347|page 347]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/dire_wolf ` |
| ` implementation ` | ` dnd2024:creature_dire_wolf ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
advantage:
  roll: attack
  target_kind: creature
  requires:
    ally_distance_to_target_max_feet: 5
    ally_incapacitated: false
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  average: 8
  amount:
    dice:
      count: 1
      sides: 10
    add: 3
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: prone
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 4 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

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
