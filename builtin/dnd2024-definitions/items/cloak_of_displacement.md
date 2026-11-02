---
splat_schema: 1
splat_id: d9f396fd-3080-5390-b321-289e8c9082fc
splat_kind: item
splat_key: dnd2024:items/cloak_of_displacement
splat_name: Cloak of Displacement
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_displacement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
---

# Cloak of Displacement

## Source text

<!-- SRD page 215 -->

Wondrous Item, Rare (Requires Attunement)

While you wear this cloak, it magically projects an illusion that makes you appear to be standing in a place near your actual location, causing any creature to have Disadvantage on attack rolls against you. If you take damage, the property ceases to function until the start of your next turn. This property is suppressed while your Speed is 0.

Original layout: [[../../srd-5.2.1/pages/215|page 215]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/cloak_of_displacement ` |
| ` implementation ` | ` dnd2024:item_cloak_of_displacement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` attacks_against_wearer ` | ` disadvantage ` |
| ` effect_type ` | ` illusion ` |

#### Damage taken

<!-- splat:values path=splat_rule/parameters/damage_taken -->

| Field | Value |
| --- | --- |
| ` suppress_until ` | ` start_of_wearer_next_turn ` |

#### Suppress when

<!-- splat:values path=splat_rule/parameters/suppress_when -->

| Field | Value |
| --- | --- |
| ` speed_ft ` | ` 0 ` |

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
