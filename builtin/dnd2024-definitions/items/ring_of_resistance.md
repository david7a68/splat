---
splat_schema: 1
splat_id: e61dfba9-0409-5feb-b290-94310960df25
splat_kind: item
splat_key: dnd2024:items/ring_of_resistance
splat_name: Ring of Resistance
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
---

# Ring of Resistance

## Source text

<!-- SRD page 238 -->

Ring, Rare

You have Resistance to one damage type while wearing this ring. The gemstone in the ring indicates the type, which the GM chooses or determines randomly by rolling on the following table.

| 1d10 | Damage Type | Gemstone |
| --- | --- | --- |
| 1 | Acid | Pearl |
| 2 | Cold | Tourmaline |
| 3 | Fire | Garnet |
| 4 | Force | Sapphire |
| 5 | Lightning | Citrine |
| 6 | Necrotic | Jet |
| 7 | Poison | Amethyst |
| 8 | Psychic | Jade |
| 9 | Radiant | Topaz |
| 10 | Thunder | Spinel |

Original layout: [[../../srd-5.2.1/pages/238|page 238]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
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
| ` mechanic ` | ` dnd2024:items/ring_of_resistance ` |
| ` implementation ` | ` dnd2024:item_ring_of_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` variant_selection ` | ` ["gm_choice", "d10"] ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
columns:
  Key:
    type: key
  Roll:
    path: roll
    type: integer
  Resistance:
    path: resistance
    type: symbol
  Gemstone:
    path: gemstone
    type: symbol
-->

| Key | Roll | Resistance | Gemstone |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` Acid ` | ` Pearl ` |
| ` row_2 ` | ` 2 ` | ` Cold ` | ` Tourmaline ` |
| ` row_3 ` | ` 3 ` | ` Fire ` | ` Garnet ` |
| ` row_4 ` | ` 4 ` | ` Force ` | ` Sapphire ` |
| ` row_5 ` | ` 5 ` | ` Lightning ` | ` Citrine ` |
| ` row_6 ` | ` 6 ` | ` Necrotic ` | ` Jet ` |
| ` row_7 ` | ` 7 ` | ` Poison ` | ` Amethyst ` |
| ` row_8 ` | ` 8 ` | ` Psychic ` | ` Jade ` |
| ` row_9 ` | ` 9 ` | ` Radiant ` | ` Topaz ` |
| ` row_10 ` | ` 10 ` | ` Thunder ` | ` Spinel ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
