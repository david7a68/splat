---
splat_schema: 2
splat_id: f7dc75fc-f4ea-50ec-9325-11612921416a
splat_kind: rule
splat_key: dnd2024:rules/warlock_devouring_blade
splat_name: Devouring Blade
splat_origin: dnd2024:reference/pages/072
splat_sources:
- SRD 5.2.1, pp. 72, https://www.dndbeyond.com/srd
---

# Devouring Blade

## Source text

<!-- SRD page 72 -->

Prerequisite: Level 12+ Warlock, Thirsting Blade Invocation

The Extra Attack of your Thirsting Blade invocation confers two extra attacks rather than one.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_devouring_blade ` |
| ` implementation ` | ` dnd2024:rules_warlock_devouring_blade ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` thirsting_blade_extra_attacks ` | ` 2 ` |

#### Prerequisites

<!-- splat:yaml path=splat_rule/parameters/prerequisites -->

```yaml
warlock_level_min: 12
invocation:
  key: dnd2024:rules/warlock_thirsting_blade
```
