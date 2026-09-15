---
splat_schema: 1
splat_id: e623e0c3-a0b7-5bae-ba4e-818aa4064f97
splat_kind: item
splat_key: dnd2024:items/ring_of_telekinesis
splat_name: Ring of Telekinesis
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_telekinesis
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
---

# Ring of Telekinesis

## Source text

<!-- SRD page 239 -->

Ring, Very Rare (Requires Attunement)\
While wearing this ring, you can cast Telekinesis\
from it.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_telekinesis ` |
| ` implementation ` | ` dnd2024:item_ring_of_telekinesis ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | usage |
| --- | --- |
| ` 0 ` | ` at_will ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/telekinesis ` |
