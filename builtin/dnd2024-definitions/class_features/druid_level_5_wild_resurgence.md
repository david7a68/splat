---
splat_schema: 1
splat_id: e101c2c8-5dda-5491-9022-5b6a93455595
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_5_wild_resurgence
splat_name: 'Level 5: Wild Resurgence'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
---

# Level 5: Wild Resurgence

## Source text

<!-- SRD page 43 -->

Once on each of your turns, if you have no uses of Wild Shape left, you can give yourself one use by expending a spell slot (no action required).

In addition, you can expend one use of Wild Shape (no action required) to give yourself a level 1 spell slot, but you can’t do so again until you finish a Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_5_wild_resurgence ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_5_wild_resurgence ` |

### Parameters

#### Conversions

<!-- splat:rows path=splat_rule/parameters/conversions -->

| Row | from | amount | to | gain | requires_remaining_wild_shape | action | slot_level |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` spell_slot ` | ` 1 ` | ` wild_shape_use ` | ` 1 ` | ` 0 ` | ` none ` |  |
| ` 1 ` | ` wild_shape_use ` | ` 1 ` | ` spell_slot ` | ` 1 ` |  | ` none ` | ` 1 ` |

##### Limit

<!-- splat:values path=splat_rule/parameters/conversions/0/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` own_turn ` |

##### Uses

<!-- splat:yaml path=splat_rule/parameters/conversions/1/uses -->

```yaml
maximum: 1
recover:
  event: rest.long_rest_completed
  amount: all
```
