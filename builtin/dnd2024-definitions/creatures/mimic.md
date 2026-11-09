---
splat_schema: 2
splat_id: ce1a24e3-0409-5378-afef-531280f43679
splat_kind: creature
splat_key: dnd2024:creatures/mimic
splat_name: Mimic
splat_origin: dnd2024:reference/creature/mimic
splat_sources:
- SRD 5.2.1, pp. 308, 309, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 212, Habitat
---

# Mimic


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Monstrosity |
| Alignment | Neutral |
| Armor Class | 12 |
| Hit Points | 58 (9d8 + 18) |
| Initiative | +3 (13) |
| Speed | 20 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Stealth +5 |
| Senses | Darkvision 60 ft.; Passive Perception 11 |
| Languages | None |
| Immunities | Acid; Prone |
| Habitat | Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 12 | +1 | +1 |
| Con | 15 | +2 | +2 |
| Int | 5 | -3 | -3 |
| Wis | 13 | +1 | +1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 308 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Monstrosity, Neutral\
> AC 12 Initiative +3 (13)\
> HP 58 (9d8 + 18)\
> Speed 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +2\
> Int 5 −3 −3 Wis 13 +1 +1 Cha 8 −1 −1\
> Skills Stealth +5\
> Immunities Acid; Prone\
> Senses Darkvision 60 ft.; Passive Perception 11\
> Languages None\
> CR 2 (XP 450; PB +2)


<!-- SRD page 309 -->

### Traits

Adhesive (Object Form Only). The mimic adheres to anything that touches it. A Huge or smaller creature adhered to the mimic has the Grappled condition (escape DC 13). Ability checks made to escape this grapple have Disadvantage.

### Actions

Bite. Melee Attack Roll: +5 (with Advantage if the target is Grappled by the mimic), reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage—or 12 (2d8 + 3) Piercing damage if the target is Grappled by the mimic—plus 4 (1d8) Acid damage.

Pseudopod. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Bludgeoning damage plus 4 (1d8) Acid damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 13). Ability checks made to escape this grapple have Disadvantage.

### Bonus Actions

Shape-Shift. The mimic shape-shifts to resemble a Medium or Small object while retaining its game statistics, or it returns to its true blob form. Any equipment it is wearing or carrying isn’t transformed.

Original layout: [[../../srd-5.2.1/pages/308|page 308]], [[../../srd-5.2.1/pages/309|page 309]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/mimic ` |
| ` implementation ` | ` dnd2024:creature_mimic ` |

### Parameters

#### Features

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
- op: choose_by_predicate
  test:
    target_grappled_by: self
  then:
  - op: damage
    average: 12
    amount:
      dice:
        count: 2
        sides: 8
      add: 3
    damage_type: piercing
  else:
  - op: damage
    average: 7
    amount:
      dice:
        count: 1
        sides: 8
      add: 3
    damage_type: piercing
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 8
    add: 0
  damage_type: acid
reach_feet: 5
advantage_when:
  target_grappled_by: self
```

##### Pseudopod

<!-- splat:yaml path=splat_rule/parameters/features/pseudopod -->

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
  average: 7
  amount:
    dice:
      count: 1
      sides: 8
    add: 3
  damage_type: bludgeoning
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 8
    add: 0
  damage_type: acid
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: grappled
    escape_dc: 13
    source: self
    escape_check_disadvantage: true
reach_feet: 5
```

##### Adhesive

<!-- splat:yaml path=splat_rule/parameters/features/adhesive -->

```yaml
economy: passive
requires_form:
- object
trigger: self.touched
op: adhere
targets: touching_thing
creature_effect:
  op: condition
  condition: grappled
  escape_dc: 13
  source: self
  size_max: huge
  escape_check_disadvantage: true
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` shape_shift ` |
| ` retain_statistics ` | ` true ` |
| ` equipment_transformed ` | ` false ` |

###### Forms

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/forms
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Form:
    path: form
    type: symbol
  Size choice:
    path: size_choice
    type: yaml
    optional: true
-->

| Key | Form | Size choice |
| --- | --- | --- |
| ` row_1 ` | ` Object ` | ` ["medium", "small"] ` |
| ` row_2 ` | ` True blob ` |  |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 5 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["acid"] ` |
| ` conditions ` | ` ["prone"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

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
