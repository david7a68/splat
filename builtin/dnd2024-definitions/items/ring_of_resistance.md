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

Ring, Rare\
You have Resistance to one damage type while\
wearing this ring. The gemstone in the ring indi-\
cates the type, which the GM chooses or determines\
randomly by rolling on the following table.\
1d10 Damage Type Gemstone\
1 Acid Pearl\
2 Cold Tourmaline\
3 Fire Garnet\
4 Force Sapphire\
5 Lightning Citrine\
6 Necrotic Jet\
7 Poison Amethyst\
8 Psychic Jade\
9 Radiant Topaz\
10 Thunder Spinel\

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

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | roll | resistance | gemstone |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` acid ` | ` pearl ` |
| ` 1 ` | ` 2 ` | ` cold ` | ` tourmaline ` |
| ` 2 ` | ` 3 ` | ` fire ` | ` garnet ` |
| ` 3 ` | ` 4 ` | ` force ` | ` sapphire ` |
| ` 4 ` | ` 5 ` | ` lightning ` | ` citrine ` |
| ` 5 ` | ` 6 ` | ` necrotic ` | ` jet ` |
| ` 6 ` | ` 7 ` | ` poison ` | ` amethyst ` |
| ` 7 ` | ` 8 ` | ` psychic ` | ` jade ` |
| ` 8 ` | ` 9 ` | ` radiant ` | ` topaz ` |
| ` 9 ` | ` 10 ` | ` thunder ` | ` spinel ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
