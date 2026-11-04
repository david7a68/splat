---
splat_schema: 2
splat_id: e623e0c3-a0b7-5bae-ba4e-818aa4064f97
splat_kind: item
splat_key: dnd2024:items/ring_of_telekinesis
splat_name: Ring of Telekinesis
splat_origin: dnd2024:reference/magic_item/ring_of_telekinesis
splat_sources:
- SRD 5.2.1, pp. 239, https://www.dndbeyond.com/srd
---

# Ring of Telekinesis


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 239 -->

Ring, Very Rare (Requires Attunement)

While wearing this ring, you can cast Telekinesis from it.

Original layout: [[../../srd-5.2.1/pages/239|page 239]].





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
