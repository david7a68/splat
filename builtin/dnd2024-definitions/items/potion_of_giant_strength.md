---
splat_schema: 1
splat_id: bea1d8df-9a89-5e1c-a913-4262061ae9e9
splat_kind: item
splat_key: dnd2024:items/potion_of_giant_strength
splat_name: Potion of Giant Strength
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_giant_strength
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
---

# Potion of Giant Strength

## Source text

<!-- SRD page 235 -->

Potion, Rarity Varies

When you drink this potion, your Strength score changes for 1 hour. The type of giant determines the score (see the table below). The potion has no effect on you if your Strength is equal to or greater than that score.

This potion’s transparent liquid has floating in it a sliver of light resembling a giant’s fingernail.

| Potion | Str. | Rarity |
| --- | --- | --- |
| Potion of Giant Strength (hill) | 21 | Uncommon |
| Potion of Giant Strength (frost or stone) | 23 | Rare |
| Potion of Giant Strength (fire) | 25 | Rare |
| Potion of Giant Strength (cloud) | 27 | Very Rare |
| Potion of Giant Strength (storm) | 29 | Legendary |

Original layout: [[../../srd-5.2.1/pages/235|page 235]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` rarity_varies ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_giant_strength ` |
| ` implementation ` | ` dnd2024:item_potion_of_giant_strength ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` potion_drunk ` |
| ` consumed ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 1 ` |
| ` unit ` | ` hours ` |

#### Ability floor

<!-- splat:values path=splat_rule/parameters/ability_floor -->

| Field | Value |
| --- | --- |
| ` ability ` | ` strength ` |
| ` when ` | ` current_score_below_variant ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
columns:
  Key:
    type: key
  Giant:
    path: giant
    type: symbol
  Score:
    path: score
    type: integer
  Rarity:
    path: rarity
    type: symbol
-->

| Key | Giant | Score | Rarity |
| --- | --- | --- | --- |
| ` row_1 ` | ` Hill ` | ` 21 ` | ` Uncommon ` |
| ` row_2 ` | ` Frost ` | ` 23 ` | ` Rare ` |
| ` row_3 ` | ` Stone ` | ` 23 ` | ` Rare ` |
| ` row_4 ` | ` Fire ` | ` 25 ` | ` Rare ` |
| ` row_5 ` | ` Cloud ` | ` 27 ` | ` Very rare ` |
| ` row_6 ` | ` Storm ` | ` 29 ` | ` Legendary ` |
