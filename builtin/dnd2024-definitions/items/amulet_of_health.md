---
splat_schema: 2
splat_id: f7747af0-db80-53aa-9416-85779ee7244c
splat_kind: item
splat_key: dnd2024:items/amulet_of_health
splat_name: Amulet of Health
splat_origin: dnd2024:reference/magic_item/amulet_of_health
splat_sources:
- SRD 5.2.1, pp. 209, https://www.dndbeyond.com/srd
---

# Amulet of Health


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 209 -->

Wondrous Item, Rare (Requires Attunement)

Your Constitution is 19 while you wear this amulet. It has no effect on you if your Constitution is 19 or higher without it.

Original layout: [[../../srd-5.2.1/pages/209|page 209]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/amulet_of_health ` |
| ` implementation ` | ` dnd2024:item_amulet_of_health ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Ability

<!-- splat:values path=splat_rule/parameters/ability -->

| Field | Value |
| --- | --- |
| ` name ` | ` constitution ` |
| ` set ` | ` 19 ` |
| ` only_if_unmodified_score_below ` | ` 19 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
