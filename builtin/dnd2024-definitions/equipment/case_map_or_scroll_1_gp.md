---
splat_schema: 1
splat_id: 55bf91d5-ff23-5d1a-b322-3d1a5559caf7
splat_kind: item
splat_key: dnd2024:equipment/case_map_or_scroll_1_gp
splat_name: Case, Map or Scroll (1 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Case, Map or Scroll (1 GP)

## Source text

<!-- SRD page 96 -->

A Map or Scroll Case holds up to 10 sheets of paper\
or 5 sheets of parchment.\

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
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/case_map_or_scroll_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_case_map_or_scroll_1_gp ` |

### Parameters

#### Capacity

##### Alternative capacities

<!-- splat:table
path: splat_rule/parameters/capacity/alternative_capacities
key: Item
order:
- paper_2_sp
- parchment_1_sp
columns:
  Item:
    path: item
    type: reference
  Count:
    path: count
    type: integer
-->

| Item | Count |
| --- | --- |
| [Paper (2 SP)](<paper_2_sp.md> "dnd2024:equipment/paper_2_sp") | ` 10 ` |
| [Parchment (1 SP)](<parchment_1_sp.md> "dnd2024:equipment/parchment_1_sp") | ` 5 ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |
