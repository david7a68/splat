---
splat_schema: 1
splat_id: 33a773d0-815d-50c6-b92f-fff4b138a5ce
splat_kind: rule
splat_key: dnd2024:rules/poison_malice_250_gp
splat_name: Malice (250 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
---

# Malice (250 GP)

## Source text

<!-- SRD page 197 -->

Inhaled Poison

A creature subjected to Malice must succeed on a DC 15 Constitution saving throw or have the Poisoned condition for 1 hour. The creature also has the Blinded condition while Poisoned in this way.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_malice_250_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_malice_250_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_gp ` | ` 250 ` |
| ` exposure_route ` | ` inhaled ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 15 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition: poisoned
duration:
  value: 1
  unit: hour
```

#### While poisoned by this

<!-- splat:values path=splat_rule/parameters/while_poisoned_by_this -->

| Field | Value |
| --- | --- |
| ` condition ` | ` blinded ` |
