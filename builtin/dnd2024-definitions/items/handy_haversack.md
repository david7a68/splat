---
splat_schema: 1
splat_id: f1a6257f-6147-50ed-9212-ee1a407bf2f1
splat_kind: item
splat_key: dnd2024:items/handy_haversack
splat_name: Handy Haversack
splat_origin:
  key: dnd2024:reference/magic_item/handy_haversack
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
---

# Handy Haversack

## Source text

<!-- SRD page 224 -->

Wondrous Item, Rare\
This backpack has a central pouch and two side\
pouches, each of which is an extradimensional\
space. Each side pouch can hold up to 200 pounds\
of material, not exceeding a volume of 25 cubic feet.\
The central pouch can hold up to 500 pounds of ma-\
terial, not exceeding a volume of 64 cubic feet. The\
haversack always weighs 5 pounds, regardless of its\
contents.\
Retrieving an item from the haversack requires a\
Utilize action or a Bonus Action (your choice). When\
you reach into the haversack for a specific item, the\
item is always magically on top.\
If any of its pouches is overloaded, pierced, or\
torn, the haversack ruptures and is destroyed. If the\
haversack is destroyed, its contents are lost forever,\
although an Artifact always turns up again some-\
where. If the haversack is turned inside out, its con-\
tents spill forth unharmed, and the haversack must\
be put right before it can be used again.\
Each pouch of the haversack holds enough air for\
10 minutes of breathing, divided by the number of\
breathing creatures inside.\
Placing the haversack inside an extradimensional\
space created by a Bag of Holding, Portable Hole,\
or similar item instantly destroys both items and\
opens a gate to the Astral Plane. The gate originates\
where the one item was placed inside the other. Any\
creature within 10 feet of the gate and not behind\
Total Cover is sucked through it and deposited in\
a random location on the Astral Plane. The gate\
then closes. The gate is one-way only and can’t be\
reopened.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/handy_haversack ` |
| ` implementation ` | ` dnd2024:item_handy_haversack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weight_lb ` | ` 5 ` |
| ` weight_independent_of_contents ` | ` true ` |
| ` extradimensional ` | ` true ` |
| ` destroy_on ` | ` ["pouch_overload", "pouch_pierced", "pouch_torn"] ` |
| ` attunement_required ` | ` false ` |

#### Pouches

<!-- splat:rows path=splat_rule/parameters/pouches -->

| Row | name | count | capacity_lb | capacity_cubic_ft |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` side ` | ` 2 ` | ` 200 ` | ` 25 ` |
| ` 1 ` | ` central ` | ` 1 ` | ` 500 ` | ` 64 ` |

#### Retrieve

<!-- splat:values path=splat_rule/parameters/retrieve -->

| Field | Value |
| --- | --- |
| ` action_choice ` | ` ["utilize", "bonus_action"] ` |
| ` requested_item_position ` | ` top ` |

#### Destroyed contents

<!-- splat:values path=splat_rule/parameters/destroyed_contents -->

| Field | Value |
| --- | --- |
| ` non_artifacts ` | ` lost_forever ` |
| ` artifacts ` | ` reappear_elsewhere ` |

#### Inside out

<!-- splat:values path=splat_rule/parameters/inside_out -->

| Field | Value |
| --- | --- |
| ` contents ` | ` spill_unharmed ` |
| ` usable ` | ` false ` |
| ` restore ` | ` turn_right_side_out ` |

#### Breath per pouch

<!-- splat:yaml path=splat_rule/parameters/breath_per_pouch -->

```yaml
minutes:
  divide:
  - 10
  - number_of_breathing_creatures
```

#### Extradimensional nesting

<!-- splat:yaml path=splat_rule/parameters/extradimensional_nesting -->

```yaml
trigger: placed_inside_extradimensional_item_space
examples:
- key: dnd2024:items/bag_of_holding
- key: dnd2024:items/portable_hole
destroy:
- this_item
- containing_item
gate:
  origin: insertion_location
  targets: creatures
  radius_ft: 10
  total_cover_blocks: true
  destination:
    plane: astral
    location: random
  one_way: true
  close_after_transport: true
  can_reopen: false
```
