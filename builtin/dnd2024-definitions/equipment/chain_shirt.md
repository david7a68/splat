---
splat_schema: 2
splat_id: c540d98d-1960-508d-997f-581ab48f8d57
splat_kind: item
splat_key: dnd2024:equipment/chain_shirt
splat_name: Chain Shirt
splat_origin: dnd2024:reference/pages/092
splat_sources:
- SRD 5.2.1, pp. 92, https://www.dndbeyond.com/srd
---

# Chain Shirt


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
    "category": "medium armor",
    "stealth_disadvantage": false,
    "base_ac": 13,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 20.0,
    "unit": "lb"
  },
  "category": "armor"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/chain_shirt ` |
| ` implementation ` | ` dnd2024:equipment_chain_shirt ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` while_equipped ` | ` true ` |
| ` maximum_simultaneous_same_category ` | ` 1 ` |
| ` stacking_group ` | ` body_armor ` |

#### Armor

<!-- splat:values path=splat_rule/parameters/armor -->

| Field | Value |
| --- | --- |
| ` category ` | ` medium armor ` |
| ` stealth_disadvantage ` | ` false ` |
| ` base_ac ` | ` 13 ` |
| ` add_dexterity_modifier ` | ` true ` |
| ` dexterity_modifier_max ` | ` 2 ` |

#### Equip

<!-- splat:yaml path=splat_rule/parameters/equip -->

```yaml
don:
  value: 5
  unit: minute
doff:
  value: 1
  unit: minute
```

#### Training

<!-- splat:yaml path=splat_rule/parameters/training -->

```yaml
without_training:
  d20_test_disadvantage_abilities:
  - strength
  - dexterity
  can_cast_spells: false
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
> armor:
>   add_dexterity_modifier: true
>   base_ac: 13
>   category: medium armor
>   dexterity_modifier_max: 2
>   stealth_disadvantage: false
> price:
>   unit: gp
>   value: 50
> mass:
>   unit: lb
>   value: 20.0
> ```

