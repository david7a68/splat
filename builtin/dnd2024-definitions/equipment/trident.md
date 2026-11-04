---
splat_schema: 2
splat_id: 6c9dd52c-59b0-595d-8f70-19587d9c0690
splat_kind: item
splat_key: dnd2024:equipment/trident
splat_name: Trident
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Trident


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
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 20/60), Versatile (1d10)",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/trident ` |
| ` implementation ` | ` dnd2024:equipment_trident ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d8
type: piercing
amount:
  dice:
    count: 1
    sides: 8
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | draw_as_part_of_attack | attack_type | melee_weapon_uses_melee_ability |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` thrown ` | ` true ` | ` ranged ` | ` true ` |
| ` 1 ` | ` versatile ` |  |  |  |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_thrown ` |

##### Range

<!-- splat:yaml path=splat_rule/parameters/properties/0/range -->

```yaml
normal_ft: 20
long_ft: 60
normal_range_mode: normal
long_range_mode: disadvantage
beyond_long_range: cannot_attack
rule:
  key: dnd2024:rules/weapons_range
```

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/1/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_versatile ` |

##### Two hand melee damage

<!-- splat:values path=splat_rule/parameters/properties/1/two_hand_melee_damage -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d10 ` |

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
>   expression: 1d8
>   type: piercing
> mass:
>   unit: lb
>   value: 4.0
> mastery: topple
> price:
>   unit: gp
>   value: 5
> properties_text: Thrown (Range 20/60), Versatile (1d10)
> ```

