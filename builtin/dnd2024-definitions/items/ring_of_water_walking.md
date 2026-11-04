---
splat_schema: 2
splat_id: a74684b9-44a9-5787-93cb-114328f7a365
splat_kind: item
splat_key: dnd2024:items/ring_of_water_walking
splat_name: Ring of Water Walking
splat_origin: dnd2024:reference/magic_item/ring_of_water_walking
splat_sources:
- SRD 5.2.1, pp. 239, https://www.dndbeyond.com/srd
---

# Ring of Water Walking


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 239 -->

Ring, Uncommon

While wearing this ring, you cast Water Walk from it, targeting only yourself.

Original layout: [[../../srd-5.2.1/pages/239|page 239]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_water_walking ` |
| ` implementation ` | ` dnd2024:item_ring_of_water_walking ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | targets |
| --- | --- |
| ` 0 ` | ` self_only ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/water_walk ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
