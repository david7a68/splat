---
splat_schema: 2
splat_id: c64adbe1-134f-520f-8dbb-a1702feee4d9
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls_time_and_cost
splat_name: Time and Cost
splat_origin: dnd2024:reference/pages/103
splat_sources:
- SRD 5.2.1, pp. 103, https://www.dndbeyond.com/srd
---

# Time and Cost

## Source text

<!-- SRD page 103 -->

Scribing a scroll takes an amount of time and money based on the level of the spell, as shown in the Spell Scroll Costs table. For each day of inscription, you must work for 8 hours. If a scroll requires multiple days, those days needn’t be consecutive.

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

<!-- splat:table
path: splat_rule/parameters/table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
columns:
  Key:
    type: key
  Spell level:
    path: spell_level
    type: integer
  Days:
    path: days
    type: integer
  Cost gp:
    path: cost_gp
    type: integer
-->

| Key | Spell level | Days | Cost gp |
| --- | --- | --- | --- |
| ` row_1 ` | ` 0 ` | ` 1 ` | ` 15 ` |
| ` row_2 ` | ` 1 ` | ` 1 ` | ` 25 ` |
| ` row_3 ` | ` 2 ` | ` 3 ` | ` 100 ` |
| ` row_4 ` | ` 3 ` | ` 5 ` | ` 150 ` |
| ` row_5 ` | ` 4 ` | ` 10 ` | ` 1000 ` |
| ` row_6 ` | ` 5 ` | ` 25 ` | ` 1500 ` |
| ` row_7 ` | ` 6 ` | ` 40 ` | ` 10000 ` |
| ` row_8 ` | ` 7 ` | ` 50 ` | ` 12500 ` |
| ` row_9 ` | ` 8 ` | ` 60 ` | ` 15000 ` |
| ` row_10 ` | ` 9 ` | ` 120 ` | ` 50000 ` |
