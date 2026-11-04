---
splat_schema: 2
splat_id: 2f94f18b-c79b-581d-85d2-0ad46fd926a3
splat_kind: item
splat_key: dnd2024:equipment/dart
splat_name: Dart
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Dart


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
    "expression": "1d4",
    "type": "piercing"
  },
  "properties_text": "Finesse, Thrown (Range 20/60)",
  "mastery": "vex",
  "weapon_category": "simple ranged",
  "price": {
    "value": 5,
    "unit": "cp"
  },
  "mass": {
    "value": 0.25,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/dart ` |
| ` implementation ` | ` dnd2024:equipment_dart ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` simple_ranged ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d4
type: piercing
amount:
  dice:
    count: 1
    sides: 4
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | attack_and_damage_ability_choice | same_ability_for_both | draw_as_part_of_attack | attack_type | melee_weapon_uses_melee_ability |
| --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` finesse ` | ` ["strength", "dexterity"] ` | ` true ` |  |  |  |
| ` 1 ` | ` thrown ` |  |  | ` true ` | ` ranged ` | ` true ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_finesse ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/1/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_thrown ` |

##### Range

<!-- splat:yaml path=splat_rule/parameters/properties/1/range -->

```yaml
normal_ft: 20
long_ft: 60
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
| ` base_ability ` | ` dexterity ` |
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> weapon_category: simple ranged
> damage:
>   expression: 1d4
>   type: piercing
> mass:
>   unit: lb
>   value: 0.25
> mastery: vex
> price:
>   unit: cp
>   value: 5
> properties_text: Finesse, Thrown (Range 20/60)
> ```

