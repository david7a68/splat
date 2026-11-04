---
splat_schema: 2
splat_id: 1c902810-86c2-5b9c-a873-5706d7ebec05
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_7_countercharm
splat_name: 'Level 7: Countercharm'
splat_origin: dnd2024:reference/pages/033
splat_sources:
- SRD 5.2.1, pp. 33, https://www.dndbeyond.com/srd
---

# Level 7: Countercharm

## Source text

<!-- SRD page 33 -->

You can use musical notes or words of power to disrupt mind-influencing effects. If you or a creature within 30 feet of you fails a saving throw against an effect that applies the Charmed or Frightened condition, you can take a Reaction to cause the save to be rerolled, and the new roll has Advantage.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_7_countercharm ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_7_countercharm ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` reaction ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` saving_throw ` | ` failed ` |
| ` effect_applies_any_condition ` | ` ["charmed", "frightened"] ` |
| ` target ` | ` self_or_creature ` |
| ` target_range_feet ` | ` 30 ` |

#### Reroll

<!-- splat:values path=splat_rule/parameters/reroll -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |
