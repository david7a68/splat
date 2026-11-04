---
splat_schema: 2
splat_id: 6d670695-2d10-5575-9de8-1b4380a46191
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_10_tireless
splat_name: 'Level 10: Tireless'
splat_origin: dnd2024:reference/pages/059
splat_sources:
- SRD 5.2.1, pp. 59, https://www.dndbeyond.com/srd
---

# Level 10: Tireless

## Source text

<!-- SRD page 59 -->

Primal forces now help fuel you on your journeys, granting you the following benefits.

Temporary Hit Points. As a Magic action, you can give yourself a number of Temporary Hit Points equal to 1d8 plus your Wisdom modifier (minimum of 1). You can use this action a number of times equal to your Wisdom modifier (minimum of once), and you regain all expended uses when you finish a Long Rest.

Decrease Exhaustion. Whenever you finish a Short Rest, your Exhaustion level, if any, decreases by 1.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_10_tireless ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_10_tireless ` |

### Parameters

#### Temporary hit points

<!-- splat:yaml path=splat_rule/parameters/temporary_hit_points -->

```yaml
action: magic
amount:
  op: max
  args:
  - 1
  - op: add
    args:
    - dice:
        count: 1
        sides: 8
    - stat: abilities.wisdom.modifier
uses:
  maximum:
    op: max
    args:
    - 1
    - stat: abilities.wisdom.modifier
  recover:
    event: rest.long_rest_completed
    amount: all
```

#### Short rest

<!-- splat:values path=splat_rule/parameters/short_rest -->

| Field | Value |
| --- | --- |
| ` exhaustion_reduction ` | ` 1 ` |
| ` minimum_exhaustion ` | ` 0 ` |
