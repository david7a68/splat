---
splat_schema: 1
splat_id: 49100522-211c-5116-959b-d6cbdc807828
splat_kind: rule
splat_key: dnd2024:rules/poison_truth_serum_150_gp
splat_name: Truth Serum (150 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
---

# Truth Serum (150 GP)

## Source text

<!-- SRD page 198 -->

Ingested Poison\
A creature subjected to Truth Serum must succeed\
on a DC 11 Constitution saving throw or have the\
Poisoned condition for 1 hour. The Poisoned crea-\
ture can’t knowingly communicate a lie.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_truth_serum_150_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_truth_serum_150_gp ` |

### Parameters

#### Poison

<!-- splat:values path=splat_rule/parameters/poison -->

| Field | Value |
| --- | --- |
| ` type ` | ` ingested ` |
| ` cost_gp ` | ` 150 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 11 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition:
  id: poisoned
  duration:
    value: 1
    unit: hour
while_poisoned:
  knowingly_communicate_lie: false
```
