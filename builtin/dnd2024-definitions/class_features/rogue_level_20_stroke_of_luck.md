---
splat_schema: 1
splat_id: 89fe3b83-8941-5c4c-8f78-9bc0f6ff966f
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_20_stroke_of_luck
splat_name: 'Level 20: Stroke of Luck'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
---

# Level 20: Stroke of Luck

## Source text

<!-- SRD page 63 -->

You have a marvelous knack for succeeding when you need to. If you fail a D20 Test, you can turn the roll into a 20.

Once you use this feature, you can’t use it again until you finish a Short or Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_20_stroke_of_luck ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_20_stroke_of_luck ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.d20_test.failed ` |
| ` optional ` | ` true ` |
| ` replace_d20 ` | ` 20 ` |
| ` recompute_result ` | ` true ` |
| ` cost ` | ` 1 ` |

#### Resource

<!-- splat:values path=splat_rule/parameters/resource -->

| Field | Value |
| --- | --- |
| ` id ` | ` rogue.stroke_of_luck ` |
| ` maximum ` | ` 1 ` |

##### Recovery

<!-- splat:table
path: splat_rule/parameters/resource/recovery
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Amount:
    path: amount
    type: symbol
-->

| Key | Event | Amount |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_completed ` | ` All ` |
| ` row_2 ` | ` rest.long_completed ` | ` All ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 20 ` |
