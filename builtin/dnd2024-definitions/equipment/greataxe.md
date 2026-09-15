---
splat_schema: 1
splat_id: afa7549a-80b1-5fae-b129-44c01853b6c5
splat_kind: item
splat_key: dnd2024:equipment/greataxe
splat_name: Greataxe
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
---

# Greataxe

## Source text

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d12",
    "type": "slashing"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "cleave",
  "weapon_category": "martial melee",
  "price": {
    "value": 30,
    "unit": "gp"
  },
  "mass": {
    "value": 7.0,
    "unit": "lb"
  },
  "category": "weapon"
}

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` properties_text ` | ` Heavy, Two-Handed ` |
| ` mastery ` | ` cleave ` |
| ` weapon_category ` | ` martial melee ` |
| ` category ` | ` weapon ` |

### Damage

<!-- splat:values path=splat_item/damage -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d12 ` |
| ` type ` | ` slashing ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 30 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 7.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/greataxe ` |
| ` implementation ` | ` dnd2024:equipment_greataxe ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d12
type: slashing
amount:
  dice:
    count: 1
    sides: 12
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
property: cleave
rule:
  key: dnd2024:rules/weapons_cleave
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: melee_attack_hits_creature
  optional: true
  extra_melee_attack:
    target: second_creature
    within_first_target_ft: 5
    within_weapon_reach: true
    damage_ability_modifier: negative_only
  maximum_per_turn: 1
```

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` actor ` | ` equipper ` |
| ` item ` | ` equipped_instance ` |
| ` inactive_if_not_equipped ` | ` true ` |
