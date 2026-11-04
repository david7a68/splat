---
splat_schema: 2
splat_id: 1063d842-0468-5b8a-86de-f8e52750cbac
splat_kind: item
splat_key: dnd2024:items/potions_of_healing
splat_name: Potions of Healing
splat_origin: dnd2024:reference/magic_item/potions_of_healing
splat_sources:
- SRD 5.2.1, pp. 236, https://www.dndbeyond.com/srd
---

# Potions of Healing


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Rarity Varies |
| Attunement | No |


## Source text

<!-- SRD page 236 -->

Potion, Rarity Varies

You regain Hit Points when you drink this potion. The number of Hit Points depends on the potion’s rarity, as shown in the table below.

Whatever its potency, the potion’s red liquid glimmers when agitated.

| Potion | HP Regained | Rarity |
| --- | --- | --- |
| Potion of Healing | 2d4 + 2 | Common |
| Potion of Healing (greater) | 4d4 + 4 | Uncommon |
| Potion of Healing (superior) | 8d4 + 8 | Rare |
| Potion of Healing (supreme) | 10d4 + 20 | Very Rare |

Original layout: [[../../srd-5.2.1/pages/236|page 236]].





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
