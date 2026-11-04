---
splat_schema: 2
splat_id: 889b763b-f4d3-54b4-9b29-a2e2af1dfa0a
splat_kind: item
splat_key: dnd2024:equipment/padded_armor
splat_name: Padded Armor
splat_origin: dnd2024:reference/pages/092
splat_sources:
- SRD 5.2.1, pp. 92, https://www.dndbeyond.com/srd
---

# Padded Armor


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
    "category": "light armor",
    "stealth_disadvantage": true,
    "base_ac": 11,
    "add_dexterity_modifier": true
  },
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 8.0,
    "unit": "lb"
  },
  "category": "armor"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/padded_armor ` |
| ` implementation ` | ` dnd2024:equipment_padded_armor ` |

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
| ` category ` | ` light armor ` |
| ` stealth_disadvantage ` | ` true ` |
| ` base_ac ` | ` 11 ` |
| ` add_dexterity_modifier ` | ` true ` |

#### Equip

<!-- splat:yaml path=splat_rule/parameters/equip -->

```yaml
don:
  value: 1
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
>   base_ac: 11
>   category: light armor
>   stealth_disadvantage: true
> price:
>   unit: gp
>   value: 5
> mass:
>   unit: lb
>   value: 8.0
> ```

