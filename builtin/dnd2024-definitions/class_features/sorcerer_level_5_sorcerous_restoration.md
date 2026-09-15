---
splat_schema: 1
splat_id: 7cf2b7d8-d092-5ede-adbc-77b4ab2cacff
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
splat_name: 'Level 5: Sorcerous Restoration'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
---

# Level 5: Sorcerous Restoration

## Source text

<!-- SRD page 66 -->

When you finish a Short Rest, you can regain ex-\
pended Sorcery Points, but no more than a number\
equal to half your Sorcerer level (round down). Once\
you use this feature, you can’t do so again until you\
finish a Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_5_sorcerous_restoration ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_5_sorcerous_restoration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` rest.short_completed ` |
| ` optional ` | ` true ` |
| ` cost ` | ` 1 ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: sorcerer.sorcerous_restoration
maximum: 1
recovery:
- event: rest.long_completed
  amount: all
```

#### Restore

<!-- splat:yaml path=splat_rule/parameters/restore -->

```yaml
resource: sorcerer.sorcery_points
maximum:
  op: floor
  args:
  - op: divide
    args:
    - var: actor.class_levels.sorcerer
    - 2
expended_only: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` minimum_level ` | ` 5 ` |
