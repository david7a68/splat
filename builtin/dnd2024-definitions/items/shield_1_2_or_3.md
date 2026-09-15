---
splat_schema: 1
splat_id: 1b29595a-8b0a-54a2-ac5a-12fe422c33e6
splat_kind: item
splat_key: dnd2024:items/shield_1_2_or_3
splat_name: Shield, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/shield_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
---

# Shield, +1, +2, or +3

## Source text

<!-- SRD page 243 -->

Armor (Shield), Uncommon (+1), Rare (+2), or Very\
Rare (+3)\
While holding this Shield, you have a bonus to Ar-\
mor Class determined by the Shield’s rarity, in addi-\
tion to the Shield’s normal bonus to AC.\

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
| ` mechanic ` | ` dnd2024:items/shield_1_2_or_3 ` |
| ` implementation ` | ` dnd2024:item_shield_1_2_or_3 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Rarity:
    path: rarity
    type: symbol
  Ac bonus:
    path: ac_bonus
    type: integer
  Additional to normal shield bonus:
    path: additional_to_normal_shield_bonus
    type: boolean
-->

| Key | Rarity | Ac bonus | Additional to normal shield bonus |
| --- | --- | --- | --- |
| ` row_1 ` | ` Uncommon ` | ` 1 ` | true |
| ` row_2 ` | ` Rare ` | ` 2 ` | true |
| ` row_3 ` | ` Very rare ` | ` 3 ` | true |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_armor ` |
