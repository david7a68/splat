---
splat_schema: 1
splat_id: 8e4d8e47-b47e-5fce-abfc-f501e8ddac94
splat_kind: item
splat_key: dnd2024:items/bag_of_holding
splat_name: Bag of Holding
splat_origin:
  key: dnd2024:reference/magic_item/bag_of_holding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212
  url: https://www.dndbeyond.com/srd
---

# Bag of Holding

## Source text

<!-- SRD page 212 -->

Wondrous Item, Uncommon\
This bag has an interior space considerably larger\
than its outside dimensions—roughly 2 feet square\
and 4 feet deep on the inside. The bag can hold up\
to 500 pounds, not exceeding a volume of 64 cubic\
feet. The bag weighs 5 pounds, regardless of its\
contents. Retrieving an item from the bag requires\
a Utilize action.\
If the bag is overloaded, pierced, or torn, it is de-\
stroyed, and its contents are scattered in the Astral\
Plane. If the bag is turned inside out, its contents\
spill forth unharmed, but the bag must be put right\
before it can be used again. The bag holds enough\
air for 10 minutes of breathing, divided by the num-\
ber of breathing creatures inside.\
Placing a Bag of Holding inside an extradimen-\
sional space created by a Handy Haversack, Portable\
Hole, or similar item instantly destroys both items\
and opens a gate to the Astral Plane. The gate orig-\
inates where the one item was placed inside the\
other. Any creature within a 10-foot-radius Sphere\
centered on the gate is sucked through it to a ran-\
dom location on the Astral Plane. The gate then\
closes. The gate is one-way and can’t be reopened.\

Original layout: [[../../srd-5.2.1/pages/212|page 212]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |
| ` stackable ` | ` false ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` lb ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` allowed ` | ` ["carried"] ` |
| ` attunement ` | ` none ` |

### Containment

#### Capacity

<!-- splat:yaml path=splat_item/containment/capacity -->

```yaml
mass:
  value: 500
  unit: lb
volume:
  value: 64
  unit: ft3
```

#### Carried mass

<!-- splat:values path=splat_item/containment/carried_mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/bag_of_holding ` |
| ` implementation ` | ` dnd2024:item_bag_of_holding ` |

### Parameters

#### Container

<!-- splat:yaml path=splat_rule/parameters/container -->

```yaml
extradimensional: true
interior_approximate_ft:
- 2
- 2
- 4
capacity_weight_lb: 500
capacity_volume_cubic_ft: 64
weight_lb: 5
weight_independent_of_contents: true
retrieve_action: utilize
destruction_triggers:
- overloaded
- pierced
- torn
destruction_contents_destination:
  plane: astral
  distribution: scattered
inverted:
  contents: spill_unharmed
  usable: false
  restore: turn_right_side_out
air_supply:
  minutes: 10
  divide_by: breathing_creature_count
interior_dimensions_are_approximate: true
```

#### Nested extradimensional

<!-- splat:yaml path=splat_rule/parameters/nested_extradimensional -->

```yaml
trigger: placed_inside_item_created_extradimensional_space
examples:
- key: dnd2024:items/handy_haversack
- key: dnd2024:items/portable_hole
includes_similar_items: true
destroy: both_items
gate:
  origin: nesting_location
  shape: sphere
  radius_ft: 10
  targets: all_creatures_in_area
  destination:
    plane: astral
    location: random
  one_way: true
  closes_immediately_after_transport: true
  reopenable: false
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
