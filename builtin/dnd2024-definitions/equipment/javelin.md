---
splat_schema: 2
splat_id: 6b0fd1dd-b906-5d1e-8c19-8ee5ac25b492
splat_kind: item
splat_key: dnd2024:equipment/javelin
splat_name: Javelin
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Javelin


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity |  |
| Attunement |  |


## Source text

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 30/120)",
  "mastery": "slow",
  "weapon_category": "simple melee",
  "price": {
    "value": 5,
    "unit": "sp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/javelin ` |
| ` implementation ` | ` dnd2024:equipment_javelin ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` simple_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d6
type: piercing
amount:
  dice:
    count: 1
    sides: 6
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | draw_as_part_of_attack | attack_type | melee_weapon_uses_melee_ability |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` thrown ` | ` true ` | ` ranged ` | ` true ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_thrown ` |

##### Range

<!-- splat:yaml path=splat_rule/parameters/properties/0/range -->

```yaml
normal_ft: 30
long_ft: 120
normal_range_mode: normal
long_range_mode: disadvantage
beyond_long_range: cannot_attack
rule:
  key: dnd2024:rules/weapons_range
```

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` proficiency_bonus_if_proficient ` | ` true ` |
| ` base_ability ` | ` strength ` |
| ` normal_hit_damage_ability_modifier ` | ` attack_ability_modifier ` |

#### Mastery

<!-- splat:yaml path=splat_rule/parameters/mastery -->

```yaml
property: slow
rule:
  key: dnd2024:rules/weapons_slow
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_and_damage_creature
  optional: true
  speed_reduction_ft: 10
  expiry: attacker_next_turn_start
  maximum_stacked_reduction_ft: 10
```

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` actor ` | ` equipper ` |
| ` item ` | ` equipped_instance ` |
| ` inactive_if_not_equipped ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> weapon_category: simple melee
> damage:
>   expression: 1d6
>   type: piercing
> mass:
>   unit: lb
>   value: 2.0
> mastery: slow
> price:
>   unit: sp
>   value: 5
> properties_text: Thrown (Range 30/120)
> ```

