---
splat_schema: 1
splat_id: 862e1840-e112-5b5f-b1b9-61821712e71c
splat_kind: item
splat_key: dnd2024:items/mantle_of_spell_resistance
splat_name: Mantle of Spell Resistance
splat_origin:
  key: dnd2024:reference/magic_item/mantle_of_spell_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
---

# Mantle of Spell Resistance

## Source text

<!-- SRD page 229 -->

Wondrous Item, Rare (Requires Attunement)

You have Advantage on saving throws against spells while you wear this cloak.

Original layout: [[../../srd-5.2.1/pages/229|page 229]].

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
| ` mechanic ` | ` dnd2024:items/mantle_of_spell_resistance ` |
| ` implementation ` | ` dnd2024:item_mantle_of_spell_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Saving throw advantage

<!-- splat:values path=splat_rule/parameters/saving_throw_advantage -->

| Field | Value |
| --- | --- |
| ` against ` | ` spells ` |
