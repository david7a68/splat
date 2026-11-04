---
splat_schema: 2
splat_id: 49386059-2b85-5599-93b5-5dd5bbf41c46
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_dimensional_travel
splat_name: Boon of Dimensional Travel
splat_origin: dnd2024:reference/pages/088
splat_sources:
- SRD 5.2.1, pp. 88, https://www.dndbeyond.com/srd
---

# Boon of Dimensional Travel

## Source text

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)

You gain the following benefits.

Ability Score Increase. Increase one ability score of your choice by 1, to a maximum of 30.

Blink Steps. Immediately after you take the Attack action or the Magic action, you can teleport up to 30 feet to an unoccupied space you can see.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/epic_boon_feats_boon_of_dimensional_travel ` |
| ` implementation ` | ` dnd2024:feats_epic_boon_feats_boon_of_dimensional_travel ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` epic_boon ` |
| ` minimum_character_level ` | ` 19 ` |

#### Ability score increase

<!-- splat:values path=splat_rule/parameters/ability_score_increase -->

| Field | Value |
| --- | --- |
| ` choice ` | ` any_ability ` |
| ` amount ` | ` 1 ` |
| ` maximum ` | ` 30 ` |

#### Blink steps

<!-- splat:yaml path=splat_rule/parameters/blink_steps -->

```yaml
trigger: immediately_after_attack_or_magic_action
optional: true
teleport:
  maximum_feet: 30
  destination_unoccupied: true
  destination_visible: true
```
