---
splat_schema: 1
splat_id: 11c332b0-68ac-58d7-a946-daa7134cee08
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage
splat_name: 'Level 6: Mindless Rage'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Level 6: Mindless Rage

## Source text

<!-- SRD page 30 -->

You have Immunity to the Charmed and Frightened conditions while your Rage is active. If you’re Charmed or Frightened when you enter your Rage, the condition ends on you.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage ` |
| ` implementation ` | ` dnd2024:class_features_path_of_the_berserker_level_6_mindless_rage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunities ` | ` ["charmed", "frightened"] ` |
| ` rage_activation_ends_conditions ` | ` ["charmed", "frightened"] ` |

#### While

<!-- splat:values path=splat_rule/parameters/while -->

| Field | Value |
| --- | --- |
| ` rage_active ` | ` true ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 6 ` |
