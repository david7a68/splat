---
splat_schema: 2
splat_id: 10512d84-4e76-5d69-ace0-3a78554c747a
splat_kind: item
splat_key: dnd2024:items/efficient_quiver
splat_name: Efficient Quiver
splat_origin: dnd2024:reference/magic_item/efficient_quiver
splat_sources:
- SRD 5.2.1, pp. 219, https://www.dndbeyond.com/srd
---

# Efficient Quiver


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 219 -->

Wondrous Item, Uncommon

Each of the quiver’s three compartments connects to an extradimensional space that allows the quiver to hold numerous items while never weighing more than 2 pounds. The shortest compartment can hold up to 60 Arrows, Bolts, or similar objects. The midsize compartment holds up to 18 Javelins or similar objects. The longest compartment holds up to 6 long objects, such as bows, Quarterstaffs, or Spears.

You can draw any item the quiver contains as if doing so from a regular quiver or scabbard.

Original layout: [[../../srd-5.2.1/pages/219|page 219]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/efficient_quiver ` |
| ` implementation ` | ` dnd2024:item_efficient_quiver ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` maximum_total_weight_lb ` | ` 2 ` |
| ` retrieve_economy ` | ` normal_quiver_or_scabbard ` |

#### Extradimensional compartments

<!-- splat:table
path: splat_rule/parameters/extradimensional_compartments
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Capacity:
    path: capacity
    type: integer
  Item shapes:
    path: item_shapes
    type: yaml
-->

| Key | Capacity | Item shapes |
| --- | --- | --- |
| ` row_1 ` | ` 60 ` | ` ["arrow", "bolt", "similar"] ` |
| ` row_2 ` | ` 18 ` | ` ["javelin", "similar"] ` |
| ` row_3 ` | ` 6 ` | ` ["bow", "quarterstaff", "spear", "similar_long_object"] ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
