---
splat_schema: 1
splat_id: c205b4a5-f3dc-527a-b7d7-38b0ee8afbd8
splat_kind: item
splat_key: dnd2024:items/tome_of_clear_thought
splat_name: Tome of Clear Thought
splat_origin:
  key: dnd2024:reference/magic_item/tome_of_clear_thought
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249
  url: https://www.dndbeyond.com/srd
---

# Tome of Clear Thought

## Source text

<!-- SRD page 249 -->

Wondrous Item, Very Rare

This book contains memory and logic exercises, and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying the book’s contents and practicing its guidelines, your Intelligence increases by 2, to a maximum of 30. The manual then loses its magic but regains it in a century.

Original layout: [[../../srd-5.2.1/pages/249|page 249]].

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
| ` mechanic ` | ` dnd2024:items/tome_of_clear_thought ` |
| ` implementation ` | ` dnd2024:item_tome_of_clear_thought ` |

### Parameters

#### Study

<!-- splat:values path=splat_rule/parameters/study -->

| Field | Value |
| --- | --- |
| ` hours ` | ` 48 ` |
| ` within_days ` | ` 6 ` |
| ` practice_required ` | ` true ` |

#### Completion

<!-- splat:values path=splat_rule/parameters/completion -->

| Field | Value |
| --- | --- |
| ` ability ` | ` intelligence ` |
| ` increase ` | ` 2 ` |
| ` maximum ` | ` 30 ` |
| ` duration ` | ` permanent ` |
| ` item_magical ` | ` false ` |

#### Recovery

<!-- splat:yaml path=splat_rule/parameters/recovery -->

```yaml
after:
  amount: 100
  unit: year
item_magical: true
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
