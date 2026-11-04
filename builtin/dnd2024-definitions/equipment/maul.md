---
splat_schema: 2
splat_id: 91c6c731-ec05-5094-91d8-e2ecc313ae6a
splat_kind: item
splat_key: dnd2024:equipment/maul
splat_name: Maul
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Maul


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
    "type": "bludgeoning"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/maul ` |
| ` implementation ` | ` dnd2024:equipment_maul ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 2d6
type: bludgeoning
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
property: topple
rule:
  key: dnd2024:rules/weapons_topple
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_creature
  optional: true
  save:
    ability: constitution
    dc:
      op: add
      operands:
      - 8
      - stat: attack_ability_modifier
      - stat: proficiency_bonus
    failure:
      condition: prone
    success: no_effect
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
>   type: bludgeoning
> mass:
>   unit: lb
>   value: 10.0
> mastery: topple
> price:
>   unit: gp
>   value: 10
> properties_text: Heavy, Two-Handed
> ```

