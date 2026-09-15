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

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | required_ability | minimum_score | below_minimum_attack_mode | reach_bonus_ft | applies_to | hands_to_attack |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` heavy ` | ` strength ` | ` 13 ` | ` disadvantage ` |  |  |  |
| ` 1 ` | ` reach ` |  |  |  | ` 5 ` | ` ["attacks", "opportunity_attack_reach"] ` |  |
| ` 2 ` | ` two_handed ` |  |  |  |  |  | ` 2 ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_heavy ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/1/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_reach ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/2/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_two_handed ` |

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
