---
splat_schema: 1
splat_id: dc25b6ca-51bc-5670-9d90-7e3ee3861b9a
splat_kind: item
splat_key: dnd2024:equipment/chain_mail
splat_name: Chain Mail
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
---

# Chain Mail

## Source text

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 16,
    "add_dexterity_modifier": false,
    "strength_minimum": 13
  },
  "price": {
    "value": 75,
    "unit": "gp"
  },
  "mass": {
    "value": 55.0,
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
| ` category ` | ` heavy armor ` |
| ` stealth_disadvantage ` | ` true ` |
| ` base_ac ` | ` 16 ` |
| ` add_dexterity_modifier ` | ` false ` |
| ` strength_minimum ` | ` 13 ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 75 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 55.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/chain_mail ` |
| ` implementation ` | ` dnd2024:equipment_chain_mail ` |

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
| ` base_ac ` | ` 16 ` |
| ` add_dexterity_modifier ` | ` false ` |
| ` strength_minimum ` | ` 13 ` |

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

#### Strength penalty

<!-- splat:values path=splat_rule/parameters/strength_penalty -->

| Field | Value |
| --- | --- |
| ` if_strength_below ` | ` 13 ` |
| ` speed_reduction_ft ` | ` 10 ` |

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` actor ` | ` equipper ` |
| ` item ` | ` equipped_instance ` |
| ` inactive_if_not_equipped ` | ` true ` |
