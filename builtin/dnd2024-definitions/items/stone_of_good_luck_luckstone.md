---
splat_schema: 1
splat_id: 1def14bc-dd94-5aed-aa96-1603e2e9b7d5
splat_kind: item
splat_key: dnd2024:items/stone_of_good_luck_luckstone
splat_name: Stone of Good Luck (Luckstone)
splat_origin:
  key: dnd2024:reference/magic_item/stone_of_good_luck_luckstone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
---

# Stone of Good Luck (Luckstone)

## Source text

<!-- SRD page 248 -->

Wondrous Item, Uncommon (Requires Attunement)\
While this polished agate is on your person, you\
gain a +1 bonus to ability checks and saving throws.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/stone_of_good_luck_luckstone ` |
| ` implementation ` | ` dnd2024:item_stone_of_good_luck_luckstone ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` on_person ` |
| ` ability_check_bonus ` | ` 1 ` |
| ` saving_throw_bonus ` | ` 1 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
