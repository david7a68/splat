---
splat_schema: 2
splat_id: 690a9342-67db-5b7d-bc34-05af8c204771
splat_kind: item
splat_key: dnd2024:equipment/greatsword
splat_name: Greatsword
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Greatsword


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
    "expression": "2d6",
    "type": "slashing"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "graze",
  "weapon_category": "martial melee",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/greatsword ` |
| ` implementation ` | ` dnd2024:equipment_greatsword ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 2d6
type: slashing
amount:
  dice:
    count: 2
    sides: 6
  bonus: 0
```

#### Properties

<!-- splat:table
path: splat_rule/parameters/properties
key: Variant
order:
- heavy
- two_handed
columns:
  Variant:
    path: id
    type: symbol
  Rule:
    path: rule
    type: reference
  Required ability:
    path: required_ability
    type: symbol
    optional: true
  Minimum score:
    path: minimum_score
    type: integer
    optional: true
  Below minimum attack mode:
    path: below_minimum_attack_mode
    type: symbol
    optional: true
  Hands to attack:
    path: hands_to_attack
    type: integer
    optional: true
-->

| Variant | Rule | Required ability | Minimum score | Below minimum attack mode | Hands to attack |
| --- | --- | --- | --- | --- | --- |
| ` Heavy ` | [Heavy](<../rules/weapons_heavy.md> "dnd2024:rules/weapons_heavy") | ` Strength ` | ` 13 ` | ` Disadvantage ` |  |
| ` Two handed ` | [Two-Handed](<../rules/weapons_two_handed.md> "dnd2024:rules/weapons_two_handed") |  |  |  | ` 2 ` |

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
property: graze
rule:
  key: dnd2024:rules/weapons_graze
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: attack_misses_creature
  optional: true
  damage:
    amount:
      stat: attack_ability_modifier
    type: weapon_damage_type
    increases_only_from: attack_ability_modifier
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
> weapon_category: martial melee
> damage:
>   expression: 2d6
>   type: slashing
> mass:
>   unit: lb
>   value: 6.0
> mastery: graze
> price:
>   unit: gp
>   value: 50
> properties_text: Heavy, Two-Handed
> ```

