---
splat_schema: 2
splat_id: 1d1221e4-9603-5796-a1b2-0c96e76eb210
splat_kind: item
splat_key: dnd2024:items/tome_of_understanding
splat_name: Tome of Understanding
splat_origin: dnd2024:reference/magic_item/tome_of_understanding
splat_sources:
- SRD 5.2.1, pp. 250, https://www.dndbeyond.com/srd
---

# Tome of Understanding


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | No |


## Source text

<!-- SRD page 250 -->

Wondrous Item, Very Rare

This book contains intuition and insight exercises, and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying the book’s contents and practicing its guidelines, your Wisdom increases by 2, to a maximum of 30. The manual then loses its magic, but regains it in a century.

Original layout: [[../../srd-5.2.1/pages/250|page 250]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/tome_of_understanding ` |
| ` implementation ` | ` dnd2024:item_tome_of_understanding ` |

### Parameters

#### Study

<!-- splat:yaml path=splat_rule/parameters/study -->

```yaml
hours_required: 48
maximum_elapsed_days: 6
practice_required: true
ability: wisdom
increase: 2
maximum_score: 30
after_completion: item_loses_magic
magic_recovery:
  value: 100
  unit: year
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
