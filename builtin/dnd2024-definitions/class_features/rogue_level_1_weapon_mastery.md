---
splat_schema: 1
splat_id: 1f702a92-311b-5016-a30f-02d6db1a137f
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
---

# Level 1: Weapon Mastery

## Source text

<!-- SRD page 62 -->

Your training with weapons allows you to use the\
mastery properties of two kinds of weapons of your\
choice with which you have proficiency, such as\
Daggers and Shortbows.\
Sneak Attack\
1d6\
1d6\
2d6\
2d6\
3d6\
3d6\
4d6\
4d6\
5d6\
5d6\
6d6\
6d6\
7d6\
7d6\
8d6\
8d6\
9d6\
9d6\
10d6\
10d6\
Whenever you finish a Long Rest, you can change\
the kinds of weapons you chose. For example, you\
could switch to using the mastery properties of\
Scimitars and Shortswords.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_1_weapon_mastery ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_1_weapon_mastery ` |

### Parameters

#### Mastery choices

<!-- splat:yaml path=splat_rule/parameters/mastery_choices -->

```yaml
count: 2
eligible:
  actor_proficient: true
replacement:
  event: rest.long_completed
  maximum: 2
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 1 ` |
