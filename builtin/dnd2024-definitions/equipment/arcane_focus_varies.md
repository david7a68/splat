---
splat_schema: 1
splat_id: 2d7b19f1-002c-505c-a275-8ca535664a40
splat_kind: item
splat_key: dnd2024:equipment/arcane_focus_varies
splat_name: Arcane Focus (Varies)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Arcane Focus (Varies)

## Source text

<!-- SRD page 96 -->

An Arcane Focus takes one of the forms in the\
Arcane Focuses table and is bejeweled or carved to\
channel arcane magic. A Sorcerer, Warlock, or Wiz-\
ard can use such an item as a Spellcasting Focus.\
Arcane Focuses\
Focus Weight Cost\
Crystal 1 lb. 10 GP\
Orb 3 lb. 20 GP\
Rod 2 lb. 10 GP\
Staff (also a Quarterstaff) 4 lb. 5 GP\
Wand 1 lb. 10 GP\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/arcane_focus_varies ` |
| ` implementation ` | ` dnd2024:equipment_arcane_focus_varies ` |

### Parameters

#### Spellcasting focus

<!-- splat:yaml path=splat_rule/parameters/spellcasting_focus -->

```yaml
eligible_classes:
- key: dnd2024:classes/sorcerer
- key: dnd2024:classes/warlock
- key: dnd2024:classes/wizard
rule:
  key: dnd2024:glossary/spellcasting_focus
```

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | id | valid_use_states |
| --- | --- | --- |
| ` 0 ` | ` crystal ` | ` ["held"] ` |
| ` 1 ` | ` orb ` | ` ["held"] ` |
| ` 2 ` | ` rod ` | ` ["held"] ` |
| ` 3 ` | ` staff ` | ` ["held"] ` |
| ` 4 ` | ` wand ` | ` ["held"] ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/0/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/0/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/1/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/1/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 20 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/2/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/2/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/3/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/3/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` gp ` |

##### Also weapon

<!-- splat:values path=splat_rule/parameters/variants/3/also_weapon -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/quarterstaff ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/4/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/4/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10 ` |
| ` unit ` | ` gp ` |
