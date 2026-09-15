---
splat_schema: 1
splat_id: 110756e4-fd60-5ef1-90e2-dd5e90eb40c2
splat_kind: item
splat_key: dnd2024:equipment/component_pouch_25_gp
splat_name: Component Pouch (25 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Component Pouch (25 GP)

## Source text

<!-- SRD page 97 -->

A Component Pouch is watertight and filled with\
compartments that hold all the free Material com-\
ponents of your spells.\

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

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 25 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/component_pouch_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_component_pouch_25_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` watertight ` | ` true ` |

#### Material components

<!-- splat:yaml path=splat_rule/parameters/material_components -->

```yaml
available: all_free_components_of_owner_spells
priced_components_included: false
rule:
  key: dnd2024:rules/spellcasting
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |
