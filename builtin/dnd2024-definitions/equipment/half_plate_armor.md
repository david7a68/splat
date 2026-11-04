---
splat_schema: 2
splat_id: af613d48-01e3-585f-8f80-6838767ee644
splat_kind: item
splat_key: dnd2024:equipment/half_plate_armor
splat_name: Half Plate Armor
splat_origin: dnd2024:reference/pages/092
splat_sources:
- SRD 5.2.1, pp. 92, https://www.dndbeyond.com/srd
---

# Half Plate Armor


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
    "stealth_disadvantage": true,
    "base_ac": 15,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 750,
    "unit": "gp"
  },
  "mass": {
    "value": 40.0,
    "unit": "lb"
  },
  "category": "armor"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/half_plate_armor ` |
| ` implementation ` | ` dnd2024:equipment_half_plate_armor ` |

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
| ` stealth_disadvantage ` | ` true ` |
| ` base_ac ` | ` 15 ` |
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

#### Stealth

<!-- splat:values path=splat_rule/parameters/stealth -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` skill ` | ` stealth ` |
| ` mode ` | ` disadvantage ` |

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
>   base_ac: 15
>   category: medium armor
>   dexterity_modifier_max: 2
>   stealth_disadvantage: true
> price:
>   unit: gp
>   value: 750
> mass:
>   unit: lb
>   value: 40.0
> ```

