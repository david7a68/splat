---
splat_schema: 1
splat_id: c569bfa9-d21c-5911-a569-55d35d089f17
splat_kind: rule
splat_key: dnd2024:glossary/experience_points
splat_name: Experience Points
splat_origin:
  key: dnd2024:reference/glossary/experience_points
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
---

# Experience Points

## Source text

<!-- SRD page 182 -->

As they overcome challenges and complete adventures, characters earn Experience Points (XP), which are awarded by the Game Master. When a character’s XP total crosses certain thresholds, the character’s level increases. See also “Level Advancement.”

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/experience_points ` |
| ` implementation ` | ` dnd2024:glossary_experience_points ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` accumulation ` | ` add_to_total ` |
| ` level_increase ` | ` level_advancement_threshold_reached ` |

#### Award

<!-- splat:values path=splat_rule/parameters/award -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` amount_type ` | ` integer ` |
| ` triggers ` | ` ["challenge_overcome", "adventure_completed"] ` |
