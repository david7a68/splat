---
splat_schema: 1
splat_id: 74b311c4-16a8-53a2-a8bf-e375602b61e8
splat_kind: item
splat_key: dnd2024:items/armor_of_resistance
splat_name: Armor of Resistance
splat_origin:
  key: dnd2024:reference/magic_item/armor_of_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 210
  url: https://www.dndbeyond.com/srd
---

# Armor of Resistance

## Source text

<!-- SRD page 210 -->

Armor (Any Light, Medium, or Heavy), Rare (Requires Attunement)

You have Resistance to one type of damage while you wear this armor. The GM chooses the type or determines it randomly by rolling on the following table.

| 1d10 | Damage Type | 1d10 | Damage Type |
| --- | --- | --- | --- |
| 1 | Acid | 6 | Necrotic |
| 2 | Cold | 7 | Poison |
| 3 | Fire | 8 | Psychic |
| 4 | Force | 9 | Radiant |
| 5 | Lightning | 10 | Thunder |

Original layout: [[../../srd-5.2.1/pages/210|page 210]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
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
| ` mechanic ` | ` dnd2024:items/armor_of_resistance ` |
| ` implementation ` | ` dnd2024:item_armor_of_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

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
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_armor ` |
