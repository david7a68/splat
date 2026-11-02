---
splat_schema: 1
splat_id: c76ed8ed-0c74-5d58-8cfa-24e9a38c1094
splat_kind: item
splat_key: dnd2024:items/potion_of_climbing
splat_name: Potion of Climbing
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_climbing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
---

# Potion of Climbing

## Source text

<!-- SRD page 235 -->

Potion, Common

When you drink this potion, you gain a Climb Speed equal to your Speed for 1 hour. During this time, you have Advantage on Strength (Athletics) checks to climb.

This potion is separated into brown, silver, and gray layers resembling bands of stone. Shaking the bottle fails to mix the colors.

Original layout: [[../../srd-5.2.1/pages/235|page 235]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` common ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_climbing ` |
| ` implementation ` | ` dnd2024:item_potion_of_climbing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` drink ` |
| ` climb_speed ` | ` speed ` |
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

#### Check advantage

<!-- splat:values path=splat_rule/parameters/check_advantage -->

| Field | Value |
| --- | --- |
| ` ability ` | ` strength ` |
| ` skill ` | ` athletics ` |
| ` task ` | ` climb ` |

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
