---
splat_schema: 1
splat_id: 284a950f-5282-5b29-9200-ab6307b8ccf3
splat_kind: item
splat_key: dnd2024:items/gauntlets_of_ogre_power
splat_name: Gauntlets of Ogre Power
splat_origin:
  key: dnd2024:reference/magic_item/gauntlets_of_ogre_power
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
---

# Gauntlets of Ogre Power

## Source text

<!-- SRD page 223 -->

Wondrous Item, Uncommon (Requires Attunement)

Your Strength is 19 while you wear these gauntlets. They have no effect on you if your Strength is 19 or higher without them.

Original layout: [[../../srd-5.2.1/pages/223|page 223]].

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
| ` mechanic ` | ` dnd2024:items/gauntlets_of_ogre_power ` |
| ` implementation ` | ` dnd2024:item_gauntlets_of_ogre_power ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Strength

<!-- splat:values path=splat_rule/parameters/strength -->

| Field | Value |
| --- | --- |
| ` set ` | ` 19 ` |
| ` only_if_unmodified_less_than ` | ` 19 ` |

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
