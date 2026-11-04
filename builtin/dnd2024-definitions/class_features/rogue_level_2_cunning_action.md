---
splat_schema: 2
splat_id: 6ead4dce-099b-5206-83f0-e8fe0deb917e
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_2_cunning_action
splat_name: 'Level 2: Cunning Action'
splat_origin: dnd2024:reference/pages/062
splat_sources:
- SRD 5.2.1, pp. 62, https://www.dndbeyond.com/srd
---

# Level 2: Cunning Action

## Source text

<!-- SRD page 62 -->

Your quick thinking and agility allow you to move and act quickly. On your turn, you can take one of the following actions as a Bonus Action: Dash, Disengage, or Hide.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_2_cunning_action ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_2_cunning_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` turn ` | ` own ` |
| ` action ` | ` bonus_action ` |
| ` choose_action ` | ` ["dash", "disengage", "hide"] ` |
