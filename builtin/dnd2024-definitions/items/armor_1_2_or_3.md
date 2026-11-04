---
splat_schema: 2
splat_id: ad64e00b-cd76-5a1e-86d8-83a29ebc8a6a
splat_kind: item
splat_key: dnd2024:items/armor_1_2_or_3
splat_name: Armor, +1, +2, or +3
splat_origin: dnd2024:reference/magic_item/armor_1_2_or_3
splat_sources:
- SRD 5.2.1, pp. 210, https://www.dndbeyond.com/srd
---

# Armor, +1, +2, or +3


## Item

| Field | Value |
| --- | --- |
| Type | Armor |
| Rarity | Varies |
| Attunement | No |


## Source text

<!-- SRD page 210 -->

Armor (Any Light, Medium, or Heavy), Rare (+1), Very Rare (+2), or Legendary (+3)

You have a bonus to Armor Class while wearing this armor. The bonus is determined by its rarity.

Original layout: [[../../srd-5.2.1/pages/210|page 210]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/armor_1_2_or_3 ` |
| ` implementation ` | ` dnd2024:item_armor_1_2_or_3 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` base_armor_categories ` | ` ["light", "medium", "heavy"] ` |
| ` attunement_required ` | ` false ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Rarity:
    path: rarity
    type: symbol
  Ac bonus:
    path: ac_bonus
    type: integer
-->

| Key | Rarity | Ac bonus |
| --- | --- | --- |
| ` row_1 ` | ` Rare ` | ` 1 ` |
| ` row_2 ` | ` Very rare ` | ` 2 ` |
| ` row_3 ` | ` Legendary ` | ` 3 ` |
