---
splat_schema: 1
splat_id: ac18c3fb-1491-5fe6-8af5-9f43d342ae4d
splat_kind: item
splat_key: dnd2024:equipment/glaive
splat_name: Glaive
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
---

# Glaive

## Source text

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "slashing"
  },
  "properties_text": "Heavy, Reach, Two-Handed",
  "mastery": "graze",
  "weapon_category": "martial melee",
  "price": {
    "value": 20,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` properties_text ` | ` Heavy, Reach, Two-Handed ` |
| ` mastery ` | ` graze ` |
| ` weapon_category ` | ` martial melee ` |
| ` category ` | ` weapon ` |

### Damage

<!-- splat:values path=splat_item/damage -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d10 ` |
| ` type ` | ` slashing ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 20 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/glaive ` |
| ` implementation ` | ` dnd2024:equipment_glaive ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d10
type: slashing
amount:
  dice:
    count: 1
    sides: 10
  bonus: 0
```

#### Properties

<!-- splat:table
path: splat_rule/parameters/properties
key: Variant
order:
- heavy
- reach
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
  Reach bonus ft:
    path: reach_bonus_ft
    type: integer
    optional: true
  Applies to:
    path: applies_to
    type: yaml
    optional: true
  Hands to attack:
    path: hands_to_attack
    type: integer
    optional: true
-->

| Variant | Rule | Required ability | Minimum score | Below minimum attack mode | Reach bonus ft | Applies to | Hands to attack |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` Heavy ` | [Heavy](<../rules/weapons_heavy.md> "dnd2024:rules/weapons_heavy") | ` Strength ` | ` 13 ` | ` Disadvantage ` |  |  |  |
| ` Reach ` | [Reach](<../rules/weapons_reach.md> "dnd2024:rules/weapons_reach") |  |  |  | ` 5 ` | ` ["attacks", "opportunity_attack_reach"] ` |  |
| ` Two handed ` | [Two-Handed](<../rules/weapons_two_handed.md> "dnd2024:rules/weapons_two_handed") |  |  |  |  |  | ` 2 ` |

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
