---
splat_schema: 1
splat_id: 56b40ce8-a49c-51f7-b098-238f909165d5
splat_kind: rule
splat_key: dnd2024:rules/exploration_carrying_objects
splat_name: Carrying Objects
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# Carrying Objects

## Source text

<!-- SRD page 12 -->

You can usually carry your gear and treasure without worrying about the weight of those objects. If you try to haul an unusually heavy object or a massive number of lighter objects, the GM might require you to abide by the rules for carrying capacity in “Rules Glossary.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_carrying_objects ` |
| ` implementation ` | ` dnd2024:rules_exploration_carrying_objects ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ordinary_gear_weight_tracking ` | ` optional ` |

#### Exceptional load

<!-- splat:yaml path=splat_rule/parameters/exceptional_load -->

```yaml
triggers:
- unusually_heavy_object
- massive_object_count
authority: gm
enforce:
  key: dnd2024:glossary/carrying_capacity
```
