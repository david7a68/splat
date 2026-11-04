---
splat_schema: 2
splat_id: d5a04c8e-8259-516d-9ba6-0bc5d8fe607a
splat_kind: item
splat_key: dnd2024:items/gloves_of_thievery
splat_name: Gloves of Thievery
splat_origin: dnd2024:reference/magic_item/gloves_of_thievery
splat_sources:
- SRD 5.2.1, pp. 224, https://www.dndbeyond.com/srd
---

# Gloves of Thievery


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 224 -->

Wondrous Item, Uncommon

These gloves are imperceptible while worn. While wearing them, you gain a +5 bonus to Dexterity (Sleight of Hand) checks.

Original layout: [[../../srd-5.2.1/pages/224|page 224]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/gloves_of_thievery ` |
| ` implementation ` | ` dnd2024:item_gloves_of_thievery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` item_imperceptible ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Checks

<!-- splat:rows path=splat_rule/parameters/checks -->

| Row | ability | skill | bonus |
| --- | --- | --- | --- |
| ` 0 ` | ` dexterity ` | ` sleight_of_hand ` | ` 5 ` |
