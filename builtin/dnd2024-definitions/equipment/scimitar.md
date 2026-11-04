---
splat_schema: 2
splat_id: 37150ba9-bad3-505b-9d1b-70afd910b940
splat_kind: item
splat_key: dnd2024:equipment/scimitar
splat_name: Scimitar
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Scimitar


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
    "type": "slashing"
  },
  "properties_text": "Finesse, Light",
  "mastery": "nick",
  "weapon_category": "martial melee",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/scimitar ` |
| ` implementation ` | ` dnd2024:equipment_scimitar ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d6
type: slashing
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
property: nick
rule:
  key: dnd2024:rules/weapons_nick
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: light_property_extra_attack
  replace_economy: bonus_action
  economy: part_of_attack_action
  maximum_per_turn: 1
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
>   expression: 1d6
>   type: slashing
> mass:
>   unit: lb
>   value: 3.0
> mastery: nick
> price:
>   unit: gp
>   value: 25
> properties_text: Finesse, Light
> ```

