---
splat_schema: 1
splat_id: 1f3c4507-d7eb-5c94-8a92-f7d397a1b5a7
splat_kind: item
splat_key: dnd2024:items/boots_of_the_winterlands
splat_name: Boots of the Winterlands
splat_origin:
  key: dnd2024:reference/magic_item/boots_of_the_winterlands
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
---

# Boots of the Winterlands

## Source text

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)

These furred boots are snug and feel warm. While wearing them, you gain the following benefits.

Cold Resistance. You have Resistance to Cold damage and can tolerate temperatures of 0 degrees Fahrenheit or lower without any additional protection.

Winter Strider. You ignore Difficult Terrain created by ice or snow.

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
| ` mechanic ` | ` dnd2024:items/boots_of_the_winterlands ` |
| ` implementation ` | ` dnd2024:item_boots_of_the_winterlands ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` resistance ` | ` ["cold"] ` |
| ` ignore_difficult_terrain ` | ` ["ice", "snow"] ` |
| ` attunement_required ` | ` true ` |

#### Temperature protection

<!-- splat:values path=splat_rule/parameters/temperature_protection -->

| Field | Value |
| --- | --- |
| ` fahrenheit_maximum ` | ` 0 ` |
| ` lower_limit ` | ` null ` |
| ` additional_protection_required ` | ` false ` |
