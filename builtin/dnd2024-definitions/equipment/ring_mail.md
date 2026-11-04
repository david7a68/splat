---
splat_schema: 2
splat_id: b228ce26-626e-5cf9-a96d-9834db85a2bb
splat_kind: item
splat_key: dnd2024:equipment/ring_mail
splat_name: Ring Mail
splat_origin: dnd2024:reference/pages/092
splat_sources:
- SRD 5.2.1, pp. 92, https://www.dndbeyond.com/srd
---

# Ring Mail


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
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 14,
    "add_dexterity_modifier": false
  },
  "price": {
    "value": 30,
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
| ` mechanic ` | ` dnd2024:equipment/ring_mail ` |
| ` implementation ` | ` dnd2024:equipment_ring_mail ` |

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
| ` category ` | ` heavy armor ` |
| ` stealth_disadvantage ` | ` true ` |
| ` base_ac ` | ` 14 ` |
| ` add_dexterity_modifier ` | ` false ` |

#### Equip

<!-- splat:yaml path=splat_rule/parameters/equip -->

```yaml
don:
  value: 10
  unit: minute
doff:
  value: 5
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
>   add_dexterity_modifier: false
>   base_ac: 14
>   category: heavy armor
>   stealth_disadvantage: true
> price:
>   unit: gp
>   value: 30
> mass:
>   unit: lb
>   value: 40.0
> ```

