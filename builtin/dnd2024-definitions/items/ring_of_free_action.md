---
splat_schema: 2
splat_id: 8780760b-3ed7-5057-8cc1-336f263c3fe4
splat_kind: item
splat_key: dnd2024:items/ring_of_free_action
splat_name: Ring of Free Action
splat_origin: dnd2024:reference/magic_item/ring_of_free_action
splat_sources:
- SRD 5.2.1, pp. 238, https://www.dndbeyond.com/srd
---

# Ring of Free Action


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)

While you wear this ring, Difficult Terrain doesn’t cost you extra movement. In addition, magic can neither reduce any of your Speeds nor cause you to have the Paralyzed or Restrained condition.

Original layout: [[../../srd-5.2.1/pages/238|page 238]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_free_action ` |
| ` implementation ` | ` dnd2024:item_ring_of_free_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` difficult_terrain_extra_cost ` | ` 0 ` |

#### Magic prevention

<!-- splat:values path=splat_rule/parameters/magic_prevention -->

| Field | Value |
| --- | --- |
| ` speed_reductions ` | ` all_speeds ` |
| ` conditions ` | ` ["paralyzed", "restrained"] ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
