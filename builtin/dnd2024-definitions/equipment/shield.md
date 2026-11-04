---
splat_schema: 2
splat_id: 3c73c0a9-979d-5f24-abb3-9d5522c14f78
splat_kind: item
splat_key: dnd2024:equipment/shield
splat_name: Shield
splat_origin: dnd2024:reference/pages/092
splat_sources:
- SRD 5.2.1, pp. 92, https://www.dndbeyond.com/srd
---

# Shield


## Item

| Field | Value |
| --- | --- |
| Type | Armor |
| Rarity |  |
| Attunement |  |


## Source text

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "shield",
    "stealth_disadvantage": false,
    "ac_bonus": 2
  },
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "armor"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/shield ` |
| ` implementation ` | ` dnd2024:equipment_shield ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` while_equipped ` | ` true ` |
| ` maximum_simultaneous_same_category ` | ` 1 ` |
| ` stacking_group ` | ` shield ` |

#### Armor

<!-- splat:values path=splat_rule/parameters/armor -->

| Field | Value |
| --- | --- |
| ` category ` | ` shield ` |
| ` stealth_disadvantage ` | ` false ` |
| ` ac_bonus ` | ` 2 ` |

#### Equip

<!-- splat:values path=splat_rule/parameters/equip -->

| Field | Value |
| --- | --- |
| ` don ` | ` utilize ` |
| ` doff ` | ` utilize ` |
| ` requires_wielding ` | ` true ` |

#### Training

<!-- splat:values path=splat_rule/parameters/training -->

| Field | Value |
| --- | --- |
| ` ac_bonus_requires_training ` | ` true ` |

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
> armor:
>   ac_bonus: 2
>   category: shield
>   stealth_disadvantage: false
> price:
>   unit: gp
>   value: 10
> mass:
>   unit: lb
>   value: 6.0
> ```

