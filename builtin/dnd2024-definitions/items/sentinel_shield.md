---
splat_schema: 1
splat_id: 842752d3-d1b1-51bc-a8dc-9fb075f0617a
splat_kind: item
splat_key: dnd2024:items/sentinel_shield
splat_name: Sentinel Shield
splat_origin:
  key: dnd2024:reference/magic_item/sentinel_shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
---

# Sentinel Shield

## Source text

<!-- SRD page 243 -->

Armor (Shield), Uncommon\
While holding this Shield, you have Advantage on\
Initiative rolls and Wisdom (Perception) checks.\
The Shield is emblazoned with a symbol of an eye.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sentinel_shield ` |
| ` implementation ` | ` dnd2024:item_sentinel_shield ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |
| ` advantage ` | ` ["initiative", "wisdom_perception"] ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
