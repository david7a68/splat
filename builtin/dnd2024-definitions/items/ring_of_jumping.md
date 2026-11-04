---
splat_schema: 2
splat_id: adaee6e3-5a29-5e3d-8d5b-3de170c7b8c9
splat_kind: item
splat_key: dnd2024:items/ring_of_jumping
splat_name: Ring of Jumping
splat_origin: dnd2024:reference/magic_item/ring_of_jumping
splat_sources:
- SRD 5.2.1, pp. 238, https://www.dndbeyond.com/srd
---

# Ring of Jumping


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 238 -->

Ring, Uncommon (Requires Attunement)

While wearing this ring, you can cast Jump from it, but can target only yourself when you do so.

Original layout: [[../../srd-5.2.1/pages/238|page 238]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_jumping ` |
| ` implementation ` | ` dnd2024:item_ring_of_jumping ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | target | usage |
| --- | --- | --- |
| ` 0 ` | ` self ` | ` at_will ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/jump ` |
