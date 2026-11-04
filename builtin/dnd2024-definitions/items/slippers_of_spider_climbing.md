---
splat_schema: 2
splat_id: ad506ac5-cb59-56b9-9b08-035ca83a9f86
splat_kind: item
splat_key: dnd2024:items/slippers_of_spider_climbing
splat_name: Slippers of Spider Climbing
splat_origin: dnd2024:reference/magic_item/slippers_of_spider_climbing
splat_sources:
- SRD 5.2.1, pp. 244, https://www.dndbeyond.com/srd
---

# Slippers of Spider Climbing


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 244 -->

Wondrous Item, Uncommon (Requires Attunement)

While you wear these light shoes, you can move up, down, and across vertical surfaces and along ceilings, while leaving your hands free. You have a Climb Speed equal to your Speed. However, the slippers don’t allow you to move this way on a slippery surface, such as one covered by ice or oil.

Original layout: [[../../srd-5.2.1/pages/244|page 244]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/slippers_of_spider_climbing ` |
| ` implementation ` | ` dnd2024:item_slippers_of_spider_climbing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` climb_speed ` | ` speed ` |

#### Spider movement

<!-- splat:values path=splat_rule/parameters/spider_movement -->

| Field | Value |
| --- | --- |
| ` surfaces ` | ` ["vertical", "ceiling"] ` |
| ` hands_free ` | ` true ` |
| ` disallowed_surface ` | ` slippery ` |
| ` examples ` | ` ["ice", "oil"] ` |

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
