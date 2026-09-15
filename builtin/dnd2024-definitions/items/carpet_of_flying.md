---
splat_schema: 1
splat_id: 8867b69e-fad0-5895-9ac4-1a490d43be74
splat_kind: item
splat_key: dnd2024:items/carpet_of_flying
splat_name: Carpet of Flying
splat_origin:
  key: dnd2024:reference/magic_item/carpet_of_flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
---

# Carpet of Flying

## Source text

<!-- SRD page 215 -->

Wondrous Item, Very Rare\
You can make this carpet hover and fly by taking a\
Magic action and using the carpet’s command word.\
It moves according to your directions if you are\
within 30 feet of it.\
Four sizes of Carpet of Flying exist. The GM\
chooses the size of a given carpet or determines it\
randomly by rolling on the following table. A carpet\
can carry up to twice the weight shown on the table,\
but its Fly Speed is halved if it carries more than its\
normal capacity.\
1d100 Size Capacity Fly Speed\
01–20 3 ft. × 5 ft. 200 lb. 80 feet\
21–55 4 ft. × 6 ft. 400 lb. 60 feet\
56–80 5 ft. × 7 ft. 600 lb. 40 feet\
81–00 6 ft. × 9 ft. 800 lb. 30 feet\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].

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
| ` mechanic ` | ` dnd2024:items/carpet_of_flying ` |
| ` implementation ` | ` dnd2024:item_carpet_of_flying ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` direction_control_range_ft ` | ` 30 ` |
| ` variant_selection ` | ` gm_choice_or_roll ` |
| ` variant_die ` | ` 100 ` |
| ` maximum_capacity_multiplier ` | ` 2 ` |
| ` over_normal_capacity_speed_multiplier ` | ` 0.5 ` |
| ` attunement_required ` | ` false ` |

#### Activate

<!-- splat:values path=splat_rule/parameters/activate -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` command_word ` | ` true ` |
| ` mode ` | ` hovering_flying ` |

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | roll | dimensions_ft | normal_capacity_lb | fly_speed_ft |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` [1, 20] ` | ` [3, 5] ` | ` 200 ` | ` 80 ` |
| ` 1 ` | ` [21, 55] ` | ` [4, 6] ` | ` 400 ` | ` 60 ` |
| ` 2 ` | ` [56, 80] ` | ` [5, 7] ` | ` 600 ` | ` 40 ` |
| ` 3 ` | ` [81, 100] ` | ` [6, 9] ` | ` 800 ` | ` 30 ` |
