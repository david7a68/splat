---
splat_schema: 2
splat_id: 857d9b74-589d-5a41-8739-5970167b793b
splat_kind: item
splat_key: dnd2024:equipment/quarterstaff
splat_name: Quarterstaff
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Quarterstaff


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
    "type": "bludgeoning"
  },
  "properties_text": "Versatile (1d8)",
  "mastery": "topple",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "sp"
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
| ` mechanic ` | ` dnd2024:equipment/quarterstaff ` |
| ` implementation ` | ` dnd2024:equipment_quarterstaff ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` simple_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d6
type: bludgeoning
amount:
  dice:
    count: 1
    sides: 6
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id |
| --- | --- |
| ` 0 ` | ` versatile ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_versatile ` |

##### Two hand melee damage

<!-- splat:values path=splat_rule/parameters/properties/0/two_hand_melee_damage -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d8 ` |

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
> weapon_category: simple melee
> damage:
>   expression: 1d6
>   type: bludgeoning
> mass:
>   unit: lb
>   value: 4.0
> mastery: topple
> price:
>   unit: sp
>   value: 2
> properties_text: Versatile (1d8)
> ```

