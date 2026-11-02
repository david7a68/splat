---
splat_schema: 1
splat_id: c2faff68-cbd0-58f9-9e07-68dc7c5b0d33
splat_kind: item
splat_key: dnd2024:items/potion_of_resistance
splat_name: Potion of Resistance
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potion of Resistance

## Source text

<!-- SRD page 236 -->

Potion, Uncommon

When you drink this potion, you have Resistance to one type of damage for 1 hour. The GM chooses the type or determines it randomly by rolling on the following table.

| 1d10 | Damage Type | 1d10 | Damage Type |
| --- | --- | --- | --- |
| 1 | Acid | 6 | Necrotic |
| 2 | Cold | 7 | Poison |
| 3 | Fire | 8 | Psychic |
| 4 | Force | 9 | Radiant |
| 5 | Lightning | 10 | Thunder |

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
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
| ` mechanic ` | ` dnd2024:items/potion_of_resistance ` |
| ` implementation ` | ` dnd2024:item_potion_of_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` drink ` |
| ` consume_action ` | ` bonus_action ` |
| ` administer_action ` | ` bonus_action ` |
| ` effect_target ` | ` drinker ` |
| ` consumed_quantity ` | ` 1 ` |
| ` effect_starts ` | ` immediately_after_consumption ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 1 ` |
| ` unit ` | ` hour ` |

#### Resistance

<!-- splat:values path=splat_rule/parameters/resistance -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` method ` | ` ["choose", "roll"] ` |

##### Table

###### Roll

<!-- splat:values path=splat_rule/parameters/resistance/table/roll -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d10 ` |
| ` scope ` | ` use ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/resistance/table/rows
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Damage type:
    path: result/damage_type
    type: symbol
-->

| Key | Roll | Damage type |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` Acid ` |
| ` row_2 ` | ` 2 ` | ` Cold ` |
| ` row_3 ` | ` 3 ` | ` Fire ` |
| ` row_4 ` | ` 4 ` | ` Force ` |
| ` row_5 ` | ` 5 ` | ` Lightning ` |
| ` row_6 ` | ` 6 ` | ` Necrotic ` |
| ` row_7 ` | ` 7 ` | ` Poison ` |
| ` row_8 ` | ` 8 ` | ` Psychic ` |
| ` row_9 ` | ` 9 ` | ` Radiant ` |
| ` row_10 ` | ` 10 ` | ` Thunder ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_potions ` |
