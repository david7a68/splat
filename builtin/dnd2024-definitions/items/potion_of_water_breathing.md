---
splat_schema: 2
splat_id: 52803957-ef91-5928-b69d-66c179ae2818
splat_kind: item
splat_key: dnd2024:items/potion_of_water_breathing
splat_name: Potion of Water Breathing
splat_origin: dnd2024:reference/magic_item/potion_of_water_breathing
splat_sources:
- SRD 5.2.1, pp. 236, 237, https://www.dndbeyond.com/srd
---

# Potion of Water Breathing


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 236 -->

Potion, Uncommon

You can breathe underwater for 24 hours after drinking this potion.

<!-- SRD page 237 -->

This potion’s cloudy green fluid smells of the sea and has a jellyfish-like bubble floating in it.

Original layout: [[../../srd-5.2.1/pages/236|page 236]], [[../../srd-5.2.1/pages/237|page 237]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_water_breathing ` |
| ` implementation ` | ` dnd2024:item_potion_of_water_breathing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` drink ` |
| ` breathe_underwater ` | ` true ` |
| ` consume_action ` | ` bonus_action ` |
| ` administer_action ` | ` bonus_action ` |
| ` effect_target ` | ` drinker ` |
| ` consumed_quantity ` | ` 1 ` |
| ` effect_starts ` | ` immediately_after_consumption ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 24 ` |
| ` unit ` | ` hour ` |

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
