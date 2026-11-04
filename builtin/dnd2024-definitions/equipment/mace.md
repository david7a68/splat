---
splat_schema: 2
splat_id: 66dd5ecc-0fb7-5919-ad05-d867cb25d41f
splat_kind: item
splat_key: dnd2024:equipment/mace
splat_name: Mace
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Mace


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
  "properties_text": "—",
  "mastery": "sap",
  "weapon_category": "simple melee",
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
| ` mechanic ` | ` dnd2024:equipment/mace ` |
| ` implementation ` | ` dnd2024:equipment_mace ` |

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

<!-- splat:yaml path=splat_rule/parameters/properties -->

```yaml
[]
```

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
> weapon_category: simple melee
> damage:
>   expression: 1d6
>   type: bludgeoning
> mass:
>   unit: lb
>   value: 4.0
> mastery: sap
> price:
>   unit: gp
>   value: 5
> properties_text: —
> ```

