---
splat_schema: 1
splat_id: da47ba02-6568-5a39-b99b-082f0309b34d
splat_kind: item
splat_key: dnd2024:items/brooch_of_shielding
splat_name: Brooch of Shielding
splat_origin:
  key: dnd2024:reference/magic_item/brooch_of_shielding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
---

# Brooch of Shielding

## Source text

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing this brooch, you have Resistance to Force damage, and you have Immunity to damage from the Magic Missile spell.

Original layout: [[../../srd-5.2.1/pages/214|page 214]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

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
