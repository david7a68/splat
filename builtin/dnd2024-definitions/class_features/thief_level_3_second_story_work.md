---
splat_schema: 2
splat_id: 42c2ba87-b5c4-5a89-bbc9-441fa04b65e9
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_3_second_story_work
splat_name: 'Level 3: Second-Story Work'
splat_origin: dnd2024:reference/pages/064
splat_sources:
- SRD 5.2.1, pp. 64, https://www.dndbeyond.com/srd
---

# Level 3: Second-Story Work

## Source text

<!-- SRD page 64 -->

You’ve trained to get into especially hard-to-reach places, granting you these benefits.

Climber. You gain a Climb Speed equal to your Speed.

Jumper. You can determine your jump distance using your Dexterity rather than your Strength.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/thief_level_3_second_story_work ` |
| ` implementation ` | ` dnd2024:class_features_thief_level_3_second_story_work ` |

### Parameters

#### Climb speed

<!-- splat:values path=splat_rule/parameters/climb_speed -->

| Field | Value |
| --- | --- |
| ` stat ` | ` speed ` |

#### Jump distance ability substitution

<!-- splat:values path=splat_rule/parameters/jump_distance_ability_substitution -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` from ` | ` strength ` |
| ` to ` | ` dexterity ` |
