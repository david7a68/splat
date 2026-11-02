---
splat_schema: 1
splat_id: db471f07-b63f-599f-8670-4fa50832dc6f
splat_kind: item
splat_key: dnd2024:items/potion_of_heroism
splat_name: Potion of Heroism
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_heroism
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potion of Heroism

## Source text

<!-- SRD page 236 -->

Potion, Rare

When you drink this potion, you gain 10 Temporary Hit Points that last for 1 hour. For the same duration, you are under the effect of the Bless spell (no Concentration required).

This potion’s blue liquid bubbles and steams as if boiling.

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

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
| ` mechanic ` | ` dnd2024:items/potion_of_heroism ` |
| ` implementation ` | ` dnd2024:item_potion_of_heroism ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` drink ` |
| ` temporary_hp ` | ` 10 ` |
| ` concentration ` | ` false ` |
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

#### Spell effect

<!-- splat:values path=splat_rule/parameters/spell_effect -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bless ` |

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
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
