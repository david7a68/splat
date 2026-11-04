---
splat_schema: 2
splat_id: 191c5e14-4406-50f6-bd02-5344a2611871
splat_kind: rule
splat_key: dnd2024:glossary/per_day
splat_name: Per Day
splat_origin: dnd2024:reference/glossary/per_day
splat_sources:
- SRD 5.2.1, pp. 186, https://www.dndbeyond.com/srd
---

# Per Day

## Source text

<!-- SRD page 186 -->

If a rule says you can use something a certain number of times per day, that means you must finish a Long Rest to use it again after you run out of uses.

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/per_day ` |
| ` implementation ` | ` dnd2024:glossary_per_day ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` usage_limit ` | ` feature_specified ` |
| ` reset ` | ` long_rest_completed ` |
| ` exhausted_uses_require_reset ` | ` true ` |
