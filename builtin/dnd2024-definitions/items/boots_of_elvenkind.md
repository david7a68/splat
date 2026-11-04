---
splat_schema: 2
splat_id: fa387b75-f756-59fe-81e1-37fa1384c3b6
splat_kind: item
splat_key: dnd2024:items/boots_of_elvenkind
splat_name: Boots of Elvenkind
splat_origin: dnd2024:reference/magic_item/boots_of_elvenkind
splat_sources:
- SRD 5.2.1, pp. 213, https://www.dndbeyond.com/srd
---

# Boots of Elvenkind


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 213 -->

Wondrous Item, Uncommon

While you wear these boots, your steps make no sound, regardless of the surface you are moving across. You also have Advantage on Dexterity (Stealth) checks.

Original layout: [[../../srd-5.2.1/pages/213|page 213]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/boots_of_elvenkind ` |
| ` implementation ` | ` dnd2024:item_boots_of_elvenkind ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Steps silent

<!-- splat:values path=splat_rule/parameters/steps_silent -->

| Field | Value |
| --- | --- |
| ` all_surfaces ` | ` true ` |

#### Check advantage

<!-- splat:rows path=splat_rule/parameters/check_advantage -->

| Row | ability | skill |
| --- | --- | --- |
| ` 0 ` | ` dexterity ` | ` stealth ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
