---
splat_schema: 2
splat_id: 1def14bc-dd94-5aed-aa96-1603e2e9b7d5
splat_kind: item
splat_key: dnd2024:items/stone_of_good_luck_luckstone
splat_name: Stone of Good Luck (Luckstone)
splat_origin: dnd2024:reference/magic_item/stone_of_good_luck_luckstone
splat_sources:
- SRD 5.2.1, pp. 248, https://www.dndbeyond.com/srd
---

# Stone of Good Luck (Luckstone)


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 248 -->

Wondrous Item, Uncommon (Requires Attunement)

While this polished agate is on your person, you gain a +1 bonus to ability checks and saving throws.

Original layout: [[../../srd-5.2.1/pages/248|page 248]].





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
