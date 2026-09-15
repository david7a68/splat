---
splat_schema: 1
splat_id: 7fb74bcd-4799-55b3-9662-b3a9c42e6975
splat_kind: item
splat_key: dnd2024:items/bracers_of_defense
splat_name: Bracers of Defense
splat_origin:
  key: dnd2024:reference/magic_item/bracers_of_defense
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
---

# Bracers of Defense

## Source text

<!-- SRD page 214 -->

Wondrous Item, Rare (Requires Attunement)\
While wearing these bracers, you gain a +2 bonus to\
Armor Class if you are wearing no armor and using\
no Shield.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/bracers_of_defense ` |
| ` implementation ` | ` dnd2024:item_bracers_of_defense ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` ac_bonus ` | ` 2 ` |
| ` attunement_required ` | ` true ` |

#### Requires equipment state

<!-- splat:values path=splat_rule/parameters/requires_equipment_state -->

| Field | Value |
| --- | --- |
| ` armor_worn ` | ` false ` |
| ` shield_used ` | ` false ` |
