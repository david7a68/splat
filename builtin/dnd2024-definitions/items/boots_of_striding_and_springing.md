---
splat_schema: 1
splat_id: 0a152959-c402-533a-ab75-eabcaf59b4fa
splat_kind: item
splat_key: dnd2024:items/boots_of_striding_and_springing
splat_name: Boots of Striding and Springing
splat_origin:
  key: dnd2024:reference/magic_item/boots_of_striding_and_springing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
---

# Boots of Striding and Springing

## Source text

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)

While you wear these boots, your Speed becomes 30 feet unless your Speed is higher, and your Speed isn’t reduced by you carrying weight in excess of your carrying capacity or wearing Heavy Armor.

Once on each of your turns, you can jump up to 30 feet by spending only 10 feet of movement.

Original layout: [[../../srd-5.2.1/pages/214|page 214]].

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
| ` mechanic ` | ` dnd2024:items/boots_of_striding_and_springing ` |
| ` implementation ` | ` dnd2024:item_boots_of_striding_and_springing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` speed_minimum_ft ` | ` 30 ` |
| ` ignore_speed_reductions_from ` | ` ["excess_carried_weight", "heavy_armor"] ` |

#### Jump

<!-- splat:values path=splat_rule/parameters/jump -->

| Field | Value |
| --- | --- |
| ` uses_per_own_turn ` | ` 1 ` |
| ` maximum_ft ` | ` 30 ` |
| ` movement_cost_ft ` | ` 10 ` |

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
