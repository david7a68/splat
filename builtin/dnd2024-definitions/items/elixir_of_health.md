---
splat_schema: 1
splat_id: 1d6f9e46-bab7-52d0-8377-59a9e84f2098
splat_kind: item
splat_key: dnd2024:items/elixir_of_health
splat_name: Elixir of Health
splat_origin:
  key: dnd2024:reference/magic_item/elixir_of_health
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
---

# Elixir of Health

## Source text

<!-- SRD page 220 -->

Potion, Rare

When you drink this potion, you are cured of all magical contagions. In addition, the following conditions end on you: Blinded, Deafened, Paralyzed, and Poisoned.

The clear, red liquid has tiny bubbles of light in it.

Original layout: [[../../srd-5.2.1/pages/220|page 220]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/elixir_of_health ` |
| ` implementation ` | ` dnd2024:item_elixir_of_health ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` drink ` |
| ` cure ` | ` all_magical_contagions ` |
| ` end_conditions ` | ` ["blinded", "deafened", "paralyzed", "poisoned"] ` |
| ` consume_action ` | ` bonus_action ` |
| ` administer_action ` | ` bonus_action ` |
| ` effect_target ` | ` drinker ` |
| ` consumed_quantity ` | ` 1 ` |
| ` effect_starts ` | ` immediately_after_consumption ` |

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
