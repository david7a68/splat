---
splat_schema: 2
splat_id: e2e011ec-40d9-5e52-99eb-e06a89ee9bda
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_poor_2_sp_per_day
splat_name: Poor (2 SP per Day)
splat_origin: dnd2024:reference/pages/101
splat_sources:
- SRD 5.2.1, pp. 101, https://www.dndbeyond.com/srd
---

# Poor (2 SP per Day)

## Source text

<!-- SRD page 101 -->

You spend frugally for your necessities.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/lifestyle_expenses_poor_2_sp_per_day ` |
| ` implementation ` | ` dnd2024:rule_lifestyle_expenses_poor_2_sp_per_day ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` lifestyle ` | ` poor ` |

#### Cost per day

<!-- splat:values path=splat_rule/parameters/cost_per_day -->

| Field | Value |
| --- | --- |
| ` currency ` | ` sp ` |
| ` amount ` | ` 2 ` |
