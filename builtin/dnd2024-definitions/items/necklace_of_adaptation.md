---
splat_schema: 2
splat_id: bc54a8b3-ca8f-5c4d-bc4f-a54ec1222ff2
splat_kind: item
splat_key: dnd2024:items/necklace_of_adaptation
splat_name: Necklace of Adaptation
splat_origin: dnd2024:reference/magic_item/necklace_of_adaptation
splat_sources:
- SRD 5.2.1, pp. 232, https://www.dndbeyond.com/srd
---

# Necklace of Adaptation


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 232 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing this necklace, you can breathe normally in any environment, and you have Advantage on saving throws made to avoid or end the Poisoned condition.

Original layout: [[../../srd-5.2.1/pages/232|page 232]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/necklace_of_adaptation ` |
| ` implementation ` | ` dnd2024:item_necklace_of_adaptation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` breathing ` | ` normal_in_any_environment ` |

#### Saving throws

<!-- splat:values path=splat_rule/parameters/saving_throws -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |
| ` purpose ` | ` ["avoid_poisoned", "end_poisoned"] ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
