---
splat_schema: 2
splat_id: d81e65a5-6499-5575-ab01-276809f5c7a0
splat_kind: item
splat_key: dnd2024:items/cloak_of_protection
splat_name: Cloak of Protection
splat_origin: dnd2024:reference/magic_item/cloak_of_protection
splat_sources:
- SRD 5.2.1, pp. 216, https://www.dndbeyond.com/srd
---

# Cloak of Protection


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 216 -->

Wondrous Item, Uncommon (Requires Attunement)

You gain a +1 bonus to Armor Class and saving throws while you wear this cloak.

Original layout: [[../../srd-5.2.1/pages/216|page 216]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/cloak_of_protection ` |
| ` implementation ` | ` dnd2024:item_cloak_of_protection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` ac_bonus ` | ` 1 ` |
| ` saving_throw_bonus ` | ` 1 ` |

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
