---
splat_schema: 2
splat_id: 545e5e8e-ebf6-5af6-bf39-05c4a1ed43b2
splat_kind: item
splat_key: dnd2024:items/potion_of_gaseous_form
splat_name: Potion of Gaseous Form
splat_origin: dnd2024:reference/magic_item/potion_of_gaseous_form
splat_sources:
- SRD 5.2.1, pp. 235, https://www.dndbeyond.com/srd
---

# Potion of Gaseous Form


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 235 -->

Potion, Rare

When you drink this potion, you gain the effect of the Gaseous Form spell for 1 hour (no Concentration required) or until you end the effect as a Bonus Action.

This potion’s container seems to hold fog that moves and pours like water.

Original layout: [[../../srd-5.2.1/pages/235|page 235]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_gaseous_form ` |
| ` implementation ` | ` dnd2024:item_potion_of_gaseous_form ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` drink ` |
| ` concentration ` | ` false ` |
| ` end_early_action ` | ` bonus_action ` |
| ` consume_action ` | ` bonus_action ` |
| ` administer_action ` | ` bonus_action ` |
| ` effect_target ` | ` drinker ` |
| ` consumed_quantity ` | ` 1 ` |
| ` effect_starts ` | ` immediately_after_consumption ` |

#### Spell effect

<!-- splat:values path=splat_rule/parameters/spell_effect -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gaseous_form ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 1 ` |
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
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
