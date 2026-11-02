---
splat_schema: 1
splat_id: 852f4cfa-561d-5617-ac29-10e26082efe0
splat_kind: item
splat_key: dnd2024:items/gloves_of_swimming_and_climbing
splat_name: Gloves of Swimming and Climbing
splat_origin:
  key: dnd2024:reference/magic_item/gloves_of_swimming_and_climbing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
---

# Gloves of Swimming and Climbing

## Source text

<!-- SRD page 224 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing these gloves, you have a Climb Speed and a Swim Speed equal to your Speed, and you gain a +5 bonus to Strength (Athletics) checks made to climb or swim.

Original layout: [[../../srd-5.2.1/pages/224|page 224]].

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
| ` mechanic ` | ` dnd2024:items/gloves_of_swimming_and_climbing ` |
| ` implementation ` | ` dnd2024:item_gloves_of_swimming_and_climbing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` climb_speed ` | ` speed ` |
| ` swim_speed ` | ` speed ` |

#### Check bonus

<!-- splat:values path=splat_rule/parameters/check_bonus -->

| Field | Value |
| --- | --- |
| ` ability ` | ` strength ` |
| ` skill ` | ` athletics ` |
| ` tasks ` | ` ["climb", "swim"] ` |
| ` amount ` | ` 5 ` |

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
