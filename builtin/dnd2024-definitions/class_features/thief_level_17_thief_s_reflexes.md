---
splat_schema: 1
splat_id: 28692370-9533-5df1-a69a-db6fa755fb0b
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_17_thief_s_reflexes
splat_name: 'Level 17: Thief’s Reflexes'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
---

# Level 17: Thief’s Reflexes

## Source text

<!-- SRD page 64 -->

You are adept at laying ambushes and quickly es-\
caping danger. You can take two turns during the\
first round of any combat. You take your first turn\
at your normal Initiative and your second turn at\
your Initiative minus 10.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/thief_level_17_thief_s_reflexes ` |
| ` implementation ` | ` dnd2024:class_features_thief_level_17_thief_s_reflexes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` combat_round ` | ` 1 ` |

#### Turns

<!-- splat:yaml path=splat_rule/parameters/turns -->

```yaml
- initiative: normal
- initiative:
    op: subtract
    args:
    - var: actor.initiative
    - 10
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 17 ` |
