---
splat_schema: 1
splat_id: 35b2e639-26f3-5173-9847-68f8312e20c2
splat_kind: item
splat_key: dnd2024:items/portable_hole
splat_name: Portable Hole
splat_origin:
  key: dnd2024:reference/magic_item/portable_hole
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234, 235
  url: https://www.dndbeyond.com/srd
---

# Portable Hole

## Source text

<!-- SRD page 234 -->

Wondrous Item, Rare\
This fine black cloth, soft as silk, is folded up to the\
dimensions of a handkerchief. It unfolds into a cir-\
cular sheet 6 feet in diameter.\
You can take a Magic action to unfold a Por-\
table Hole and place it on or against a solid sur-\
face, whereupon the Portable Hole creates an\

<!-- SRD page 235 -->

extradimensional hole 10 feet deep. The cylindrical\
space within the hole exists on a different plane\
of existence, so it can’t be used to create open pas-\
sages. Any creature inside an open Portable Hole\
can exit the hole by climbing out of it.\
You can take a Magic action to close a Portable\
Hole by taking hold of the edges of the cloth and\
folding it up. Folding the cloth closes the hole, and\
any creatures or objects within remain in the ex-\
tradimensional space. No matter what’s in it, the\
hole weighs next to nothing.\
If the hole is folded up, a creature within the hole’s\
extradimensional space can take an action to make\
a DC 10 Strength (Athletics) check. On a successful\
check, the creature forces its way out and appears\
within 5 feet of the Portable Hole. A closed Portable\
Hole holds enough air for 1 hour of breathing, di-\
vided by the number of breathing creatures inside.\
Placing a Portable Hole inside an extradimensional\
space created by a Bag of Holding, Handy Haversack,\
or similar item instantly destroys both items and\
opens a gate to the Astral Plane. The gate originates\
where the one item was placed inside the other. Any\
creature within 10 feet of the gate and not behind\
Total Cover is sucked through it and deposited in\
a random location on the Astral Plane. The gate\
then closes. The gate is one-way only and can’t be\
reopened.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]], [[../../srd-5.2.1/pages/235|page 235]].

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
| ` mechanic ` | ` dnd2024:items/portable_hole ` |
| ` implementation ` | ` dnd2024:item_portable_hole ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` unfolded_diameter_ft ` | ` 6 ` |
| ` weight ` | ` negligible_regardless_of_contents ` |

#### Open

<!-- splat:yaml path=splat_rule/parameters/open -->

```yaml
action: magic
place_on_or_against: solid_surface
space:
  shape: cylinder
  diameter_ft: 6
  depth_ft: 10
  plane: extradimensional
creates_through_passage: false
exit: climb_out
```

#### Close

<!-- splat:values path=splat_rule/parameters/close -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` grasp_edges_and_fold ` | ` true ` |
| ` retain_contents ` | ` true ` |

#### Closed

<!-- splat:yaml path=splat_rule/parameters/closed -->

```yaml
escape:
  action: action
  check:
    ability: strength
    skill: athletics
    dc: 10
  success:
    exit_distance_ft: 5
breathing_hours:
  op: divide
  args:
  - 1
  - binding: breathing_occupants
zero_breathing_occupants_air_depletion: false
```

#### Nested extradimensional

<!-- splat:yaml path=splat_rule/parameters/nested_extradimensional -->

```yaml
trigger: placed_inside_extradimensional_item
examples:
- key: dnd2024:items/bag_of_holding
- key: dnd2024:items/handy_haversack
include_similar_items: true
destroy_both_items: true
gate:
  origin: insertion_location
  destination_plane: astral
  destination_location: random
  creatures_radius_ft: 10
  exclude_total_cover: true
  forced_transport: true
  one_way: true
  close_after_transport: true
  reopenable: false
```

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
