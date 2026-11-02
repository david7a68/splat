---
splat_schema: 1
splat_id: a7ccef37-0c61-5edd-9b61-3852277811a9
splat_kind: rule
splat_key: dnd2024:rules/poison_wyvern_poison_1_200_gp
splat_name: Wyvern Poison (1,200 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
---

# Wyvern Poison (1,200 GP)

## Source text

<!-- SRD page 198 -->

Injury Poison

A creature subjected to Wyvern Poison makes a DC 14 Constitution saving throw, taking 24 (7d6) Poison damage on a failed save or half as much damage on a successful one.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_wyvern_poison_1_200_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_wyvern_poison_1_200_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_gp ` | ` 1200 ` |
| ` exposure_route ` | ` injury ` |
| ` success_damage ` | ` half_round_down ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 14 ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
dice:
  count: 7
  sides: 6
average: 24
type: poison
```
