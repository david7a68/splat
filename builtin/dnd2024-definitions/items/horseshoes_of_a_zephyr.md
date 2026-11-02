---
splat_schema: 1
splat_id: 1080cfe9-ff7f-51a3-8f7b-128ae97ccd80
splat_kind: item
splat_key: dnd2024:items/horseshoes_of_a_zephyr
splat_name: Horseshoes of a Zephyr
splat_origin:
  key: dnd2024:reference/magic_item/horseshoes_of_a_zephyr
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
---

# Horseshoes of a Zephyr

## Source text

<!-- SRD page 226 -->

Wondrous Item, Very Rare

These horseshoes come in a set of four. As a Magic action, you can touch one of the horseshoes to the hoof of a horse or similar creature, whereupon the horseshoe affixes itself to the hoof. Removing a horseshoe also takes a Magic action.

While all four shoes are affixed to the hooves of a horse or similar creature, they allow the creature to move normally while floating 4 inches above a surface. This effect means the creature can cross or stand above nonsolid or unstable surfaces, such as water or lava. The creature leaves no tracks and ignores Difficult Terrain. In addition, the creature can travel for up to 12 hours a day without gaining Exhaustion levels from extended travel.

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/horseshoes_of_a_zephyr ` |
| ` implementation ` | ` dnd2024:item_horseshoes_of_a_zephyr ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` set_size ` | ` 4 ` |

#### Affix

<!-- splat:values path=splat_rule/parameters/affix -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` target ` | ` hoof ` |
| ` eligible_creatures ` | ` ["horse", "similar_creature"] ` |

#### Remove

<!-- splat:values path=splat_rule/parameters/remove -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |

#### While all four affixed

<!-- splat:values path=splat_rule/parameters/while_all_four_affixed -->

| Field | Value |
| --- | --- |
| ` float_height_inches ` | ` 4 ` |
| ` normal_movement ` | ` true ` |
| ` surface_support_required ` | ` false ` |
| ` surface_examples ` | ` ["water", "lava"] ` |
| ` leaves_tracks ` | ` false ` |
| ` ignore_difficult_terrain ` | ` true ` |
| ` daily_travel_without_extended_travel_exhaustion_hours ` | ` 12 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
