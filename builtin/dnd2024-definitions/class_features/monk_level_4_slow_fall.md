---
splat_schema: 2
splat_id: 8402faf1-91d2-5af0-83de-a1fa4f3f61a5
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_4_slow_fall
splat_name: 'Level 4: Slow Fall'
splat_origin: dnd2024:reference/pages/051
splat_sources:
- SRD 5.2.1, pp. 51, https://www.dndbeyond.com/srd
---

# Level 4: Slow Fall

## Source text

<!-- SRD page 51 -->

You can take a Reaction when you fall to reduce any damage you take from the fall by an amount equal to five times your Monk level.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_4_slow_fall ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_4_slow_fall ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` self_falls ` |
| ` action ` | ` reaction ` |

#### Fall damage reduction

<!-- splat:yaml path=splat_rule/parameters/fall_damage_reduction -->

```yaml
op: multiply
args:
- 5
- stat: classes.monk.level
```
