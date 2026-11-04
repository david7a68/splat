---
splat_schema: 2
splat_id: f1b1ad07-9181-5438-a9a5-355447e5ccb0
splat_kind: item
splat_key: dnd2024:equipment/greatclub
splat_name: Greatclub
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Greatclub


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
    "type": "bludgeoning"
  },
  "properties_text": "Two-Handed",
  "mastery": "push",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "sp"
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
| ` mechanic ` | ` dnd2024:equipment/greatclub ` |
| ` implementation ` | ` dnd2024:equipment_greatclub ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` simple_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d8
type: bludgeoning
amount:
  dice:
    count: 1
    sides: 8
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | hands_to_attack |
| --- | --- | --- |
| ` 0 ` | ` two_handed ` | ` 2 ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

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
property: push
rule:
  key: dnd2024:rules/weapons_push
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_creature
  optional: true
  maximum_target_size: large
  push_maximum_ft: 10
  direction: straight_away_from_attacker
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
>   expression: 1d8
>   type: bludgeoning
> mass:
>   unit: lb
>   value: 10.0
> mastery: push
> price:
>   unit: sp
>   value: 2
> properties_text: Two-Handed
> ```

