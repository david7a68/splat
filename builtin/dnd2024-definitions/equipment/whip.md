---
splat_schema: 1
splat_id: 784710b1-91c1-5b33-9394-492f4b54b64e
splat_kind: item
splat_key: dnd2024:equipment/whip
splat_name: Whip
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
---

# Whip

## Source text

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "slashing"
  },
  "properties_text": "Finesse, Reach",
  "mastery": "slow",
  "weapon_category": "martial melee",
  "price": {
    "value": 2,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` properties_text ` | ` Finesse, Reach ` |
| ` mastery ` | ` slow ` |
| ` weapon_category ` | ` martial melee ` |
| ` category ` | ` weapon ` |

### Damage

<!-- splat:values path=splat_item/damage -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d4 ` |
| ` type ` | ` slashing ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/whip ` |
| ` implementation ` | ` dnd2024:equipment_whip ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d4
type: slashing
amount:
  dice:
    count: 1
    sides: 4
  bonus: 0
```

#### Properties

<!-- splat:table
path: splat_rule/parameters/properties
key: Variant
order:
- finesse
- reach
columns:
  Variant:
    path: id
    type: symbol
  Rule:
    path: rule
    type: reference
  Attack and damage ability choice:
    path: attack_and_damage_ability_choice
    type: yaml
    optional: true
  Same ability for both:
    path: same_ability_for_both
    type: boolean
    optional: true
  Reach bonus ft:
    path: reach_bonus_ft
    type: integer
    optional: true
  Applies to:
    path: applies_to
    type: yaml
    optional: true
-->

| Variant | Rule | Attack and damage ability choice | Same ability for both | Reach bonus ft | Applies to |
| --- | --- | --- | --- | --- | --- |
| ` Finesse ` | [Finesse](<../rules/weapons_finesse.md> "dnd2024:rules/weapons_finesse") | ` ["strength", "dexterity"] ` | true |  |  |
| ` Reach ` | [Reach](<../rules/weapons_reach.md> "dnd2024:rules/weapons_reach") |  |  | ` 5 ` | ` ["attacks", "opportunity_attack_reach"] ` |

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
