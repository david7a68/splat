---
splat_schema: 1
splat_id: a01a6417-c815-552f-ae68-dc11eb74e350
splat_kind: item
splat_key: dnd2024:items/manual_of_quickness_of_action
splat_name: Manual of Quickness of Action
splat_origin:
  key: dnd2024:reference/magic_item/manual_of_quickness_of_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 230
  url: https://www.dndbeyond.com/srd
---

# Manual of Quickness of Action

## Source text

<!-- SRD page 230 -->

Wondrous Item, Very Rare\
This book contains coordination and balance ex-\
ercises, and its words are charged with magic.\
If you spend 48 hours over a period of 6 days or\
fewer studying the book’s contents and practicing\
its guidelines, your Dexterity increases by 2, to a\
maximum of 30. The manual then loses its magic\
but regains it in a century.\

Original layout: [[../../srd-5.2.1/pages/230|page 230]].

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
| ` mechanic ` | ` dnd2024:items/manual_of_quickness_of_action ` |
| ` implementation ` | ` dnd2024:item_manual_of_quickness_of_action ` |

### Parameters

#### Study

<!-- splat:yaml path=splat_rule/parameters/study -->

```yaml
hours_required: 48
maximum_elapsed_days: 6
practice_required: true
ability: dexterity
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
