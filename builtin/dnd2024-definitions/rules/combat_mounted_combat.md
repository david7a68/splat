---
splat_schema: 2
splat_id: 8d21c974-b51d-560a-8d82-c9d24c4dd0a8
splat_kind: rule
splat_key: dnd2024:rules/combat_mounted_combat
splat_name: Mounted Combat
splat_origin: dnd2024:reference/pages/015
splat_sources:
- SRD 5.2.1, pp. 15, https://www.dndbeyond.com/srd
---

# Mounted Combat

## Source text

<!-- SRD page 15 -->

A willing creature that is at least one size larger than a rider and that has an appropriate anatomy can serve as a mount, using the following rules.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_mounted_combat ` |
| ` implementation ` | ` dnd2024:rules_combat_mounted_combat ` |

### Parameters

#### Mount eligibility

<!-- splat:yaml path=splat_rule/parameters/mount_eligibility -->

```yaml
willing: true
size_categories_larger_minimum: 1
anatomy_appropriate:
  authority: gm
  type: boolean
```
