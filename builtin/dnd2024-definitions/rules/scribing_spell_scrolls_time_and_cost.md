---
splat_schema: 1
splat_id: c64adbe1-134f-520f-8dbb-a1702feee4d9
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls_time_and_cost
splat_name: Time and Cost
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
---

# Time and Cost

## Source text

<!-- SRD page 103 -->

Scribing a scroll takes an amount of time and money\
based on the level of the spell, as shown in the Spell\
Scroll Costs table. For each day of inscription, you\
must work for 8 hours. If a scroll requires multiple\
days, those days needn’t be consecutive.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/scribing_spell_scrolls_time_and_cost ` |
| ` implementation ` | ` dnd2024:rules_scribing_spell_scrolls_time_and_cost ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` work_hours_per_day ` | ` 8 ` |
| ` days_must_be_consecutive ` | ` false ` |

#### Table

<!-- splat:rows path=splat_rule/parameters/table -->

| Row | spell_level | days | cost_gp |
| --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` 1 ` | ` 15 ` |
| ` 1 ` | ` 1 ` | ` 1 ` | ` 25 ` |
| ` 2 ` | ` 2 ` | ` 3 ` | ` 100 ` |
| ` 3 ` | ` 3 ` | ` 5 ` | ` 150 ` |
| ` 4 ` | ` 4 ` | ` 10 ` | ` 1000 ` |
| ` 5 ` | ` 5 ` | ` 25 ` | ` 1500 ` |
| ` 6 ` | ` 6 ` | ` 40 ` | ` 10000 ` |
| ` 7 ` | ` 7 ` | ` 50 ` | ` 12500 ` |
| ` 8 ` | ` 8 ` | ` 60 ` | ` 15000 ` |
| ` 9 ` | ` 9 ` | ` 120 ` | ` 50000 ` |
