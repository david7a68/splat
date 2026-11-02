---
splat_schema: 1
splat_id: be8a502a-255c-5496-9d4b-2175967d22b3
splat_kind: item
splat_key: dnd2024:items/manual_of_gainful_exercise
splat_name: Manual of Gainful Exercise
splat_origin:
  key: dnd2024:reference/magic_item/manual_of_gainful_exercise
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
---

# Manual of Gainful Exercise

## Source text

<!-- SRD page 229 -->

Wondrous Item, Very Rare

This book describes fitness exercises, and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying the book’s contents and practicing its guidelines, your Strength increases by 2, to a maximum of 30. The manual then loses its magic but regains it in a century.

Original layout: [[../../srd-5.2.1/pages/229|page 229]].

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
| ` mechanic ` | ` dnd2024:items/manual_of_gainful_exercise ` |
| ` implementation ` | ` dnd2024:item_manual_of_gainful_exercise ` |

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
| ` ability ` | ` strength ` |
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
