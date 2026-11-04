---
splat_schema: 2
splat_id: a3da0248-14fc-54fb-ba30-33a3e4e24d16
splat_kind: item
splat_key: dnd2024:equipment/war_pick
splat_name: War Pick
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# War Pick


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
  "properties_text": "Versatile (1d10)",
  "mastery": "sap",
  "weapon_category": "martial melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/war_pick ` |
| ` implementation ` | ` dnd2024:equipment_war_pick ` |

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
property: sap
rule:
  key: dnd2024:rules/weapons_sap
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_creature
  effect:
    attack_roll_mode: disadvantage
    count: 1
    expiry: attacker_next_turn_start
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
>   value: 2.0
> mastery: sap
> price:
>   unit: gp
>   value: 5
> properties_text: Versatile (1d10)
> ```

