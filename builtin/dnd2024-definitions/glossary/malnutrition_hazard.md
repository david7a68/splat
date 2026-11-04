---
splat_schema: 2
splat_id: 06b355bc-8fc6-593d-b2e3-6d4ac8dc9c70
splat_kind: rule
splat_key: dnd2024:glossary/malnutrition_hazard
splat_name: Malnutrition [Hazard]
splat_origin: dnd2024:reference/glossary/malnutrition_hazard
splat_sources:
- SRD 5.2.1, pp. 185, https://www.dndbeyond.com/srd
---

# Malnutrition [Hazard]

## Source text

<!-- SRD page 185 -->

A creature needs an amount of food per day based on its size, as shown in the Food Needs per Day table. A creature that eats but consumes less than half the required food for a day must succeed on a DC 10 Constitution saving throw or gain 1 Exhaustion level at the day’s end. A creature that eats nothing for 5 days automatically gains 1 Exhaustion level at the end of the fifth day as well as an additional level at the end of each subsequent day without food.

Exhaustion caused by malnutrition can’t be removed until the creature eats the full amount of food required for a day. See also “Exhaustion.”

**Food Needs per Day**

| Size | Food | Size | Food |
| --- | --- | --- | --- |
| Tiny | 1/4 pound | Large | 4 pounds |
| Small | 1 pound | Huge | 16 pounds |
| Medium | 1 pound | Gargantuan | 64 pounds |

Original layout: [[../../srd-5.2.1/pages/185|page 185]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/malnutrition_hazard ` |
| ` implementation ` | ` dnd2024:glossary_malnutrition_hazard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` exhaustion_cause ` | ` malnutrition ` |

#### Daily food pounds

<!-- splat:values path=splat_rule/parameters/daily_food_pounds -->

| Field | Value |
| --- | --- |
| ` tiny ` | ` 0.25 ` |
| ` small ` | ` 1 ` |
| ` medium ` | ` 1 ` |
| ` large ` | ` 4 ` |
| ` huge ` | ` 16 ` |
| ` gargantuan ` | ` 64 ` |

#### Partial food

<!-- splat:yaml path=splat_rule/parameters/partial_food -->

```yaml
predicate:
  greater_than: 0
  less_than_daily_requirement_fraction: 0.5
day_end_save:
  ability: constitution
  dc: 10
  failure_exhaustion_levels: 1
```

#### No food

<!-- splat:values path=splat_rule/parameters/no_food -->

| Field | Value |
| --- | --- |
| ` days_before_first_exhaustion ` | ` 5 ` |
| ` day_5_end_exhaustion_levels ` | ` 1 ` |
| ` each_subsequent_day_end_levels ` | ` 1 ` |
| ` save_allowed ` | ` false ` |

#### Cause exhaustion removal locked until

<!-- splat:values path=splat_rule/parameters/cause_exhaustion_removal_locked_until -->

| Field | Value |
| --- | --- |
| ` food_consumed_pounds_at_least ` | ` full_daily_requirement ` |
