---
splat_schema: 1
splat_id: d3117ef7-c32c-50e2-a05e-f718d87a83ca
splat_kind: item
splat_key: dnd2024:items/eyes_of_charming
splat_name: Eyes of Charming
splat_origin:
  key: dnd2024:reference/magic_item/eyes_of_charming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 221
  url: https://www.dndbeyond.com/srd
---

# Eyes of Charming

## Source text

<!-- SRD page 221 -->

Wondrous Item, Uncommon (Requires Attunement)

These crystal lenses fit over the eyes. They have 3 charges. While wearing them, you can expend 1 or more charges to cast Charm Person (save DC 13). For 1 charge, you cast the level 1 version of the spell. You increase the spell’s level by one for each additional charge you expend. The lenses regain all expended charges daily at dawn.

Original layout: [[../../srd-5.2.1/pages/221|page 221]].

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
| ` mechanic ` | ` dnd2024:items/eyes_of_charming ` |
| ` implementation ` | ` dnd2024:item_eyes_of_charming ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 3
maximum: 3
recovery:
  event: time.dawn
  amount: all
  cap_at_maximum: true
```

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc | cast_level |
| --- | --- | --- |
| ` 0 ` | ` 13 ` | ` charges_spent ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/charm_person ` |

##### Charge choice

<!-- splat:values path=splat_rule/parameters/spells/0/charge_choice -->

| Field | Value |
| --- | --- |
| ` min ` | ` 1 ` |
| ` max ` | ` available_charges ` |
