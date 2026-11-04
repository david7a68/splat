---
splat_schema: 2
splat_id: 53f5b970-f413-563d-92e5-e8c7759f9df6
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_3_hunter_s_lore
splat_name: 'Level 3: Hunter’s Lore'
splat_origin: dnd2024:reference/pages/061
splat_sources:
- SRD 5.2.1, pp. 61, https://www.dndbeyond.com/srd
---

# Level 3: Hunter’s Lore

## Source text

<!-- SRD page 61 -->

You can call on the forces of nature to reveal certain strengths and weaknesses of your prey. While a creature is marked by your Hunter’s Mark, you know whether that creature has any Immunities, Resistances, or Vulnerabilities, and if the creature has any, you know what they are.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/hunter_level_3_hunter_s_lore ` |
| ` implementation ` | ` dnd2024:class_features_hunter_level_3_hunter_s_lore ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` while_target_marked_by ` | ` own_hunters_mark ` |
| ` reveal_target_properties ` | ` ["immunities", "resistances", "vulnerabilities"] ` |
| ` reveal_absence ` | ` true ` |
