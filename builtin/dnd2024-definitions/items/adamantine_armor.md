---
splat_schema: 1
splat_id: 40eb362b-1778-5ef8-a9ca-62e90cc1c81f
splat_kind: item
splat_key: dnd2024:items/adamantine_armor
splat_name: Adamantine Armor
splat_origin:
  key: dnd2024:reference/magic_item/adamantine_armor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
---

# Adamantine Armor

## Source text

<!-- SRD page 209 -->

Armor (Any Medium or Heavy, Except Hide Armor),\
Uncommon\
This suit of armor is reinforced with adamantine,\
one of the hardest substances in existence. While\
you’re wearing it, any Critical Hit against you be-\
comes a normal hit.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
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
| ` mechanic ` | ` dnd2024:items/adamantine_armor ` |
| ` implementation ` | ` dnd2024:item_adamantine_armor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Critical hits received

<!-- splat:values path=splat_rule/parameters/critical_hits_received -->

| Field | Value |
| --- | --- |
| ` replace ` | ` normal_hit ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
