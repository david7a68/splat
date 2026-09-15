---
splat_schema: 1
splat_id: c726b4ab-3d36-5b41-89f1-a7a19c8ccf2a
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_experience_points
splat_name: Experience Points
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
---

# Experience Points

## Source text

<!-- SRD page 25 -->

The Experience Point cost to gain a level is based on\
your total character level, not your level in a partic-\
ular class, as shown in the Character Advancement\
table in “Character Creation.” For example, if you\
are a level 6 Cleric / level 1 Fighter, you must gain\
enough XP to reach level 8 before you can take your\
second level as a Fighter or your seventh level as a\
Cleric.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_experience_points ` |
| ` implementation ` | ` dnd2024:rules_multiclassing_experience_points ` |

### Parameters

#### Xp threshold level basis

<!-- splat:values path=splat_rule/parameters/xp_threshold_level_basis -->

| Field | Value |
| --- | --- |
| ` op ` | ` sum ` |
| ` terms ` | ` class_levels ` |

#### Advancement

<!-- splat:values path=splat_rule/parameters/advancement -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/level_advancement ` |
