---
splat_schema: 1
splat_id: 01599a29-8e25-5f64-8700-c40a986f7395
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_2_uncanny_metabolism
splat_name: 'Level 2: Uncanny Metabolism'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 2: Uncanny Metabolism

## Source text

<!-- SRD page 51 -->

When you roll Initiative, you can regain all ex-\
pended Focus Points. When you do so, roll your\
Martial Arts die, and regain a number of Hit Points\
equal to your Monk level plus the number rolled.\
Once you use this feature, you can’t use it again\
until you finish a Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_2_uncanny_metabolism ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_2_uncanny_metabolism ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |

#### Recover

<!-- splat:values path=splat_rule/parameters/recover -->

| Field | Value |
| --- | --- |
| ` resource ` | ` focus_points ` |
| ` amount ` | ` all_expended ` |

#### Heal self

<!-- splat:yaml path=splat_rule/parameters/heal_self -->

```yaml
op: add
args:
- stat: classes.monk.level
- dice:
    count: 1
    sides:
      stat: martial_arts_die.sides
```

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 1
recover:
  event: rest.long_rest_completed
  amount: all
```
