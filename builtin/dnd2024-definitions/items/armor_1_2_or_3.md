---
splat_schema: 1
splat_id: ad64e00b-cd76-5a1e-86d8-83a29ebc8a6a
splat_kind: item
splat_key: dnd2024:items/armor_1_2_or_3
splat_name: Armor, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/armor_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 210
  url: https://www.dndbeyond.com/srd
---

# Armor, +1, +2, or +3

## Source text

<!-- SRD page 210 -->

Armor (Any Light, Medium, or Heavy), Rare (+1), Very\
Rare (+2), or Legendary (+3)\
You have a bonus to Armor Class while wearing this\
armor. The bonus is determined by its rarity.\

Original layout: [[../../srd-5.2.1/pages/210|page 210]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
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

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | rarity | ac_bonus |
| --- | --- | --- |
| ` 0 ` | ` rare ` | ` 1 ` |
| ` 1 ` | ` very_rare ` | ` 2 ` |
| ` 2 ` | ` legendary ` | ` 3 ` |
