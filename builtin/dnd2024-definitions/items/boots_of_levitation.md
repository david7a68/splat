---
splat_schema: 2
splat_id: a462de3b-b0f7-5b2d-9fb3-107759b2ea62
splat_kind: item
splat_key: dnd2024:items/boots_of_levitation
splat_name: Boots of Levitation
splat_origin: dnd2024:reference/magic_item/boots_of_levitation
splat_sources:
- SRD 5.2.1, pp. 213, https://www.dndbeyond.com/srd
---

# Boots of Levitation


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 213 -->

Wondrous Item, Rare (Requires Attunement)

While you wear these boots, you can cast Levitate on yourself.

Original layout: [[../../srd-5.2.1/pages/213|page 213]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/boots_of_levitation ` |
| ` implementation ` | ` dnd2024:item_boots_of_levitation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | target | usage |
| --- | --- | --- |
| ` 0 ` | ` self ` | ` at_will ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/levitate ` |
