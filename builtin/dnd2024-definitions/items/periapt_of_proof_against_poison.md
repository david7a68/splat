---
splat_schema: 1
splat_id: 09fe0a5d-af15-5b04-8c16-627584d991bf
splat_kind: item
splat_key: dnd2024:items/periapt_of_proof_against_poison
splat_name: Periapt of Proof against Poison
splat_origin:
  key: dnd2024:reference/magic_item/periapt_of_proof_against_poison
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
---

# Periapt of Proof against Poison

## Source text

<!-- SRD page 234 -->

Wondrous Item, Rare (Requires Attunement)

This delicate silver chain has a brilliant-cut black gem pendant. While you wear it, you have Immunity to the Poisoned condition and Poison damage.

Original layout: [[../../srd-5.2.1/pages/234|page 234]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/periapt_of_proof_against_poison ` |
| ` implementation ` | ` dnd2024:item_periapt_of_proof_against_poison ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` damage_immunities ` | ` ["poison"] ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
