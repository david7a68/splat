---
splat_schema: 1
splat_id: 97528665-617f-569a-90be-475da1c599f7
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items_time
splat_name: Time
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
---

# Time

## Source text

<!-- SRD page 103 -->

To determine how many days (working 8 hours a\
day) it takes to make an item, divide its purchase\
cost in GP by 10 (round a fraction up to a day). For\
example, you need 5 days to make a Heavy Cross-\
bow, which sells for 50 GP.\
If an item requires multiple days, the days needn’t\
be consecutive.\
Characters can combine their efforts to shorten\
the crafting time. Divide the time needed to create\
an item by the number of characters working on it.\
Normally, only one other character can assist you,\
but the GM might allow more assistants.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_nonmagical_items_time ` |
| ` implementation ` | ` dnd2024:rules_crafting_nonmagical_items_time ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` work_hours_per_day ` | ` 8 ` |
| ` days_must_be_consecutive ` | ` false ` |

#### Base days

<!-- splat:yaml path=splat_rule/parameters/base_days -->

```yaml
op: ceil
args:
- op: divide
  args:
  - var: item.purchase_cost_gp
  - 10
```

#### Assistance

<!-- splat:values path=splat_rule/parameters/assistance -->

| Field | Value |
| --- | --- |
| ` divide_time_by ` | ` worker_count ` |
| ` default_assistants_max ` | ` 1 ` |
| ` higher_assistant_limit_authority ` | ` gm ` |
