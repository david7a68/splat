---
splat_schema: 2
splat_id: dc1a41b8-fe17-5e55-9061-81c41e8570cb
splat_kind: item
splat_key: dnd2024:items/mithral_armor
splat_name: Mithral Armor
splat_origin: dnd2024:reference/magic_item/mithral_armor
splat_sources:
- SRD 5.2.1, pp. 231, https://www.dndbeyond.com/srd
---

# Mithral Armor


## Item

| Field | Value |
| --- | --- |
| Type | Armor |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 231 -->

Armor (Any Medium or Heavy, Except Hide Armor), Uncommon

Mithral is a light, flexible metal. Armor made of this substance can be worn under normal clothes. If the armor normally imposes Disadvantage on Dexterity (Stealth) checks or has a Strength requirement, the mithral version of the armor doesn’t.

Original layout: [[../../srd-5.2.1/pages/231|page 231]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/mithral_armor ` |
| ` implementation ` | ` dnd2024:item_mithral_armor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` wear_under_normal_clothes ` | ` true ` |
| ` remove_stealth_disadvantage ` | ` true ` |
| ` remove_strength_requirement ` | ` true ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_armor ` |
