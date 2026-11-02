---
splat_schema: 1
splat_id: 6c1c08de-c6cc-5b75-a110-5cd4f15ea2e2
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_7_reliable_talent
splat_name: 'Level 7: Reliable Talent'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
---

# Level 7: Reliable Talent

## Source text

<!-- SRD page 63 -->

Whenever you make an ability check that uses one of your skill or tool proficiencies, you can treat a d20 roll of 9 or lower as a 10.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_7_reliable_talent ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_7_reliable_talent ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional_d20_floor ` | ` 10 ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` test ` | ` ability_check ` |
| ` uses_skill_or_tool_proficiency ` | ` true ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 7 ` |
