---
splat_schema: 1
splat_id: 1063d842-0468-5b8a-86de-f8e52750cbac
splat_kind: item
splat_key: dnd2024:items/potions_of_healing
splat_name: Potions of Healing
splat_origin:
  key: dnd2024:reference/magic_item/potions_of_healing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potions of Healing

## Source text

<!-- SRD page 236 -->

Potion, Rarity Varies\
You regain Hit Points when you drink this potion.\
The number of Hit Points depends on the potion’s\
rarity, as shown in the table below.\
Whatever its potency, the potion’s red liquid glim-\
mers when agitated.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` rarity_varies ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potions_of_healing ` |
| ` implementation ` | ` dnd2024:item_potions_of_healing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` potion_drunk ` |
| ` consumed ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Name
order:
- healing
- greater
- superior
- supreme
columns:
  Name:
    path: name
    type: symbol
  Rarity:
    path: rarity
    type: symbol
  Healing:
    path: heal_hp/roll
    type: dice
-->

| Name | Rarity | Healing |
| --- | --- | --- |
| ` Healing ` | ` Common ` | ` 2d4+2 ` |
| ` Greater ` | ` Uncommon ` | ` 4d4+4 ` |
| ` Superior ` | ` Rare ` | ` 8d4+8 ` |
| ` Supreme ` | ` Very rare ` | ` 10d4+20 ` |
