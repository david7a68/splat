---
splat_schema: 1
splat_id: 3c73c0a9-979d-5f24-abb3-9d5522c14f78
splat_kind: item
splat_key: dnd2024:equipment/shield
splat_name: Shield
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
---

# Shield

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

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |

### Armor

<!-- splat:values path=splat_item/armor -->

| Field | Value |
| --- | --- |
| ` category ` | ` shield ` |
| ` stealth_disadvantage ` | ` false ` |
| ` ac_bonus ` | ` 2 ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6.0 ` |
| ` unit ` | ` lb ` |

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
