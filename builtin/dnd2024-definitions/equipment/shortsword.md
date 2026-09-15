---
splat_schema: 1
splat_id: d9b30ace-5934-5262-8603-88580f1712ff
splat_kind: item
splat_key: dnd2024:equipment/shortsword
splat_name: Shortsword
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
---

# Shortsword

## Source text

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Finesse, Light",
  "mastery": "vex",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` properties_text ` | ` Finesse, Light ` |
| ` mastery ` | ` vex ` |
| ` weapon_category ` | ` martial melee ` |
| ` category ` | ` weapon ` |

### Damage

<!-- splat:values path=splat_item/damage -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d6 ` |
| ` type ` | ` piercing ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/shortsword ` |
| ` implementation ` | ` dnd2024:equipment_shortsword ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

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

<!-- splat:table
path: splat_rule/parameters/properties
key: Variant
order:
- finesse
- light
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
  Trigger:
    path: trigger
    type: symbol
    optional: true
  Extra attack count:
    path: extra_attack/count
    type: integer
    optional: true
  Extra attack economy:
    path: extra_attack/economy
    type: symbol
    optional: true
  Extra attack timing:
    path: extra_attack/timing
    type: symbol
    optional: true
  Extra attack weapon:
    path: extra_attack/weapon
    type: symbol
    optional: true
  Extra attack damage ability modifier:
    path: extra_attack/damage_ability_modifier
    type: symbol
    optional: true
-->

| Variant | Rule | Attack and damage ability choice | Same ability for both | Trigger | Extra attack count | Extra attack economy | Extra attack timing | Extra attack weapon | Extra attack damage ability modifier |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` Finesse ` | [Finesse](<../rules/weapons_finesse.md> "dnd2024:rules/weapons_finesse") | ` ["strength", "dexterity"] ` | true |  |  |  |  |  |  |
| ` Light ` | [Light](<../rules/weapons_light.md> "dnd2024:rules/weapons_light") |  |  | ` Attack with light weapon during own attack action ` | ` 1 ` | ` Bonus action ` | ` Later same turn ` | ` Different light weapon ` | ` Negative only ` |

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
property: vex
rule:
  key: dnd2024:rules/weapons_vex
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_and_damage_creature
  effect:
    attacker_attack_roll_mode: advantage
    against: same_target
    count: 1
    expiry: attacker_next_turn_end
```

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` actor ` | ` equipper ` |
| ` item ` | ` equipped_instance ` |
| ` inactive_if_not_equipped ` | ` true ` |
