---
splat_schema: 1
splat_id: 899853f5-26b9-5157-b5ae-f699c05369d0
splat_kind: item
splat_key: dnd2024:equipment/studded_leather_armor
splat_name: Studded Leather Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
---

# Studded Leather Armor

## Source text

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "light armor",
    "stealth_disadvantage": false,
    "base_ac": 12,
    "add_dexterity_modifier": true
  },
  "price": {
    "value": 45,
    "unit": "gp"
  },
  "mass": {
    "value": 13.0,
    "unit": "lb"
  },
  "category": "armor"
}

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |

### Armor

<!-- splat:values path=splat_item/armor -->

| Field | Value |
| --- | --- |
| ` category ` | ` light armor ` |
| ` stealth_disadvantage ` | ` false ` |
| ` base_ac ` | ` 12 ` |
| ` add_dexterity_modifier ` | ` true ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 45 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 13.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/studded_leather_armor ` |
| ` implementation ` | ` dnd2024:equipment_studded_leather_armor ` |

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
| ` stealth_disadvantage ` | ` false ` |
| ` base_ac ` | ` 12 ` |
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

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` actor ` | ` equipper ` |
| ` item ` | ` equipped_instance ` |
| ` inactive_if_not_equipped ` | ` true ` |
