---
splat_schema: 2
splat_id: 403f8926-daa8-52ca-9d11-89b35d68de02
splat_kind: item
splat_key: dnd2024:items/horseshoes_of_speed
splat_name: Horseshoes of Speed
splat_origin: dnd2024:reference/magic_item/horseshoes_of_speed
splat_sources:
- SRD 5.2.1, pp. 227, https://www.dndbeyond.com/srd
---

# Horseshoes of Speed


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 227 -->

Wondrous Item, Rare

These horseshoes come in a set of four. As a Magic action, you can touch one of the horseshoes to the hoof of a horse or similar creature, whereupon the horseshoe affixes itself to the hoof. Removing a horseshoe also takes a Magic action.

While all four horseshoes are attached to the same creature, its Speed is increased by 30 feet.

Original layout: [[../../srd-5.2.1/pages/227|page 227]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/horseshoes_of_speed ` |
| ` implementation ` | ` dnd2024:item_horseshoes_of_speed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` set_quantity ` | ` 4 ` |
| ` attunement_required ` | ` false ` |

#### Attach

<!-- splat:values path=splat_rule/parameters/attach -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` target ` | ` horse_or_similar_creature_hoof ` |
| ` range ` | ` touch ` |
| ` count_per_action ` | ` 1 ` |

#### Remove

<!-- splat:values path=splat_rule/parameters/remove -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` count_per_action ` | ` 1 ` |

#### Benefit

<!-- splat:values path=splat_rule/parameters/benefit -->

| Field | Value |
| --- | --- |
| ` requires_all_four_same_creature ` | ` true ` |
| ` speed_bonus_ft ` | ` 30 ` |
