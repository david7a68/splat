---
splat_schema: 2
splat_id: da47ba02-6568-5a39-b99b-082f0309b34d
splat_kind: item
splat_key: dnd2024:items/brooch_of_shielding
splat_name: Brooch of Shielding
splat_origin: dnd2024:reference/magic_item/brooch_of_shielding
splat_sources:
- SRD 5.2.1, pp. 214, https://www.dndbeyond.com/srd
---

# Brooch of Shielding


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing this brooch, you have Resistance to Force damage, and you have Immunity to damage from the Magic Missile spell.

Original layout: [[../../srd-5.2.1/pages/214|page 214]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/brooch_of_shielding ` |
| ` implementation ` | ` dnd2024:item_brooch_of_shielding ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` damage_resistance ` | ` force ` |

#### Damage immunity from spells

<!-- splat:rows path=splat_rule/parameters/damage_immunity_from_spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/magic_missile ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
