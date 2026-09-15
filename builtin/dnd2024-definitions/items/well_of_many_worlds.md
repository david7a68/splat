---
splat_schema: 1
splat_id: 06b372a2-f70b-5b3c-adbf-af31ca9529bf
splat_kind: item
splat_key: dnd2024:items/well_of_many_worlds
splat_name: Well of Many Worlds
splat_origin:
  key: dnd2024:reference/magic_item/well_of_many_worlds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
---

# Well of Many Worlds

## Source text

<!-- SRD page 253 -->

Wondrous Item, Legendary\
This fine black cloth, soft as silk, is folded up to the\
dimensions of a handkerchief. It unfolds into a cir-\
cular sheet 6 feet in diameter.\
You can take a Magic action to unfold the Well of\
Many Worlds and place it on a solid surface, where-\
upon it forms a two-way, 6-foot-diameter, circular\
portal to another world or plane of existence. Each\
time the item opens a portal, the GM decides where\
it leads. The portal remains open until a creature\
within 5 feet of it takes a Magic action to close it\
by taking hold of the edges of the cloth and folding\
it up.\
Once the Well of Many Worlds has opened a portal,\
it can’t do so again for 1d8 hours.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/well_of_many_worlds ` |
| ` implementation ` | ` dnd2024:item_well_of_many_worlds ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` folded_size ` | ` handkerchief ` |
| ` attunement_required ` | ` false ` |

#### Unfolded

<!-- splat:values path=splat_rule/parameters/unfolded -->

| Field | Value |
| --- | --- |
| ` shape ` | ` circle ` |
| ` diameter_ft ` | ` 6 ` |

#### Open

<!-- splat:yaml path=splat_rule/parameters/open -->

```yaml
action: magic
unfold_and_place_on: solid_surface
portal:
  two_way: true
  diameter_ft: 6
  shape: circle
  destination:
    choice: gm_each_opening
    options:
    - another_world
    - another_plane
```

#### Close

<!-- splat:values path=splat_rule/parameters/close -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` actor ` | ` any_creature ` |
| ` maximum_range_ft ` | ` 5 ` |
| ` take_edges_and_fold ` | ` true ` |

#### Cooldown

<!-- splat:yaml path=splat_rule/parameters/cooldown -->

```yaml
starts: portal_opened
duration:
  amount:
    roll:
      count: 1
      sides: 8
  unit: hours
```
