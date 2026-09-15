---
splat_schema: 1
splat_id: 4f4a42a4-7be8-5409-835f-5cbc741e0753
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_rest_and_recuperation
splat_name: Rest and Recuperation
splat_origin:
  key: dnd2024:reference/pages/194
splat_sources:
- title: SRD 5.2.1
  locator: pp. 194
  url: https://www.dndbeyond.com/srd
---

# Rest and Recuperation

## Source text

<!-- SRD page 194 -->

If a creature infected with a magical contagion\
spends 3 days recuperating—engaging in no\
activities that would interrupt a Long Rest—the\
creature makes a DC 15 Constitution saving throw\
at the end of the recuperation period. On a suc-\
cessful save, the creature has Advantage on saving\
throws to fight off the magical contagion for the\
next 24 hours.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_rest_and_recuperation ` |
| ` implementation ` | ` dnd2024:rules_magical_contagions_rest_and_recuperation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligibility ` | ` infected_with_magical_contagion ` |

#### Recuperation

<!-- splat:values path=splat_rule/parameters/recuperation -->

| Field | Value |
| --- | --- |
| ` days ` | ` 3 ` |
| ` forbid_long_rest_interrupting_activities ` | ` true ` |

#### End save

<!-- splat:values path=splat_rule/parameters/end_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 15 ` |

#### Success

<!-- splat:yaml path=splat_rule/parameters/success -->

```yaml
advantage_scope: saves_against_same_magical_contagion
duration:
  value: 24
  unit: hour
```
