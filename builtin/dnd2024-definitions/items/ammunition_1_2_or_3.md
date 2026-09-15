---
splat_schema: 1
splat_id: a96e093e-8791-5622-8bea-c073ba0b175c
splat_kind: item
splat_key: dnd2024:items/ammunition_1_2_or_3
splat_name: Ammunition, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/ammunition_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
---

# Ammunition, +1, +2, or +3

## Source text

<!-- SRD page 209 -->

Weapon (Any Ammunition), Uncommon (+1), Rare (+2),\
or Very Rare (+3)\
You have a bonus to attack rolls and damage rolls\
made with this piece of magic ammunition. The bo-\
nus is determined by the rarity of the ammunition.\
Once it hits a target, the ammunition is no longer\
magical.\
This ammunition is typically found or sold in\
quantities of ten or twenty pieces. Ten pieces of this\
ammunition are equivalent in value to a potion of\
the same rarity.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
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
| ` mechanic ` | ` dnd2024:items/ammunition_1_2_or_3 ` |
| ` implementation ` | ` dnd2024:item_ammunition_1_2_or_3 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` typical_lot_sizes ` | ` [10, 20] ` |

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | rarity | attack_roll_bonus | damage_roll_bonus | scope |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` uncommon ` | ` 1 ` | ` 1 ` | ` attacks_with_source_item ` |
| ` 1 ` | ` rare ` | ` 2 ` | ` 2 ` | ` attacks_with_source_item ` |
| ` 2 ` | ` very_rare ` | ` 3 ` | ` 3 ` | ` attacks_with_source_item ` |

#### On hit

<!-- splat:values path=splat_rule/parameters/on_hit -->

| Field | Value |
| --- | --- |
| ` magical ` | ` false ` |

#### Valuation

<!-- splat:values path=splat_rule/parameters/valuation -->

| Field | Value |
| --- | --- |
| ` pieces ` | ` 10 ` |
| ` equivalent_category ` | ` potion ` |
| ` rarity ` | ` same ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
