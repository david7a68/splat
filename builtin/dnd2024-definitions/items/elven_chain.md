---
splat_schema: 1
splat_id: 2b2754cf-08c3-58ac-bb4c-ab067f7cd300
splat_kind: item
splat_key: dnd2024:items/elven_chain
splat_name: Elven Chain
splat_origin:
  key: dnd2024:reference/magic_item/elven_chain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
---

# Elven Chain

## Source text

<!-- SRD page 220 -->

Armor (Chain Mail or Chain Shirt), Rare

You gain a +1 bonus to Armor Class while you wear this armor. You are considered trained with this armor even if you lack training with Medium or Heavy armor.

Original layout: [[../../srd-5.2.1/pages/220|page 220]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
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
| ` mechanic ` | ` dnd2024:items/elven_chain ` |
| ` implementation ` | ` dnd2024:item_elven_chain ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` ac_bonus ` | ` 1 ` |
| ` attunement_required ` | ` false ` |

#### Armor training

<!-- splat:values path=splat_rule/parameters/armor_training -->

| Field | Value |
| --- | --- |
| ` applies_to ` | ` this_item ` |
| ` granted ` | ` true ` |
| ` requires_prior_training ` | ` false ` |
