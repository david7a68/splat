---
splat_schema: 1
splat_id: 4dbe2137-d0a6-5ecd-9dce-fc02548f2a1c
splat_kind: item
splat_key: dnd2024:items/circlet_of_blasting
splat_name: Circlet of Blasting
splat_origin:
  key: dnd2024:reference/magic_item/circlet_of_blasting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
---

# Circlet of Blasting

## Source text

<!-- SRD page 215 -->

Wondrous Item, Uncommon

While wearing this circlet, you can cast Scorching Ray with it (+5 to hit). The circlet can’t cast this spell again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/215|page 215]].

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
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/circlet_of_blasting ` |
| ` implementation ` | ` dnd2024:item_circlet_of_blasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` false ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | attack_bonus |
| --- | --- |
| ` 0 ` | ` 5 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scorching_ray ` |

##### Uses

<!-- splat:values path=splat_rule/parameters/spells/0/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |
