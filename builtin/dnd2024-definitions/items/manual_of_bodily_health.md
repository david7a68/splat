---
splat_schema: 2
splat_id: 7778c6e8-9f45-54cb-b8c2-b83cd45b4304
splat_kind: item
splat_key: dnd2024:items/manual_of_bodily_health
splat_name: Manual of Bodily Health
splat_origin: dnd2024:reference/magic_item/manual_of_bodily_health
splat_sources:
- SRD 5.2.1, pp. 229, https://www.dndbeyond.com/srd
---

# Manual of Bodily Health


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | No |


## Source text

<!-- SRD page 229 -->

Wondrous Item, Very Rare

This book contains health and nutrition tips, and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying the book’s contents and practicing its guidelines, your Constitution increases by 2, to a maximum of 30. The manual then loses its magic but regains it in a century.

Original layout: [[../../srd-5.2.1/pages/229|page 229]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/manual_of_bodily_health ` |
| ` implementation ` | ` dnd2024:item_manual_of_bodily_health ` |

### Parameters

#### Study

<!-- splat:yaml path=splat_rule/parameters/study -->

```yaml
hours_required: 48
maximum_elapsed_days: 6
practice_required: true
ability: constitution
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
