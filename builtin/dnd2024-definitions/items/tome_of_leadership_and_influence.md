---
splat_schema: 1
splat_id: 9aaddf99-9737-5c85-9fb7-14adddc17aa0
splat_kind: item
splat_key: dnd2024:items/tome_of_leadership_and_influence
splat_name: Tome of Leadership and Influence
splat_origin:
  key: dnd2024:reference/magic_item/tome_of_leadership_and_influence
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249, 250
  url: https://www.dndbeyond.com/srd
---

# Tome of Leadership and Influence

## Source text

<!-- SRD page 249 -->

Wondrous Item, Very Rare

This book contains guidelines for influencing and charming others, and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying the book’s contents and practicing its guidelines, your Charisma increases by 2, to a maximum of 30. The manual then loses its magic but regains it in a century.

Original layout: [[../../srd-5.2.1/pages/249|page 249]], [[../../srd-5.2.1/pages/250|page 250]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/tome_of_leadership_and_influence ` |
| ` implementation ` | ` dnd2024:item_tome_of_leadership_and_influence ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` false ` |

#### Study

<!-- splat:values path=splat_rule/parameters/study -->

| Field | Value |
| --- | --- |
| ` total_hours ` | ` 48 ` |
| ` within_days ` | ` 6 ` |
| ` practice_required ` | ` true ` |

#### Completion

<!-- splat:values path=splat_rule/parameters/completion -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` increase ` | ` 2 ` |
| ` maximum ` | ` 30 ` |
| ` item_magic ` | ` false ` |

#### Magic recovery

<!-- splat:values path=splat_rule/parameters/magic_recovery -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 100 ` |
| ` unit ` | ` years ` |
