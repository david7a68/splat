---
splat_schema: 1
splat_id: ad24d43b-f077-5484-a9d1-04d040dd4afd
splat_kind: item
splat_key: dnd2024:items/belt_of_giant_strength
splat_name: Belt of Giant Strength
splat_origin:
  key: dnd2024:reference/magic_item/belt_of_giant_strength
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
---

# Belt of Giant Strength

## Source text

<!-- SRD page 213 -->

Wondrous Item, Rarity Varies (Requires Attunement)\
While wearing this belt, your Strength changes to\
a score granted by the belt. The type of giant deter-\
mines the score (see the table below). The item has\
no effect on you if your Strength without the belt is\
equal to or greater than the belt’s score.\
Belt Str. Rarity\
Belt of Giant Strength (hill) 21 Rare\
Belt of Giant Strength (frost or 23 Very Rare\
stone)\
Belt of Giant Strength (fire) 25 Very Rare\
Belt of Giant Strength (cloud) 27 Legendary\
Belt of Giant Strength (storm) 29 Legendary\

Original layout: [[../../srd-5.2.1/pages/213|page 213]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rarity_varies ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/belt_of_giant_strength ` |
| ` implementation ` | ` dnd2024:item_belt_of_giant_strength ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Ability floor

<!-- splat:values path=splat_rule/parameters/ability_floor -->

| Field | Value |
| --- | --- |
| ` ability ` | ` strength ` |
| ` when ` | ` unmodified_score_below_variant ` |

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | giant | score | rarity |
| --- | --- | --- | --- |
| ` 0 ` | ` hill ` | ` 21 ` | ` rare ` |
| ` 1 ` | ` frost ` | ` 23 ` | ` very_rare ` |
| ` 2 ` | ` stone ` | ` 23 ` | ` very_rare ` |
| ` 3 ` | ` fire ` | ` 25 ` | ` very_rare ` |
| ` 4 ` | ` cloud ` | ` 27 ` | ` legendary ` |
| ` 5 ` | ` storm ` | ` 29 ` | ` legendary ` |
