---
splat_schema: 1
splat_id: c56f82f6-583d-559f-a482-c9b586cd3638
splat_kind: item
splat_key: dnd2024:items/potion_of_clairvoyance
splat_name: Potion of Clairvoyance
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_clairvoyance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
---

# Potion of Clairvoyance

## Source text

<!-- SRD page 235 -->

Potion, Rare

When you drink this potion, you gain the effect of the Clairvoyance spell (no Concentration required).

An eyeball bobs in this potion’s yellowish liquid but vanishes when the potion is opened.

Original layout: [[../../srd-5.2.1/pages/235|page 235]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_clairvoyance ` |
| ` implementation ` | ` dnd2024:item_potion_of_clairvoyance ` |

### Parameters

#### Consumption

<!-- splat:yaml path=splat_rule/parameters/consumption -->

```yaml
trigger: drink
consume_item: true
gain_spell_effect:
  spell:
    key: dnd2024:spells/clairvoyance
  concentration: false
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
