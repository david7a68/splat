---
splat_schema: 1
splat_id: 8d40a00d-c470-5f4d-8c8a-89b7ba31e9c5
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_skilled
splat_name: Skilled
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
---

# Skilled

## Source text

<!-- SRD page 87 -->

Origin Feat

You gain proficiency in any combination of three skills or tools of your choice.

Repeatable. You can take this feat more than once.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/origin_feats_skilled ` |
| ` implementation ` | ` dnd2024:feats_origin_feats_skilled ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` origin ` |
| ` repeatable ` | ` true ` |

#### Proficiency choices

<!-- splat:values path=splat_rule/parameters/proficiency_choices -->

| Field | Value |
| --- | --- |
| ` count ` | ` 3 ` |
| ` categories ` | ` ["skill", "tool"] ` |
| ` mix_categories ` | ` true ` |
