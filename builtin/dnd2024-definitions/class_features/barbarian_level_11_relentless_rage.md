---
splat_schema: 1
splat_id: e4eff469-829a-502d-b0ef-343801956453
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_11_relentless_rage
splat_name: 'Level 11: Relentless Rage'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Level 11: Relentless Rage

## Source text

<!-- SRD page 30 -->

Your Rage can keep you fighting despite grievous\
wounds. If you drop to 0 Hit Points while your Rage\
is active and don’t die outright, you can make a DC\
10 Constitution saving throw. If you succeed, your\
Hit Points instead change to a number equal to\
twice your Barbarian level.\
Each time you use this feature after the first, the\
DC increases by 5. When you finish a Short or Long\
Rest, the DC resets to 10.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_11_relentless_rage ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_11_relentless_rage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` hit_points_would_be ` | ` 0 ` |
| ` rage_active ` | ` true ` |
| ` instant_death ` | ` false ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  op: add
  args:
  - 10
  - op: multiply
    args:
    - 5
    - stat: relentless_rage.uses_since_rest
success:
  replace_hit_points:
    op: multiply
    args:
    - 2
    - stat: classes.barbarian.level
```

#### On use

<!-- splat:values path=splat_rule/parameters/on_use -->

| Field | Value |
| --- | --- |
| ` increment ` | ` relentless_rage.uses_since_rest ` |

#### Reset

<!-- splat:values path=splat_rule/parameters/reset -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["rest.short_rest_completed", "rest.long_rest_completed"] ` |
| ` uses_since_rest ` | ` 0 ` |
