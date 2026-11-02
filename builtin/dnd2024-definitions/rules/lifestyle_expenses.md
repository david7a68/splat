---
splat_schema: 1
splat_id: 143844d5-cccb-5c09-ae59-0074289a55e7
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses
splat_name: Lifestyle Expenses
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
---

# Lifestyle Expenses

## Source text

<!-- SRD page 101 -->

Lifestyle expenses summarize the cost of living in a fantasy world. They cover lodging, food, equipment maintenance, and other necessities.

At the start of each week or month (GM’s choice), choose a lifestyle below—Wretched, Squalid, Poor, Modest, Comfortable, Wealthy, or Aristocratic—and pay the price to sustain that lifestyle.

Lifestyles have no inherent consequences, but the GM might take them into account when determining risks or how others perceive your character.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/lifestyle_expenses ` |
| ` implementation ` | ` dnd2024:rule_lifestyle_expenses ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` selection_at ` | ` interval_start ` |
| ` lifestyles ` | ` ["wretched", "squalid", "poor", "modest", "comfortable", "wealthy", "aristocratic"] ` |
| ` pay ` | ` daily_price_times_interval_days ` |
| ` covers ` | ` ["lodging", "food", "equipment_maintenance", "necessities"] ` |
| ` inherent_consequences ` | ` false ` |
| ` gm_considerations ` | ` ["risks", "social_perception"] ` |

#### Payment interval choice

<!-- splat:values path=splat_rule/parameters/payment_interval_choice -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` options ` | ` ["week", "month"] ` |
