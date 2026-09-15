---
splat_schema: 1
splat_id: 3ddfd7cf-91ff-5401-a196-606f53747af4
splat_kind: rule
splat_key: dnd2024:rules/poison_spider_s_sting_200_gp
splat_name: Spider’s Sting (200 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
---

# Spider’s Sting (200 GP)

## Source text

<!-- SRD page 198 -->

Injury Poison\
A creature subjected to Spider’s Sting must succeed\
on a DC 13 Constitution saving throw or have the\
Poisoned condition for 1 hour. If the creature fails\
the save by 5 or more, the creature also has the Un-\
conscious condition while Poisoned in this way. The\
creature wakes up if it takes damage or if another\
creature takes an action to shake it awake.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_spider_s_sting_200_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_spider_s_sting_200_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_gp ` | ` 200 ` |
| ` exposure_route ` | ` injury ` |
| ` unconscious_ends_early ` | ` ["damage_taken", "other_creature_action_shake_awake"] ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 13 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition: poisoned
duration:
  value: 1
  unit: hour
```

#### Failure margin at least 5

<!-- splat:values path=splat_rule/parameters/failure_margin_at_least_5 -->

| Field | Value |
| --- | --- |
| ` condition ` | ` unconscious ` |
| ` duration ` | ` while_poisoned_by_this ` |
